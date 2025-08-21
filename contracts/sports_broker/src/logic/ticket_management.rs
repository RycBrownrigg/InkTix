use ink::prelude::*;
use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Ticket management logic
pub struct TicketManagement;

impl TicketManagement {
    /// Purchase a ticket for a sports event
    pub fn purchase_ticket(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        buyer: AccountId,
        seat_number: u32,
        section: String,
        row: String,
        seat_type: SeatType,
        access_level: AccessLevel,
        currency: CurrencyId,
        payment_amount: u128,
    ) -> Result<Vec<u64>, String> {
        // Validate event exists and has capacity
        let event = storage.events.get(event_id)
            .ok_or("Event not found")?;
        
        if !event.active {
            return Err("Event is not active".to_string());
        }
        
        if event.sold_tickets >= event.capacity {
            return Err("Event is sold out".to_string());
        }
        
        // Validate payment amount
        let ticket_price = Self::calculate_ticket_price(storage, event_id, seat_type, access_level);
        if payment_amount < ticket_price {
            return Err("Insufficient payment amount".to_string());
        }
        
        // Create ticket
        let ticket_id = storage.get_next_ticket_id();
        let ticket = SportsTicket {
            id: ticket_id,
            event_id,
            owner: buyer,
            purchase_price: ticket_price,
            purchase_currency: currency,
            purchase_date: 0, // Will be set by caller
            seat_number,
            transferable: true,
            section,
            row,
            seat_type,
            access_level,
            loyalty_points_earned: Self::calculate_loyalty_points(ticket_price),
            season_pass_discount_applied: false,
            is_season_pass_ticket: false,
            dynamic_price_paid: ticket_price,
            performance_multiplier_applied: 1,
            dot_equivalent_paid: Self::convert_to_dot_equivalent(storage, ticket_price, currency),
        };
        
        // Store ticket
        storage.tickets.insert(ticket_id, &ticket);
        
        // Update event sold tickets count
        if let Some(mut event) = storage.events.get(event_id) {
            event.sold_tickets += 1;
            event.revenue_generated += ticket_price;
            storage.events.insert(event_id, &event);
        }
        
        // Update user tickets mapping
        let mut user_tickets = storage.user_tickets.get(buyer).unwrap_or(Vec::new());
        user_tickets.push(ticket_id);
        storage.user_tickets.insert(buyer, &user_tickets);
        
        // Update currency revenue
        let current_revenue = storage.currency_revenue.get(currency).unwrap_or(0);
        storage.currency_revenue.insert(currency, &(current_revenue + ticket_price));
        
        // Update user profile if exists
        if let Some(mut profile) = storage.loyalty_profiles.get(buyer) {
            profile.total_tickets_purchased += 1;
            profile.total_spent += ticket_price;
            profile.last_activity = 0; // Will be set by caller
            storage.loyalty_profiles.insert(buyer, &profile);
        }
        
        Ok(vec![ticket_id])
    }
    
    /// Transfer a ticket to another user
    pub fn transfer_ticket(
        storage: &mut SportsBrokerStorage,
        ticket_id: u64,
        from: AccountId,
        to: AccountId,
        reason: String,
    ) -> Result<(), String> {
        let mut ticket = storage.tickets.get(ticket_id)
            .ok_or("Ticket not found")?;
        
        if ticket.owner != from {
            return Err("Not the ticket owner".to_string());
        }
        
        if !ticket.transferable {
            return Err("Ticket is not transferable".to_string());
        }
        
        // Check if recipient is blacklisted
        if let Some(profile) = storage.user_behavior_profiles.get(to) {
            if profile.blacklist_status == BlacklistStatus::Banned {
                return Err("Recipient is banned from receiving tickets".to_string());
            }
        }
        
        // Update ticket ownership
        ticket.owner = to;
        storage.tickets.insert(ticket_id, &ticket);
        
        // Update user tickets mappings
        let mut from_tickets = storage.user_tickets.get(from).unwrap_or(Vec::new());
        from_tickets.retain(|&id| id != ticket_id);
        storage.user_tickets.insert(from, &from_tickets);
        
        let mut to_tickets = storage.user_tickets.get(to).unwrap_or(Vec::new());
        to_tickets.push(ticket_id);
        storage.user_tickets.insert(to, &to_tickets);
        
        // Record transfer history
        let transfer_id = storage.get_next_id("transfer");
        let transfer = TicketTransferHistory {
            ticket_id,
            original_owner: from,
            current_owner: to,
            transfer_count: 1,
            first_transfer_time: 0, // Will be set by caller
            last_transfer_time: 0, // Will be set by caller
            transfer_reasons: vec![TransferReason::Gift],
            price_history: vec![],
        };
        
        storage.ticket_transfer_history.insert(transfer_id as u64, &transfer);
        
        // Update user profiles
        if let Some(mut profile) = storage.loyalty_profiles.get(from) {
            profile.total_tickets_purchased = profile.total_tickets_purchased.saturating_sub(1);
            profile.last_activity = 0; // Will be set by caller
            storage.loyalty_profiles.insert(from, &profile);
        }
        
        if let Some(mut profile) = storage.loyalty_profiles.get(to) {
            profile.total_tickets_purchased += 1;
            profile.last_activity = 0; // Will be set by caller
            storage.loyalty_profiles.insert(to, &profile);
        }
        
        Ok(())
    }
    
