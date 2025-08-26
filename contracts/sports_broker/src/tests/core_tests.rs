use crate::types::*;

/// Core functionality tests
pub struct CoreTests;

impl CoreTests {
    /// Test basic contract initialization
    pub fn test_contract_initialization() {
        crate::tests::setup_with_test_env(|contract| {
            let stats = contract.get_stats();
            assert_eq!(stats.0, 0); // Total teams
            assert_eq!(stats.1, 0); // Total venues
            assert_eq!(stats.2, 0); // Total events
            assert_eq!(stats.3, 0); // Total tickets
            assert_eq!(stats.4, 0); // Total seasons
        });
    }

    /// Test team registration
    pub fn test_register_team() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            assert_eq!(team_id, 1);
            let stats = contract.get_stats();
            assert_eq!(stats.0, 1); // Total teams

            let team = contract.get_team(team_id).unwrap();
            assert_eq!(team.name, "Lakers");
            assert_eq!(team.city, "Los Angeles");
        });
    }

    /// Test venue registration
    pub fn test_register_venue() {
        crate::tests::setup_with_test_env(|contract| {
            let venue_id = contract.register_basic_venue(
                "Staples Center".to_string(),
                20000,
                "Los Angeles".to_string(),
                "Basketball".to_string(),
            );
            assert!(venue_id > 0, "Venue should have a valid ID");
            let stats = contract.get_stats();
            assert!(stats.1 > 0, "Should have at least one venue");

            let venue = contract.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.capacity, 20000);
            assert_eq!(venue.city, "Los Angeles");
        });
    }

    /// Test basic stats functionality
    pub fn test_get_stats() {
        crate::tests::setup_with_test_env(|contract| {
            // Initially empty
            let stats = contract.get_stats();
            assert_eq!(stats.0, 0);
            assert_eq!(stats.1, 0);
            assert_eq!(stats.2, 0);
            assert_eq!(stats.3, 0);
            assert_eq!(stats.4, 0);

            // Add a team
            contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );

            let stats = contract.get_stats();
            assert_eq!(stats.0, 1); // One team
            assert_eq!(stats.1, 0); // No venues
            assert_eq!(stats.2, 0); // No events
            assert_eq!(stats.3, 0); // No tickets
            assert_eq!(stats.4, 0); // No seasons
        });
    }

    /// Test team performance updates
    pub fn test_update_team_performance() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Basketball".to_string(),
                2024,
                1000000000,
                2000000000,
            );

            // Update team performance
            contract
                .update_team_performance(
                    team_id, season_id, 30,    // wins
                    10,    // losses
                    8000,  // points scored
                    5,     // playoff rounds
                    11000, // points allowed
                    10000, // total games
                )
                .unwrap();

            let performance = contract.get_team_performance(team_id).unwrap();
            assert_eq!(performance.wins, 30);
            assert_eq!(performance.losses, 10);
            assert_eq!(performance.win_percentage, 7500); // 75%
        });
    }

    /// Test owner management
    pub fn test_owner_management() {
        crate::tests::setup_with_test_env(|contract| {
            let owner = contract.get_owner();
            // Owner should be set to the caller in constructor
            assert!(owner != ink::primitives::AccountId::from([0u8; 32]));
        });
    }

    /// Test dynamic pricing multipliers
    pub fn test_dynamic_pricing_multipliers() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Basketball".to_string(),
                2024,
                1000000000,
                2000000000,
            );

            // Update performance to trigger multiplier calculation
            contract
                .update_team_performance(
                    team_id, season_id, 30,    // wins
                    10,    // losses
                    8000,  // points scored
                    5,     // playoff rounds
                    11000, // points allowed
                    10000, // total games
                )
                .unwrap();

            let multiplier = contract.get_pricing_multiplier(team_id).unwrap();
            // With 75% win rate, should have 1.2x performance multiplier
            assert_eq!(multiplier.performance_multiplier, 12000);
        });
    }

    /// Test currency management
    pub fn test_currency_management() {
        crate::tests::setup_with_test_env(|contract| {
            let currencies = contract.get_supported_currencies();
            assert!(currencies.contains(&CurrencyId::DOT));
            assert!(currencies.contains(&CurrencyId::ACA));

            let dot_rate = contract.get_currency_rate(CurrencyId::DOT).unwrap();
            assert_eq!(dot_rate, 1_000_000_000_000_000_000);
        });
    }
}
