#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// InkTix Core - Foundation Ticket Marketplace Contract
/// 
/// This serves as the base template for specialized marketplace contracts.
/// Features:
/// - Event creation and management
/// - Payable ticket purchasing  
/// - User inventory tracking
/// - Secure ticket transfers
/// - Capacity management
/// - Multi-currency support
/// 
/// Specialization Guide:
/// - Copy this contract for each specialized broker
/// - Extend Event struct with domain-specific fields
/// - Add specialized purchasing logic
/// - Integrate with chain-specific features (DeFi, tokens, etc.)

// Import our new modules
mod types;
mod utils;
mod tests;

// Re-export commonly used types for easy access
pub use types::*;
pub use utils::*;

// Make ink types available to our modules
pub use ink::prelude::*;
pub use ink::storage::Mapping;

#[ink::contract]
mod inktix_core {
    use super::*;
    
    // Import from our modules
    use crate::types::{BaseEvent, BaseTicket, InkTixError, InkTixResult, CurrencyId};
    use crate::utils::{validate_non_empty_string, validate_positive_number, validate_positive_balance};

    // ============================================================================
    // STORAGE & STATE
    // ============================================================================
    
    /// Core marketplace storage
    #[ink(storage)]
    pub struct InkTixCore {
        // Event management
        events: Mapping<u32, BaseEvent>,
        next_event_id: u32,
        
        // Ticket management
        tickets: Mapping<u64, BaseTicket>,
        user_tickets: Mapping<AccountId, Vec<u64>>,
        next_ticket_id: u64,
        
        // Currency management
        currency_rates: Mapping<CurrencyId, Balance>,
        supported_currencies: Vec<CurrencyId>,
        
        // Contract management
        owner: AccountId,
    }

    // ============================================================================
    // IMPLEMENTATION
    // ============================================================================

    impl InkTixCore {
        /// Creates a new InkTix Core contract
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut supported_currencies = Vec::new();
            supported_currencies.push(CurrencyId::DOT);
            supported_currencies.push(CurrencyId::ACA);
            supported_currencies.push(CurrencyId::AUSD);
            supported_currencies.push(CurrencyId::LDOT);
            supported_currencies.push(CurrencyId::KSM);

            let mut contract = Self {
                events: Mapping::new(),
                next_event_id: 1,
                tickets: Mapping::new(),
                user_tickets: Mapping::new(),
                next_ticket_id: 1,
                currency_rates: Mapping::new(),
                supported_currencies,
                owner: Self::env().caller(),
            };

            // Initialize currency rates (DOT as base)
            contract.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000);
            contract.currency_rates.insert(CurrencyId::ACA, &50_000_000_000);
            contract.currency_rates.insert(CurrencyId::AUSD, &150_000_000_000);
            contract.currency_rates.insert(CurrencyId::LDOT, &950_000_000_000);
            contract.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000);

            contract
        }

        /// Create a new event
        #[ink(message)]
        pub fn create_event(
            &mut self,
            name: String,
            venue: String,
            date: u64,
            capacity: u32,
            base_price: Balance,
        ) -> InkTixResult<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(InkTixError::NotOwner);
            }

            // Validate inputs using our utility functions
            validate_non_empty_string(&name, "name")?;
            validate_non_empty_string(&venue, "venue")?;
            validate_positive_number(capacity, "capacity")?;
            validate_positive_balance(base_price, "base_price")?;

            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(InkTixError::IdOverflow)?;

            let event = BaseEvent {
                id: event_id,
                name,
                venue,
                date,
                capacity,
                sold_tickets: 0,
                base_price,
                active: true,
            };

            self.events.insert(event_id, &event);
            Ok(event_id)
        }

        /// Purchase a ticket
        #[ink(message, payable)]
        pub fn purchase_ticket(
            &mut self,
            event_id: u32,
            _section: String,  // ← Add underscore
            _row: String,      // ← Add underscore
            seat_number: u32,
        ) -> InkTixResult<u64> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            let event = self.events.get(event_id).ok_or(InkTixError::NotFound)?;
            
            if !event.active {
                return Err(InkTixError::InvalidData);
            }

            if event.sold_tickets >= event.capacity {
                return Err(InkTixError::InvalidData);
            }

            if payment < event.base_price {
                return Err(InkTixError::InsufficientPayment);
            }

            let ticket_id = self.next_ticket_id;
            self.next_ticket_id = self.next_ticket_id
                .checked_add(1)
                .ok_or(InkTixError::IdOverflow)?;

            let ticket = BaseTicket {
                id: ticket_id,
                event_id,
                owner: buyer,
                purchase_price: payment,
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
            };

            self.tickets.insert(ticket_id, &ticket);

            let mut user_tickets = self.user_tickets.get(buyer).unwrap_or_default();
            user_tickets.push(ticket_id);
            self.user_tickets.insert(buyer, &user_tickets);

            // Update event
            let mut updated_event = event;
            updated_event.sold_tickets = updated_event.sold_tickets.checked_add(1)
                .ok_or(InkTixError::IdOverflow)?;
            self.events.insert(event_id, &updated_event);

            Ok(ticket_id)
        }

        /// Transfer a ticket
        #[ink(message)]
        pub fn transfer_ticket(&mut self, ticket_id: u64, to: AccountId) -> InkTixResult<()> {
            let caller = self.env().caller();

            let mut ticket = self.tickets.get(ticket_id).ok_or(InkTixError::NotFound)?;

            if ticket.owner != caller {
                return Err(InkTixError::NotOwner);
            }

            if !ticket.transferable {
                return Err(InkTixError::InvalidData);
            }

            // Update ticket owner
            let old_owner = ticket.owner;
            ticket.owner = to;
            self.tickets.insert(ticket_id, &ticket);

            // Update user ticket lists
            if let Some(mut old_tickets) = self.user_tickets.get(old_owner) {
                old_tickets.retain(|&x| x != ticket_id);
                self.user_tickets.insert(old_owner, &old_tickets);
            }

            let mut new_tickets = self.user_tickets.get(to).unwrap_or_default();
            new_tickets.push(ticket_id);
            self.user_tickets.insert(to, &new_tickets);

            Ok(())
        }

        /// Get event details
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<BaseEvent> {
            self.events.get(event_id)
        }

        /// Get ticket details
        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u64) -> Option<BaseTicket> {
            self.tickets.get(ticket_id)
        }

        /// Get user's tickets
        #[ink(message)]
        pub fn get_user_tickets(&self, user: AccountId) -> Vec<u64> {
            self.user_tickets.get(user).unwrap_or_default()
        }

        /// Get supported currencies
        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.supported_currencies.clone()
        }

        /// Get currency rate
        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<Balance> {
            self.currency_rates.get(currency)
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total counts
        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_tickets(&self) -> u64 {
            self.next_ticket_id.saturating_sub(1)
        }
    }

    impl Default for InkTixCore {
        fn default() -> Self {
            Self::new()
        }
    }
}