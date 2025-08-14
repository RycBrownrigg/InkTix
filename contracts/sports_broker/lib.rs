#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sports_broker {
    use ink::prelude::string::String;

    /// The Sports Broker contract storage.
    #[ink(storage)]
    pub struct SportsBroker {
        /// The contract owner
        owner: AccountId,
        
        // Team management
        teams: ink::storage::Mapping<u32, Team>,
        next_team_id: u32,
        
        // Venue management  
        venues: ink::storage::Mapping<u32, Venue>,
        next_venue_id: u32,
    }

    /// Team information
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Team {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub sport_type: SportType,
        pub verified: bool,
    }

    /// Venue information
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Venue {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub capacity: u32,
    }

    /// Sport types
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SportType {
        Basketball,
        Football,     // American Football
        Soccer,       // Football/Soccer
        Baseball,
        Hockey,
        Tennis,
        Other(String),
    }

    /// Sports broker errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Team not found
        TeamNotFound,
        /// Venue not found
        VenueNotFound,
        /// ID overflow
        IdOverflow,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl SportsBroker {
        /// Creates a new Sports Broker contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                teams: ink::storage::Mapping::new(),
                next_team_id: 1,
                venues: ink::storage::Mapping::new(),
                next_venue_id: 1,
            }
        }

        /// Get the owner of the contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total teams registered
        #[ink(message)]
        pub fn total_teams(&self) -> u32 {
            self.next_team_id.saturating_sub(1)
        }

        /// Get total venues registered
        #[ink(message)]
        pub fn total_venues(&self) -> u32 {
            self.next_venue_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Unit tests for Step 1 - Basic Storage & Data Types
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
        }

        #[ink::test]
        fn storage_initialization_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.next_team_id, 1);
            assert_eq!(sports_broker.next_venue_id, 1);
        }

        #[ink::test]
        fn owner_is_set_correctly() {
            let sports_broker = SportsBroker::new();
            let expected_owner = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            assert_eq!(sports_broker.get_owner(), expected_owner);
        }

        #[ink::test]
        fn sport_type_variants_work() {
            // Test that SportType enum variants can be created and compared
            let basketball = SportType::Basketball;
            let football = SportType::Football;
            let custom = SportType::Other("Cricket".to_string());
            
            assert_eq!(basketball, SportType::Basketball);
            assert_ne!(basketball, football);
            assert_eq!(custom, SportType::Other("Cricket".to_string()));
        }

        #[ink::test]
        fn team_struct_creation_works() {
            let team = Team {
                id: 1,
                name: "Test Team".to_string(),
                city: "Test City".to_string(),
                sport_type: SportType::Basketball,
                verified: true,
            };
            
            assert_eq!(team.id, 1);
            assert_eq!(team.name, "Test Team");
            assert_eq!(team.sport_type, SportType::Basketball);
            assert!(team.verified);
        }

        #[ink::test]
        fn venue_struct_creation_works() {
            let venue = Venue {
                id: 1,
                name: "Test Arena".to_string(),
                city: "Test City".to_string(),
                capacity: 20000,
            };
            
            assert_eq!(venue.id, 1);
            assert_eq!(venue.name, "Test Arena");
            assert_eq!(venue.capacity, 20000);
        }
    }
}