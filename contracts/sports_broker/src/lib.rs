#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;

// Import all modular components
pub mod logic;
pub mod storage;
pub mod types;
pub mod utils;

// Re-export key types for external use
pub use storage::contract_storage::SportsBrokerStorage;
pub use types::*;

/// Comprehensive Sports Broker Contract
///
/// A full-featured sports ticketing platform with advanced functionality:
/// - Team and venue management
/// - Event creation and management
/// - Advanced ticket purchasing with anti-scalping
/// - Season pass management with DeFi staking
/// - Fantasy sports integration
/// - Team loyalty programs with staking
/// - Venue-specific features (parking, concessions, merchandise)
/// - Cross-chain functionality via XCM
/// - Comprehensive analytics and reporting
/// - Multi-currency support
///
#[ink::contract]
#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
pub mod sports_broker {
    use super::*;
    use crate::logic::analytics;
    use crate::logic::anti_scalping;
    use crate::logic::{
        advanced_team_loyalty, cross_chain_management, currency_management, event_management,
        fantasy_sports_management, season_pass_management, team_management, ticket_management,
        venue_management,
    };
    use crate::storage::contract_storage::SportsBrokerStorage;
    use crate::types::*;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;

    #[ink(storage)]
    pub struct SportsBroker {
        /// Main storage structure containing all contract data
        storage: SportsBrokerStorage,
    }

    impl SportsBroker {
        /// Initialize the contract with default settings
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut storage = SportsBrokerStorage::default();
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
        // CORE TEAM MANAGEMENT
        // =============================================================================

        /// Register a new team
        #[ink(message)]
        pub fn register_team(
            &mut self,
            name: String,
            city: String,
            sport_type: SportType,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            team_management::TeamManagement::register_team(
                &mut self.storage,
                name,
                city,
                sport_type,
            )
        }

        /// Get team information
        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.storage.teams.get(team_id)
        }

        /// Get all teams
        #[ink(message)]
        pub fn get_all_teams(&self) -> Vec<Team> {
            team_management::TeamManagement::get_all_teams(&self.storage)
        }

        /// Update team information
        #[ink(message)]
        pub fn update_team(
            &mut self,
            team_id: u32,
            name: Option<String>,
            city: Option<String>,
            sport_type: Option<SportType>,
        ) -> Result<(), String> {
            self.ensure_owner()?;
            team_management::TeamManagement::update_team(
                &mut self.storage,
                team_id,
                name,
                city,
                sport_type,
            )
        }

        // =============================================================================
        // VENUE MANAGEMENT
        // =============================================================================

