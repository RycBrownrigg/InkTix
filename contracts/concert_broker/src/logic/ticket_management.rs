use crate::types::{ConcertTicket, SeatType, AccessLevel, CurrencyId, InkTixError, InkTixResult};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Ticket management business logic
pub struct TicketManager {
    pub concert_tickets: Mapping<u64, ConcertTicket>,
    pub next_concert_ticket_id: u64,
    pub user_concert_tickets: Mapping<AccountId, Vec<u64>>,
    pub user_purchase_limits: Mapping<(AccountId, u32), u32>,
    pub event_purchase_limits: Mapping<u32, u32>,
}

impl TicketManager {
    pub fn new() -> Self {
        Self {
            concert_tickets: Mapping::new(),
            next_concert_ticket_id: 1,
            user_concert_tickets: Mapping::new(),
            user_purchase_limits: Mapping::new(),
            event_purchase_limits: Mapping::new(),
        }
    }

    /// Purchase concert ticket with multi-currency support
    pub fn purchase_concert_ticket(
        &mut self,
        event_id: u32,
        buyer: AccountId,
        seat_section: String,
        seat_row: String,
        seat_type: SeatType,
        currency: CurrencyId,
        payment: u128,
        base_price: u128,
    ) -> InkTixResult<u64> {
        // Check purchase limits
        self.check_purchase_limits(buyer, event_id)?;

        // Calculate final price based on seat type
        let final_price = self.calculate_seat_price(base_price, &seat_type);
        
        if payment < final_price {
            return Err(InkTixError::InsufficientPayment);
        }

        let ticket_id = self.next_concert_ticket_id;
        self.next_concert_ticket_id = self.next_concert_ticket_id
            .checked_add(1)
            .ok_or(InkTixError::IdOverflow)?;

        let seat_number = self.get_next_seat_number(event_id)?;

        // Generate QR code
        let qr_code = format!("INKTIX-{}-{}-{}", event_id, ticket_id, seat_number);

        let concert_ticket = ConcertTicket {
            id: ticket_id,
            event_id,
            owner: buyer,
            purchase_price: payment,
            purchase_currency: currency,
            purchase_date: ink::env::block_timestamp(),
            seat_section,
            seat_row,
            seat_number,
            seat_type,
            access_level: self.determine_access_level(&seat_type, false),
            transferable: true,
            vip_package_id: None,
            merchandise_bundle: Vec::new(),
            fan_token_discount_applied: false,
            loyalty_points_earned: self.calculate_loyalty_points(&seat_type, final_price),
            special_access: Vec::new(),
            qr_code,
            resale_allowed: true,
            resale_price_limit: Some(final_price.saturating_mul(150) / 100),
            artist_revenue_share: final_price / 20,
            dynamic_price_paid: final_price,
            dot_equivalent_paid: final_price, // Simplified for now
            verified_fan_purchase: false,
            created_at: ink::env::block_timestamp(),
            last_updated: ink::env::block_timestamp(),
        };

        // Store ticket and update indexes
        self.concert_tickets.insert(ticket_id, &concert_ticket);

        let mut user_tickets = self.user_concert_tickets.get(buyer).unwrap_or_default();
        user_tickets.push(ticket_id);
        self.user_concert_tickets.insert(buyer, &user_tickets);

        // Update purchase limits
        self.update_purchase_limits(buyer, event_id);

        Ok(ticket_id)
    }

    /// Transfer concert ticket to another user
    pub fn transfer_concert_ticket(&mut self, ticket_id: u64, from: AccountId, to: AccountId) -> InkTixResult<()> {
        let mut ticket = self.concert_tickets.get(ticket_id).ok_or(InkTixError::NotFound)?;

        if ticket.owner != from {
            return Err(InkTixError::NotOwner);
        }

        if !ticket.transferable {
            return Err(InkTixError::InvalidData);
        }

        // Update ticket owner
        let old_owner = ticket.owner;
        ticket.owner = to;
        ticket.last_updated = ink::env::block_timestamp();
        self.concert_tickets.insert(ticket_id, &ticket);

        // Update user ticket lists
        if let Some(mut old_tickets) = self.user_concert_tickets.get(old_owner) {
            old_tickets.retain(|&x| x != ticket_id);
            self.user_concert_tickets.insert(old_owner, &old_tickets);
        }

        let mut new_tickets = self.user_concert_tickets.get(to).unwrap_or_default();
        new_tickets.push(ticket_id);
        self.user_concert_tickets.insert(to, &new_tickets);

        Ok(())
    }

