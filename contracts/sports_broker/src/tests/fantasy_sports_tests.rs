use crate::sports_broker::SportsBroker;
use crate::types::event::GameType;
use crate::types::fantasy_sports::FantasyLeagueType;
use ink::env::test;

// Simplified Fantasy Sports Tests - Core Functionality Only
#[test]
pub fn test_fantasy_league_creation() {
    crate::tests::setup_with_test_env(|contract| {
        let current_time: u64 = 2000000000000; // Much larger than test env timestamp

        // Create a season first
        let season_id = contract.create_season(
            "2024 NFL Season".to_string(),
            "NFL".to_string(),
            2024,                                     // year
            current_time + 365 * 24 * 60 * 60 * 1000, // start_date (1 year from now)
            current_time + 400 * 24 * 60 * 60 * 1000, // end_date (1+ year from now)
        );

        // Create fantasy league
        let league_id = contract
            .create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,              // 100 DOT
                current_time + 7 * 24 * 60 * 60 * 1000,   // start_date
                current_time + 180 * 24 * 60 * 60 * 1000, // end_date
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            )
            .unwrap();

        assert!(league_id > 0, "League should have been created with ID > 0");
    });
}

#[test]
pub fn test_fantasy_team_creation() {
    crate::tests::setup_with_test_env(|contract| {
        let current_time: u64 = 2000000000000; // Much larger than test env timestamp

        // Create a season
        let season_id = contract.create_season(
            "2024 NFL Season".to_string(),
            "NFL".to_string(),
            2024,
            current_time + 365 * 24 * 60 * 60 * 1000, // start_date (1 year from now)
            current_time + 400 * 24 * 60 * 60 * 1000, // end_date (1+ year from now)
        );

        // Create two teams (home and away)
        let home_team_id = contract.register_team(
            "Patriots".to_string(),
            "Football".to_string(),
            "New England".to_string(),
        );

        let away_team_id = contract.register_team(
            "Jets".to_string(),
            "Football".to_string(),
            "New York".to_string(),
        );

        // Create a venue
        let venue_id = contract.register_basic_venue(
            "Gillette Stadium".to_string(),
            65878,                       // capacity
            "Massachusetts".to_string(), // address
            "Football".to_string(),      // sport_type
        );

        // Create a sports event
        let event_id = contract.create_sports_event(
            "Patriots vs Jets".to_string(),
            home_team_id,
            away_team_id, // Using different teams
            venue_id,
            season_id,
            current_time + 60 * 24 * 60 * 60 * 1000, // event_time
            65878,                                   // capacity
            50_000_000_000_000_000_000,              // ticket_price
            GameType::RegularSeason,
        );

        // Purchase a ticket
        let ticket_id = contract
            .purchase_ticket(
                event_id,
                "A".to_string(), // section
                "1".to_string(), // row
                1,               // seat
            )
            .unwrap();

        // Create fantasy league
        let league_id = contract
            .create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                current_time + 7 * 24 * 60 * 60 * 1000,
                current_time + 180 * 24 * 60 * 60 * 1000,
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            )
            .unwrap();

        // Join fantasy league
        let fantasy_team_id = contract
            .join_fantasy_league(league_id, "Alice's All-Stars".to_string(), ticket_id)
            .unwrap();

        assert!(
            fantasy_team_id > 0,
            "Fantasy team should have been created with ID > 0"
        );
    });
}

