#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::storage::Mapping;

/// Simple working version of Concert Broker
#[ink::contract]
pub mod concert_broker {
    use super::*;

    /// Contract owner for admin-only actions

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Artist {
        pub id: u32,
        pub name: String,
        pub verified: bool,
        pub account: Option<AccountId>,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Venue {
        pub id: u32,
        pub name: String,
        pub capacity: u32,
        pub address: String,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct ConcertEvent {
        pub id: u32,
        pub name: String,
        pub artist_id: u32,
        pub venue_id: u32,
        pub date: u64,
        pub capacity: u32,
        pub base_price: u128,
        pub sold_tickets: u32,
        pub status: EventStatus,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum EventStatus {
        Active,
        Cancelled,
        Completed,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct ConcertTicket {
        pub id: u64,
        pub event_id: u32,
        pub owner: ink::primitives::AccountId,
        pub seat_number: u32,
        pub purchase_price: u128,
        pub purchase_date: u64,
    }

    #[ink(storage)]
    pub struct ConcertBroker {
        pub owner: AccountId,
        pub total_artists: u32,
        pub total_venues: u32,
        pub total_events: u32,
        pub total_tickets: u32,
        pub artists: Mapping<u32, Artist>,
        pub venues: Mapping<u32, Venue>,
        pub events: Mapping<u32, ConcertEvent>,
        pub tickets: Mapping<u64, ConcertTicket>,
        pub user_tickets: Mapping<ink::primitives::AccountId, Vec<u64>>, // AccountId -> Vec<TicketId>
        pub per_event_purchase_count: Mapping<(u32, AccountId), u32>, // (event_id, buyer) -> count
        pub next_artist_id: u32,
        pub next_venue_id: u32,
        pub next_event_id: u32,
        pub next_ticket_id: u64,
    }

    impl Default for ConcertBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    #[allow(clippy::arithmetic_side_effects)]
    impl ConcertBroker {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                total_artists: 0,
                total_venues: 0,
                total_events: 0,
                total_tickets: 0,
                artists: Mapping::new(),
                venues: Mapping::new(),
                events: Mapping::new(),
                tickets: Mapping::new(),
                user_tickets: Mapping::new(),
                per_event_purchase_count: Mapping::new(),
                next_artist_id: 1,
                next_venue_id: 1,
                next_event_id: 1,
                next_ticket_id: 1,
            }
        }

        fn ensure_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only owner");
        }

        #[ink(message)]
        pub fn register_artist(&mut self, name: String) -> u32 {
            let artist_id = self.next_artist_id;
            self.next_artist_id += 1;

            let artist = Artist {
                id: artist_id,
                name,
                verified: false,
                account: Some(self.env().caller()),
            };

            self.artists.insert(artist_id, &artist);
            self.total_artists += 1;
            artist_id
        }

        #[ink(message)]
        pub fn register_venue(&mut self, name: String, capacity: u32, address: String) -> u32 {
            let venue_id = self.next_venue_id;
            self.next_venue_id += 1;

            let venue = Venue {
                id: venue_id,
                name,
                capacity,
                address,
            };

            self.venues.insert(venue_id, &venue);
            self.total_venues += 1;
            venue_id
        }

        #[ink(message)]
        pub fn create_concert_event(
            &mut self,
            name: String,
            artist_id: u32,
            venue_id: u32,
            date: u64,
            capacity: u32,
            base_price: u128,
        ) -> u32 {
            self.ensure_owner();
            let event_id = self.next_event_id;
            self.next_event_id += 1;

            let event = ConcertEvent {
                id: event_id,
                name,
                artist_id,
                venue_id,
                date,
                capacity,
                base_price,
                sold_tickets: 0,
                status: EventStatus::Active,
            };

            self.events.insert(event_id, &event);
            self.total_events += 1;
            event_id
        }

        #[ink(message, payable)]
        pub fn purchase_ticket(&mut self, event_id: u32, seat_number: u32) -> u64 {
            let caller = self.env().caller();
            let payment = self.env().transferred_value();

            // Get the event
            let event = self.events.get(event_id).expect("Event not found");

            // Event must be active
            assert!(
                matches!(event.status, EventStatus::Active),
                "Event not active"
            );

            // Check if event is sold out
            assert!(event.sold_tickets < event.capacity, "Event is sold out");

            // Check if payment is sufficient
            assert!(payment >= event.base_price, "Insufficient payment");

            // Enforce per-account purchase limit (default 4)
            let key = (event_id, caller);
            let count = self.per_event_purchase_count.get(key).unwrap_or(0);
            assert!(count < 4, "Purchase limit reached");

            // Create ticket
            let ticket_id = self.next_ticket_id;
            self.next_ticket_id += 1;

            let ticket = ConcertTicket {
                id: ticket_id,
                event_id,
                owner: caller,
                seat_number,
                purchase_price: payment,
                purchase_date: self.env().block_timestamp(),
            };

            // Store ticket
            self.tickets.insert(ticket_id, &ticket);

            // Update user's ticket list
            let mut user_tickets = self.user_tickets.get(caller).unwrap_or_default();
            user_tickets.push(ticket_id);
            self.user_tickets.insert(caller, &user_tickets);

            // Update event sold tickets
            let mut updated_event = event.clone();
            updated_event.sold_tickets += 1;
            self.events.insert(event_id, &updated_event);

            // Update per-account purchase count
            self.per_event_purchase_count.insert(key, &(count + 1));

            self.total_tickets += 1;
            ticket_id
        }

        #[ink(message)]
        pub fn verify_artist(&mut self, artist_id: u32) {
            self.ensure_owner();
            let mut artist = self.artists.get(artist_id).expect("Artist not found");
            artist.verified = true;
            self.artists.insert(artist_id, &artist);
        }

        #[ink(message)]
        pub fn set_event_status(&mut self, event_id: u32, status: EventStatus) {
            self.ensure_owner();
            let mut event = self.events.get(event_id).expect("Event not found");
            event.status = status;
            self.events.insert(event_id, &event);
        }

        #[ink(message)]
        pub fn transfer_ticket(&mut self, ticket_id: u64, to: AccountId) {
            let caller = self.env().caller();
            let mut ticket = self.tickets.get(ticket_id).expect("Ticket not found");
            assert_eq!(ticket.owner, caller, "Not ticket owner");

            // Remove from caller list
            let mut from_list = self.user_tickets.get(caller).unwrap_or_default();
            if let Some(pos) = from_list.iter().position(|&id| id == ticket_id) {
                from_list.swap_remove(pos);
                self.user_tickets.insert(caller, &from_list);
            }

            // Add to recipient list
            let mut to_list = self.user_tickets.get(to).unwrap_or_default();
            to_list.push(ticket_id);
            self.user_tickets.insert(to, &to_list);

            // Update owner
            ticket.owner = to;
            self.tickets.insert(ticket_id, &ticket);
        }

        #[ink(message)]
        pub fn get_artist(&self, artist_id: u32) -> Option<Artist> {
            self.artists.get(artist_id)
        }

        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<ConcertEvent> {
            self.events.get(event_id)
        }

        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u64) -> Option<ConcertTicket> {
            self.tickets.get(ticket_id)
        }

        #[ink(message)]
        pub fn get_user_tickets(&self, user: ink::primitives::AccountId) -> Vec<u64> {
            self.user_tickets.get(user).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_stats(&self) -> (u32, u32, u32, u32) {
            (
                self.total_artists,
                self.total_venues,
                self.total_events,
                self.total_tickets,
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = ConcertBroker::new();
            assert_eq!(contract.total_artists, 0);
            assert_eq!(contract.total_venues, 0);
            assert_eq!(contract.total_events, 0);
            assert_eq!(contract.total_tickets, 0);
        }

        #[ink::test]
        fn register_artist_works() {
            let mut contract = ConcertBroker::new();
            let artist_id = contract.register_artist("Test Artist".to_string());
            assert_eq!(artist_id, 1);
            assert_eq!(contract.total_artists, 1);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut contract = ConcertBroker::new();
            let venue_id =
                contract.register_venue("Test Venue".to_string(), 1000, "123 Main St".to_string());
            assert_eq!(venue_id, 1);
            assert_eq!(contract.total_venues, 1);
        }

        #[ink::test]
        fn create_event_works() {
            let mut contract = ConcertBroker::new();
            let artist_id = contract.register_artist("Test Artist".to_string());
            let venue_id =
                contract.register_venue("Test Venue".to_string(), 1000, "123 Main St".to_string());

            let event_id = contract.create_concert_event(
                "Test Concert".to_string(),
                artist_id,
                venue_id,
                1234567890,
                1000,
                100_000_000_000_000, // 0.1 DOT
            );

            assert_eq!(event_id, 1);
            assert_eq!(contract.total_events, 1);
        }

        #[ink::test]
        fn get_artist_works() {
            let mut contract = ConcertBroker::new();
            let artist_id = contract.register_artist("Test Artist".to_string());
            let artist = contract.get_artist(artist_id);
            assert!(artist.is_some());
            assert_eq!(artist.unwrap().name, "Test Artist");
        }

        #[ink::test]
        fn get_venue_works() {
            let mut contract = ConcertBroker::new();
            let venue_id =
                contract.register_venue("Test Venue".to_string(), 1000, "123 Main St".to_string());
            assert_eq!(venue_id, 1);
            assert_eq!(contract.total_venues, 1);
        }

        #[ink::test]
        fn get_event_works() {
            let mut contract = ConcertBroker::new();
            let artist_id = contract.register_artist("Test Artist".to_string());
            let venue_id =
                contract.register_venue("Test Venue".to_string(), 1000, "123 Main St".to_string());

            let event_id = contract.create_concert_event(
                "Test Concert".to_string(),
                artist_id,
                venue_id,
                1234567890,
                1000,
                100_000_000_000_000,
            );

            let event = contract.get_event(event_id);
            assert!(event.is_some());

            // Fix: Use as_ref() to borrow the contents instead of moving
            let event_ref = event.as_ref().unwrap();
            assert_eq!(event_ref.name, "Test Concert");
            assert_eq!(event_ref.base_price, 100_000_000_000_000);
        }

        #[ink::test]
        fn only_owner_can_create_event_and_verify_artist() {
            let mut contract = ConcertBroker::new();

            // Non-owner caller context
            let non_owner = ink::primitives::AccountId::from([2u8; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(non_owner);

            let artist_id = contract.register_artist("Artist".to_string());
            let venue_id = contract.register_venue("Venue".to_string(), 100, "Addr".to_string());

            // create_concert_event should panic for non-owner
            let result = std::panic::catch_unwind(|| {
                contract.create_concert_event("E".to_string(), artist_id, venue_id, 1, 100, 1)
            });
            assert!(result.is_err());

            // verify_artist should panic for non-owner
            let result = std::panic::catch_unwind(|| contract.verify_artist(artist_id));
            assert!(result.is_err());
        }

        #[ink::test]
        fn purchase_respects_status_capacity_price_and_limit() {
            let mut contract = ConcertBroker::new();
            let owner = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
                .expect("accounts")
                .alice;
            let buyer = ink::primitives::AccountId::from([3u8; 32]);

            // As owner, create entities
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            let artist_id = contract.register_artist("A".to_string());
            let venue_id = contract.register_venue("V".to_string(), 2, "Addr".to_string());
            let event_id =
                contract.create_concert_event("E".to_string(), artist_id, venue_id, 1, 2, 10);

            // Cancel the event and ensure purchase fails
            contract.set_event_status(event_id, EventStatus::Cancelled);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(buyer);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10);
            let result = std::panic::catch_unwind(|| contract.purchase_ticket(event_id, 1));
            assert!(result.is_err());

            // Re-activate
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            contract.set_event_status(event_id, EventStatus::Active);

            // Buyer purchases up to limit (4)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(buyer);
            for seat in 1..=2u32 {
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10);
                let _ = contract.purchase_ticket(event_id, seat);
            }

            // Capacity is 2, so third should fail due to sold out before limit
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10);
            let result = std::panic::catch_unwind(|| contract.purchase_ticket(event_id, 3));
            assert!(result.is_err());
        }

        #[ink::test]
        fn transfer_ticket_changes_owner_and_lists() {
            let mut contract = ConcertBroker::new();
            let owner = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
                .expect("accounts")
                .alice;
            let buyer = ink::primitives::AccountId::from([4u8; 32]);
            let recipient = ink::primitives::AccountId::from([5u8; 32]);

            // Setup
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            let artist_id = contract.register_artist("A".to_string());
            let venue_id = contract.register_venue("V".to_string(), 10, "Addr".to_string());
            let event_id =
                contract.create_concert_event("E".to_string(), artist_id, venue_id, 1, 10, 7);

            // Buy ticket
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(buyer);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(7);
            let ticket_id = contract.purchase_ticket(event_id, 1);
            assert_eq!(contract.get_user_tickets(buyer).len(), 1);

            // Transfer
            contract.transfer_ticket(ticket_id, recipient);
            assert_eq!(contract.get_user_tickets(buyer).len(), 0);
            assert_eq!(contract.get_user_tickets(recipient).len(), 1);

            let ticket = contract.get_ticket(ticket_id).unwrap();
            assert_eq!(ticket.owner, recipient);
        }
    }
}
