use crate::types::{Team, Venue, Season, SportsEvent, SportsTicket, EventStatus};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;
use ink::prelude::string::String;
use ink::primitives::AccountId;

/// Central storage management for Sports Broker
pub struct StorageManager {
    // Team management
    pub teams: Mapping<u32, Team>,
    pub next_team_id: u32,
    
    // Venue management  
    pub venues: Mapping<u32, Venue>,
    pub next_venue_id: u32,

    // Season management
    pub seasons: Mapping<u32, Season>,
    pub next_season_id: u32,

    // Event management
    pub events: Mapping<u32, SportsEvent>,
    pub next_event_id: u32,

    // Ticket management
    pub tickets: Mapping<u64, SportsTicket>,
    pub next_ticket_id: u64,
    pub user_tickets: Mapping<AccountId, Vec<u64>>,

    // Counters
    pub total_teams: u32,
    pub total_venues: u32,
    pub total_events: u32,
    pub total_tickets: u32,
    pub total_seasons: u32,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            teams: Mapping::new(),
            venues: Mapping::new(),
            seasons: Mapping::new(),
            events: Mapping::new(),
            tickets: Mapping::new(),
            user_tickets: Mapping::new(),
            next_team_id: 1,
            next_venue_id: 1,
            next_season_id: 1,
            next_event_id: 1,
            next_ticket_id: 1,
            total_teams: 0,
            total_venues: 0,
            total_events: 0,
            total_tickets: 0,
            total_seasons: 0,
        }
    }

    pub fn register_team(&mut self, name: String, sport: String, city: String) -> u32 {
        let team_id = self.next_team_id;
        self.next_team_id += 1;

        let team = Team {
            id: team_id,
            name,
            sport,
            city,
            verified: false,
        };

        self.teams.insert(team_id, &team);
        self.total_teams += 1;
        team_id
    }

    pub fn register_venue(&mut self, name: String, capacity: u32, address: String, sport_type: String) -> u32 {
        let venue_id = self.next_venue_id;
        self.next_venue_id += 1;

        let venue = Venue {
            id: venue_id,
            name,
            capacity,
            address,
            sport_type,
        };

        self.venues.insert(venue_id, &venue);
        self.total_venues += 1;
        venue_id
    }

    pub fn create_season(&mut self, name: String, sport: String, year: u32, start_date: u64, end_date: u64) -> u32 {
        let season_id = self.next_season_id;
        self.next_season_id += 1;

        let season = Season {
            id: season_id,
            name,
            sport,
            year,
            start_date,
            end_date,
            active: true,
        };

        self.seasons.insert(season_id, &season);
        self.total_seasons += 1;
        season_id
    }

    pub fn get_team(&self, team_id: u32) -> Option<Team> {
        self.teams.get(team_id)
    }

    pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
        self.venues.get(venue_id)
    }

    pub fn get_season(&self, season_id: u32) -> Option<Season> {
        self.seasons.get(season_id)
    }

    pub fn get_stats(&self) -> (u32, u32, u32, u32, u32) {
        (self.total_teams, self.total_venues, self.total_events, self.total_tickets, self.total_seasons)
    }
}