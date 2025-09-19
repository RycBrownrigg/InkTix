use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;
use ink::prelude::vec;

use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Event management logic for comprehensive sports event handling
pub struct EventManagement;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl EventManagement {
    /// Create a new sports event with comprehensive features
    pub fn create_event(
        storage: &mut SportsBrokerStorage,
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
        // Validate inputs
        if name.is_empty() {
            return Err("Event name cannot be empty".to_string());
        }
        if capacity == 0 {
            return Err("Event capacity must be greater than 0".to_string());
        }
        if base_price == 0 {
            return Err("Base ticket price must be greater than 0".to_string());
        }
        if home_team_id == away_team_id {
            return Err("Home and away teams must be different".to_string());
        }

        // Validate venue exists
        let venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        // Validate teams exist
        let home_team = storage
            .teams
            .get(home_team_id)
            .ok_or("Home team not found")?;
        let away_team = storage
            .teams
            .get(away_team_id)
            .ok_or("Away team not found")?;

        // Validate season exists
        let _season = storage.seasons.get(season_id).ok_or("Season not found")?;

        // Validate sport type consistency
        if home_team.sport_type != away_team.sport_type {
            return Err("Home and away teams must play the same sport".to_string());
        }

        let event_id = storage.get_next_id("event");

        // Calculate rivalry multiplier
        let rivalry_multiplier = Self::calculate_rivalry_multiplier(&home_team, &away_team);

        // Determine season pass discount based on game type
        let season_pass_discount = Self::get_season_pass_discount_for_game_type(&game_type);

        let event = SportsEvent {
            id: event_id,
            name,
            venue_id,
            date,
            capacity: if capacity == 0 {
                venue.capacity
            } else {
                capacity
            },
            sold_tickets: 0,
            base_price,
            active: true,
            sport_type,
            home_team_id,
            away_team_id,
            season_id,
            game_type,
            season_pass_discount,
            dynamic_pricing_enabled: true,
            rivalry_multiplier,
            revenue_generated: 0,
        };

        // Store event
        storage.events.insert(event_id, &event);
        storage.total_events += 1;

        // Update search indexes for efficient event discovery
        Self::update_search_indexes(storage, event_id, &event);

        // Update event pricing multipliers for both teams
        Self::update_event_pricing_multipliers(
            storage,
            home_team_id,
            away_team_id,
            &game_type,
            rivalry_multiplier,
        );

        // Create event analytics
        Self::create_event_analytics(storage, event_id, &event);

        // Update platform statistics
        Self::update_platform_stats_for_new_event(storage, &event);

        Ok(event_id)
    }

    /// Get event by ID
    pub fn get_event(storage: &SportsBrokerStorage, event_id: u32) -> Option<SportsEvent> {
        storage.events.get(event_id)
    }

    /// Update event status (active/inactive)
    pub fn update_event_status(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        status: EventStatus,
    ) -> Result<(), String> {
        let mut event = storage.events.get(event_id).ok_or("Event not found")?;

        event.active = status == EventStatus::OnSale;
        storage.events.insert(event_id, &event);

        Ok(())
    }

