use ink::env::test;
use crate::types::*;


/// Advanced Team Loyalty Tests
pub struct AdvancedTeamLoyaltyTests;

impl AdvancedTeamLoyaltyTests {
    /// Test creating a team loyalty profile
    pub fn test_create_team_loyalty_profile() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);

            let result = contract.create_team_loyalty_profile(team_id);
            assert!(result.is_ok(), "Should create team loyalty profile successfully");

            // Verify profile was created
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert_eq!(profile.user_id, accounts.alice);
            assert_eq!(profile.team_id, team_id);
            assert_eq!(profile.loyalty_points, 0);
            assert_eq!(profile.loyalty_tier, TeamLoyaltyTier::Rookie);
            assert_eq!(profile.attendance_streak, 0);
            assert_eq!(profile.total_events_attended, 0);
            assert_eq!(profile.favorite_team_status, false);
            assert_eq!(profile.staked_amount, 0);
        });
    }

    /// Test staking on a team
    pub fn test_stake_on_team() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Stake on the team
            let staking_amount = 5_000_000_000_000_000_000; // 5 DOT
            let result = contract.stake_on_team(team_id, staking_amount);
            assert!(result.is_ok(), "Should stake on team successfully");

            let staking_id = result.unwrap();
            assert!(staking_id > 0, "Should return valid staking ID");

            // Verify profile was updated
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert_eq!(profile.staked_amount, staking_amount);
            assert_eq!(profile.favorite_team_status, true);
            assert!(profile.staking_start_date > 0);
        });
    }

    /// Test recording team attendance
    pub fn test_record_team_attendance() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Record attendance
            let event_id = 1; // Mock event ID
            let result = contract.record_team_attendance(team_id, event_id);
            assert!(result.is_ok(), "Should record attendance successfully");

            let attendance_id = result.unwrap();
            assert!(attendance_id > 0, "Should return valid attendance ID");

            // Verify profile was updated
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert_eq!(profile.total_events_attended, 1);
            assert_eq!(profile.attendance_streak, 1);
            assert!(profile.loyalty_points > 0, "Should earn loyalty points");
            assert!(profile.last_attendance > 0);
        });
    }

    /// Test team performance rewards
    pub fn test_team_performance_reward() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Award performance reward
            let reward_type = TeamPerformanceRewardType::PlayoffAppearance;
            let points_multiplier = 2;
            let start_date = current_time;
            let end_date = current_time + 30 * 24 * 60 * 60 * 1000; // 30 days

            let result = contract.award_team_performance_reward(
                team_id,
                reward_type,
                points_multiplier,
                start_date,
                end_date,
            );
            assert!(result.is_ok(), "Should award performance reward successfully");

            let reward_id = result.unwrap();
            assert!(reward_id > 0, "Should return valid reward ID");

            // Verify profile was updated with bonus points
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert!(profile.loyalty_points >= 20, "Should earn bonus points"); // 2 * 10 base points
        });
    }

    /// Test team loyalty challenges
    pub fn test_team_loyalty_challenge() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Create a challenge
            let challenge_name = "Attend 5 Games".to_string();
            let challenge_description = "Attend 5 consecutive home games".to_string();
            let challenge_type = TeamChallengeType::AttendanceStreak;
            let points_reward = 100;
            let start_date = current_time;
            let end_date = current_time + 90 * 24 * 60 * 60 * 1000; // 90 days
            let completion_criteria = "Attend 5 consecutive home games".to_string();

            let result = contract.create_team_loyalty_challenge(
                team_id,
                challenge_name.clone(),
                challenge_description.clone(),
                challenge_type,
                points_reward,
                start_date,
                end_date,
                completion_criteria.clone(),
            );
            assert!(result.is_ok(), "Should create challenge successfully");

            let challenge_id = result.unwrap();
            assert!(challenge_id > 0, "Should return valid challenge ID");

            // Join the challenge
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let join_result = contract.join_team_challenge(challenge_id);
            assert!(join_result.is_ok(), "Should join challenge successfully");

            // Complete the challenge
            let complete_result = contract.complete_team_challenge(challenge_id);
            assert!(complete_result.is_ok(), "Should complete challenge successfully");

            // Verify profile was updated with challenge points
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert!(profile.loyalty_points >= points_reward, "Should earn challenge points");
        });
    }

    /// Test multiple attendance records for streak tracking
    pub fn test_attendance_streak_tracking() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Record multiple attendances within streak window
            for i in 1..=5 {
                current_time += 3 * 24 * 60 * 60 * 1000; // 3 days apart
                test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
                
                let result = contract.record_team_attendance(team_id, i);
                assert!(result.is_ok(), "Should record attendance {} successfully", i);
            }

            // Verify streak was maintained
            let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
            assert!(profile.is_some(), "Profile should exist");
            
            let profile = profile.unwrap();
            assert_eq!(profile.attendance_streak, 5, "Should maintain 5-game streak");
            assert_eq!(profile.total_events_attended, 5, "Should have attended 5 events");
            assert!(profile.loyalty_points > 50, "Should earn significant loyalty points");
        });
    }

    /// Test staking multiplier calculation
    pub fn test_staking_multipliers() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create loyalty profile for the user
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
            contract.create_loyalty_profile(accounts.alice).unwrap();
            contract.create_team_loyalty_profile(team_id).unwrap();

            // Test different staking amounts
            let test_amounts = vec![
                (1_000_000_000_000_000_000, "1 DOT"),      // 1.0x multiplier
                (5_000_000_000_000_000_000, "5 DOT"),      // 1.2x multiplier
                (50_000_000_000_000_000_000, "50 DOT"),    // 1.5x multiplier
                (200_000_000_000_000_000_000, "200 DOT"),  // 2.0x multiplier
            ];

            for (amount, description) in test_amounts {
                // Reset profile
                contract.unstake_from_team(team_id).ok();

                // Stake new amount
                let result = contract.stake_on_team(team_id, amount);
                assert!(result.is_ok(), "Should stake {} successfully", description);

                // Verify staking was recorded
                let profile = contract.get_team_loyalty_profile(accounts.alice, team_id);
                assert!(profile.is_some(), "Profile should exist");
                
                let profile = profile.unwrap();
                assert_eq!(profile.staked_amount, amount, "Should record correct staked amount");
                assert_eq!(profile.favorite_team_status, true, "Should mark as favorite team");
            }
        });
    }

    /// Test team loyalty analytics
    pub fn test_team_loyalty_analytics() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time = 1000000;

            // Create a team first
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create multiple loyalty profiles
            let test_users = vec![accounts.alice, accounts.bob, accounts.charlie];
            
            for user in &test_users {
                test::set_caller::<ink::env::DefaultEnvironment>(*user);
                test::set_block_timestamp::<ink::env::DefaultEnvironment>(current_time);
                contract.create_loyalty_profile(*user).unwrap();
                contract.create_team_loyalty_profile(team_id).unwrap();
            }

            // Get analytics
            let analytics = contract.get_team_loyalty_analytics(team_id);
            assert!(analytics.is_some(), "Should have analytics");

            let analytics = analytics.unwrap();
            assert_eq!(analytics.team_id, team_id);
            assert_eq!(analytics.total_fans, 3, "Should have 3 fans");
            assert_eq!(analytics.total_loyalty_points, 0, "Should start with 0 points");
            assert_eq!(analytics.average_loyalty_tier, TeamLoyaltyTier::Rookie, "Should start as Rookie");
            assert_eq!(analytics.total_staked_amount, 0, "Should start with 0 staked");
            assert_eq!(analytics.total_attendance, 0, "Should start with 0 attendance");
            assert_eq!(analytics.longest_attendance_streak, 0, "Should start with 0 streak");
        });
    }
}