        /// Register a new venue
        #[ink(message)]
        pub fn register_venue(
            &mut self,
            name: String,
            capacity: u32,
            location: String,
            venue_type: VenueType,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            venue_management::VenueManagement::register_venue(
                &mut self.storage,
                name,
                capacity,
                location,
                venue_type,
            )
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
        pub fn update_venue_capacity(
            &mut self,
            venue_id: u32,
            new_capacity: u32,
        ) -> Result<(), String> {
            self.ensure_owner()?;
            venue_management::VenueManagement::update_venue_capacity(
                &mut self.storage,
                venue_id,
                new_capacity,
            )
        }

        // =============================================================================
        // EVENT MANAGEMENT
        // =============================================================================

        /// Create a new sports event
        #[ink(message)]
        pub fn create_event(
            &mut self,
            name: String,
            venue_id: u32,
            date: u64,
            capacity: u32,
            base_price: u128,
            sport_type: SportType,
            home_team_id: u32,
            away_team_id: u32,
            season_id: u32,
            game_type: GameType,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            event_management::EventManagement::create_event(
                &mut self.storage,
                name,
                venue_id,
                date,
                capacity,
                base_price,
                sport_type,
                home_team_id,
                away_team_id,
                season_id,
                game_type,
            )
        }

        /// Get event information
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<SportsEvent> {
            self.storage.events.get(event_id)
        }

        /// Get all events
        #[ink(message)]
        pub fn get_all_events(&self) -> Vec<SportsEvent> {
            event_management::EventManagement::get_all_events(&self.storage)
        }

        /// Update event status
        #[ink(message)]
        pub fn update_event_status(
            &mut self,
            event_id: u32,
            status: EventStatus,
        ) -> Result<(), String> {
            self.ensure_owner()?;
            event_management::EventManagement::update_event_status(
                &mut self.storage,
                event_id,
                status,
            )
        }

        // =============================================================================
        // TICKET MANAGEMENT
        // =============================================================================

        /// Purchase a ticket
        #[ink(message)]
        pub fn purchase_ticket(
            &mut self,
            event_id: u32,
            seat: Seat,
            currency: CurrencyId,
        ) -> Result<u64, String> {
            let caller = self.env().caller();
            ticket_management::TicketManagement::purchase_ticket(
                &mut self.storage,
                caller,
                event_id,
                seat,
                currency,
            )
        }

        /// Get ticket information
        #[ink(message)]
        pub fn get_ticket(&self, ticket_id: u64) -> Option<SportsTicket> {
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
            ticket_management::TicketManagement::transfer_ticket(
                &mut self.storage,
                caller,
                ticket_id,
                to,
            )
        }

        /// Resell ticket
        #[ink(message)]
        pub fn resell_ticket(
            &mut self,
            ticket_id: u64,
            price: u128,
            currency: CurrencyId,
        ) -> Result<(), String> {
            let caller = self.env().caller();
            ticket_management::TicketManagement::resell_ticket(
                &mut self.storage,
                caller,
                ticket_id,
                price,
                currency,
            )
        }

        // =============================================================================
        // SEASON PASS MANAGEMENT
        // =============================================================================

        /// Create a season pass package
        #[ink(message)]
        pub fn create_season_pass_package(
            &mut self,
            name: String,
            team_id: u32,
            season_id: u32,
            price: u128,
            currency: CurrencyId,
            staking_requirement: u128,
            benefits: Vec<SeasonPassBenefits>,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            season_pass_management::SeasonPassManagement::create_season_pass_package(
                &mut self.storage,
                name,
                team_id,
                season_id,
                price,
                currency,
                staking_requirement,
                benefits,
            )
        }

        /// Purchase a season pass
        #[ink(message)]
        pub fn purchase_season_pass(&mut self, package_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            season_pass_management::SeasonPassManagement::purchase_season_pass(
                &mut self.storage,
                caller,
                package_id,
            )
        }

        /// Use season pass for event
        #[ink(message)]
        pub fn use_season_pass_for_event(
            &mut self,
            season_pass_id: u32,
            event_id: u32,
        ) -> Result<u64, String> {
            let caller = self.env().caller();
            season_pass_management::SeasonPassManagement::use_season_pass_for_event(
                &mut self.storage,
                caller,
                season_pass_id,
                event_id,
            )
        }

        /// Get user's season passes
        #[ink(message)]
        pub fn get_user_season_passes(&self, user: AccountId) -> Vec<u32> {
            self.storage
                .user_season_passes
                .get(user)
                .unwrap_or_default()
        }

        // =============================================================================
        // FANTASY SPORTS MANAGEMENT
        // =============================================================================

        /// Create a fantasy league
        #[ink(message)]
        pub fn create_fantasy_league(
            &mut self,
            name: String,
            description: String,
            max_participants: u32,
            entry_fee: u128,
            currency: CurrencyId,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::create_fantasy_league(
                &mut self.storage,
                caller,
                name,
                description,
                max_participants,
                entry_fee,
                currency,
            )
        }

        /// Join a fantasy league
        #[ink(message)]
        pub fn join_fantasy_league(&mut self, league_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::join_fantasy_league(
                &mut self.storage,
                caller,
                league_id,
            )
        }

        /// Create fantasy team
        #[ink(message)]
        pub fn create_fantasy_team(&mut self, league_id: u32, name: String) -> Result<u32, String> {
            let caller = self.env().caller();
            fantasy_sports_management::FantasySportsManagement::create_fantasy_team(
                &mut self.storage,
                caller,
                league_id,
                name,
            )
        }

        /// Get fantasy league leaderboard
        #[ink(message)]
        pub fn get_fantasy_leaderboard(&self, league_id: u32) -> Option<FantasyLeaderboard> {
            self.storage.fantasy_leaderboards.get(league_id)
        }

        // =============================================================================
        // TEAM LOYALTY PROGRAMS
        // =============================================================================

        /// Create team loyalty profile
        #[ink(message)]
        pub fn create_team_loyalty_profile(&mut self, team_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::create_team_loyalty_profile(
                &mut self.storage,
                caller,
                team_id,
            )
        }

        /// Stake on favorite team
        #[ink(message)]
        pub fn stake_on_team(
            &mut self,
            team_id: u32,
            amount: u128,
            currency: CurrencyId,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::stake_on_team(
                &mut self.storage,
                caller,
                team_id,
                amount,
                currency,
            )
        }

        /// Record attendance for team loyalty
        #[ink(message)]
        pub fn record_attendance(&mut self, team_id: u32, event_id: u32) -> Result<u32, String> {
            let caller = self.env().caller();
            advanced_team_loyalty::AdvancedTeamLoyalty::record_attendance(
                &mut self.storage,
                caller,
                team_id,
                event_id,
            )
        }

        /// Get team loyalty profile
        #[ink(message)]
        pub fn get_team_loyalty_profile(
            &self,
            user: AccountId,
            team_id: u32,
        ) -> Option<TeamLoyaltyProfile> {
            self.storage.team_loyalty_profiles.get((user, team_id))
        }

        // =============================================================================
        // VENUE-SPECIFIC FEATURES
        // =============================================================================

        /// Purchase parking pass
        #[ink(message)]
        pub fn purchase_parking_pass(
            &mut self,
            venue_id: u32,
            event_id: u32,
            pass_type: ParkingPassType,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_parking_pass(
                &mut self.storage,
                caller,
                venue_id,
                pass_type,
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>()
                    + 365 * 24 * 60 * 60 * 1000,
                "Main Lot".to_string(),
                "DOT".to_string(),
            )
        }

        /// Purchase concession credits
        #[ink(message)]
        pub fn purchase_concession_credits(
            &mut self,
            venue_id: u32,
            amount: u128,
            currency: CurrencyId,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_concession_credits(
                &mut self.storage,
                caller,
                venue_id,
                amount,
                venue::ConcessionCreditType::General,
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>()
                    + 365 * 24 * 60 * 60 * 1000,
                amount,
                "DOT".to_string(),
            )
        }

        /// Purchase merchandise bundle
        #[ink(message)]
        pub fn purchase_merchandise_bundle(
            &mut self,
            venue_id: u32,
            bundle_id: u32,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            venue_management::VenueManagement::purchase_merchandise_bundle(
                &mut self.storage,
                caller,
                venue_id,
                bundle_id,
                None,
                "Main Store".to_string(),
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>()
                    + 7 * 24 * 60 * 60 * 1000,
            )
        }

        // =============================================================================
        // CROSS-CHAIN FUNCTIONALITY
        // =============================================================================

        /// Create cross-chain event
        #[ink(message)]
        pub fn create_cross_chain_event(
            &mut self,
            event_id: u32,
            target_chain: BlockchainNetwork,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            cross_chain_management::CrossChainManagement::create_cross_chain_event(
                &mut self.storage,
                event_id,
                target_chain,
            )
        }

        /// Request cross-chain ticket purchase
        #[ink(message)]
        pub fn request_cross_chain_ticket_purchase(
            &mut self,
            event_id: u32,
            target_chain: BlockchainNetwork,
            seat: Seat,
            currency: CurrencyId,
        ) -> Result<u32, String> {
            let caller = self.env().caller();
            cross_chain_management::CrossChainManagement::request_cross_chain_ticket_purchase(
                &mut self.storage,
                caller,
                event_id,
                target_chain,
                seat,
                currency,
            )
        }

        /// Get cross-chain connectivity status
        #[ink(message)]
        pub fn get_chain_connectivity(
            &self,
            chain: BlockchainNetwork,
        ) -> Option<ChainConnectivityStatus> {
            self.storage.chain_connectivity.get(chain)
        }

        // =============================================================================
        // ANALYTICS AND REPORTING
        // =============================================================================

        /// Get platform statistics
        #[ink(message)]
        pub fn get_platform_stats(&self) -> PlatformStats {
            self.storage.platform_stats.clone()
        }

        /// Generate analytics report
        #[ink(message)]
        pub fn generate_analytics_report(
            &mut self,
            report_type: ReportType,
            start_date: u64,
            end_date: u64,
        ) -> Result<u32, String> {
            self.ensure_owner()?;
            analytics::Analytics::generate_analytics_report(
                &mut self.storage,
                report_type,
                start_date,
                end_date,
            )
        }

        /// Get analytics report
        #[ink(message)]
        pub fn get_analytics_report(&self, report_id: u32) -> Option<AnalyticsReport> {
            self.storage.analytics_reports.get(report_id)
        }

        /// Get event analytics
        #[ink(message)]
        pub fn get_event_analytics(&self, event_id: u32) -> Option<EventAnalytics> {
            self.storage.event_analytics.get(event_id)
        }

        // =============================================================================
        // CURRENCY MANAGEMENT
        // =============================================================================

        /// Get supported currencies
        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.storage.supported_currencies.clone()
        }