    /// Get all events
    pub fn get_all_events(storage: &SportsBrokerStorage) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                events.push(event);
            }
        }
        events
    }

    /// Get events by season
    pub fn get_events_by_season(storage: &SportsBrokerStorage, season_id: u32) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.season_id == season_id {
                    events.push(event);
                }
            }
        }
        events
    }

    /// Get events by team (home or away)
    pub fn get_events_by_team(storage: &SportsBrokerStorage, team_id: u32) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.home_team_id == team_id || event.away_team_id == team_id {
                    events.push(event);
                }
            }
        }
        events
    }

    /// Get events by venue
    pub fn get_events_by_venue(storage: &SportsBrokerStorage, venue_id: u32) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.venue_id == venue_id {
                    events.push(event);
                }
            }
        }
        events
    }

    /// Get events by sport type
    pub fn get_events_by_sport(
        storage: &SportsBrokerStorage,
        sport_type: SportType,
    ) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.sport_type == sport_type {
                    events.push(event);
                }
            }
        }
        events
    }

    /// Get events by date range
    pub fn get_events_by_date_range(
        storage: &SportsBrokerStorage,
        start_date: u64,
        end_date: u64,
    ) -> Result<Vec<SportsEvent>, String> {
        if start_date > end_date {
            return Err("Start date must be before end date".to_string());
        }

        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.date >= start_date && event.date <= end_date {
                    events.push(event);
                }
            }
        }

        Ok(events)
    }

    /// Advanced event search with multiple filters
    pub fn search_events_advanced(
        storage: &SportsBrokerStorage,
        sport_type: Option<SportType>,
        team_id: Option<u32>,
        venue_id: Option<u32>,
        min_date: Option<u64>,
        max_date: Option<u64>,
        game_type: Option<GameType>,
        max_price: Option<u128>,
        min_availability: Option<u32>,
        active_only: bool,
    ) -> Vec<SportsEvent> {
        let mut results = Vec::new();

        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                // Apply filters
                if active_only && !event.active {
                    continue;
                }

                if let Some(sport) = &sport_type {
                    if &event.sport_type != sport {
                        continue;
                    }
                }

                if let Some(team) = team_id {
                    if event.home_team_id != team && event.away_team_id != team {
                        continue;
                    }
                }

                if let Some(venue) = venue_id {
                    if event.venue_id != venue {
                        continue;
                    }
                }

                if let Some(min_date) = min_date {
                    if event.date < min_date {
                        continue;
                    }
                }

                if let Some(max_date) = max_date {
                    if event.date > max_date {
                        continue;
                    }
                }

                if let Some(game) = game_type {
                    if event.game_type != game {
                        continue;
                    }
                }

                if let Some(max_price) = max_price {
                    if event.base_price > max_price {
                        continue;
                    }
                }

                if let Some(min_availability) = min_availability {
                    let available = event.capacity.saturating_sub(event.sold_tickets);
                    if available < min_availability {
                        continue;
                    }
                }

                results.push(event);
            }
        }

        results
    }

    /// Get recommended events for a user based on preferences
    pub fn get_recommended_events(
        storage: &SportsBrokerStorage,
        user: AccountId,
        limit: u32,
    ) -> Vec<SportsEvent> {
        let mut recommendations = Vec::new();

        // Get user's loyalty profile to understand preferences
        if let Some(profile) = storage.loyalty_profiles.get(user) {
            // For now, recommend events based on user's activity level
            // Users with more tickets purchased get priority access to premium events
            let priority_threshold = if profile.total_tickets_purchased > 10 {
                // High-activity user: recommend premium events
                GameType::Championship
            } else if profile.total_tickets_purchased > 5 {
                // Medium-activity user: recommend playoff events
                GameType::Playoff
            } else {
                // New user: recommend regular season events
                GameType::RegularSeason
            };

            // Find events matching the user's priority level
            for event_id in 1..=storage.total_events {
                if let Some(event) = storage.events.get(event_id) {
                    if event.active
                        && event.game_type == priority_threshold
                        && recommendations.len() < limit as usize
                    {
                        recommendations.push(event);
                    }
                }
            }

            // If we don't have enough recommendations, add more events
            if recommendations.len() < limit as usize {
                for event_id in 1..=storage.total_events {
                    if let Some(event) = storage.events.get(event_id) {
                        if event.active
                            && !recommendations.iter().any(|r| r.id == event.id)
                            && recommendations.len() < limit as usize
                        {
                            recommendations.push(event);
                        }
                    }
                }
            }
        } else {
            // No loyalty profile: recommend popular events (regular season)
            for event_id in 1..=storage.total_events {
                if let Some(event) = storage.events.get(event_id) {
                    if event.active
                        && event.game_type == GameType::RegularSeason
                        && recommendations.len() < limit as usize
                    {
                        recommendations.push(event);
                    }
                }
            }
        }

        recommendations
    }

    /// Update event capacity
    pub fn update_event_capacity(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        new_capacity: u32,
    ) -> Result<(), String> {
        if new_capacity == 0 {
            return Err("Capacity must be greater than 0".to_string());
        }

        let mut event = storage.events.get(event_id).ok_or("Event not found")?;

        if new_capacity < event.sold_tickets {
            return Err("New capacity cannot be less than sold tickets".to_string());
        }

        event.capacity = new_capacity;
        storage.events.insert(event_id, &event);

        // Update analytics
        if let Some(mut analytics) = storage.event_analytics.get(event_id) {
            analytics.attendance_forecast = new_capacity;
            analytics.revenue_forecast = event.base_price * new_capacity as u128;
            storage.event_analytics.insert(event_id, &analytics);
        }

        Ok(())
    }

    /// Update base ticket price
    pub fn update_base_ticket_price(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        new_price: u128,
    ) -> Result<(), String> {
        if new_price == 0 {
            return Err("Price must be greater than 0".to_string());
        }

        let mut event = storage.events.get(event_id).ok_or("Event not found")?;

        event.base_price = new_price;
        storage.events.insert(event_id, &event);

        // Update analytics
        if let Some(mut analytics) = storage.event_analytics.get(event_id) {
            analytics.average_price = new_price;
            analytics.revenue_forecast = new_price * event.capacity as u128;
            storage.event_analytics.insert(event_id, &analytics);
        }

        Ok(())
    }

    /// Get comprehensive event statistics
    pub fn get_event_stats(storage: &SportsBrokerStorage, event_id: u32) -> Option<EventStats> {
        let event = storage.events.get(event_id)?;

        let sellout_percentage = if event.capacity > 0 {
            (event.sold_tickets * 100) / event.capacity
        } else {
            0
        };

        let revenue_per_ticket = if event.sold_tickets > 0 {
            event.revenue_generated / event.sold_tickets as u128
        } else {
            0
        };

        Some(EventStats {
            event_id,
            total_capacity: event.capacity,
            sold_tickets: event.sold_tickets,
            available_tickets: event.capacity.saturating_sub(event.sold_tickets),
            sellout_percentage: sellout_percentage as u8,
            total_revenue: event.revenue_generated,
            revenue_per_ticket,
            is_sold_out: event.sold_tickets >= event.capacity,
        })
    }

    /// Get event analytics
    pub fn get_event_analytics(
        storage: &SportsBrokerStorage,
        event_id: u32,
    ) -> Option<EventAnalytics> {
        storage.event_analytics.get(event_id)
    }

    /// Update event analytics when tickets are sold
    pub fn update_event_analytics_for_ticket_sale(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        ticket_price: u128,
        currency: CurrencyId,
    ) -> Result<(), String> {
        let mut event = storage.events.get(event_id).ok_or("Event not found")?;

        let mut analytics = storage
            .event_analytics
            .get(event_id)
            .ok_or("Event analytics not found")?;

        // Update event
        event.sold_tickets = event.sold_tickets.saturating_add(1);
        event.revenue_generated = event.revenue_generated.saturating_add(ticket_price);
        storage.events.insert(event_id, &event);

        // Update analytics
        analytics.tickets_sold = event.sold_tickets;
        analytics.revenue_generated = event.revenue_generated;
        analytics.sellout_percentage = if event.capacity > 0 {
            ((event.sold_tickets * 100) / event.capacity) as u8
        } else {
            0
        };

        // Update average price
        analytics.average_price = if event.sold_tickets > 0 {
            event.revenue_generated / event.sold_tickets as u128
        } else {
            event.base_price
        };

        // Update currency breakdown
        Self::update_currency_breakdown(&mut analytics.currency_breakdown, currency, ticket_price);

        analytics.last_updated = 0; // Will be set by caller with current timestamp

        storage.event_analytics.insert(event_id, &analytics);

        Ok(())
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Calculate rivalry multiplier based on team characteristics
    fn calculate_rivalry_multiplier(home_team: &Team, away_team: &Team) -> u32 {
        // Same city rivalry (e.g., Lakers vs Clippers)
        if home_team.city == away_team.city {
            return 12000; // 1.2x
        }

        // Historic rivalries
        match (home_team.name.as_str(), away_team.name.as_str()) {
            ("Lakers", "Celtics") | ("Celtics", "Lakers") => 15000, // 1.5x
            ("Yankees", "Red Sox") | ("Red Sox", "Yankees") => 15000, // 1.5x
            ("Cowboys", "Giants") | ("Giants", "Cowboys") => 14000, // 1.4x
            ("Packers", "Bears") | ("Bears", "Packers") => 14000,   // 1.4x
            ("Real Madrid", "Barcelona") | ("Barcelona", "Real Madrid") => 16000, // 1.6x
            _ => 10000,                                             // 1.0x base
        }
    }

    /// Get season pass discount based on game type
    fn get_season_pass_discount_for_game_type(game_type: &GameType) -> u8 {
        match game_type {
            GameType::RegularSeason => 15, // 15% discount
            GameType::Playoff => 10,       // 10% discount
            GameType::Championship => 5,   // 5% discount
            GameType::AllStar => 20,       // 20% discount
            GameType::Preseason => 25,     // 25% discount
            GameType::Tournament => 12,    // 12% discount
            GameType::Exhibition => 30,    // 30% discount
        }
    }

    /// Update search indexes for efficient event discovery
    fn update_search_indexes(
        _storage: &mut SportsBrokerStorage,
        _event_id: u32,
        _event: &SportsEvent,
    ) {
        // Note: In a full implementation, you would maintain separate search indexes
        // For now, we'll rely on the existing get_events_by_* methods
        // This could be enhanced with actual indexing structures for better performance
    }

    /// Update event pricing multipliers for both teams
    fn update_event_pricing_multipliers(
        storage: &mut SportsBrokerStorage,
        home_team_id: u32,
        away_team_id: u32,
        game_type: &GameType,
        rivalry_multiplier: u32,
    ) {
        // Update home team pricing
        if let Some(mut home_pricing) = storage.pricing_multipliers.get(home_team_id) {
            home_pricing.rivalry_multiplier = rivalry_multiplier;

            // Set base multiplier based on game type
            home_pricing.base_multiplier = match game_type {
                GameType::RegularSeason => 10000, // 1.0x
                GameType::Playoff => 15000,       // 1.5x
                GameType::Championship => 25000,  // 2.5x
                GameType::AllStar => 20000,       // 2.0x
                GameType::Preseason => 7500,      // 0.75x
                GameType::Tournament => 18000,    // 1.8x
                GameType::Exhibition => 8000,     // 0.8x
            };

            Self::recalculate_final_multiplier(&mut home_pricing);
            storage
                .pricing_multipliers
                .insert(home_team_id, &home_pricing);
        }

        // Update away team pricing (slightly lower rivalry impact)
        if let Some(mut away_pricing) = storage.pricing_multipliers.get(away_team_id) {
            away_pricing.rivalry_multiplier = (rivalry_multiplier + 10000) / 2; // Average with base
            Self::recalculate_final_multiplier(&mut away_pricing);
            storage
                .pricing_multipliers
                .insert(away_team_id, &away_pricing);
        }
    }

    /// Recalculate final multiplier based on all components
    fn recalculate_final_multiplier(pricing: &mut PricingMultiplier) {
        // Calculate final multiplier step by step to avoid overflow
        let temp1 =
            (pricing.base_multiplier as u128 * pricing.performance_multiplier as u128) / 10000;
        let temp2 = (temp1 * pricing.playoff_multiplier as u128) / 10000;
        let temp3 = (temp2 * pricing.streak_multiplier as u128) / 10000;
        let temp4 = (temp3 * pricing.rivalry_multiplier as u128) / 10000;
        let final_result = (temp4 * pricing.demand_multiplier as u128) / 10000;

        pricing.final_multiplier = final_result as u32;
    }

    /// Create initial event analytics
    fn create_event_analytics(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        event: &SportsEvent,
    ) {
        let analytics = EventAnalytics {
            event_id,
            tickets_sold: 0,
            revenue_generated: 0,
            average_price: event.base_price,
            sellout_percentage: 0,
            currency_breakdown: vec![(CurrencyId::DOT, 0)],
            attendance_forecast: event.capacity,
            revenue_forecast: event.base_price * event.capacity as u128,
            last_updated: 0, // Will be set by caller
        };

        storage.event_analytics.insert(event_id, &analytics);
    }

    /// Update platform statistics for new event
    fn update_platform_stats_for_new_event(
        storage: &mut SportsBrokerStorage,
        _event: &SportsEvent,
    ) {
        storage.platform_stats.total_events = storage.platform_stats.total_events.saturating_add(1);
        storage.platform_stats.last_updated = 0; // Will be set by caller
    }

    /// Update currency breakdown in analytics
    fn update_currency_breakdown(
        currency_breakdown: &mut Vec<(CurrencyId, u128)>,
        currency: CurrencyId,
        amount: u128,
    ) {
        // Find existing currency entry
        if let Some(entry) = currency_breakdown.iter_mut().find(|(c, _)| *c == currency) {
            entry.1 = entry.1.saturating_add(amount);
        } else {
            // Add new currency entry
            currency_breakdown.push((currency, amount));
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
