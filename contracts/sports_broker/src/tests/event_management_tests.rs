
use crate::types::*;

/// Event management functionality tests
pub struct EventManagementTests;

impl EventManagementTests {
    /// Test creating a season
    pub fn test_create_season() {
        crate::tests::setup_with_test_env(|contract| {
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            assert_eq!(season_id, 1);
            let stats = contract.get_stats();
            assert_eq!(stats.4, 1); // Total seasons
        });
    }

    /// Test creating a sports event
    pub fn test_create_sports_event() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            assert_eq!(event_id, 1);
            let event = contract.get_sports_event(event_id).unwrap();
            assert_eq!(event.name, "Lakers vs Celtics");
            assert_eq!(event.home_team_id, team_id);
            assert_eq!(event.away_team_id, away_team_id);
            assert_eq!(event.venue_id, venue_id);
            assert_eq!(event.capacity, 18000);
            assert!(event.active);
        });
    }

    /// Test event capacity updates
    pub fn test_update_event_capacity() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            // Update capacity
            contract.update_event_capacity(event_id, 20000).unwrap();
            let event = contract.get_sports_event(event_id).unwrap();
            assert_eq!(event.capacity, 20000);
        });
    }

    /// Test event pricing updates
    pub fn test_update_base_ticket_price() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            // Update base price
            contract.update_base_ticket_price(event_id, 75_000_000_000_000).unwrap();
            let event = contract.get_sports_event(event_id).unwrap();
            assert_eq!(event.base_price, 75_000_000_000_000);
        });
    }

    /// Test advanced event search
    pub fn test_search_events_advanced() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            // Search by sport type
            let basketball_events = contract.search_events_advanced(
                Some(SportType::Basketball),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                true,
            );
            assert_eq!(basketball_events.len(), 1);
            
            // Search by team
            let lakers_events = contract.search_events_advanced(
                None,
                Some(team_id),
                None,
                None,
                None,
                None,
                None,
                None,
                true,
            );
            assert_eq!(lakers_events.len(), 1);
            
            // Search by max price
            let affordable_events = contract.search_events_advanced(
                None,
                None,
                None,
                None,
                None,
                None,
                Some(100_000_000_000_000), // 0.1 DOT
                None,
                true,
            );
            assert_eq!(affordable_events.len(), 1);
        });
    }

    /// Test event status updates
    pub fn test_update_event_status() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            // Initially active
            let event = contract.get_sports_event(event_id).unwrap();
            assert!(event.active);
            
            // Deactivate event
            contract.update_event_status(event_id, false).unwrap();
            let event = contract.get_sports_event(event_id).unwrap();
            assert!(!event.active);
            
            // Reactivate event
            contract.update_event_status(event_id, true).unwrap();
            let event = contract.get_sports_event(event_id).unwrap();
            assert!(event.active);
        });
    }

    /// Test event statistics
    pub fn test_get_event_stats() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            let stats = contract.get_event_stats(event_id).unwrap();
            assert_eq!(stats.event_id, event_id);
            assert_eq!(stats.total_capacity, 18000);
            assert_eq!(stats.sold_tickets, 0);
            assert_eq!(stats.available_tickets, 18000);
            assert_eq!(stats.sellout_percentage, 0);
            assert_eq!(stats.total_revenue, 0);
            assert!(!stats.is_sold_out);
        });
    }

    /// Test event analytics
    pub fn test_get_event_analytics() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(), 
                "Basketball".to_string(), 
                "Los Angeles".to_string()
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(), 
                "Basketball".to_string(), 
                "Boston".to_string()
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(), 
                20000, 
                "Los Angeles".to_string(), 
                "Basketball".to_string()
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(), 
                "Basketball".to_string(), 
                2024, 
                1000000000, 
                2000000000
            );
            
            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            let analytics = contract.get_event_analytics(event_id);
            assert!(analytics.is_some());
            
            let analytics = analytics.unwrap();
            assert_eq!(analytics.event_id, event_id);
            assert_eq!(analytics.tickets_sold, 0);
            assert_eq!(analytics.revenue_generated, 0);
        });
    }

    /// Test get events by team
    pub fn test_get_events_by_team() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(),
                "Basketball".to_string(),
                "Boston".to_string(),
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(),
                20000,
                "Los Angeles".to_string(),
                "Basketball".to_string(),
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Basketball".to_string(),
                2024,
                1000000000,
                2000000000,
            );
            
            // Create multiple events
            contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            contract.create_sports_event(
                "Celtics vs Lakers".to_string(),
                away_team_id,
                team_id,
                venue_id,
                season_id,
                1704153600000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            let lakers_events = contract.get_events_by_team(team_id);
            let celtics_events = contract.get_events_by_team(away_team_id);
            
            assert_eq!(lakers_events.len(), 2); // Lakers appear in both events
            assert_eq!(celtics_events.len(), 2); // Celtics appear in both events
        });
    }

    /// Test get events by venue
    pub fn test_get_events_by_venue() {
        crate::tests::setup_with_test_env(|contract| {
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            let away_team_id = contract.register_team(
                "Celtics".to_string(),
                "Basketball".to_string(),
                "Boston".to_string(),
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(),
                20000,
                "Los Angeles".to_string(),
                "Basketball".to_string(),
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Basketball".to_string(),
                2024,
                1000000000,
                2000000000,
            );
            
            contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            let venue_events = contract.get_events_by_venue(venue_id);
            assert_eq!(venue_events.len(), 1);
            assert_eq!(venue_events[0].venue_id, venue_id);
        });
    }

    /// Test get events by sport
    pub fn test_get_events_by_sport() {
        crate::tests::setup_with_test_env(|contract| {
            let basketball_team = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            let away_team = contract.register_team(
                "Celtics".to_string(),
                "Basketball".to_string(),
                "Boston".to_string(),
            );
            let venue_id = contract.register_venue(
                "Staples Center".to_string(),
                20000,
                "Los Angeles".to_string(),
                "Basketball".to_string(),
            );
            let season_id = contract.create_season(
                "2024 Season".to_string(),
                "Basketball".to_string(),
                2024,
                1000000000,
                2000000000,
            );
            
            contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                basketball_team,
                away_team,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            
            let basketball_events = contract.get_events_by_sport(SportType::Basketball);
            let football_events = contract.get_events_by_sport(SportType::Football);
            
            assert_eq!(basketball_events.len(), 1);
            assert_eq!(football_events.len(), 0);
        });
    }
}