        /// Get currency rate
        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<u128> {
            self.storage.currency_rates.get(currency)
        }

        /// Update currency rate
        #[ink(message)]
        pub fn update_currency_rate(
            &mut self,
            currency: CurrencyId,
            rate: u128,
        ) -> Result<(), String> {
            self.ensure_owner()?;
            currency_management::CurrencyManagement::update_currency_rate(
                &mut self.storage,
                currency,
                rate,
            )
        }

        // =============================================================================
        // ANTI-SCALPING FEATURES
        // =============================================================================

        /// Configure anti-scalping for event
        #[ink(message)]
        pub fn configure_anti_scalping(
            &mut self,
            event_id: u32,
            config: AntiScalpingConfig,
        ) -> Result<(), String> {
            self.ensure_owner()?;
            anti_scalping::AntiScalping::configure_anti_scalping(
                &mut self.storage,
                event_id,
                config,
            )
        }

        /// Get anti-scalping configuration
        #[ink(message)]
        pub fn get_anti_scalping_config(&self, event_id: u32) -> Option<AntiScalpingConfig> {
            self.storage.anti_scalping_configs.get(event_id)
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
        pub fn is_analytics_enabled(&self) -> bool {
            self.storage.analytics_enabled
        }

        /// Toggle analytics
        #[ink(message)]
        pub fn toggle_analytics(&mut self) -> Result<(), String> {
            self.ensure_owner()?;
            self.storage.analytics_enabled = !self.storage.analytics_enabled;
            Ok(())
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

        #[ink::test]
        fn test_new_contract() {
            let contract = SportsBroker::new();
            let totals = contract.get_totals();
            assert_eq!(totals, (0, 0, 0, 0, 0, 0));
            assert!(contract.is_analytics_enabled());
        }

        #[ink::test]
        fn test_register_team() {
            let mut contract = SportsBroker::new();
            let team_id = contract
                .register_team(
                    "Lakers".to_string(),
                    "Los Angeles".to_string(),
                    SportType::Basketball,
                )
                .unwrap();
            assert_eq!(team_id, 1);

            let team = contract.get_team(team_id).unwrap();
            assert_eq!(team.name, "Lakers");
            assert_eq!(team.city, "Los Angeles");
            assert_eq!(team.sport_type, SportType::Basketball);
        }

        #[ink::test]
        fn test_register_venue() {
            let mut contract = SportsBroker::new();
            let venue_id = contract
                .register_venue(
                    "Staples Center".to_string(),
                    20000,
                    "Los Angeles".to_string(),
                    VenueType::Arena,
                )
                .unwrap();
            assert_eq!(venue_id, 1);

            let venue = contract.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.capacity, 20000);
        }

        #[ink::test]
        fn test_create_event() {
            let mut contract = SportsBroker::new();

            // First create teams and venue
            let team1_id = contract
                .register_team(
                    "Lakers".to_string(),
                    "Los Angeles".to_string(),
                    SportType::Basketball,
                )
                .unwrap();

            let team2_id = contract
                .register_team(
                    "Warriors".to_string(),
                    "San Francisco".to_string(),
                    SportType::Basketball,
                )
                .unwrap();

            let venue_id = contract
                .register_venue(
                    "Staples Center".to_string(),
                    20000,
                    "Los Angeles".to_string(),
                    VenueType::Arena,
                )
                .unwrap();

            // Create season
            let season_id = 1; // Assuming season management is implemented

            let event_id = contract
                .create_event(
                    "Lakers vs Warriors".to_string(),
                    venue_id,
                    1640995200, // Timestamp
                    20000,
                    1000000000000000000, // 1 DOT
                    SportType::Basketball,
                    team1_id,
                    team2_id,
                    season_id,
                    GameType::RegularSeason,
                )
                .unwrap();

            assert_eq!(event_id, 1);

            let event = contract.get_event(event_id).unwrap();
            assert_eq!(event.name, "Lakers vs Warriors");
            assert_eq!(event.home_team_id, team1_id);
            assert_eq!(event.away_team_id, team2_id);
        }
    }
}