#[test]
pub fn test_fantasy_player_management() {
    crate::tests::setup_with_test_env(|contract| {
        let current_time: u64 = 2000000000000; // Much larger than test env timestamp

        // Setup: Create season, league, team, venue, event, ticket
        let season_id = contract.create_season(
            "2024 NFL Season".to_string(),
            "NFL".to_string(),
            2024,
            current_time + 365 * 24 * 60 * 60 * 1000, // start_date (1 year from now)
            current_time + 400 * 24 * 60 * 60 * 1000, // end_date (1+ year from now)
        );

        // Create two teams (home and away)
        let home_team_id = contract.register_team(
            "Patriots".to_string(),
            "Football".to_string(),
            "New England".to_string(),
        );

        let away_team_id = contract.register_team(
            "Jets".to_string(),
            "Football".to_string(),
            "New York".to_string(),
        );

        let venue_id = contract.register_basic_venue(
            "Gillette Stadium".to_string(),
            65878,
            "Massachusetts".to_string(),
            "Football".to_string(),
        );

        let event_id = contract.create_sports_event(
            "Patriots vs Jets".to_string(),
            home_team_id,
            away_team_id, // Using different teams
            venue_id,
            season_id,
            current_time + 60 * 24 * 60 * 60 * 1000,
            65878,
            50_000_000_000_000_000_000,
            GameType::RegularSeason,
        );

        let ticket_id = contract
            .purchase_ticket(event_id, "A".to_string(), "1".to_string(), 1)
            .unwrap();

        let league_id = contract
            .create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                current_time + 7 * 24 * 60 * 60 * 1000,
                current_time + 180 * 24 * 60 * 60 * 1000,
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            )
            .unwrap();

        let fantasy_team_id = contract
            .join_fantasy_league(league_id, "Alice's All-Stars".to_string(), ticket_id)
            .unwrap();

        // Add two players to fantasy team (using dummy player IDs)
        let player1_id = 1; // Dummy player ID for testing
        let player2_id = 2; // Different dummy player ID for testing

        let result = contract.add_player_to_fantasy_team(fantasy_team_id, player1_id);
        assert!(result.is_ok(), "Should be able to add first player to team");

        let result = contract.add_player_to_fantasy_team(fantasy_team_id, player2_id);
        assert!(
            result.is_ok(),
            "Should be able to add second player to team"
        );

        // Set team captains (using different players)
        let result = contract.set_team_captains(
            fantasy_team_id,
            player1_id, // captain
            player2_id, // vice_captain (different player)
        );
        assert!(result.is_ok(), "Should be able to set team captains");
    });
}

#[test]
pub fn test_fantasy_game_week() {
    crate::tests::setup_with_test_env(|contract| {
        let current_time: u64 = 2000000000000; // Much larger than test env timestamp

        // Setup: Create season, league
        let season_id = contract.create_season(
            "2024 NFL Season".to_string(),
            "NFL".to_string(),
            2024,
            current_time + 365 * 24 * 60 * 60 * 1000, // start_date (1 year from now)
            current_time + 400 * 24 * 60 * 60 * 1000, // end_date (1+ year from now)
        );

        let league_id = contract
            .create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                current_time + 7 * 24 * 60 * 60 * 1000,
                current_time + 180 * 24 * 60 * 60 * 1000,
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            )
            .unwrap();

        // Create fantasy game week
        let week_id = contract
            .create_fantasy_game_week(
                league_id,
                season_id,                               // season_id
                current_time + 8 * 24 * 60 * 60 * 1000,  // start_date
                current_time + 15 * 24 * 60 * 60 * 1000, // end_date
                vec![],                                  // games (empty for this test)
                current_time + 6 * 24 * 60 * 60 * 1000,  // transfer_deadline
                current_time + 6 * 24 * 60 * 60 * 1000,  // captain_selection_deadline
            )
            .unwrap();

        assert!(
            week_id > 0,
            "Game week should have been created with ID > 0"
        );

        // Try to activate game week (should fail before start time)
        let result = contract.activate_fantasy_game_week(week_id);
        assert!(
            result.is_err(),
            "Should not be able to activate game week before start time"
        );

        // Move time forward and try again
        test::set_block_timestamp::<ink::env::DefaultEnvironment>(
            current_time + 8 * 24 * 60 * 60 * 1000,
        );

        let result = contract.activate_fantasy_game_week(week_id);
        assert!(
            result.is_ok(),
            "Should be able to activate game week after start time"
        );
    });
}

#[test]
pub fn test_fantasy_loyalty_integration() {
    crate::tests::setup_with_test_env(|contract| {
        let current_time: u64 = 2000000000000; // Much larger than test env timestamp

        // Create loyalty profile first (using test accounts)
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
        contract.create_loyalty_profile(accounts.alice).unwrap();

        // Setup: Create season, league
        let season_id = contract.create_season(
            "2024 NFL Season".to_string(),
            "NFL".to_string(),
            2024,
            current_time + 365 * 24 * 60 * 60 * 1000, // start_date (1 year from now)
            current_time + 400 * 24 * 60 * 60 * 1000, // end_date (1+ year from now)
        );

        let league_id = contract
            .create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                current_time + 7 * 24 * 60 * 60 * 1000,
                current_time + 180 * 24 * 60 * 60 * 1000,
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            )
            .unwrap();

        // Award fantasy loyalty points
        let result = contract.award_fantasy_loyalty_points(
            accounts.alice, // user
            100,            // points
            league_id,
        );
        assert!(
            result.is_ok(),
            "Should be able to award fantasy loyalty points"
        );

        // Check loyalty profile
        let loyalty_profile = contract.get_loyalty_profile(accounts.alice).unwrap();
        assert!(
            loyalty_profile.fantasy_sports_points > 0,
            "Fantasy sports points should be awarded"
        );
    });
}
