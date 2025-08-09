#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_event_manager {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};

    #[ink(storage)]
    pub struct SimpleEventManager {
        // Events storage
        events: Mapping<u32, Event>,
        next_event_id: u32,
        
        // Tickets storage
        tickets: Mapping<u64, Ticket>,
        user_tickets: Mapping<AccountId, Vec<u64>>,
        next_ticket_id: u64,
        
        // Contract management
        owner: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Event {
        id: u32,
        name: String,
        venue: String,
        date: u64,
        capacity: u32,
        sold_tickets: u32,
        base_price: Balance,
        active: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Ticket {
        id: u64,
        event_id: u32,
        owner: AccountId,
        purchase_price: Balance,
        purchase_date: u64,
        seat_number: u32,
        transferable: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        EventNotFound,
        Unauthorized,
        EventNotActive,
        InsufficientCapacity,
        EventIdOverflow,
        InsufficientPayment,
        EventSoldOut,
        TicketNotFound,
        NotTicketOwner,
        TicketNotTransferable,
        TicketIdOverflow,
    }

    impl SimpleEventManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                events: Mapping::new(),
                next_event_id: 1,
                tickets: Mapping::new(),
                user_tickets: Mapping::new(),
                next_ticket_id: 1,
                owner: Self::env().caller(),
            }
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
        ) -> Result<u32, Error> {
            let event_id = self.next_event_id;
            
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::EventIdOverflow)?;

            let event = Event {
                id: event_id,
                name: name.clone(),
                venue,
                date,
                capacity,
                sold_tickets: 0,
                base_price,
                active: true,
            };

            self.events.insert(event_id, &event);
            
            self.env().emit_event(EventCreated {
                event_id,
                name,
                date,
            });
            
            Ok(event_id)
        }

        /// Purchase a ticket for an event (payable)
        #[ink(message, payable)]
        pub fn purchase_ticket(&mut self, event_id: u32) -> Result<u64, Error> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();
            
            // Get and validate event
            let mut event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            
            if !event.active {
                return Err(Error::EventNotActive);
            }
            
            if event.sold_tickets >= event.capacity {
                return Err(Error::EventSoldOut);
            }
            
            if payment < event.base_price {
                return Err(Error::InsufficientPayment);
            }
            
            // Create ticket with safe arithmetic
            let ticket_id = self.next_ticket_id;
            self.next_ticket_id = self.next_ticket_id
                .checked_add(1)
                .ok_or(Error::TicketIdOverflow)?;
            
            let seat_number = event.sold_tickets
                .checked_add(1)
                .ok_or(Error::InsufficientCapacity)?;
            
            let ticket = Ticket {
                id: ticket_id,
                event_id,
                owner: buyer,
                purchase_price: payment,
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
            };
            
            // Update storage
            self.tickets.insert(ticket_id, &ticket);
            
            // Update user tickets
            let mut user_ticket_list = self.user_tickets.get(buyer).unwrap_or_default();
            user_ticket_list.push(ticket_id);
            self.user_tickets.insert(buyer, &user_ticket_list);
            
            // Update event sold count with safe arithmetic
            event.sold_tickets = event.sold_tickets
                .checked_add(1)
                .ok_or(Error::InsufficientCapacity)?;
            self.events.insert(event_id, &event);
            
            // Emit events
            self.env().emit_event(TicketPurchased {
                ticket_id,
                event_id,
                buyer,
                price: payment,
                seat_number: ticket.seat_number,
            });
            
            if event.sold_tickets == event.capacity {
                self.env().emit_event(EventSoldOut { event_id });
            }
            
            Ok(ticket_id)
        }

        /// Transfer a ticket to another account
        #[ink(message)]
        pub fn transfer_ticket(&mut self, ticket_id: u64, to: AccountId) -> Result<(), Error> {
            let caller = self.env().caller();
            
            // Get and validate ticket
            let mut ticket = self.tickets.get(ticket_id).ok_or(Error::TicketNotFound)?;
            
            if ticket.owner != caller {
                return Err(Error::NotTicketOwner);
            }
            
            if !ticket.transferable {
                return Err(Error::TicketNotTransferable);
            }
            
            let old_owner = ticket.owner;
            
            // Update ticket owner
            ticket.owner = to;
            self.tickets.insert(ticket_id, &ticket);
            
            // Remove from old owner's list
            let mut old_owner_tickets = self.user_tickets.get(old_owner).unwrap_or_default();
            old_owner_tickets.retain(|&x| x != ticket_id);
            if old_owner_tickets.is_empty() {
                self.user_tickets.remove(old_owner);
            } else {
                self.user_tickets.insert(old_owner, &old_owner_tickets);
            }
            
            // Add to new owner's list
            let mut new_owner_tickets = self.user_tickets.get(to).unwrap_or_default();
            new_owner_tickets.push(ticket_id);
            self.user_tickets.insert(to, &new_owner_tickets);
            
            self.env().emit_event(TicketTransferred {
                ticket_id,
                from: old_owner,
                to,
            });
            
            Ok(())
        }

        /// Get event details
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<Event> {
            self.events.get(event_id)
        }

        /// Get ticket details
        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u64) -> Option<Ticket> {
            self.tickets.get(ticket_id)
        }

        /// Search events by name
        #[ink(message)]
        pub fn search_events_by_name(&self, query: String) -> Vec<u32> {
            let mut results = Vec::new();
            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    if event.name.contains(&query) && event.active {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Get user's tickets
        #[ink(message)]
        pub fn get_user_tickets(&self, user: AccountId) -> Vec<u64> {
            self.user_tickets.get(user).unwrap_or_default()
        }

        /// Get total number of events
        #[ink(message)]
        pub fn get_total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get available tickets for an event
        #[ink(message)]
        pub fn get_available_tickets(&self, event_id: u32) -> u32 {
            if let Some(event) = self.events.get(event_id) {
                event.capacity.saturating_sub(event.sold_tickets)
            } else {
                0
            }
        }

        /// Check if an event is sold out
        #[ink(message)]
        pub fn is_event_sold_out(&self, event_id: u32) -> bool {
            if let Some(event) = self.events.get(event_id) {
                event.sold_tickets >= event.capacity
            } else {
                true
            }
        }
    }

    impl Default for SimpleEventManager {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct EventCreated {
        #[ink(topic)]
        event_id: u32,
        #[ink(topic)]  
        name: String,
        date: u64,
    }

    #[ink(event)]
    pub struct TicketPurchased {
        #[ink(topic)]
        ticket_id: u64,
        #[ink(topic)]
        event_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        price: Balance,
        seat_number: u32,
    }

    #[ink(event)]
    pub struct TicketTransferred {
        #[ink(topic)]
        ticket_id: u64,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
    }

    #[ink(event)]
    pub struct EventSoldOut {
        #[ink(topic)]
        event_id: u32,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // Helper function to get test accounts
        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        #[ink::test]
        fn new_works() {
            let contract = SimpleEventManager::new();
            assert_eq!(contract.get_total_events(), 0);
        }

        #[ink::test]
        fn create_event_works() {
            let mut contract = SimpleEventManager::new();
            
            let result = contract.create_event(
                "Lakers vs Warriors".to_string(),
                "Staples Center".to_string(),
                1672531200,
                20000,
                50_000_000_000_000, // 0.05 DOT
            );
            
            assert_eq!(result, Ok(1));
            assert_eq!(contract.get_total_events(), 1);
            
            let event = contract.get_event(1).unwrap();
            assert_eq!(event.name, "Lakers vs Warriors");
            assert_eq!(event.capacity, 20000);
            assert_eq!(event.sold_tickets, 0);
            assert!(event.active);
        }

        #[ink::test]
        fn purchase_ticket_insufficient_payment() {
            let mut contract = SimpleEventManager::new();
            
            // Create an event with price 1 DOT
            contract.create_event(
                "Test Event".to_string(),
                "Test Venue".to_string(),
                1672531200,
                100,
                1_000_000_000_000, // 0.001 DOT
            ).unwrap();
            
            // Try to purchase without sufficient payment (no payment transferred)
            let result = contract.purchase_ticket(1);
            assert_eq!(result, Err(Error::InsufficientPayment));
        }

        #[ink::test]
        fn purchase_ticket_event_not_found() {
            let mut contract = SimpleEventManager::new();
            
            // Try to purchase ticket for non-existent event
            let result = contract.purchase_ticket(999);
            assert_eq!(result, Err(Error::EventNotFound));
        }

        #[ink::test]
        fn get_user_tickets_works() {
            let accounts = get_accounts();
            let contract = SimpleEventManager::new();
            
            // Should return empty vector for user with no tickets
            let tickets = contract.get_user_tickets(accounts.alice);
            assert_eq!(tickets.len(), 0);
        }

        #[ink::test]
        fn search_events_works() {
            let mut contract = SimpleEventManager::new();
            
            contract.create_event(
                "Lakers vs Warriors".to_string(),
                "Staples Center".to_string(),
                1672531200,
                20000,
                50_000_000_000_000,
            ).unwrap();
            
            contract.create_event(
                "Warriors vs Clippers".to_string(),
                "Chase Center".to_string(),
                1672617600,
                18000,
                45_000_000_000_000,
            ).unwrap();

            let results = contract.search_events_by_name("Lakers".to_string());
            assert_eq!(results.len(), 1);
            assert!(results.contains(&1));
            
            let warriors_results = contract.search_events_by_name("Warriors".to_string());
            assert_eq!(warriors_results.len(), 2);
            assert!(warriors_results.contains(&1));
            assert!(warriors_results.contains(&2));
        }

        #[ink::test]
        fn get_available_tickets_works() {
            let mut contract = SimpleEventManager::new();
            
            contract.create_event(
                "Test Event".to_string(),
                "Test Venue".to_string(),
                1672531200,
                100,
                1_000_000_000_000,
            ).unwrap();
            
            assert_eq!(contract.get_available_tickets(1), 100);
            assert!(!contract.is_event_sold_out(1));
        }

        #[ink::test]
        fn event_not_found_error() {
            let contract = SimpleEventManager::new();
            
            // Try to get non-existent event
            assert_eq!(contract.get_event(999), None);
            assert_eq!(contract.get_available_tickets(999), 0);
            assert!(contract.is_event_sold_out(999)); // Non-existent events are "sold out"
        }

        #[ink::test]
        fn ticket_not_found_error() {
            let contract = SimpleEventManager::new();
            
            // Try to get non-existent ticket
            assert_eq!(contract.get_ticket(999), None);
        }

        #[ink::test]
        fn transfer_ticket_validation() {
            let accounts = get_accounts();
            let mut contract = SimpleEventManager::new();
            
            // Try to transfer non-existent ticket
            let result = contract.transfer_ticket(999, accounts.bob);
            assert_eq!(result, Err(Error::TicketNotFound));
        }

        #[ink::test]
        fn event_capacity_management() {
            let mut contract = SimpleEventManager::new();
            
            // Create small capacity event
            contract.create_event(
                "Small Event".to_string(),
                "Small Venue".to_string(),
                1672531200,
                2, // Only 2 tickets
                1_000_000_000_000,
            ).unwrap();
            
            // Check initial state
            assert_eq!(contract.get_available_tickets(1), 2);
            assert!(!contract.is_event_sold_out(1));
            
            let event = contract.get_event(1).unwrap();
            assert_eq!(event.sold_tickets, 0);
            assert_eq!(event.capacity, 2);
        }

        #[ink::test]
        fn create_multiple_events() {
            let mut contract = SimpleEventManager::new();
            
            // Create multiple events
            let event1 = contract.create_event(
                "Event 1".to_string(),
                "Venue 1".to_string(),
                1672531200,
                100,
                1_000_000_000_000,
            ).unwrap();
            
            let event2 = contract.create_event(
                "Event 2".to_string(),
                "Venue 2".to_string(),
                1672617600,
                200,
                2_000_000_000_000,
            ).unwrap();
            
            assert_eq!(event1, 1);
            assert_eq!(event2, 2);
            assert_eq!(contract.get_total_events(), 2);
            
            // Verify each event
            let e1 = contract.get_event(1).unwrap();
            let e2 = contract.get_event(2).unwrap();
            
            assert_eq!(e1.name, "Event 1");
            assert_eq!(e1.capacity, 100);
            assert_eq!(e2.name, "Event 2");
            assert_eq!(e2.capacity, 200);
        }

        #[ink::test]
        fn error_handling_comprehensive() {
            let accounts = get_accounts();
            let mut contract = SimpleEventManager::new();
            
            // Test various error conditions
            assert_eq!(contract.purchase_ticket(999), Err(Error::EventNotFound));
            assert_eq!(contract.transfer_ticket(999, accounts.bob), Err(Error::TicketNotFound));
            assert_eq!(contract.get_ticket(999), None);
            assert_eq!(contract.get_event(999), None);
        }
    }
}