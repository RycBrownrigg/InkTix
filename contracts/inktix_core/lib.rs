#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_event_manager {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};

    #[ink(storage)]
    pub struct SimpleEventManager {
        events: Mapping<u32, Event>,
        next_event_id: u32,
        owner: AccountId,
    }

    #[derive(scale::Encode, scale::Decode)]
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
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        EventNotFound,
        Unauthorized,
        EventNotActive,
        InsufficientCapacity,
        EventIdOverflow,
    }

    impl SimpleEventManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                events: Mapping::new(),
                next_event_id: 1,
                owner: Self::env().caller(),
            }
        }

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

        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<Event> {
            self.events.get(event_id)
        }

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

        #[ink(message)]
        pub fn get_total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    #[ink(event)]
    pub struct EventCreated {
        #[ink(topic)]
        event_id: u32,
        #[ink(topic)]  
        name: String,
        date: u64,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
                50_000_000_000_000,
            );
            
            assert_eq!(result, Ok(1));
            assert_eq!(contract.get_total_events(), 1);
            
            let event = contract.get_event(1).unwrap();
            assert_eq!(event.name, "Lakers vs Warriors");
            assert_eq!(event.capacity, 20000);
            assert!(event.active);
        }

        #[ink::test]
        fn search_works() {
            let mut contract = SimpleEventManager::new();
            
            contract.create_event(
                "Lakers vs Warriors".to_string(),
                "Staples Center".to_string(),
                1672531200,
                20000,
                50_000_000_000_000,
            ).unwrap();

            let results = contract.search_events_by_name("Lakers".to_string());
            assert_eq!(results.len(), 1);
            assert!(results.contains(&1));
        }
    }
}