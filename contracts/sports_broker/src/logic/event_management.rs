use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Event management logic
pub struct EventManagement;

impl EventManagement {
    /// Create a new sports event
    pub fn create_sports_event(
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
        season_pass_discount: u8,
        dynamic_pricing_enabled: bool,
        rivalry_multiplier: u32,
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
        let _venue = storage.venues.get(venue_id)
            .ok_or("Venue not found")?;
        
        // Validate teams exist
        let _home_team = storage.teams.get(home_team_id)
            .ok_or("Home team not found")?;
        let _away_team = storage.teams.get(away_team_id)
            .ok_or("Away team not found")?;
        
        // Validate season exists
        let _season = storage.seasons.get(season_id)
            .ok_or("Season not found")?;
        
        let event_id = storage.get_next_id("event");
        
        let event = SportsEvent {
            id: event_id,
            name,
            venue_id,
            date,
            capacity,
            sold_tickets: 0,
            base_price,
            active: true,
            sport_type,
            home_team_id,
            away_team_id,
            season_id,
            game_type,
            season_pass_discount,
            dynamic_pricing_enabled,
            rivalry_multiplier,
            revenue_generated: 0,
        };
        
        // Store event
        storage.events.insert(event_id, &event);
        
        // Create event analytics
        Self::create_event_analytics(storage, event_id, &event);
        
        Ok(event_id)
    }
    
    /// Get event by ID
    pub fn get_event(storage: &SportsBrokerStorage, event_id: u32) -> Option<SportsEvent> {
        storage.events.get(event_id)
    }
    
    /// Update event status
    pub fn update_event_status(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        active: bool,
    ) -> Result<(), String> {
        let mut event = storage.events.get(event_id)
            .ok_or("Event not found")?;
        
        event.active = active;
        storage.events.insert(event_id, &event);
        
        Ok(())
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
    
    /// Get events by team
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
    
    /// Update event capacity
    pub fn update_event_capacity(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        new_capacity: u32,
    ) -> Result<(), String> {
        if new_capacity == 0 {
            return Err("Capacity must be greater than 0".to_string());
        }
        
        let mut event = storage.events.get(event_id)
            .ok_or("Event not found")?;
        
        if new_capacity < event.sold_tickets {
            return Err("New capacity cannot be less than sold tickets".to_string());
        }
        
        event.capacity = new_capacity;
        storage.events.insert(event_id, &event);
        
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
        
        let mut event = storage.events.get(event_id)
            .ok_or("Event not found")?;
        
        event.base_price = new_price;
        storage.events.insert(event_id, &event);
        
        Ok(())
    }
    
    /// Get event statistics
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
    
    // Helper methods
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
}

/// Event statistics
#[derive(Debug, Clone, PartialEq)]
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
