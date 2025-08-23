use ink::env::test;
use crate::types::*;
use crate::tests::*;

pub struct FantasySportsTests;

impl FantasySportsTests {
    #[test]
    pub fn test_create_fantasy_league() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000; // Fixed timestamp for testing
            
            // Create a season first
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                2024, // year
                (current_time + 30 * 24 * 60 * 60 * 1000).into(), // 30 days from now
                (current_time + 200 * 24 * 60 * 60 * 1000).into(), // 200 days from now
            );

            // Create fantasy league
            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000, // 100 DOT
                (current_time + 7 * 24 * 60 * 60 * 1000).into(), // 7 days from now
                (current_time + 180 * 24 * 60 * 60 * 1000).into(), // 180 days from now
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            assert!(league_id > 0, "League should have been created with ID > 0");

            // Verify league was created
            let user_leagues = contract.get_user_fantasy_leagues(accounts.alice);
            assert!(!user_leagues.is_empty(), "User should have fantasy leagues");
            assert_eq!(user_leagues[0].league_id, league_id, "League ID should match");
            assert_eq!(user_leagues[0].name, "NFL Fantasy League 2024", "League name should match");
            assert_eq!(user_leagues[0].max_teams, 20, "Max teams should match");
            assert_eq!(user_leagues[0].entry_fee, 100_000_000_000_000_000_000, "Entry fee should match");
        });
    }

    #[test]
    pub fn test_join_fantasy_league() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000; // Fixed timestamp for testing
            
            // Create a season
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                2024, // year
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
            );

            // Create a team
            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            // Create a venue
            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                65878, // capacity
                "Massachusetts".to_string(), // address
                "Football".to_string(), // sport_type
            );

            // Create a sports event
            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id, // Using same team for simplicity
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(), // 60 days from now
                65878,
                50_000_000_000_000_000_000, // 50 DOT
                GameType::RegularSeason,
            );

            // Purchase a ticket
            let ticket_id = contract.purchase_ticket(
                event_id,
                "A".to_string(), // section
                "1".to_string(), // row
                1, // seat
            ).unwrap();

            // Create fantasy league
            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Join fantasy league
            let team_id = contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            assert!(team_id > 0, "Team should have been created with ID > 0");

            // Verify team was created
            let user_teams = contract.get_user_fantasy_teams(accounts.alice);
            assert!(!user_teams.is_empty(), "User should have fantasy teams");
            assert_eq!(user_teams[0].team_id, team_id, "Team ID should match");
            assert_eq!(user_teams[0].name, "Alice's All-Stars", "Team name should match");
            assert_eq!(user_teams[0].league_id, league_id, "League ID should match");
        });
    }

    #[test]
    pub fn test_add_player_to_fantasy_team() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                2024,
                (current_time + 30 * 24 * 60 * 60 * 1000),
                (current_time + 200 * 24 * 60 * 60 * 1000),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                65878,
                "Massachusetts".to_string(),
                "Football".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                "A".to_string(),
                "1".to_string(),
                1,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            let fantasy_team_id = contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Create a player ID (in a real implementation, this would be a registered player)
            let player_id = 1;

            // Add player to fantasy team
            let result = contract.add_player_to_fantasy_team(
                fantasy_team_id,
                player_id,
            );

            assert!(result.is_ok(), "Should be able to add player to team");

            // Verify player was added
            let user_teams = contract.get_user_fantasy_teams(accounts.alice);
            let team = user_teams.iter().find(|t| t.team_id == fantasy_team_id).unwrap();
            assert!(team.players.contains(&player_id), "Player should be in team");
        });
    }

    #[test]
    pub fn test_set_team_captains() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                "Foxborough".to_string(),
                "Massachusetts".to_string(),
                65878,
                "Football stadium".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                1,
                CurrencyId::DOT,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            let fantasy_team_id = contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Create player IDs (in a real implementation, these would be registered players)
            let player1_id = 1;
            let player2_id = 2;

            // Add players to team
            contract.add_player_to_fantasy_team(fantasy_team_id, player1_id).unwrap();
            contract.add_player_to_fantasy_team(fantasy_team_id, player2_id).unwrap();

            // Set captains
            let result = contract.set_team_captains(
                fantasy_team_id,
                player1_id,
                player2_id,
            );

            assert!(result.is_ok(), "Should be able to set team captains");

            // Verify captains were set
            let user_teams = contract.get_user_fantasy_teams(accounts.alice);
            let team = user_teams.iter().find(|t| t.team_id == fantasy_team_id).unwrap();
            assert_eq!(team.captain_id, Some(player1_id), "Captain should be set");
            assert_eq!(team.vice_captain_id, Some(player2_id), "Vice-captain should be set");
        });
    }

    #[test]
    pub fn test_transfer_players() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                "Foxborough".to_string(),
                "Massachusetts".to_string(),
                65878,
                "Football stadium".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                1,
                CurrencyId::DOT,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            let fantasy_team_id = contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Create player IDs (in a real implementation, these would be registered players)
            let player1_id = 1;
            let player2_id = 2;
            let player3_id = 3;

            // Add players to team
            contract.add_player_to_fantasy_team(fantasy_team_id, player1_id).unwrap();
            contract.add_player_to_fantasy_team(fantasy_team_id, player2_id).unwrap();

            // Create game week
            let week_id = contract.create_fantasy_game_week(
                league_id,
                season_id,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 14 * 24 * 60 * 60 * 1000).into(),
                vec![event_id],
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
            ).unwrap();

            // Transfer players
            let result = contract.transfer_players(
                fantasy_team_id,
                player1_id,
                player3_id,
                week_id,
            );

            assert!(result.is_ok(), "Should be able to transfer players");

            // Verify transfer occurred
            let user_teams = contract.get_user_fantasy_teams(accounts.alice);
            let team = user_teams.iter().find(|t| t.team_id == fantasy_team_id).unwrap();
            assert!(!team.players.contains(&player1_id), "Player1 should be removed");
            assert!(team.players.contains(&player3_id), "Player3 should be added");
        });
    }

    #[test]
    pub fn test_update_player_stats() {
        crate::tests::setup_with_test_env(|contract| {
            // Update player stats (placeholder implementation)
            let result = contract.update_player_stats(
                1, // player_id
                25, // points
                2,  // touchdowns
                250, // yards
                Some(65), // completion_percentage
                None, // field_goal_percentage
            );

            assert!(result.is_ok(), "Should be able to update player stats");

            // Note: In a real implementation, this would update actual player stats
            // For now, this is a placeholder that always succeeds
        });
    }

    #[test]
    pub fn test_calculate_team_points() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                "Foxborough".to_string(),
                "Massachusetts".to_string(),
                65878,
                "Football stadium".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                1,
                CurrencyId::DOT,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            let fantasy_team_id = contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Create game week
            let week_id = contract.create_fantasy_game_week(
                league_id,
                season_id,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 14 * 24 * 60 * 60 * 1000).into(),
                vec![event_id],
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
            ).unwrap();

            // Calculate team points
            let result = contract.calculate_team_points(fantasy_team_id, week_id);

            assert!(result.is_ok(), "Should be able to calculate team points");
            let points = result.unwrap();
            // With simplified scoring: 0 players = 0 points
            assert_eq!(points, 0, "Initial points should be 0");
        });
    }

    #[test]
    pub fn test_get_league_leaderboard() {
        crate::tests::setup_with_test_env(|contract| {
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                "Foxborough".to_string(),
                "Massachusetts".to_string(),
                65878,
                "Football stadium".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                1,
                CurrencyId::DOT,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Get leaderboard
            let result = contract.get_league_leaderboard(league_id);

            assert!(result.is_ok(), "Should be able to get league leaderboard");
            let leaderboard = result.unwrap();
            assert_eq!(leaderboard.league_id, league_id, "League ID should match");
            assert!(!leaderboard.entries.is_empty(), "Leaderboard should have entries");
        });
    }

    #[test]
    pub fn test_get_user_fantasy_teams() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league, team, venue, event, ticket
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let team_id = contract.register_team(
                "Patriots".to_string(),
                "Football".to_string(),
                "New England".to_string(),
            );

            let venue_id = contract.register_venue(
                "Gillette Stadium".to_string(),
                "Foxborough".to_string(),
                "Massachusetts".to_string(),
                65878,
                "Football stadium".to_string(),
            );

            let event_id = contract.create_sports_event(
                "Patriots vs Jets".to_string(),
                team_id,
                team_id,
                venue_id,
                season_id,
                (current_time + 60 * 24 * 60 * 60 * 1000).into(),
                65878,
                50_000_000_000_000_000_000,
                GameType::RegularSeason,
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                1,
                CurrencyId::DOT,
            ).unwrap();

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            contract.join_fantasy_league(
                league_id,
                "Alice's All-Stars".to_string(),
                ticket_id,
            ).unwrap();

            // Get user fantasy teams
            let teams = contract.get_user_fantasy_teams(accounts.alice);

            assert!(!teams.is_empty(), "User should have fantasy teams");
            assert_eq!(teams[0].owner, accounts.alice, "Team owner should match");
            assert_eq!(teams[0].name, "Alice's All-Stars", "Team name should match");
        });
    }

    #[test]
    pub fn test_get_user_fantasy_leagues() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Setup: Create season
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            // Create fantasy league
            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Get user fantasy leagues
            let leagues = contract.get_user_fantasy_leagues(accounts.alice);

            assert!(!leagues.is_empty(), "User should have fantasy leagues");
            assert_eq!(leagues[0].league_id, league_id, "League ID should match");
            assert_eq!(leagues[0].name, "NFL Fantasy League 2024", "League name should match");
        });
    }

    #[test]
    pub fn test_award_fantasy_loyalty_points() {
        crate::tests::setup_with_test_env(|contract| {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let current_time: u64 = 1000000;
            
            // Create loyalty profile first
            contract.create_loyalty_profile(accounts.alice).unwrap();
            
            // Setup: Create season, league
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Award fantasy loyalty points
            let result = contract.award_fantasy_loyalty_points(
                accounts.alice,
                league_id,
                100,
            );

            assert!(result.is_ok(), "Should be able to award fantasy loyalty points");

            // Verify loyalty profile was updated
            let loyalty_profile = contract.get_loyalty_profile(accounts.alice).unwrap();
            assert!(loyalty_profile.fantasy_sports_points > 0, "Fantasy sports points should be awarded");
        });
    }

    #[test]
    pub fn test_create_fantasy_game_week() {
        crate::tests::setup_with_test_env(|contract| {
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Create fantasy game week
            let week_id = contract.create_fantasy_game_week(
                league_id,
                season_id,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 14 * 24 * 60 * 60 * 1000).into(),
                vec![], // No games for this test
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
            ).unwrap();

            assert!(week_id > 0, "Game week should have been created with ID > 0");
        });
    }

    #[test]
    pub fn test_activate_fantasy_game_week() {
        crate::tests::setup_with_test_env(|contract| {
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Create fantasy game week
            let week_id = contract.create_fantasy_game_week(
                league_id,
                season_id,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 14 * 24 * 60 * 60 * 1000).into(),
                vec![],
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
                (current_time + 6 * 24 * 60 * 60 * 1000).into(),
            ).unwrap();

            // Activate game week (this will fail because it's not time yet)
            let result = contract.activate_fantasy_game_week(week_id);
            assert!(result.is_err(), "Should not be able to activate game week before start time");

            // Move time forward and try again
            test::set_block_timestamp::<ink::env::DefaultEnvironment>(((current_time + 8 * 24 * 60 * 60 * 1000).into()).into());

            let result = contract.activate_fantasy_game_week(week_id);
            assert!(result.is_ok(), "Should be able to activate game week after start time");
        });
    }

    #[test]
    pub fn test_get_fantasy_settings() {
        crate::tests::setup_with_test_env(|contract| {
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Get fantasy settings (should return default settings)
            let result = contract.get_fantasy_settings(league_id);
            assert!(result.is_err(), "Should not have settings for new league");
        });
    }

    #[test]
    pub fn test_update_fantasy_settings() {
        crate::tests::setup_with_test_env(|contract| {
            let current_time: u64 = 1000000;
            
            // Setup: Create season, league
            let season_id = contract.create_season(
                "2024 NFL Season".to_string(),
                "NFL".to_string(),
                (current_time + 30 * 24 * 60 * 60 * 1000).into(),
                (current_time + 200 * 24 * 60 * 60 * 1000).into(),
                "Regular season and playoffs".to_string(),
            );

            let league_id = contract.create_fantasy_league(
                "NFL Fantasy League 2024".to_string(),
                "Join the ultimate NFL fantasy experience".to_string(),
                FantasyLeagueType::SeasonLong,
                20,
                100_000_000_000_000_000_000,
                (current_time + 7 * 24 * 60 * 60 * 1000).into(),
                (current_time + 180 * 24 * 60 * 60 * 1000).into(),
                season_id,
                "NFL".to_string(),
                "Standard fantasy football rules".to_string(),
                "PPR scoring system".to_string(),
            ).unwrap();

            // Create custom settings
            let settings = FantasySettings {
                max_transfers_per_week: 2,
                captain_multiplier: 3,
                vice_captain_multiplier: 2,
                bench_boost_enabled: false,
                triple_captain_enabled: false,
                wildcard_enabled: true,
                free_hit_enabled: true,
                max_players_per_team: 20,
                max_players_per_position: 4,
            };

            // Update fantasy settings
            let result = contract.update_fantasy_settings(league_id, settings.clone());
            assert!(result.is_ok(), "Should be able to update fantasy settings");

            // Get updated settings
            let result = contract.get_fantasy_settings(league_id);
            assert!(result.is_ok(), "Should be able to get updated fantasy settings");
            let updated_settings = result.unwrap();
            assert_eq!(updated_settings.max_transfers_per_week, 2, "Max transfers should be updated");
            assert_eq!(updated_settings.captain_multiplier, 3, "Captain multiplier should be updated");
        });
    }
}