    /// Get concert ticket details
    pub fn get_concert_ticket(&self, ticket_id: u64) -> Option<ConcertTicket> {
        self.concert_tickets.get(ticket_id)
    }

    /// Get user's concert tickets
    pub fn get_user_concert_tickets(&self, user: AccountId) -> Vec<u64> {
        self.user_concert_tickets.get(user).unwrap_or_default()
    }

    /// Check purchase limits for anti-scalping
    fn check_purchase_limits(&self, buyer: AccountId, event_id: u32) -> InkTixResult<()> {
        let max_per_user = self.event_purchase_limits.get(event_id).unwrap_or(5);
        let current_purchases = self.user_purchase_limits.get((buyer, event_id)).unwrap_or(0);

        if current_purchases >= max_per_user {
            return Err(InkTixError::InvalidData);
        }

        Ok(())
    }

    /// Update purchase limits after successful purchase
    fn update_purchase_limits(&mut self, buyer: AccountId, event_id: u32) {
        let current_purchases = self.user_purchase_limits.get((buyer, event_id)).unwrap_or(0);
        self.user_purchase_limits.insert((buyer, event_id), &(current_purchases + 1));
    }

    /// Calculate seat-based pricing
    fn calculate_seat_price(&self, base_price: u128, seat_type: &SeatType) -> u128 {
        let multiplier = match seat_type {
            SeatType::GeneralAdmission => 100,
            SeatType::Reserved => 120,
            SeatType::PremiumReserved => 150,
            SeatType::VIPSeating => 200,
            SeatType::FrontRow => 300,
            SeatType::Balcony => 110,
            SeatType::FloorSeating => 180,
            SeatType::BoxSeats => 400,
            SeatType::StandingRoom => 80,
            SeatType::AccessibleSeating => 120,
        };

        (base_price * multiplier) / 100
    }

    /// Determine access level based on seat type and VIP status
    fn determine_access_level(&self, seat_type: &SeatType, has_vip: bool) -> AccessLevel {
        if has_vip {
            match seat_type {
                SeatType::BoxSeats | SeatType::FrontRow => AccessLevel::AllAccess,
                _ => AccessLevel::VIP,
            }
        } else {
            match seat_type {
                SeatType::GeneralAdmission | SeatType::StandingRoom => AccessLevel::Standard,
                SeatType::Reserved | SeatType::AccessibleSeating | SeatType::Balcony => AccessLevel::Premium,
                SeatType::PremiumReserved | SeatType::VIPSeating | SeatType::FloorSeating => AccessLevel::VIP,
                SeatType::FrontRow | SeatType::BoxSeats => AccessLevel::AllAccess,
            }
        }
    }

    /// Calculate loyalty points based on purchase
    fn calculate_loyalty_points(&self, seat_type: &SeatType, price: u128) -> u32 {
        let base_points = match seat_type {
            SeatType::GeneralAdmission => 10,
            SeatType::Reserved => 15,
            SeatType::PremiumReserved => 25,
            SeatType::VIPSeating => 50,
            SeatType::FrontRow => 100,
            SeatType::Balcony => 12,
            SeatType::FloorSeating => 40,
            SeatType::BoxSeats => 150,
            SeatType::StandingRoom => 8,
            SeatType::AccessibleSeating => 15,
        };

        let price_bonus = (price / 10_000_000_000) as u32;
        base_points + price_bonus
    }

    /// Get next seat number for an event
    fn get_next_seat_number(&self, event_id: u32) -> InkTixResult<u32> {
        // This is a simplified implementation
        // In a real contract, you'd track seat allocation more carefully
        Ok(1) // Placeholder
    }

    /// Get total ticket count
    pub fn total_concert_tickets(&self) -> u64 {
        self.next_concert_ticket_id.saturating_sub(1)
    }
}


