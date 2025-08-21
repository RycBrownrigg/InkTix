#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::*;
use ink::storage::Mapping;

// Import our modular components
pub mod types;
pub mod storage;
pub mod logic;
pub mod utils;
pub mod tests;

use types::*;
use storage::*;
use logic::*;

/// Sports Broker Contract
/// 
/// A comprehensive sports ticketing platform with:
/// - Team and venue management
/// - Dynamic pricing based on performance
/// - Multi-currency support
/// - Analytics and reporting
/// - Anti-scalping mechanisms
/// - Loyalty and rewards system
#[ink::contract]
pub mod sports_broker {
    use super::*;

    #[ink(storage)]
    pub struct SportsBroker {
        // Use our modular storage
        storage: SportsBrokerStorage,
    }

    impl SportsBroker {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut storage = SportsBrokerStorage::default();
            storage.owner = Self::env().caller();
            storage.initialize_currency_rates();
            
            Self { storage }
        }

        // Delegate to module implementations
        #[ink(message)]
        pub fn register_team(&mut self, name: String, sport: String, city: String) -> u32 {
            TeamManagement::register_team(&mut self.storage, name, sport, city)
        }

        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            TeamManagement::get_team(&self.storage, team_id)
        }

        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            VenueManagement::get_venue(&self.storage, venue_id)
        }

        #[ink(message)]
        pub fn update_team_performance(
            &mut self,
            team_id: u32,
            season_id: u32,
            wins: u32,
            losses: u32,
            points_scored: u32,
            playoff_rounds: u32,
            points_allowed: u32,
            total_games: u32,
        ) -> Result<(), String> {
            TeamManagement::update_team_performance(
                &mut self.storage,
                team_id,
                season_id,
                wins,
                losses,
                points_scored,
                playoff_rounds,
                points_allowed,
                total_games,
            )
        }

        #[ink(message)]
        pub fn get_team_performance(&self, team_id: u32) -> Option<TeamPerformance> {
            TeamManagement::get_team_performance(&self.storage, team_id)
        }

        #[ink(message)]
        pub fn get_pricing_multiplier(&self, team_id: u32) -> Option<PricingMultiplier> {
            TeamManagement::get_pricing_multiplier(&self.storage, team_id)
        }

        // Basic getters for contract state
        #[ink(message)]
        pub fn get_owner(&self) -> ink::primitives::AccountId {
            self.storage.owner
        }

        #[ink(message)]
        pub fn get_stats(&self) -> (u32, u32, u32, u32, u32) {
            (
                self.storage.total_teams,
                self.storage.total_venues,
                self.storage.total_events,
                self.storage.total_tickets as u32,
                self.storage.total_seasons,
            )
        }

        #[ink(message)]
        pub fn is_analytics_enabled(&self) -> bool {
            self.storage.analytics_enabled
        }

        // Delegate to module implementations
        #[ink(message)]
        pub fn register_venue(&mut self, name: String, capacity: u32, address: String, sport_type: String) -> u32 {
            VenueManagement::register_venue(&mut self.storage, name, capacity, address, sport_type)
        }

        #[ink(message)]
        pub fn create_season(&mut self, name: String, sport: String, year: u32, start_date: u64, end_date: u64) -> u32 {
            SeasonManagement::create_season(&mut self.storage, name, sport, year, start_date, end_date)
        }

        #[ink(message)]
        pub fn create_sports_event(
            &mut self,
            _name: String,
            _home_team_id: u32,
            _away_team_id: u32,
            _venue_id: u32,
            _season_id: u32,
            _event_time: u64,
            _venue_capacity: u32,
            _base_ticket_price: u128,
            _game_type: GameType,
        ) -> u32 {
            // TODO: Implement in event_management module
            0
        }

        #[ink(message)]
        pub fn purchase_ticket(&mut self, _event_id: u32, _section: String, _row: String, _seat: u32) -> Result<u64, String> {
            // TODO: Implement in ticket_management module
            Err("Not implemented yet".to_string())
        }

        #[ink(message)]
        pub fn purchase_ticket_with_currency(
            &mut self,
            _event_id: u32,
            _section: String,
            _row: String,
            _seat: u32,
            _currency: CurrencyId,
        ) -> Result<u64, String> {
            // TODO: Implement in ticket_management module
            Err("Not implemented yet".to_string())
        }

        // Currency management placeholders
        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.storage.supported_currencies.clone()
        }

        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<u128> {
            self.storage.currency_rates.get(currency)
        }

        #[ink(message)]
        pub fn get_currency_revenue(&self, currency: CurrencyId) -> u128 {
            self.storage.currency_revenue.get(currency).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_revenue_dot(&self) -> u128 {
            self.storage.platform_stats.total_revenue
        }

        // Analytics placeholders
        #[ink(message)]
        pub fn get_platform_stats(&self) -> PlatformStats {
            self.storage.platform_stats.clone()
        }

        #[ink(message)]
        pub fn get_event_analytics(&self, event_id: u32) -> Option<EventAnalytics> {
            self.storage.event_analytics.get(event_id)
        }

        #[ink(message)]
        pub fn get_team_analytics(&self, team_id: u32) -> Option<TeamAnalytics> {
            self.storage.team_analytics.get(team_id)
        }

        #[ink(message)]
        pub fn get_user_analytics(&self, user: ink::primitives::AccountId) -> Option<UserAnalytics> {
            self.storage.user_analytics.get(user)
        }

        #[ink(message)]
        pub fn generate_analytics_report(&mut self, _report_type: ReportType, _time_period: TimePeriod) -> u32 {
            let report_id = self.storage.get_next_report_id();
            // TODO: Implement full report generation
            report_id
        }

        // Anti-scalping placeholders
        #[ink(message)]
        pub fn configure_anti_scalping(
            &mut self,
            _event_id: u32,
            _transfer_restrictions: bool,
            _resale_price_cap: u128,
            _blacklist_enabled: bool,
            _whitelist_enabled: bool,
        ) -> Result<(), String> {
            // TODO: Implement in anti_scalping module
            Ok(())
        }

        #[ink(message)]
        pub fn get_anti_scalping_config(&self, _event_id: u32) -> Option<AntiScalpingConfig> {
            // TODO: Implement in anti_scalping module
            None
        }

        #[ink(message)]
        pub fn transfer_ticket(&mut self, _ticket_id: u64, _new_owner: ink::primitives::AccountId) -> Result<(), String> {
            // TODO: Implement in anti_scalping module
            Ok(())
        }

        #[ink(message)]
        pub fn list_ticket_for_resale(&mut self, _ticket_id: u64, _price: u128) -> Result<(), String> {
            // TODO: Implement in anti_scalping module
            Ok(())
        }

        #[ink(message)]
        pub fn get_resale_listings(&self) -> Vec<ResaleListing> {
            // TODO: Implement in anti_scalping module
            vec![]
        }

        #[ink(message)]
        pub fn blacklist_address(&mut self, _address: ink::primitives::AccountId) -> Result<(), String> {
            // TODO: Implement in anti_scalping module
            Ok(())
        }

        #[ink(message)]
        pub fn whitelist_address(&mut self, _address: ink::primitives::AccountId) -> Result<(), String> {
            // TODO: Implement in anti_scalping module
            Ok(())
        }

        // Loyalty system placeholders
        #[ink(message)]
        pub fn create_loyalty_profile(&mut self, _user: ink::primitives::AccountId) -> Result<(), String> {
            // TODO: Implement in loyalty module
            Ok(())
        }

        #[ink(message)]
        pub fn get_loyalty_profile(&self, _user: ink::primitives::AccountId) -> Option<LoyaltyProfile> {
            // TODO: Implement in loyalty module
            None
        }

        #[ink(message)]
        pub fn earn_loyalty_points(&mut self, _user: ink::primitives::AccountId, _points: u32, _reason: String) -> Result<(), String> {
            // TODO: Implement in loyalty module
            Ok(())
        }

        #[ink(message)]
        pub fn get_loyalty_discount(&self, _user: ink::primitives::AccountId, _base_price: u128) -> u128 {
            // TODO: Implement in loyalty module
            _base_price
        }

        #[ink(message)]
        pub fn redeem_reward(&mut self, _user: ink::primitives::AccountId, _reward_type: RewardType, _points_cost: u32) -> Result<u64, String> {
            // TODO: Implement in loyalty module
            Ok(1)
        }

        #[ink(message)]
        pub fn add_points_rule(&mut self, _rule_name: String, _points_per_dollar: u32, _min_purchase: u128) -> Result<u32, String> {
            // TODO: Implement in loyalty module
            Ok(1)
        }

        #[ink(message)]
        pub fn add_promotion(&mut self, _name: String, _description: String, _discount_percentage: u8, _valid_until: u64) -> Result<u32, String> {
            // TODO: Implement in loyalty module
            Ok(1)
        }

        #[ink(message)]
        pub fn add_referral(&mut self, _referrer: ink::primitives::AccountId, _referred: ink::primitives::AccountId) -> Result<(), String> {
            // TODO: Implement in loyalty module
            Ok(())
        }

        // Additional methods placeholders
        #[ink(message)]
        pub fn update_currency_rate(&mut self, _currency: CurrencyId, _new_rate: u128) -> Result<(), String> {
            // TODO: Implement in currency_management module
            Ok(())
        }

        #[ink(message)]
        pub fn add_supported_currency(&mut self, _currency: CurrencyId, _rate: u128) -> Result<(), String> {
            // TODO: Implement in currency_management module
            Ok(())
        }

        // Helper methods
        fn get_ticket_transfer_count(&self, _ticket_id: u64) -> u32 {
            // TODO: Implement in anti_scalping module
            0
        }
    } // End of impl SportsBroker

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;
        use ink::env::DefaultEnvironment;

        fn setup() -> SportsBroker {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            test::set_value_transferred::<DefaultEnvironment>(0);
            test::set_block_timestamp::<DefaultEnvironment>(0);
            test::set_block_number::<DefaultEnvironment>(0);
            test::set_contract::<DefaultEnvironment>(accounts.alice);
            SportsBroker::new()
        }

        fn setup_with_test_env<F>(test_fn: F)
        where
            F: FnOnce(&mut SportsBroker),
        {
            ink::env::test::run_test::<DefaultEnvironment, _>(|_| {
                let mut contract = setup();
                test_fn(&mut contract);
                Ok(())
            }).unwrap()
        }

        #[test]
        fn new_works() {
            setup_with_test_env(|contract| {
                assert_eq!(contract.storage.total_teams, 0);
                assert_eq!(contract.storage.total_venues, 0);
                assert_eq!(contract.storage.total_events, 0);
                assert_eq!(contract.storage.total_tickets, 0);
                assert_eq!(contract.storage.total_seasons, 0);
            });
        }

        #[test]
        fn owner_management_works() {
            setup_with_test_env(|contract| {
                let accounts = test::default_accounts::<DefaultEnvironment>();
                let owner = contract.get_owner();
                assert_eq!(owner, accounts.alice);
            });
        }

        #[test]
        fn register_team_works() {
            setup_with_test_env(|contract| {
                let team_id = contract.register_team("Lakers".to_string(), "Basketball".to_string(), "Los Angeles".to_string());
                assert_eq!(team_id, 1);
                assert_eq!(contract.storage.total_teams, 1);
                
                let team = contract.get_team(team_id).unwrap();
                assert_eq!(team.name, "Lakers");
                assert_eq!(team.city, "Los Angeles");
            });
        }

        #[test]
        fn update_team_performance_works() {
            setup_with_test_env(|contract| {
                let team_id = contract.register_team("Lakers".to_string(), "Basketball".to_string(), "Los Angeles".to_string());
                
                let result = contract.update_team_performance(team_id, 1, 30, 10, 8000, 5, 11000, 10000);
                assert!(result.is_ok());
                
                let performance = contract.get_team_performance(team_id).unwrap();
                assert_eq!(performance.wins, 30);
                assert_eq!(performance.losses, 10);
                assert_eq!(performance.win_percentage, 7500); // 75%
            });
        }

        #[test]
        fn dynamic_pricing_multipliers_work() {
            setup_with_test_env(|contract| {
                let team_id = contract.register_team("Lakers".to_string(), "Basketball".to_string(), "Los Angeles".to_string());
                
                // Update performance to trigger multiplier calculation
                contract.update_team_performance(team_id, 1, 30, 10, 8000, 5, 11000, 10000).unwrap();
                
                let multiplier = contract.get_pricing_multiplier(team_id).unwrap();
                // With 75% win rate, should have 1.2x performance multiplier
                assert_eq!(multiplier.performance_multiplier, 12000);
            });
        }

        #[test]
        fn currency_management_works() {
            setup_with_test_env(|contract| {
                let currencies = contract.get_supported_currencies();
                assert!(currencies.contains(&CurrencyId::DOT));
                assert!(currencies.contains(&CurrencyId::ACA));
                
                let dot_rate = contract.get_currency_rate(CurrencyId::DOT).unwrap();
                assert_eq!(dot_rate, 1_000_000_000_000_000_000);
            });
        }

        #[test]
        fn get_stats_works() {
            setup_with_test_env(|contract| {
                let _team_id = contract.register_team("Lakers".to_string(), "Basketball".to_string(), "Los Angeles".to_string());
                
                let stats = contract.get_stats();
                assert_eq!(stats.0, 1); // teams
                assert_eq!(stats.1, 0); // venues
                assert_eq!(stats.2, 0); // events
                assert_eq!(stats.3, 0); // tickets
                assert_eq!(stats.4, 0); // seasons
            });
        }

        #[test]
        fn register_venue_works() {
            setup_with_test_env(|contract| {
                let venue_id = contract.register_venue("Staples Center".to_string(), 20000, "Los Angeles".to_string(), "Basketball".to_string());
                assert_eq!(venue_id, 1);
                assert_eq!(contract.storage.total_venues, 1);
                
                let venue = contract.get_venue(venue_id).unwrap();
                assert_eq!(venue.name, "Staples Center");
                assert_eq!(venue.capacity, 20000);
            });
        }

        #[test]
        fn create_season_works() {
            setup_with_test_env(|contract| {
                let season_id = contract.create_season("2024 Season".to_string(), "Basketball".to_string(), 2024, 1000000000, 2000000000);
                assert_eq!(season_id, 1);
                assert_eq!(contract.storage.total_seasons, 1);
            });
        }
    } // End of tests module
} // End of sports_broker module