    /// Get ticket by ID
    pub fn get_ticket(storage: &SportsBrokerStorage, ticket_id: u64) -> Option<SportsTicket> {
        storage.tickets.get(ticket_id)
    }
    
    /// Get all tickets owned by a user
    pub fn get_tickets_by_owner(storage: &SportsBrokerStorage, owner: AccountId) -> Vec<SportsTicket> {
        if let Some(ticket_ids) = storage.user_tickets.get(owner) {
            ticket_ids.iter()
                .filter_map(|&id| storage.tickets.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get all tickets for a specific event
    pub fn get_tickets_by_event(storage: &SportsBrokerStorage, event_id: u32) -> Vec<SportsTicket> {
        let mut tickets = Vec::new();
        for ticket_id in 1..=storage.total_tickets {
            if let Some(ticket) = storage.tickets.get(ticket_id) {
                if ticket.event_id == event_id {
                    tickets.push(ticket);
                }
            }
        }
        tickets
    }
    
    /// Set resale price for a ticket
    pub fn set_resale_price(
        storage: &mut SportsBrokerStorage,
        ticket_id: u64,
        owner: AccountId,
        resale_price: u128,
    ) -> Result<(), String> {
        let ticket = storage.tickets.get(ticket_id)
            .ok_or("Ticket not found")?;
        
        if ticket.owner != owner {
            return Err("Not the ticket owner".to_string());
        }
        
        // Create a resale listing
        let listing_id = storage.get_next_id("resale_listing") as u64;
        let listing = ResaleListing {
            listing_id,
            ticket_id,
            seller: owner,
            asking_price: resale_price,
            original_price: ticket.purchase_price,
            listing_time: 0, // Will be set by caller
            expiry_time: 0, // Will be set by caller
            is_active: true,
            approved: false,
        };
        
        storage.resale_listings.insert(listing_id, &listing);
        
        Ok(())
    }
    
    /// Purchase a ticket from resale
    pub fn purchase_resale_ticket(
        storage: &mut SportsBrokerStorage,
        ticket_id: u64,
        buyer: AccountId,
        payment_amount: u128,
    ) -> Result<(), String> {
        // Find the resale listing
        let mut found_listing = None;
        for listing_id in 1..=1000 { // Arbitrary limit, should be tracked in storage
            if let Some(listing) = storage.resale_listings.get(listing_id) {
                if listing.ticket_id == ticket_id {
                    found_listing = Some((listing_id, listing));
                    break;
                }
            }
        }
        
        let (listing_id, listing) = found_listing
            .ok_or("Resale listing not found")?;
        
        if !listing.is_active {
            return Err("Resale listing is not active".to_string());
        }
        
        if payment_amount < listing.asking_price {
            return Err("Insufficient payment amount".to_string());
        }
        
        // Transfer ticket ownership
        Self::transfer_ticket(
            storage,
            ticket_id,
            listing.seller,
            buyer,
            "Resale purchase".to_string(),
        )?;
        
        // Remove resale listing
        storage.resale_listings.remove(listing_id);
        
        Ok(())
    }
    
    // Helper methods
    fn calculate_ticket_price(
        storage: &SportsBrokerStorage,
        event_id: u32,
        seat_type: SeatType,
        access_level: AccessLevel,
    ) -> u128 {
        let base_price = if let Some(event) = storage.events.get(event_id) {
            event.base_price
        } else {
            return 0;
        };
        
        let seat_multiplier = match seat_type {
            SeatType::GeneralAdmission => 1,
            SeatType::Reserved => 2,
            SeatType::PremiumReserved => 3,
            SeatType::Club => 5,
            SeatType::Suite => 8,
            SeatType::FieldLevel => 10,
            SeatType::Courtside => 12,
            SeatType::StudentSection => 1,
        };
        
        let access_multiplier = match access_level {
            AccessLevel::Standard => 1,
            AccessLevel::Premium => 2,
            AccessLevel::VIP => 3,
            AccessLevel::AllAccess => 4,
        };
        
        base_price * seat_multiplier as u128 * access_multiplier as u128
    }
    
    fn calculate_loyalty_points(ticket_price: u128) -> u32 {
        // 1 point per 1000 units of currency
        (ticket_price / 1000) as u32
    }
    
    fn convert_to_dot_equivalent(
        storage: &SportsBrokerStorage,
        amount: u128,
        currency: CurrencyId,
    ) -> u128 {
        if let Some(rate) = storage.currency_rates.get(currency) {
            amount * rate
        } else {
            amount // Fallback to original amount
        }
    }
    
    fn update_user_ticket_count(
        storage: &mut SportsBrokerStorage,
        user: AccountId,
        increment: bool,
    ) {
        if let Some(mut profile) = storage.loyalty_profiles.get(user) {
            if increment {
                profile.total_tickets_purchased += 1;
            } else {
                profile.total_tickets_purchased = profile.total_tickets_purchased.saturating_sub(1);
            }
            profile.last_activity = 0; // Will be set by caller
            storage.loyalty_profiles.insert(user, &profile);
        }
    }
}
