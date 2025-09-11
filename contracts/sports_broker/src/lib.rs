#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::string::String;
use ink::prelude::string::ToString;
use ink::prelude::*;
use ink::storage::Mapping;

/// Minimal Sports Broker Contract
///
/// A basic sports ticketing platform with core functionality:
/// - Team registration
/// - Venue registration  
/// - Event creation
/// - Ticket purchasing
///
#[ink::contract]
pub mod sports_broker {
    use super::*;

    #[ink(storage)]
    pub struct SportsBroker {
        owner: AccountId,
        teams: Mapping<u32, Team>,
        venues: Mapping<u32, Venue>,
        events: Mapping<u32, Event>,
        tickets: Mapping<u32, Ticket>,
        next_team_id: u32,
        next_venue_id: u32,
        next_event_id: u32,
        next_ticket_id: u32,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Team {
        pub id: u32,
        pub name: String,
        pub sport: String,
        pub city: String,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Venue {
        pub id: u32,
        pub name: String,
        pub capacity: u32,
        pub location: String,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Event {
        pub id: u32,
        pub name: String,
        pub home_team_id: u32,
        pub away_team_id: u32,
        pub venue_id: u32,
        pub event_date: u64,
        pub base_price: u128,
        pub total_tickets: u32,
        pub tickets_sold: u32,
        pub active: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Ticket {
        pub id: u32,
        pub event_id: u32,
        pub owner: AccountId,
        pub purchase_price: u128,
        pub purchase_date: u64,
        pub seat_number: u32,
        pub section: String,
        pub row: String,
    }

    impl SportsBroker {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                teams: Mapping::default(),
                venues: Mapping::default(),
                events: Mapping::default(),
                tickets: Mapping::default(),
                next_team_id: 1,
                next_venue_id: 1,
                next_event_id: 1,
                next_ticket_id: 1,
            }
        }

        // Team management
        #[ink(message)]
        pub fn register_team(&mut self, name: String, sport: String, city: String) -> u32 {
            let team_id = self.next_team_id;
            let team = Team {
                id: team_id,
                name,
                sport,
                city,
            };

            self.teams.insert(team_id, &team);
            self.next_team_id = self.next_team_id.checked_add(1).unwrap_or(u32::MAX);

            team_id
        }

        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.teams.get(team_id)
        }

        // Venue management
        #[ink(message)]
        pub fn register_venue(&mut self, name: String, capacity: u32, location: String) -> u32 {
            let venue_id = self.next_venue_id;
            let venue = Venue {
                id: venue_id,
                name,
                capacity,
                location,
            };

            self.venues.insert(venue_id, &venue);
            self.next_venue_id = self.next_venue_id.checked_add(1).unwrap_or(u32::MAX);

            venue_id
        }

        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        // Event management
        #[ink(message)]
        pub fn create_event(
            &mut self,
            name: String,
            home_team_id: u32,
            away_team_id: u32,
            venue_id: u32,
            event_date: u64,
            base_price: u128,
            total_tickets: u32,
        ) -> u32 {
            let event_id = self.next_event_id;
            let event = Event {
                id: event_id,
                name,
                home_team_id,
                away_team_id,
                venue_id,
                event_date,
                base_price,
                total_tickets,
                tickets_sold: 0,
                active: true,
            };

            self.events.insert(event_id, &event);
            self.next_event_id = self.next_event_id.checked_add(1).unwrap_or(u32::MAX);

            event_id
        }

        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<Event> {
            self.events.get(event_id)
        }

        // Ticket management
        #[ink(message)]
        pub fn purchase_ticket(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat: u32,
        ) -> Result<u32, String> {
            let event = self
                .events
                .get(event_id)
                .ok_or("Event not found".to_string())?;

            if !event.active {
                return Err("Event is not active".to_string());
            }

            if event.tickets_sold >= event.total_tickets {
                return Err("Event is sold out".to_string());
            }

            let ticket_id = self.next_ticket_id;
            let ticket = Ticket {
                id: ticket_id,
                event_id,
                owner: self.env().caller(),
                purchase_price: event.base_price,
                purchase_date: self.env().block_timestamp(),
                seat_number: seat,
                section,
                row,
            };

            self.tickets.insert(ticket_id, &ticket);
            self.next_ticket_id = self.next_ticket_id.checked_add(1).unwrap_or(u32::MAX);

            // Update event
            let mut updated_event = event;
            updated_event.tickets_sold = updated_event
                .tickets_sold
                .checked_add(1)
                .unwrap_or(u32::MAX);
            self.events.insert(event_id, &updated_event);

            Ok(ticket_id)
        }

        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u32) -> Option<Ticket> {
            self.tickets.get(ticket_id)
        }

        // Analytics
        #[ink(message)]
        pub fn get_total_teams(&self) -> u32 {
            self.next_team_id.checked_sub(1).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_venues(&self) -> u32 {
            self.next_venue_id.checked_sub(1).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_events(&self) -> u32 {
            self.next_event_id.checked_sub(1).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_tickets(&self) -> u32 {
            self.next_ticket_id.checked_sub(1).unwrap_or(0)
        }

        // Owner functions
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new_contract() {
            ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
                let contract = SportsBroker::new();
                assert_eq!(contract.get_total_teams(), 0);
                assert_eq!(contract.get_total_venues(), 0);
                assert_eq!(contract.get_total_events(), 0);
                assert_eq!(contract.get_total_tickets(), 0);
                Ok(())
            })
            .unwrap();
        }

        #[test]
        fn test_register_team() {
            ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
                let mut contract = SportsBroker::new();
                let team_id = contract.register_team(
                    "Lakers".to_string(),
                    "Basketball".to_string(),
                    "LA".to_string(),
                );
                assert_eq!(team_id, 1);
                assert_eq!(contract.get_total_teams(), 1);
                Ok(())
            })
            .unwrap();
        }

        #[test]
        fn test_register_venue() {
            ink::env::test::run_test::<ink::env::DefaultEnvironment, _>(|_| {
                let mut contract = SportsBroker::new();
                let venue_id = contract.register_venue(
                    "Staples Center".to_string(),
                    20000,
                    "Los Angeles".to_string(),
                );
                assert_eq!(venue_id, 1);
                assert_eq!(contract.get_total_venues(), 1);
                Ok(())
            })
            .unwrap();
        }
    }
}
