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

        // Sports Event management
        events: ink::storage::Mapping<u32, SportsEvent>,
        next_event_id: u32,
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

    /// Enhanced Event structure for sports
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SportsEvent {
        // Core fields
        pub id: u32,
        pub name: String,
        pub venue_id: u32,
        pub date: u64, // Unix timestamp
        pub capacity: u32,
        pub sold_tickets: u32,
        pub base_price: Balance, // Price in smallest unit (plancks for DOT)
        pub active: bool,
        
        // Sports-specific fields
        pub sport_type: SportType,
        pub home_team_id: u32,
        pub away_team_id: u32,
        pub season_id: u32,
        pub game_type: GameType,
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

    /// Game/Event types
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum GameType {
        RegularSeason,
        Playoff,
        Championship,
        AllStar,
        Preseason,
        Tournament,
        Exhibition,
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
        /// Event not found
        EventNotFound,
        /// Event not active
        EventNotActive,
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
                events: ink::storage::Mapping::new(),
                next_event_id: 1,
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

        /// Create a sports event with team information
        #[ink(message)]
        pub fn create_sports_event(
            &mut self,
            name: String,
            venue_id: u32,
            date: u64,
            capacity: u32,
            base_price: Balance,
            home_team_id: u32,
            away_team_id: u32,
            season_id: u32,
            game_type: GameType,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Validate venue exists
            let venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            
            // Validate home team exists
            let home_team = self.teams.get(home_team_id).ok_or(Error::TeamNotFound)?;
            
            // Validate away team exists
            let _away_team = self.teams.get(away_team_id).ok_or(Error::TeamNotFound)?;
            
            // Validate season exists
            let season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;

            // Use venue capacity as default if provided capacity is 0
            let event_capacity = if capacity == 0 { venue.capacity } else { capacity };

            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let sports_event = SportsEvent {
                id: event_id,
                name,
                venue_id,
                date,
                capacity: event_capacity,
                sold_tickets: 0,
                base_price,
                active: true,
                
                // Sports-specific fields
                sport_type: home_team.sport_type.clone(),
                home_team_id,
                away_team_id,
                season_id,
                game_type,
            };

            // Ensure sport types are consistent
            if home_team.sport_type != season.sport_type {
                // For now, we'll use the team's sport type but this could be enhanced
            }

            self.events.insert(event_id, &sports_event);
            Ok(event_id)
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

        /// Get sports event information
        #[ink(message)]
        pub fn get_sports_event(&self, event_id: u32) -> Option<SportsEvent> {
            self.events.get(event_id)
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

        /// Get total events created
        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Unit tests for Step 4 - Sports Event Creation
    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn setup_test_data(contract: &mut SportsBroker) -> (u32, u32, u32, u32) {
            // Create venue
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                "New York".to_string(),
                20000,
            ).unwrap();

            // Create teams
            let home_team_id = contract.register_team(
                "New York Knicks".to_string(),
                "New York".to_string(),
                SportType::Basketball,
            ).unwrap();

            let away_team_id = contract.register_team(
                "Boston Celtics".to_string(),
                "Boston".to_string(),
                SportType::Basketball,
            ).unwrap();

            // Create season
            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000, // Oct 1, 2024
                1715644800000, // May 14, 2025
                82,
            ).unwrap();

            (venue_id, home_team_id, away_team_id, season_id)
        }

        #[ink::test]
        fn new_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
            assert_eq!(sports_broker.total_seasons(), 0);
            assert_eq!(sports_broker.total_events(), 0);
        }

        #[ink::test]
        fn create_sports_event_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            let result = sports_broker.create_sports_event(
                "Knicks vs Celtics".to_string(),
                venue_id,
                1704067200000, // Jan 1, 2025
                18000,
                50_000_000_000_000, // 0.05 DOT base price
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert!(result.is_ok());
            let event_id = result.unwrap();
            assert_eq!(event_id, 1);
            assert_eq!(sports_broker.total_events(), 1);

            // Verify event data
            let event = sports_broker.get_sports_event(event_id).unwrap();
            assert_eq!(event.name, "Knicks vs Celtics");
            assert_eq!(event.home_team_id, home_team_id);
            assert_eq!(event.away_team_id, away_team_id);
            assert_eq!(event.season_id, season_id);
            assert_eq!(event.venue_id, venue_id);
            assert_eq!(event.sport_type, SportType::Basketball);
            assert_eq!(event.game_type, GameType::RegularSeason);
            assert_eq!(event.capacity, 18000);
            assert_eq!(event.sold_tickets, 0);
            assert!(event.active);
        }

        #[ink::test]
        fn create_sports_event_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = sports_broker.create_sports_event(
                "Unauthorized Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::NotOwner));
            assert_eq!(sports_broker.total_events(), 0);
        }

        #[ink::test]
        fn create_sports_event_invalid_venue() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            let result = sports_broker.create_sports_event(
                "Invalid Venue Game".to_string(),
                999, // Invalid venue ID
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::VenueNotFound));
        }

        #[ink::test]
        fn create_sports_event_invalid_team() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, _, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            let result = sports_broker.create_sports_event(
                "Invalid Team Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                999, // Invalid home team ID
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::TeamNotFound));
        }

        #[ink::test]
        fn create_sports_event_invalid_season() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, _) = setup_test_data(&mut sports_broker);

            let result = sports_broker.create_sports_event(
                "Invalid Season Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                999, // Invalid season ID
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::SeasonNotFound));
        }

        #[ink::test]
        fn create_multiple_sports_events() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            // Create regular season game
            let regular_game = sports_broker.create_sports_event(
                "Regular Season Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Create playoff game
            let playoff_game = sports_broker.create_sports_event(
                "Playoff Game".to_string(),
                venue_id,
                1715644800000,
                18000,
                100_000_000_000_000, // Double price for playoffs
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            assert_eq!(regular_game, 1);
            assert_eq!(playoff_game, 2);
            assert_eq!(sports_broker.total_events(), 2);

            // Verify different game types
            let regular = sports_broker.get_sports_event(regular_game).unwrap();
            assert_eq!(regular.game_type, GameType::RegularSeason);
            assert_eq!(regular.base_price, 50_000_000_000_000);

            let playoff = sports_broker.get_sports_event(playoff_game).unwrap();
            assert_eq!(playoff.game_type, GameType::Playoff);
            assert_eq!(playoff.base_price, 100_000_000_000_000);
        }

        #[ink::test]
        fn create_sports_event_uses_venue_capacity() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            // Create event with capacity = 0 (should use venue capacity)
            let event_id = sports_broker.create_sports_event(
                "Venue Capacity Game".to_string(),
                venue_id,
                1704067200000,
                0, // Use venue capacity
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let event = sports_broker.get_sports_event(event_id).unwrap();
            assert_eq!(event.capacity, 20000); // Should match venue capacity from setup
        }

        #[ink::test]
        fn get_nonexistent_event_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_sports_event(999), None);
        }

        #[ink::test]
        fn different_game_types_work() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut sports_broker);

            // Test different game types
            let championship = sports_broker.create_sports_event(
                "Championship Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                200_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::Championship,
            ).unwrap();

            let allstar = sports_broker.create_sports_event(
                "All-Star Game".to_string(),
                venue_id,
                1704153600000,
                18000,
                150_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::AllStar,
            ).unwrap();

            // Verify game types
            let champ_event = sports_broker.get_sports_event(championship).unwrap();
            assert_eq!(champ_event.game_type, GameType::Championship);

            let all_star_event = sports_broker.get_sports_event(allstar).unwrap();
            assert_eq!(all_star_event.game_type, GameType::AllStar);
        }
    }
}