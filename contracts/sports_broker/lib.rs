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

        /// Register a new sports team
        #[ink(message)]
        pub fn register_team(
            &mut self,
            name: String,
            city: String,
            sport_type: SportType,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let team_id = self.next_team_id;
            self.next_team_id = self.next_team_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let team = Team {
                id: team_id,
                name,
                city,
                sport_type,
                verified: true,
            };

            self.teams.insert(team_id, &team);
            Ok(team_id)
        }

        /// Register a new venue
        #[ink(message)]
        pub fn register_venue(
            &mut self,
            name: String,
            city: String,
            capacity: u32,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let venue_id = self.next_venue_id;
            self.next_venue_id = self.next_venue_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let venue = Venue {
                id: venue_id,
                name,
                city,
                capacity,
            };

            self.venues.insert(venue_id, &venue);
            Ok(venue_id)
        }

        /// Get team information
        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.teams.get(team_id)
        }

        /// Get venue information
        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
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

    /// Unit tests for Step 2 - Team & Venue Management
    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        #[ink::test]
        fn new_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
        }

        #[ink::test]
        fn register_team_works() {
            let mut sports_broker = SportsBroker::new();
            
            let result = sports_broker.register_team(
                "Los Angeles Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            );

            assert!(result.is_ok());
            let team_id = result.unwrap();
            assert_eq!(team_id, 1);
            assert_eq!(sports_broker.total_teams(), 1);

            // Verify team data
            let team = sports_broker.get_team(team_id).unwrap();
            assert_eq!(team.name, "Los Angeles Lakers");
            assert_eq!(team.city, "Los Angeles");
            assert_eq!(team.sport_type, SportType::Basketball);
            assert!(team.verified);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut sports_broker = SportsBroker::new();
            
            let result = sports_broker.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                20000,
            );

            assert!(result.is_ok());
            let venue_id = result.unwrap();
            assert_eq!(venue_id, 1);
            assert_eq!(sports_broker.total_venues(), 1);

            // Verify venue data
            let venue = sports_broker.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.city, "Los Angeles");
            assert_eq!(venue.capacity, 20000);
        }

        #[ink::test]
        fn register_team_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            
            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.register_team(
                "Unauthorized Team".to_string(),
                "Somewhere".to_string(),
                SportType::Football,
            );

            assert_eq!(result, Err(Error::NotOwner));
            assert_eq!(sports_broker.total_teams(), 0);
        }

        #[ink::test]
        fn register_venue_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            
            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.register_venue(
                "Unauthorized Venue".to_string(),
                "Somewhere".to_string(),
                10000,
            );

            assert_eq!(result, Err(Error::NotOwner));
            assert_eq!(sports_broker.total_venues(), 0);
        }

        #[ink::test]
        fn multiple_teams_registration() {
            let mut sports_broker = SportsBroker::new();
            
            // Register multiple teams
            let team1 = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();
            
            let team2 = sports_broker.register_team(
                "49ers".to_string(),
                "San Francisco".to_string(),
                SportType::Football,
            ).unwrap();

            assert_eq!(team1, 1);
            assert_eq!(team2, 2);
            assert_eq!(sports_broker.total_teams(), 2);

            // Verify both teams exist and have correct data
            let lakers = sports_broker.get_team(team1).unwrap();
            assert_eq!(lakers.sport_type, SportType::Basketball);
            
            let niners = sports_broker.get_team(team2).unwrap();
            assert_eq!(niners.sport_type, SportType::Football);
        }

        #[ink::test]
        fn multiple_venues_registration() {
            let mut sports_broker = SportsBroker::new();
            
            // Register multiple venues
            let venue1 = sports_broker.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                20000,
            ).unwrap();
            
            let venue2 = sports_broker.register_venue(
                "Madison Square Garden".to_string(),
                "New York".to_string(),
                18000,
            ).unwrap();

            assert_eq!(venue1, 1);
            assert_eq!(venue2, 2);
            assert_eq!(sports_broker.total_venues(), 2);

            // Verify both venues exist and have correct data
            let staples = sports_broker.get_venue(venue1).unwrap();
            assert_eq!(staples.capacity, 20000);
            
            let msg = sports_broker.get_venue(venue2).unwrap();
            assert_eq!(msg.capacity, 18000);
        }

        #[ink::test]
        fn get_nonexistent_team_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_team(999), None);
        }

        #[ink::test]
        fn get_nonexistent_venue_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_venue(999), None);
        }

        #[ink::test]
        fn sport_type_other_variant_works() {
            let mut sports_broker = SportsBroker::new();
            
            let team_id = sports_broker.register_team(
                "Cricket Team".to_string(),
                "Mumbai".to_string(),
                SportType::Other("Cricket".to_string()),
            ).unwrap();

            let team = sports_broker.get_team(team_id).unwrap();
            assert_eq!(team.sport_type, SportType::Other("Cricket".to_string()));
        }
    }
}