//! Ticket purchase, transfer, and resale logic.
//!
//! Manages the full ticket lifecycle including dynamic price calculation at purchase,
//! anti-scalping enforcement for concert events, ownership transfers, and resale listings.
//!
//! # Functions
//! - `purchase_ticket` -- buys a ticket with dynamic pricing and anti-scalping checks
//! - `transfer_ticket` -- transfers ticket ownership between accounts
//! - `resell_ticket` -- lists a ticket on the resale marketplace

use crate::storage::*;
use crate::types::*;
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]

/// Ticket management functionality
pub struct TicketManagement;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl TicketManagement {
    /// Purchase a ticket for an event
    pub fn purchase_ticket(
        storage: &mut InkTixStorage,
        buyer: AccountId,
        event_id: u32,
        seat: Seat,
        currency: CurrencyId,
    ) -> Result<u64, String> {
        let event = storage.events.get(event_id).ok_or("Event not found")?;
        if !event.active { return Err("Event is not active".to_string()); }

        // Check anti-scalping per-event purchase count for concert events
        if let EventCategory::Concert { .. } = &event.category {
            let key = (event_id, buyer);
            let count = storage.per_event_purchase_count.get(key).unwrap_or(0);
            // Check anti-scalping config, default to 4 for concerts
            let max_tickets = storage.anti_scalping_configs.get(event_id)
                .map(|c| c.max_tickets_per_user)
                .unwrap_or(4);
            if count >= max_tickets {
                return Err("Purchase limit reached".to_string());
            }
            storage.per_event_purchase_count.insert(key, &(count + 1));
        }

        // Calculate dynamic price
        let (dynamic_price, multiplier) = super::pricing::DynamicPricing::calculate_price(
            storage, event_id, &seat, false
        )?;

        let ticket_id = storage.get_next_ticket_id();
        let ticket = Ticket {
            id: ticket_id,
            event_id,
            owner: buyer,
            purchase_price: dynamic_price,
            purchase_currency: currency,
            purchase_date: ink::env::block_timestamp::<DefaultEnvironment>(),
            seat_number: 1,
            transferable: true,
            section: seat.section,
            row: seat.row,
            seat_type: seat.seat_type,
            access_level: seat.access_level,
            loyalty_points_earned: Self::calculate_loyalty_points(dynamic_price),
            season_pass_discount_applied: false,
            is_season_pass_ticket: false,
            dynamic_price_paid: dynamic_price,
            performance_multiplier_applied: multiplier,
            dot_equivalent_paid: dynamic_price,
        };

        storage.tickets.insert(ticket_id, &ticket);

        // Update user tickets
        let mut user_tickets = storage.user_tickets.get(buyer).unwrap_or_default();
        user_tickets.push(ticket_id);
        storage.user_tickets.insert(buyer, &user_tickets);

        // Update event analytics
        if let Some(analytics) = storage.event_analytics.get(event_id) {
            let mut updated_analytics = analytics;
            updated_analytics.tickets_sold += 1;
            updated_analytics.revenue_generated += dynamic_price;
            storage.event_analytics.insert(event_id, &updated_analytics);
        }

        // Update event sold tickets
        let mut updated_event = event;
        updated_event.sold_tickets += 1;
        updated_event.revenue_generated += dynamic_price;
        storage.events.insert(event_id, &updated_event);

        storage.platform_stats.total_tickets_sold += 1;
        storage.platform_stats.total_revenue += dynamic_price;

        Ok(ticket_id)
    }

    /// Transfer ticket to another user
    pub fn transfer_ticket(
        storage: &mut InkTixStorage,
        caller: AccountId,
        ticket_id: u64,
        to: AccountId,
    ) -> Result<(), String> {
        let mut ticket = storage.tickets.get(ticket_id).ok_or("Ticket not found")?;
        if ticket.owner != caller { return Err("Only ticket owner can transfer".to_string()); }
        if !ticket.transferable { return Err("Ticket is not transferable".to_string()); }

        ticket.owner = to;
        storage.tickets.insert(ticket_id, &ticket);

        let mut from_tickets = storage.user_tickets.get(&caller).unwrap_or_default();
        from_tickets.retain(|&id| id != ticket_id);
        storage.user_tickets.insert(&caller, &from_tickets);

        let mut to_tickets = storage.user_tickets.get(&to).unwrap_or_default();
        to_tickets.push(ticket_id);
        storage.user_tickets.insert(&to, &to_tickets);

        Ok(())
    }

    /// Resell ticket
    pub fn resell_ticket(
        storage: &mut InkTixStorage,
        caller: AccountId,
        ticket_id: u64,
        price: u128,
        _currency: CurrencyId,
    ) -> Result<(), String> {
        let ticket = storage.tickets.get(ticket_id).ok_or("Ticket not found")?;
        if ticket.owner != caller { return Err("Only ticket owner can resell".to_string()); }
        if !ticket.transferable { return Err("Ticket is not transferable".to_string()); }

        let resale_id = storage.get_next_id("resale");
        let resale = ResaleListing {
            listing_id: resale_id as u64,
            ticket_id,
            seller: caller,
            asking_price: price,
            original_price: ticket.purchase_price,
            listing_time: ink::env::block_timestamp::<DefaultEnvironment>(),
            expiry_time: ink::env::block_timestamp::<DefaultEnvironment>() + 86400,
            is_active: true,
            approved: false,
        };
        storage.resale_listings.insert(resale_id as u64, &resale);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn calculate_loyalty_points(price: u128) -> u32 {
        (price / 1_000_000_000_000_000) as u32
    }
}
