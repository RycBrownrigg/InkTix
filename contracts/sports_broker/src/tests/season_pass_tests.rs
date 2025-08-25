
use ink::env::test;

use crate::types::*;

/// Season pass management tests
pub struct SeasonPassTests;

impl SeasonPassTests {
    /// Test creating a season pass package
    pub fn test_create_season_pass_package() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Register a team first
        let team_id = contract.register_team(
            "Lakers".to_string(),
            "Basketball".to_string(),
            "Los Angeles".to_string(),
        );

        // Register a season
        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        // Create season pass benefits
        let benefits = SeasonPassBenefits {
            priority_access: true,
            exclusive_events: true,
            merchandise_discount: 15,
            parking_included: true,
            concession_credits: 50_000_000_000_000_000_000, // 50 DOT
            vip_upgrades: true,
            meet_greet_access: false,
            backstage_tours: false,
            loyalty_multiplier: 15000, // 1.5x
            staking_rewards: true,
        };

        // Create season pass package
        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000; // 1 day from now
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000; // 30 days from now

        let package_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Full Season Premium".to_string(),
            SeasonPassType::FullSeason,
            82, // NBA regular season games
            1000_000_000_000_000_000_000, // 1000 DOT
            CurrencyId::DOT,
            1000, // max quantity
            benefits,
            true, // staking required
            100_000_000_000_000_000_000, // 100 DOT min staking
            500, // 5% annual reward rate
            sale_start,
            sale_end,
        ).unwrap();

        // Verify package was created
        let package = contract.get_season_pass_package(package_id).unwrap();
        assert_eq!(package.team_id, team_id);
        assert_eq!(package.season_id, season_id);
        assert_eq!(package.pass_type, SeasonPassType::FullSeason);
        assert_eq!(package.total_games, 82);
        assert_eq!(package.base_price, 1000_000_000_000_000_000_000);
        assert_eq!(package.max_quantity, 1000);
        assert_eq!(package.sold_quantity, 0);
        assert!(package.active);
        assert!(package.staking_required);
        assert_eq!(package.min_staking_amount, 100_000_000_000_000_000_000);
        assert_eq!(package.staking_reward_rate, 500);

        // Verify analytics were initialized
        let analytics = contract.get_season_pass_analytics(package_id).unwrap();
        assert_eq!(analytics.total_passes_sold, 0);
        assert_eq!(analytics.total_revenue, 0);
        assert_eq!(analytics.total_staking_amount, 0);
        assert_eq!(analytics.total_staking_rewards, 0);
        });
    }

    /// Test purchasing a season pass
    pub fn test_purchase_season_pass() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Setup: Create team, season, and package
        let team_id = contract.register_team(
            "Warriors".to_string(),
            "Basketball".to_string(),
            "Golden State".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits {
            priority_access: true,
            exclusive_events: false,
            merchandise_discount: 10,
            parking_included: false,
            concession_credits: 25_000_000_000_000_000_000, // 25 DOT
            vip_upgrades: false,
            meet_greet_access: false,
            backstage_tours: false,
            loyalty_multiplier: 12000, // 1.2x
            staking_rewards: true,
        };

        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        let package_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Half Season Basic".to_string(),
            SeasonPassType::HalfSeason,
            41, // Half season
            500_000_000_000_000_000_000, // 500 DOT
            CurrencyId::DOT,
            500, // max quantity
            benefits,
            false, // staking not required
            0, // no min staking
            0, // no reward rate
            sale_start,
            sale_end,
        ).unwrap();

        // Set block timestamp to after sale start
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_start + 1000);

        // Purchase season pass
        let staking_amount = 50_000_000_000_000_000_000; // 50 DOT
        let pass_id = contract.purchase_season_pass(package_id, staking_amount).unwrap();

        // Verify season pass was created
        let season_pass = contract.get_season_pass(pass_id).unwrap();
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
        assert_eq!(season_pass.owner, accounts.alice);
        assert_eq!(season_pass.team_id, team_id);
        assert_eq!(season_pass.season_id, season_id);
        assert_eq!(season_pass.pass_type, SeasonPassType::HalfSeason);
        assert_eq!(season_pass.status, SeasonPassStatus::PendingActivation);
        assert_eq!(season_pass.total_games, 41);
        assert_eq!(season_pass.games_attended, 0);
        assert_eq!(season_pass.games_remaining, 41);
        assert_eq!(season_pass.purchase_price, 500_000_000_000_000_000_000);
        assert_eq!(season_pass.staking_amount, staking_amount);
        assert_eq!(season_pass.staking_rewards_earned, 0);
        assert!(season_pass.transferable);

        // Verify package sold quantity was updated
        let package = contract.get_season_pass_package(package_id).unwrap();
        assert_eq!(package.sold_quantity, 1);

        // Verify user season passes list was updated
        let user_passes = contract.get_user_season_passes(accounts.alice);
        assert_eq!(user_passes.len(), 1);
        assert_eq!(user_passes[0].id, pass_id);

        // Verify team season passes list was updated
        let team_passes = contract.get_team_season_passes(team_id);
        assert_eq!(team_passes.len(), 1);
        assert_eq!(team_passes[0].id, pass_id);

        // Verify analytics were updated
        let analytics = contract.get_season_pass_analytics(package_id).unwrap();
        assert_eq!(analytics.total_passes_sold, 1);
        assert_eq!(analytics.total_revenue, 500_000_000_000_000_000_000);
        assert_eq!(analytics.total_staking_amount, staking_amount);
        });
    }

    /// Test activating a season pass
    pub fn test_activate_season_pass() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Setup: Create and purchase a season pass
        let team_id = contract.register_team(
            "Celtics".to_string(),
            "Basketball".to_string(),
            "Boston".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits::default();
        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        let package_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Playoff Only".to_string(),
            SeasonPassType::PlayoffOnly,
            16, // Playoff games
            200_000_000_000_000_000_000, // 200 DOT
            CurrencyId::DOT,
            200, // max quantity
            benefits,
            false, // staking not required
            0, // no min staking
            0, // no reward rate
            sale_start,
            sale_end,
        ).unwrap();

        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_start + 1000);
        let pass_id = contract.purchase_season_pass(package_id, 0).unwrap();

        // Verify pass is pending activation
        let season_pass = contract.get_season_pass(pass_id).unwrap();
        assert_eq!(season_pass.status, SeasonPassStatus::PendingActivation);

        // Activate the pass
        contract.activate_season_pass(pass_id).unwrap();

        // Verify pass is now active
        let updated_pass = contract.get_season_pass(pass_id).unwrap();
        assert_eq!(updated_pass.status, SeasonPassStatus::Active);
        assert!(updated_pass.activation_date > 0);
        assert!(updated_pass.expiry_date > updated_pass.activation_date);
        });
    }

    /// Test using a season pass for an event
    pub fn test_use_season_pass_for_event() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Setup: Create two teams, season, venue, and package
        let home_team_id = contract.register_team(
            "Heat".to_string(),
            "Basketball".to_string(),
            "Miami".to_string(),
        );

        let away_team_id = contract.register_team(
            "Celtics".to_string(),
            "Basketball".to_string(),
            "Boston".to_string(),
        );

        let venue_id = contract.register_venue(
            "FTX Arena".to_string(),
            20000, // capacity
            "Miami, FL".to_string(),
            "Basketball arena".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits {
            loyalty_multiplier: 15000, // 1.5x
            staking_rewards: true,
            ..SeasonPassBenefits::default()
        };

        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        let package_id = contract.create_season_pass_package(
            home_team_id,
            season_id,
            "Full Season".to_string(),
            SeasonPassType::FullSeason,
            82,
            800_000_000_000_000_000_000, // 800 DOT
            CurrencyId::DOT,
            1000, // max quantity
            benefits,
            true, // staking required
            50_000_000_000_000_000_000, // 50 DOT min staking
            500, // 5% reward rate
            sale_start,
            sale_end,
        ).unwrap();

        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_start + 1000);
        let staking_amount = 100_000_000_000_000_000_000; // 100 DOT
        let pass_id = contract.purchase_season_pass(package_id, staking_amount).unwrap();

        // Get accounts
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

        // Activate the pass
        contract.activate_season_pass(pass_id).unwrap();

        // Create loyalty profile for Alice
        contract.create_loyalty_profile(accounts.alice).unwrap();

        // Wait some time for staking rewards to accumulate
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(
            current_time + 7 * 24 * 60 * 60 * 1000 // 1 week later
        );

        // Create an event
        let event_id = contract.create_sports_event(
            "Heat vs Celtics".to_string(),
            home_team_id, // home team
            away_team_id, // away team (different team)
            venue_id,
            season_id,
            current_time + 7 * 24 * 60 * 60 * 1000, // 1 week from now
            20000, // capacity
            100_000_000_000_000_000_000, // 100 DOT base price
            GameType::RegularSeason,
        );

        // Verify event was created
        assert!(event_id > 0, "Event should have been created with ID > 0");

        // Use season pass for the event
        contract.use_season_pass_for_event(pass_id, event_id).unwrap();

        // Verify pass usage was recorded
        let updated_pass = contract.get_season_pass(pass_id).unwrap();
        assert_eq!(updated_pass.games_attended, 1);
        assert_eq!(updated_pass.games_remaining, 81);

        // Verify loyalty points were earned (with multiplier)
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
        let loyalty_profile = contract.get_loyalty_profile(accounts.alice).unwrap();
        // Base points: 800 DOT / 0.5 DOT = 1600 points
        // With 1.5x multiplier: 1600 * 1.5 = 2400 points
        assert!(loyalty_profile.total_points >= 2400);

        // Verify staking rewards were calculated
        let final_pass = contract.get_season_pass(pass_id).unwrap();
        assert!(final_pass.staking_rewards_earned > 0);
        assert!(final_pass.last_staking_update > 0);
        });
    }

    /// Test transferring a season pass
    pub fn test_transfer_season_pass() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Setup: Create and purchase a season pass
        let team_id = contract.register_team(
            "Nets".to_string(),
            "Basketball".to_string(),
            "Brooklyn".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits::default();
        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        let package_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Full Season".to_string(),
            SeasonPassType::FullSeason,
            82,
            600_000_000_000_000_000_000, // 600 DOT
            CurrencyId::DOT,
            1000, benefits, false, 0, 0, sale_start, sale_end,
        ).unwrap();

        let purchase_time = sale_start + 1000;
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(purchase_time);
        let pass_id = contract.purchase_season_pass(package_id, 0).unwrap();

        // Get accounts
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
        let alice = accounts.alice;
        let bob = accounts.bob;

        // Verify initial ownership
        let season_pass = contract.get_season_pass(pass_id).unwrap();
        assert_eq!(season_pass.owner, alice);

        // Try to transfer before cooldown (should fail)
        let transfer_result = contract.transfer_season_pass(pass_id, bob);
        assert!(transfer_result.is_err());

        // Wait for cooldown period to expire (30 days from purchase time)
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(
            purchase_time + 31 * 24 * 60 * 60 * 1000 // 31 days from purchase
        );

        // Transfer the pass
        contract.transfer_season_pass(pass_id, bob).unwrap();

        // Verify ownership changed
        let updated_pass = contract.get_season_pass(pass_id).unwrap();
        assert_eq!(updated_pass.owner, bob);

        // Verify user lists were updated
        let alice_passes = contract.get_user_season_passes(alice);
        assert_eq!(alice_passes.len(), 0);

        let bob_passes = contract.get_user_season_passes(bob);
        assert_eq!(bob_passes.len(), 1);
        assert_eq!(bob_passes[0].id, pass_id);
        });
    }

    /// Test season pass analytics
    pub fn test_season_pass_analytics() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Setup: Create team, season, and multiple packages
        let team_id = contract.register_team(
            "Suns".to_string(),
            "Basketball".to_string(),
            "Phoenix".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits::default();
        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        // Create multiple packages
        let package1_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Full Season Premium".to_string(),
            SeasonPassType::FullSeason,
            82,
            1000_000_000_000_000_000_000, // 1000 DOT
            CurrencyId::DOT,
            500, // max quantity
            benefits.clone(),
            true, // staking required
            100_000_000_000_000_000_000, // min staking
            500, // reward rate
            sale_start,
            sale_end,
        ).unwrap();

        let package2_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Half Season Basic".to_string(),
            SeasonPassType::HalfSeason,
            41,
            500_000_000_000_000_000_000, // 500 DOT
            CurrencyId::DOT,
            300, // max quantity
            benefits.clone(),
            false, // staking not required
            0, // no min staking
            0, // no reward rate
            sale_start,
            sale_end,
        ).unwrap();

        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_start + 1000);

        // Purchase passes from both packages
        let _pass1_id = contract.purchase_season_pass(package1_id, 100_000_000_000_000_000_000).unwrap();
        let _pass2_id = contract.purchase_season_pass(package2_id, 0).unwrap();

        // Verify analytics for package 1
        let analytics1 = contract.get_season_pass_analytics(package1_id).unwrap();
        assert_eq!(analytics1.total_passes_sold, 1);
        assert_eq!(analytics1.total_revenue, 1000_000_000_000_000_000_000);
        assert_eq!(analytics1.total_staking_amount, 100_000_000_000_000_000_000);

        // Verify analytics for package 2
        let analytics2 = contract.get_season_pass_analytics(package2_id).unwrap();
        assert_eq!(analytics2.total_passes_sold, 1);
        assert_eq!(analytics2.total_revenue, 500_000_000_000_000_000_000);
        assert_eq!(analytics2.total_staking_amount, 0);

        // Verify platform stats were updated
        // Note: We can't directly access platform_stats from tests, but the logic should work
        });
    }

    /// Test season pass validation and error cases
    pub fn test_season_pass_validation() {
        crate::tests::setup_with_test_env(|contract| {
        
        // Test: Cannot purchase before sale start
        let team_id = contract.register_team(
            "Kings".to_string(),
            "Basketball".to_string(),
            "Sacramento".to_string(),
        );

        let current_time = 1704067200000; // January 1, 2024
        let season_id = contract.create_season(
            "2024-25 NBA Season".to_string(),
            "Basketball".to_string(),
            2025,
            current_time,
            current_time + 365 * 24 * 60 * 60 * 1000, // 1 year
        );

        let benefits = SeasonPassBenefits::default();
        let current_time = 1704067200000; // January 1, 2024
        let sale_start = current_time + 24 * 60 * 60 * 1000;
        let sale_end = current_time + 30 * 24 * 60 * 60 * 1000;

        let package_id = contract.create_season_pass_package(
            team_id,
            season_id,
            "Full Season".to_string(),
            SeasonPassType::FullSeason,
            82,
            800_000_000_000_000_000_000,
            CurrencyId::DOT,
            100, // max quantity
            benefits,
            false, // staking not required
            0, // no min staking
            0, // no reward rate
            sale_start,
            sale_end,
        ).unwrap();

        // Try to purchase before sale start (should fail)
        let purchase_result = contract.purchase_season_pass(package_id, 0);
        assert!(purchase_result.is_err());

        // Test: Cannot purchase after sale end
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_end + 1000);
        let purchase_result = contract.purchase_season_pass(package_id, 0);
        assert!(purchase_result.is_err());

        // Test: Cannot activate pass that's not pending
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(sale_start + 1000);
        let pass_id = contract.purchase_season_pass(package_id, 0).unwrap();
        
        // Activate once
        contract.activate_season_pass(pass_id).unwrap();
        
        // Try to activate again (should fail)
        let activate_result = contract.activate_season_pass(pass_id);
        assert!(activate_result.is_err());

        // Test: Cannot use pass for event not covered by pass
        let other_team_id = contract.register_team(
            "Lakers".to_string(),
            "Basketball".to_string(),
            "Los Angeles".to_string(),
        );

        let venue_id = contract.register_venue(
            "Crypto.com Arena".to_string(),
            19000,
            "Los Angeles, CA".to_string(),
            "Basketball arena".to_string(),
        );

        let other_event_id = contract.create_sports_event(
            "Lakers vs Warriors".to_string(),
            other_team_id, // home team
            other_team_id, // away team (same for simplicity)
            venue_id,
            season_id,
            current_time + 7 * 24 * 60 * 60 * 1000,
            19000, // capacity
            150_000_000_000_000_000_000, // base price
            GameType::RegularSeason,
        );

        // Try to use pass for different team's event (should fail)
        let use_result = contract.use_season_pass_for_event(pass_id, other_event_id);
        assert!(use_result.is_err());
        });
    }
}
