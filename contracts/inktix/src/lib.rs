#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::arithmetic_side_effects)]

use ink::primitives::AccountId;

// Import all modular components
pub mod logic;
pub mod storage;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export key types for external use
pub use storage::contract_storage::InkTixStorage;
pub use types::*;

/// Unified InkTix Contract
///
/// A comprehensive event ticketing platform with:
/// - Core: Event creation, ticket purchasing, venue management, multi-currency, anti-scalping, XCM
/// - Sports (feature = "sports"): Teams, seasons, season passes, fantasy sports, team loyalty, analytics
/// - Concert (feature = "concert"): Artist management, concert-specific events
///
#[ink::contract]
#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
pub mod inktix {
    use super::*;
    use crate::logic::core::{
        anti_scalping, currency_management, event_management, nft_management, ticket_management, venue_management,
    };
    #[cfg(feature = "sports")]
    use crate::logic::sports::{
        advanced_team_loyalty, cross_chain_management, fantasy_sports_management,
        season_pass_management, team_management, analytics,
    };
    #[cfg(feature = "concert")]
    use crate::logic::concert::artist_management;
    use crate::storage::contract_storage::InkTixStorage;
    use crate::types::*;
    use crate::types::core::venue;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct InkTix {
        /// Main storage structure containing all contract data
        storage: InkTixStorage,
    }

    impl InkTix {
        /// Initialize the contract with default settings
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut storage = InkTixStorage::default();
            storage.owner = Self::env().caller();
            storage.initialize_currency_rates();
            Self { storage }
        }

