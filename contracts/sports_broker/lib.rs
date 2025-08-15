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

        // Season management
        seasons: ink::storage::Mapping<u32, Season>,
        next_season_id: u32,
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

    /// Season information for subscription management
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Season {
        pub id: u32,
        pub name: String, // "2024-25 NBA Season"
        pub sport_type: SportType,
        pub start_date: u64, // Unix timestamp
        pub end_date: u64,   // Unix timestamp
        pub regular_season_games: u32,
        pub active: bool,
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
        /// Season not found
        SeasonNotFound,
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
                seasons: ink::storage::Mapping::new(),
                next_season_id: 1,
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

        /// Create a new sports season
        #[ink(message)]
        pub fn create_season(
            &mut self,
            name: String,
            sport_type: SportType,
            start_date: u64,
            end_date: u64,
            regular_season_games: u32,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let season_id = self.next_season_id;
            self.next_season_id = self.next_season_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let season = Season {
                id: season_id,
                name,
                sport_type,
                start_date,
                end_date,
                regular_season_games,
                active: true,
            };

            self.seasons.insert(season_id, &season);
            Ok(season_id)
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

        /// Get season information
        #[ink(message)]
        pub fn get_season(&self, season_id: u32) -> Option<Season> {
            self.seasons.get(season_id)
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

        /// Get total seasons created
        #[ink(message)]
        pub fn total_seasons(&self) -> u32 {
            self.next_season_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Unit tests for Step 3 - Season Management
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
            assert_eq!(sports_broker.total_seasons(), 0);
        }

        #[ink::test]
        fn create_season_works() {
            let mut sports_broker = SportsBroker::new();
            
            let result = sports_broker.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000, // Oct 1, 2024 (Unix timestamp in milliseconds)
                1715644800000, // May 14, 2025
                82,
            );

            assert!(result.is_ok());
            let season_id = result.unwrap();
            assert_eq!(season_id, 1);
            assert_eq!(sports_broker.total_seasons(), 1);

            // Verify season data
            let season = sports_broker.get_season(season_id).unwrap();
            assert_eq!(season.name, "2024-25 NBA Season");
            assert_eq!(season.sport_type, SportType::Basketball);
            assert_eq!(season.regular_season_games, 82);
            assert!(season.active);
        }

        #[ink::test]
        fn create_season_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            
            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.create_season(
                "Unauthorized Season".to_string(),
                SportType::Football,
                1696118400000,
                1715644800000,
                16,
            );

            assert_eq!(result, Err(Error::NotOwner));
            assert_eq!(sports_broker.total_seasons(), 0);
        }

        #[ink::test]
        fn multiple_seasons_creation() {
            let mut sports_broker = SportsBroker::new();
            
            // Create NBA season
            let nba_season = sports_broker.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            ).unwrap();
            
            // Create NFL season
            let nfl_season = sports_broker.create_season(
                "2024 NFL Season".to_string(),
                SportType::Football,
                1693526400000, // Sep 1, 2024
                1707523200000, // Feb 10, 2025
                17,
            ).unwrap();

            assert_eq!(nba_season, 1);
            assert_eq!(nfl_season, 2);
            assert_eq!(sports_broker.total_seasons(), 2);

            // Verify both seasons exist and have correct data
            let nba = sports_broker.get_season(nba_season).unwrap();
            assert_eq!(nba.sport_type, SportType::Basketball);
            assert_eq!(nba.regular_season_games, 82);
            
            let nfl = sports_broker.get_season(nfl_season).unwrap();
            assert_eq!(nfl.sport_type, SportType::Football);
            assert_eq!(nfl.regular_season_games, 17);
        }

        #[ink::test]
        fn get_nonexistent_season_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_season(999), None);
        }

        #[ink::test]
        fn season_with_different_sport_types() {
            let mut sports_broker = SportsBroker::new();
            
            // Test various sport types
            let hockey_season = sports_broker.create_season(
                "2024-25 NHL Season".to_string(),
                SportType::Hockey,
                1696118400000,
                1719792000000, // June 30, 2025
                82,
            ).unwrap();

            let soccer_season = sports_broker.create_season(
                "2025 MLS Season".to_string(),
                SportType::Soccer,
                1708214400000, // Feb 18, 2025
                1730332800000, // Oct 31, 2025
                34,
            ).unwrap();

            let custom_season = sports_broker.create_season(
                "2025 Cricket League".to_string(),
                SportType::Other("Cricket".to_string()),
                1704067200000, // Jan 1, 2025
                1719792000000, // June 30, 2025
                20,
            ).unwrap();

            // Verify all seasons
            let hockey = sports_broker.get_season(hockey_season).unwrap();
            assert_eq!(hockey.sport_type, SportType::Hockey);

            let soccer = sports_broker.get_season(soccer_season).unwrap();
            assert_eq!(soccer.sport_type, SportType::Soccer);
            assert_eq!(soccer.regular_season_games, 34);

            let cricket = sports_broker.get_season(custom_season).unwrap();
            assert_eq!(cricket.sport_type, SportType::Other("Cricket".to_string()));
        }

        #[ink::test]
        fn full_workflow_teams_venues_seasons() {
            let mut sports_broker = SportsBroker::new();
            
            // Register teams
            let lakers_id = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();

            let celtics_id = sports_broker.register_team(
                "Celtics".to_string(),
                "Boston".to_string(),
                SportType::Basketball,
            ).unwrap();

            // Register venue
            let staples_id = sports_broker.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                20000,
            ).unwrap();

            // Create season
            let season_id = sports_broker.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            ).unwrap();

            // Verify everything is connected properly
            assert_eq!(sports_broker.total_teams(), 2);
            assert_eq!(sports_broker.total_venues(), 1);
            assert_eq!(sports_broker.total_seasons(), 1);

            // Check teams exist
            let lakers = sports_broker.get_team(lakers_id).unwrap();
            assert_eq!(lakers.sport_type, SportType::Basketball);

            let celtics = sports_broker.get_team(celtics_id).unwrap();
            assert_eq!(celtics.sport_type, SportType::Basketball);

            // Check venue exists  
            let venue = sports_broker.get_venue(staples_id).unwrap();
            assert_eq!(venue.capacity, 20000);

            // Check season exists
            let season = sports_broker.get_season(season_id).unwrap();
            assert_eq!(season.sport_type, SportType::Basketball);
            assert!(season.active);
        }

        #[ink::test]
        fn season_dates_validation() {
            let mut sports_broker = SportsBroker::new();
            
            let season_id = sports_broker.create_season(
                "Test Season".to_string(),
                SportType::Tennis,
                1609459200000, // Jan 1, 2021
                1640995200000, // Jan 1, 2022  
                50,
            ).unwrap();

            let season = sports_broker.get_season(season_id).unwrap();
            assert!(season.start_date < season.end_date);
            assert_eq!(season.start_date, 1609459200000);
            assert_eq!(season.end_date, 1640995200000);
        }
    }
}