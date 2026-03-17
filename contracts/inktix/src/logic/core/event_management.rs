//! Event creation and lifecycle management.
//!
//! Handles event creation with category-specific validation (sports rivalry,
//! concert artist lookup), status updates, and analytics initialization.
//!
//! # Functions
//! - `create_event` -- creates a new event with venue and category validation
//! - `get_all_events` -- returns all registered events
//! - `update_event_status` -- changes an event's active/inactive state

use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;
use ink::prelude::vec;

use crate::storage::contract_storage::InkTixStorage;
use crate::types::*;

/// Event management logic for comprehensive event handling
pub struct EventManagement;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl EventManagement {
    /// Create a new event with EventCategory
    pub fn create_event(
        storage: &mut InkTixStorage,
        name: String,
        venue_id: u32,
        date: u64,
        capacity: u32,
        base_price: u128,
        category: EventCategory,
    ) -> Result<u32, String> {
        if name.is_empty() { return Err("Event name cannot be empty".to_string()); }
        if capacity == 0 { return Err("Event capacity must be greater than 0".to_string()); }
        if base_price == 0 { return Err("Base ticket price must be greater than 0".to_string()); }

        let venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        // Validate sport-specific fields
        let (rivalry_multiplier, season_pass_discount) = match &category {
            EventCategory::Sports { home_team_id, away_team_id, season_id, game_type, sport_type } => {
                if *home_team_id == *away_team_id {
                    return Err("Home and away teams must be different".to_string());
                }
                let home_team = storage.teams.get(*home_team_id).ok_or("Home team not found")?;
                let _away_team = storage.teams.get(*away_team_id).ok_or("Away team not found")?;
                let _season = storage.seasons.get(*season_id).ok_or("Season not found")?;

                let rivalry = Self::calculate_rivalry_multiplier_by_ids(storage, *home_team_id, *away_team_id);
                let discount = Self::get_season_pass_discount_for_game_type(game_type);
                (rivalry, discount)
            }
            EventCategory::Concert { artist_id } => {
                let _artist = storage.artists.get(*artist_id).ok_or("Artist not found")?;
                (10000, 0) // 1.0x multiplier, no season pass discount
            }
            EventCategory::Generic => {
                (10000, 0)
            }
        };

        let event_id = storage.get_next_id("event");

        let event = Event {
            id: event_id,
            name,
            venue_id,
            date,
            capacity: if capacity == 0 { venue.capacity } else { capacity },
            sold_tickets: 0,
            base_price,
            active: true,
            category,
            season_pass_discount,
            dynamic_pricing_enabled: true,
            rivalry_multiplier,
            revenue_generated: 0,
        };

        storage.events.insert(event_id, &event);

        // Create event analytics
        let analytics = EventAnalytics {
            event_id,
            tickets_sold: 0,
            revenue_generated: 0,
            average_price: event.base_price,
            sellout_percentage: 0,
            currency_breakdown: vec![(CurrencyId::DOT, 0)],
            attendance_forecast: event.capacity,
            revenue_forecast: event.base_price * event.capacity as u128,
            last_updated: 0,
        };
        storage.event_analytics.insert(event_id, &analytics);

        // Update platform stats
        storage.platform_stats.total_events = storage.platform_stats.total_events.saturating_add(1);

        Ok(event_id)
    }

    /// Get all events
    pub fn get_all_events(storage: &InkTixStorage) -> Vec<Event> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                events.push(event);
            }
        }
        events
    }

    /// Update event status
    pub fn update_event_status(
        storage: &mut InkTixStorage,
        event_id: u32,
        status: EventStatus,
    ) -> Result<(), String> {
        let mut event = storage.events.get(event_id).ok_or("Event not found")?;
        event.active = status == EventStatus::OnSale;
        storage.events.insert(event_id, &event);
        Ok(())
    }

    fn calculate_rivalry_multiplier_by_ids(storage: &InkTixStorage, home_id: u32, away_id: u32) -> u32 {
        let home_team = match storage.teams.get(home_id) { Some(t) => t, None => return 10000 };
        let away_team = match storage.teams.get(away_id) { Some(t) => t, None => return 10000 };

        if home_team.city == away_team.city { return 12000; }
        match (home_team.name.as_str(), away_team.name.as_str()) {
            ("Lakers", "Celtics") | ("Celtics", "Lakers") => 15000,
            ("Yankees", "Red Sox") | ("Red Sox", "Yankees") => 15000,
            ("Cowboys", "Giants") | ("Giants", "Cowboys") => 14000,
            ("Packers", "Bears") | ("Bears", "Packers") => 14000,
            ("Real Madrid", "Barcelona") | ("Barcelona", "Real Madrid") => 16000,
            _ => 10000,
        }
    }

    fn get_season_pass_discount_for_game_type(game_type: &GameType) -> u8 {
        match game_type {
            GameType::RegularSeason => 15, GameType::Playoff => 10,
            GameType::Championship => 5, GameType::AllStar => 20,
            GameType::Preseason => 25, GameType::Tournament => 12,
            GameType::Exhibition => 30,
        }
    }
}

/// Event statistics for comprehensive reporting
#[derive(Debug, Clone, PartialEq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct EventStats {
    pub event_id: u32,
    pub total_capacity: u32,
    pub sold_tickets: u32,
    pub available_tickets: u32,
    pub sellout_percentage: u8,
    pub total_revenue: u128,
    pub revenue_per_ticket: u128,
    pub is_sold_out: bool,
}
