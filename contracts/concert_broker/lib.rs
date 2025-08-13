#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod concert_broker {
    /// The Concert Broker contract storage.
    #[ink(storage)]
    pub struct ConcertBroker {
        /// The contract owner
        owner: AccountId,
        /// Next artist ID
        next_artist_id: u32,
        /// Next event ID  
        next_event_id: u32,
        /// Simple artist storage (name by ID)
        artists: ink::storage::Mapping<u32, Option<ink::prelude::string::String>>,
        /// Simple event storage (name by ID)
        events: ink::storage::Mapping<u32, Option<ink::prelude::string::String>>,
    }

    /// Concert broker errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Artist not found
        ArtistNotFound,
        /// Event not found
        EventNotFound,
        /// ID overflow
        IdOverflow,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl ConcertBroker {
        /// Creates a new Concert Broker contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                next_artist_id: 1,
                next_event_id: 1,
                artists: ink::storage::Mapping::new(),
                events: ink::storage::Mapping::new(),
            }
        }

        /// Register a new artist
        #[ink(message)]
        pub fn register_artist(&mut self, name: ink::prelude::string::String) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let artist_id = self.next_artist_id;
            self.artists.insert(artist_id, &Some(name));
            
            // Safe arithmetic - use checked_add to prevent overflow
            self.next_artist_id = self.next_artist_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;
            
            Ok(artist_id)
        }

        /// Create a new event
        #[ink(message)]
        pub fn create_event(
            &mut self, 
            name: ink::prelude::string::String,
            artist_id: u32
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Check if artist exists
            if self.artists.get(artist_id).is_none() {
                return Err(Error::ArtistNotFound);
            }

            let event_id = self.next_event_id;
            self.events.insert(event_id, &Some(name));
            
            // Safe arithmetic - use checked_add to prevent overflow
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;
            
            Ok(event_id)
        }

        /// Get artist name
        #[ink(message)]
        pub fn get_artist(&self, artist_id: u32) -> Option<ink::prelude::string::String> {
            self.artists.get(artist_id).unwrap_or(None)
        }

        /// Get event name
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<ink::prelude::string::String> {
            self.events.get(event_id).unwrap_or(None)
        }

        /// Get the owner of the contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total artists registered
        #[ink(message)]
        pub fn total_artists(&self) -> u32 {
            // Safe arithmetic - use saturating_sub to prevent underflow
            self.next_artist_id.saturating_sub(1)
        }

        /// Get total events created
        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            // Safe arithmetic - use saturating_sub to prevent underflow
            self.next_event_id.saturating_sub(1)
        }
    }

    /// Add Default implementation as suggested by clippy
    impl Default for ConcertBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let concert_broker = ConcertBroker::new();
            assert_eq!(concert_broker.total_artists(), 0);
            assert_eq!(concert_broker.total_events(), 0);
        }

        #[ink::test]
        fn register_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let result = concert_broker.register_artist("Taylor Swift".to_string());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            assert_eq!(concert_broker.total_artists(), 1);
        }

        #[ink::test]
        fn create_event_works() {
            let mut concert_broker = ConcertBroker::new();
            
            // Register artist first
            let artist_id = concert_broker.register_artist("Drake".to_string()).unwrap();
            
            // Create event
            let result = concert_broker.create_event("Drake Concert".to_string(), artist_id);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            assert_eq!(concert_broker.total_events(), 1);
        }

        #[ink::test]
        fn create_event_artist_not_found() {
            let mut concert_broker = ConcertBroker::new();
            
            // Try to create event without artist
            let result = concert_broker.create_event("No Artist Concert".to_string(), 999);
            assert_eq!(result, Err(Error::ArtistNotFound));
        }

        #[ink::test]
        fn multiple_artists_and_events() {
            let mut concert_broker = ConcertBroker::new();
            
            // Register multiple artists
            let artist1 = concert_broker.register_artist("Taylor Swift".to_string()).unwrap();
            let artist2 = concert_broker.register_artist("Drake".to_string()).unwrap();
            
            assert_eq!(artist1, 1);
            assert_eq!(artist2, 2);
            assert_eq!(concert_broker.total_artists(), 2);
            
            // Create events for both artists
            let event1 = concert_broker.create_event("Eras Tour".to_string(), artist1).unwrap();
            let event2 = concert_broker.create_event("Drake Concert".to_string(), artist2).unwrap();
            
            assert_eq!(event1, 1);
            assert_eq!(event2, 2);
            assert_eq!(concert_broker.total_events(), 2);
            
            // Verify we can retrieve the data
            assert_eq!(concert_broker.get_artist(artist1), Some("Taylor Swift".to_string()));
            assert_eq!(concert_broker.get_artist(artist2), Some("Drake".to_string()));
            assert_eq!(concert_broker.get_event(event1), Some("Eras Tour".to_string()));
            assert_eq!(concert_broker.get_event(event2), Some("Drake Concert".to_string()));
        }
    }
}