        /// Get the contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.storage.owner
        }

        /// Check if caller is the owner
        fn ensure_owner(&self) -> Result<(), String> {
            if self.env().caller() != self.storage.owner {
                return Err("Only the owner can call this function".to_string());
            }
            Ok(())
        }

        // =============================================================================
        // CORE: VENUE MANAGEMENT
        // =============================================================================

        /// Register a new venue
        #[ink(message)]
        pub fn register_venue(
            &mut self, name: String, capacity: u32, location: String, venue_type: VenueType,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            venue_management::VenueManagement::register_venue(&mut self.storage, name, capacity, location, venue_type)
        }

        /// Get venue information
        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.storage.venues.get(venue_id)
        }

        /// Get all venues
        #[ink(message)]
        pub fn get_all_venues(&self) -> Vec<Venue> {
            venue_management::VenueManagement::get_all_venues(&self.storage)
        }

        /// Update venue capacity
        #[ink(message)]
        pub fn update_venue_capacity(&mut self, venue_id: u32, new_capacity: u32) -> Result<(), String> {
            self.ensure_owner()?;
            venue_management::VenueManagement::update_venue_capacity(&mut self.storage, venue_id, new_capacity)
        }

        // =============================================================================
        // CORE: EVENT MANAGEMENT
        // =============================================================================

        /// Create a new event with an EventCategory
        #[ink(message)]
        pub fn create_event(
            &mut self, name: String, venue_id: u32, date: u64, capacity: u32,
            base_price: u128, category: EventCategory,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            event_management::EventManagement::create_event(
                &mut self.storage, name, venue_id, date, capacity, base_price, category,
            )
        }

        /// Get event information
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<Event> {
            self.storage.events.get(event_id)
        }

        /// Get all events
        #[ink(message)]
        pub fn get_all_events(&self) -> Vec<Event> {
            event_management::EventManagement::get_all_events(&self.storage)
        }

        /// Update event status
        #[ink(message)]
        pub fn update_event_status(&mut self, event_id: u32, status: EventStatus) -> Result<(), String> {
            self.ensure_owner()?;
            event_management::EventManagement::update_event_status(&mut self.storage, event_id, status)
        }

        // =============================================================================
        // CORE: TICKET MANAGEMENT
        // =============================================================================

        /// Purchase a ticket
        #[ink(message)]
        pub fn purchase_ticket(&mut self, event_id: u32, seat: Seat, currency: CurrencyId) -> Result<u64, String> {
            let caller = self.env().caller();
            ticket_management::TicketManagement::purchase_ticket(&mut self.storage, caller, event_id, seat, currency)
        }

        /// Get ticket information
        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u64) -> Option<Ticket> {
            self.storage.tickets.get(ticket_id)
        }

        /// Get user's tickets
        #[ink(message)]
        pub fn get_user_tickets(&self, user: AccountId) -> Vec<u64> {
            self.storage.user_tickets.get(user).unwrap_or_default()
        }

        /// Transfer ticket to another user
        #[ink(message)]
        pub fn transfer_ticket(&mut self, ticket_id: u64, to: AccountId) -> Result<(), String> {
            let caller = self.env().caller();
            ticket_management::TicketManagement::transfer_ticket(&mut self.storage, caller, ticket_id, to)
        }

        /// Resell ticket
        #[ink(message)]
        pub fn resell_ticket(&mut self, ticket_id: u64, price: u128, currency: CurrencyId) -> Result<(), String> {
            let caller = self.env().caller();
            ticket_management::TicketManagement::resell_ticket(&mut self.storage, caller, ticket_id, price, currency)
        }

        // =============================================================================
        // CORE: CURRENCY MANAGEMENT
        // =============================================================================

        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> { self.storage.supported_currencies.clone() }

        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<u128> { self.storage.currency_rates.get(currency) }

        #[ink(message)]
        pub fn update_currency_rate(&mut self, currency: CurrencyId, rate: u128) -> Result<(), String> {
            self.ensure_owner()?;
            currency_management::CurrencyManagement::update_currency_rate(&mut self.storage, currency, rate)
        }

        // =============================================================================
        // CORE: ANTI-SCALPING
        // =============================================================================

        #[ink(message)]
        pub fn configure_anti_scalping(&mut self, event_id: u32, config: AntiScalpingConfig) -> Result<(), String> {
            self.ensure_owner()?;
            anti_scalping::AntiScalping::configure_anti_scalping(&mut self.storage, event_id, config)
        }

        #[ink(message)]
        pub fn get_anti_scalping_config(&self, event_id: u32) -> Option<AntiScalpingConfig> {
            self.storage.anti_scalping_configs.get(event_id)
        }

        // =============================================================================
        // CORE: NFT TICKET MANAGEMENT
        // =============================================================================

        /// Mint an NFT for an owned ticket
        #[ink(message)]
        pub fn mint_ticket_nft(&mut self, ticket_id: u64) -> Result<u64, String> {
            let caller = self.env().caller();
            nft_management::NftManagement::mint_ticket_nft(&mut self.storage, caller, ticket_id)
        }

        /// Verify a ticket NFT
        #[ink(message)]
        pub fn verify_ticket_nft(&self, token_id: u64) -> Result<TicketVerification, String> {
            nft_management::NftManagement::verify_ticket_nft(&self.storage, token_id)
        }

        /// Mark ticket as used at event entry, receive attendance token
        #[ink(message)]
        pub fn use_ticket_nft(&mut self, token_id: u64) -> Result<u64, String> {
            let caller = self.env().caller();
            nft_management::NftManagement::use_ticket_nft(&mut self.storage, caller, token_id)
        }

        /// Get all NFT tickets for a user
        #[ink(message)]
        pub fn get_user_nft_tickets(&self, user: AccountId) -> Vec<TicketNft> {
            nft_management::NftManagement::get_user_nft_tickets(&self.storage, user)
        }

        /// Get NFT for a specific ticket
        #[ink(message)]
        pub fn get_nft_by_ticket(&self, ticket_id: u64) -> Option<TicketNft> {
            nft_management::NftManagement::get_nft_by_ticket(&self.storage, ticket_id)
        }

        /// Transfer NFT to another owner
        #[ink(message)]
        pub fn transfer_nft(&mut self, token_id: u64, to: AccountId) -> Result<(), String> {
            let caller = self.env().caller();
            nft_management::NftManagement::transfer_nft(&mut self.storage, caller, token_id, to)
        }

        // =============================================================================
        // SPORTS: TEAM MANAGEMENT (feature = "sports")
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn register_team(&mut self, name: String, city: String, sport_type: SportType) -> Result<u32, String> {
            self.ensure_owner()?;
            team_management::TeamManagement::register_team(&mut self.storage, name, city, sport_type)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> { self.storage.teams.get(team_id) }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_all_teams(&self) -> Vec<Team> { team_management::TeamManagement::get_all_teams(&self.storage) }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn update_team(&mut self, team_id: u32, name: Option<String>, city: Option<String>, sport_type: Option<SportType>) -> Result<(), String> {
            self.ensure_owner()?;
            team_management::TeamManagement::update_team(&mut self.storage, team_id, name, city, sport_type)
        }

        // =============================================================================
        // SPORTS: SEASON PASS MANAGEMENT
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn create_season_pass_package(
            &mut self, name: String, team_id: u32, season_id: u32, price: u128,
            currency: CurrencyId, staking_requirement: u128, benefits: Vec<SeasonPassBenefits>,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            season_pass_management::SeasonPassManagement::create_season_pass_package(
                &mut self.storage, name, team_id, season_id, price, currency, staking_requirement, benefits,
            )
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn purchase_season_pass(&mut self, package_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            season_pass_management::SeasonPassManagement::purchase_season_pass(&mut self.storage, caller, package_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn use_season_pass_for_event(&mut self, season_pass_id: u32, event_id: u32) -> Result<u64, String> {
            let caller = self.env().caller();
            season_pass_management::SeasonPassManagement::use_season_pass_for_event(&mut self.storage, caller, season_pass_id, event_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_user_season_passes(&self, user: AccountId) -> Vec<u32> {
            self.storage.user_season_passes.get(user).unwrap_or_default()
        }

        // =============================================================================
        // SPORTS: FANTASY SPORTS
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn create_fantasy_league(&mut self, name: String, description: String, max_participants: u32, entry_fee: u128, currency: CurrencyId) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::create_fantasy_league(&mut self.storage, caller, name, description, max_participants, entry_fee, currency)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn join_fantasy_league(&mut self, league_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::join_fantasy_league(&mut self.storage, caller, league_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn create_fantasy_team(&mut self, league_id: u32, name: String) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::create_fantasy_team(&mut self.storage, caller, league_id, name)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_fantasy_leaderboard(&self, league_id: u32) -> Option<FantasyLeaderboard> {
            self.storage.fantasy_leaderboards.get(league_id)
        }

        // =============================================================================
        // SPORTS: TEAM LOYALTY
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn create_team_loyalty_profile(&mut self, team_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::create_team_loyalty_profile(&mut self.storage, caller, team_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn stake_on_team(&mut self, team_id: u32, amount: u128, currency: CurrencyId) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::stake_on_team(&mut self.storage, caller, team_id, amount, currency)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn record_attendance(&mut self, team_id: u32, event_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::record_attendance(&mut self.storage, caller, team_id, event_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_team_loyalty_profile(&self, user: AccountId, team_id: u32) -> Option<TeamLoyaltyProfile> {
            self.storage.team_loyalty_profiles.get((user, team_id))
        }

        // =============================================================================
        // SPORTS: VENUE-SPECIFIC FEATURES
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn purchase_parking_pass(&mut self, venue_id: u32, event_id: u32, pass_type: ParkingPassType) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_parking_pass(
                &mut self.storage, caller, venue_id, pass_type,
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>() + 365 * 24 * 60 * 60 * 1000,
                "Main Lot".to_string(), "DOT".to_string(),
            )
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn purchase_concession_credits(&mut self, venue_id: u32, amount: u128, currency: CurrencyId) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_concession_credits(
                &mut self.storage, caller, venue_id, amount, venue::ConcessionCreditType::General,
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>() + 365 * 24 * 60 * 60 * 1000,
                amount, "DOT".to_string(),
            )
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn purchase_merchandise_bundle(&mut self, venue_id: u32, bundle_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_merchandise_bundle(
                &mut self.storage, caller, venue_id, bundle_id, None, "Main Store".to_string(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>() + 7 * 24 * 60 * 60 * 1000,
            )
        }

        // =============================================================================
        // SPORTS: CROSS-CHAIN
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn create_cross_chain_event(&mut self, event_id: u32, target_chain: BlockchainNetwork) -> Result<u32, String> {
            self.ensure_owner()?;
            cross_chain_management::CrossChainManagement::create_cross_chain_event(&mut self.storage, event_id, target_chain)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn request_cross_chain_ticket_purchase(&mut self, event_id: u32, target_chain: BlockchainNetwork, seat: Seat, currency: CurrencyId) -> Result<u32, String> {
            let caller = self.env().caller();
            cross_chain_management::CrossChainManagement::request_cross_chain_ticket_purchase(&mut self.storage, caller, event_id, target_chain, seat, currency)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_chain_connectivity(&self, chain: BlockchainNetwork) -> Option<ChainConnectivityStatus> {
            self.storage.chain_connectivity.get(chain)
        }

        // =============================================================================
        // SPORTS: ANALYTICS
        // =============================================================================

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_platform_stats(&self) -> PlatformStats { self.storage.platform_stats.clone() }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn generate_analytics_report(&mut self, report_type: ReportType, start_date: u64, end_date: u64) -> Result<u32, String> {
            self.ensure_owner()?;
            analytics::Analytics::generate_analytics_report(&mut self.storage, report_type, start_date, end_date)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_analytics_report(&self, report_id: u32) -> Option<AnalyticsReport> {
            self.storage.analytics_reports.get(report_id)
        }

        #[cfg(feature = "sports")]
        #[ink(message)]
        pub fn get_event_analytics(&self, event_id: u32) -> Option<EventAnalytics> {
            self.storage.event_analytics.get(event_id)
        }

        // =============================================================================
        // CONCERT: ARTIST MANAGEMENT (feature = "concert")
        // =============================================================================

        /// Register a new artist
        #[cfg(feature = "concert")]
        #[ink(message)]
        pub fn register_artist(&mut self, name: String) -> Result<u32, String> {
            let caller = self.env().caller();
            artist_management::ArtistManagement::register_artist(&mut self.storage, caller, name)
        }

        /// Verify an artist (owner only)
        #[cfg(feature = "concert")]
        #[ink(message)]
        pub fn verify_artist(&mut self, artist_id: u32) -> Result<(), String> {
            self.ensure_owner()?;
            artist_management::ArtistManagement::verify_artist(&mut self.storage, artist_id)
        }

        /// Get artist information
        #[cfg(feature = "concert")]
        #[ink(message)]
        pub fn get_artist(&self, artist_id: u32) -> Option<Artist> {
            self.storage.artists.get(artist_id)
        }

        /// Create a concert event (convenience method)
        #[cfg(feature = "concert")]
        #[ink(message)]
        pub fn create_concert_event(
            &mut self, name: String, artist_id: u32, venue_id: u32,
            date: u64, capacity: u32, base_price: u128,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            // Auto-configure anti-scalping with concert preset
            let category = EventCategory::Concert { artist_id };
            let event_id = event_management::EventManagement::create_event(
                &mut self.storage, name, venue_id, date, capacity, base_price, category,
            )?;
            // Apply concert anti-scalping preset (4 tickets per user)
            let config = AntiScalpingConfig::concert_preset(event_id);
            self.storage.anti_scalping_configs.insert(event_id, &config);
            Ok(event_id)
        }

        // =============================================================================
        // UTILITY FUNCTIONS
        // =============================================================================

        /// Get total counts
        #[ink(message)]
        pub fn get_totals(&self) -> (u32, u32, u32, u64, u32, u32) {
            (
                self.storage.total_teams,
                self.storage.total_venues,
                self.storage.total_events,
                self.storage.total_tickets,
                self.storage.total_seasons,
                self.storage.total_season_passes,
            )
        }

        /// Check if analytics is enabled
        #[ink(message)]
        pub fn is_analytics_enabled(&self) -> bool { self.storage.analytics_enabled }

        /// Toggle analytics
        #[ink(message)]
        pub fn toggle_analytics(&mut self) -> Result<(), String> {
            self.ensure_owner()?;
            self.storage.analytics_enabled = !self.storage.analytics_enabled;
            Ok(())
        }
    }

    impl Default for InkTix {
        fn default() -> Self { Self::new() }
    }

    // =========================================================================
    // TESTS
    // =========================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_new_contract() {
            let contract = InkTix::new();
            let totals = contract.get_totals();
            assert_eq!(totals, (0, 0, 0, 0, 0, 0));
            assert!(contract.is_analytics_enabled());
        }

        #[ink::test]
        fn test_register_venue() {
            let mut contract = InkTix::new();
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 20000, "Los Angeles".to_string(), VenueType::Arena,
            ).unwrap();
            assert_eq!(venue_id, 1);
            let venue = contract.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.capacity, 20000);
        }

        #[cfg(feature = "sports")]
        #[ink::test]
        fn test_register_team() {
            let mut contract = InkTix::new();
            let team_id = contract.register_team(
                "Lakers".to_string(), "Los Angeles".to_string(), SportType::Basketball,
            ).unwrap();
            assert_eq!(team_id, 1);
            let team = contract.get_team(team_id).unwrap();
            assert_eq!(team.name, "Lakers");
            assert_eq!(team.city, "Los Angeles");
            assert_eq!(team.sport_type, SportType::Basketball);
        }

        #[cfg(feature = "sports")]
        #[ink::test]
        fn test_create_sports_event() {
            let mut contract = InkTix::new();

            let team1_id = contract.register_team("Lakers".to_string(), "Los Angeles".to_string(), SportType::Basketball).unwrap();
            let team2_id = contract.register_team("Warriors".to_string(), "San Francisco".to_string(), SportType::Basketball).unwrap();
            let venue_id = contract.register_venue("Staples Center".to_string(), 20000, "Los Angeles".to_string(), VenueType::Arena).unwrap();

            // Create a season (directly in storage for test)
            let season = crate::types::sports::season::Season {
                id: 1, name: "2024 Season".to_string(), sport_type: SportType::Basketball,
                start_date: 1000000000, end_date: 2000000000, regular_season_games: 82,
                active: true, season_pass_base_price: 1000000000000000000,
                early_bird_discount: 20, early_bird_deadline: 999999000,
            };
            contract.storage.seasons.insert(1, &season);
            contract.storage.total_seasons = 1;

            let event_id = contract.create_event(
                "Lakers vs Warriors".to_string(), venue_id, 1640995200, 20000,
                1000000000000000000,
                EventCategory::Sports {
                    home_team_id: team1_id, away_team_id: team2_id, season_id: 1,
                    game_type: GameType::RegularSeason, sport_type: SportType::Basketball,
                },
            ).unwrap();

            assert_eq!(event_id, 1);
            let event = contract.get_event(event_id).unwrap();
            assert_eq!(event.name, "Lakers vs Warriors");
            match &event.category {
                EventCategory::Sports { home_team_id, away_team_id, .. } => {
                    assert_eq!(*home_team_id, team1_id);
                    assert_eq!(*away_team_id, team2_id);
                }
                _ => panic!("Expected Sports event category"),
            }
        }

        #[cfg(feature = "concert")]
        #[ink::test]
        fn test_register_artist() {
            let mut contract = InkTix::new();
            let artist_id = contract.register_artist("Taylor Swift".to_string()).unwrap();
            assert_eq!(artist_id, 1);

            let artist = contract.get_artist(artist_id).unwrap();
            assert_eq!(artist.name, "Taylor Swift");
            assert!(!artist.verified);
        }

        #[cfg(feature = "concert")]
        #[ink::test]
        fn test_verify_artist() {
            let mut contract = InkTix::new();
            let artist_id = contract.register_artist("Taylor Swift".to_string()).unwrap();
            contract.verify_artist(artist_id).unwrap();

            let artist = contract.get_artist(artist_id).unwrap();
            assert!(artist.verified);
        }

        #[cfg(feature = "concert")]
        #[ink::test]
        fn test_create_concert_event() {
            let mut contract = InkTix::new();
            let artist_id = contract.register_artist("Taylor Swift".to_string()).unwrap();
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(), 20000, "New York".to_string(), VenueType::ConcertHall,
            ).unwrap();

            let event_id = contract.create_concert_event(
                "Eras Tour NYC".to_string(), artist_id, venue_id, 1640995200, 20000, 150000000000000000,
            ).unwrap();

            assert_eq!(event_id, 1);
            let event = contract.get_event(event_id).unwrap();
            assert_eq!(event.name, "Eras Tour NYC");
            match &event.category {
                EventCategory::Concert { artist_id: aid } => assert_eq!(*aid, artist_id),
                _ => panic!("Expected Concert event category"),
            }

            // Verify anti-scalping was auto-configured
            let config = contract.get_anti_scalping_config(event_id).unwrap();
            assert_eq!(config.max_tickets_per_user, 4);
            assert!(config.anti_bot_measures);
        }

        #[cfg(feature = "concert")]
        #[ink::test]
        fn test_concert_purchase_limit() {
            let mut contract = InkTix::new();
            let artist_id = contract.register_artist("Artist".to_string()).unwrap();
            let venue_id = contract.register_venue("Venue".to_string(), 100, "City".to_string(), VenueType::ConcertHall).unwrap();
            let event_id = contract.create_concert_event("Concert".to_string(), artist_id, venue_id, 1, 100, 10).unwrap();

            let seat = Seat {
                section: "A".to_string(), row: "1".to_string(), seat_number: "1".to_string(),
                seat_type: SeatType::GeneralAdmission, access_level: AccessLevel::Standard, price_multiplier: 10000,
            };

            // Should be able to purchase 4 tickets
            for _ in 0..4 {
                contract.purchase_ticket(event_id, seat.clone(), CurrencyId::DOT).unwrap();
            }

            // 5th ticket should fail
            let result = contract.purchase_ticket(event_id, seat.clone(), CurrencyId::DOT);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Purchase limit reached");
        }

        #[ink::test]
        fn test_mint_ticket_nft() {
            let mut contract = InkTix::new();
            // Setup: create venue and event
            let venue_id = contract.register_venue("Arena".to_string(), 1000, "LA".to_string(), VenueType::Arena).unwrap();
            let event_id = contract.create_event("Game".to_string(), venue_id, 1000, 1000, 100, EventCategory::Generic).unwrap();
            // Purchase ticket
            let ticket_id = contract.purchase_ticket(event_id, Seat {
                seat_number: "1".to_string(), section: "A".to_string(), row: "1".to_string(),
                seat_type: SeatType::GeneralAdmission, access_level: AccessLevel::Standard,
                price_multiplier: 10000,
            }, CurrencyId::DOT).unwrap();
            // Mint NFT
            let token_id = contract.mint_ticket_nft(ticket_id).unwrap();
            assert_eq!(token_id, 1);
            // Verify
            let verification = contract.verify_ticket_nft(token_id).unwrap();
            assert!(verification.is_valid);
            assert!(!verification.is_used);
            // Can't mint again
            assert!(contract.mint_ticket_nft(ticket_id).is_err());
        }

        #[ink::test]
        fn test_use_ticket_nft() {
            let mut contract = InkTix::new();
            let venue_id = contract.register_venue("Arena".to_string(), 1000, "LA".to_string(), VenueType::Arena).unwrap();
            let event_id = contract.create_event("Game".to_string(), venue_id, 1000, 1000, 100, EventCategory::Generic).unwrap();
            let ticket_id = contract.purchase_ticket(event_id, Seat {
                seat_number: "1".to_string(), section: "A".to_string(), row: "1".to_string(),
                seat_type: SeatType::GeneralAdmission, access_level: AccessLevel::Standard,
                price_multiplier: 10000,
            }, CurrencyId::DOT).unwrap();
            let token_id = contract.mint_ticket_nft(ticket_id).unwrap();
            // Use ticket
            let attendance_id = contract.use_ticket_nft(token_id).unwrap();
            assert_eq!(attendance_id, 1);
            // Verify shows used
            let verification = contract.verify_ticket_nft(token_id).unwrap();
            assert!(verification.is_used);
            // Can't use again
            assert!(contract.use_ticket_nft(token_id).is_err());
        }

        #[ink::test]
        fn test_currency_management() {
            let contract = InkTix::new();
            let currencies = contract.get_supported_currencies();
            assert!(currencies.contains(&CurrencyId::DOT));
            let dot_rate = contract.get_currency_rate(CurrencyId::DOT).unwrap();
            assert_eq!(dot_rate, 1_000_000_000_000_000_000);
        }

        #[ink::test]
        fn test_generic_event() {
            let mut contract = InkTix::new();
            let venue_id = contract.register_venue("Hall".to_string(), 500, "City".to_string(), VenueType::ConventionCenter).unwrap();
            let event_id = contract.create_event(
                "Conference".to_string(), venue_id, 1640995200, 500, 50000000000000000, EventCategory::Generic,
            ).unwrap();
            let event = contract.get_event(event_id).unwrap();
            assert_eq!(event.category, EventCategory::Generic);
        }
    }
}
