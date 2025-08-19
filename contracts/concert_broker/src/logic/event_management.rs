use crate::types::{ConcertEvent, EventType, AgeRestriction, InkTixError, InkTixResult};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Event management business logic
pub struct EventManager {
    pub concert_events: Mapping<u32, ConcertEvent>,
    pub next_concert_event_id: u32,
    pub events_by_artist: Mapping<u32, Vec<u32>>,
    pub events_by_venue: Mapping<u32, Vec<u32>>,
    pub events_by_type: Mapping<u32, Vec<u32>>,
    pub events_by_date: Mapping<u64, Vec<u32>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            concert_events: Mapping::new(),
            next_concert_event_id: 1,
            events_by_artist: Mapping::new(),
            events_by_venue: Mapping::new(),
            events_by_type: Mapping::new(),
            events_by_date: Mapping::new(),
        }
    }

    /// Create a comprehensive concert event
    pub fn create_concert_event(
        &mut self,
        name: String,
        artist_id: u32,
        venue_id: u32,
        date: u64,
        doors_open: u64,
        show_start: u64,
        estimated_end: u64,
        capacity: u32,
        base_price: u128,
        event_type: EventType,
        supporting_artists: Vec<u32>,
        age_restriction: AgeRestriction,
        special_notes: String,
    ) -> InkTixResult<u32> {
        if name.is_empty() {
            return Err(InkTixError::InvalidData);
        }

        if doors_open >= show_start || show_start >= estimated_end || date > doors_open {
            return Err(InkTixError::InvalidData);
        }

        if supporting_artists.len() > 10 {
            return Err(InkTixError::InvalidData);
        }

        let event_id = self.next_concert_event_id;
        self.next_concert_event_id = self.next_concert_event_id
            .checked_add(1)
            .ok_or(InkTixError::IdOverflow)?;

        let current_time = ink::env::block_timestamp();

        let concert_event = ConcertEvent {
            id: event_id,
            name,
            artist_id,
            venue_id,
            date,
            doors_open,
            show_start,
            estimated_end,
            capacity,
            sold_tickets: 0,
            base_price,
            active: true,
            event_type,
            tour_id: None,
            festival_id: None,
            supporting_artists,
            merchandise_available: false,
            vip_packages: Vec::new(),
            age_restriction,
            revenue_generated: 0,
            special_notes,
            presale_enabled: false,
            presale_start: None,
            general_sale_start: None,
            sound_check_available: false,
            meet_greet_available: false,
            recording_allowed: false,
            live_stream_available: false,
            setlist_length_minutes: None,
            encore_expected: false,
            festival_stage: None,
            created_at: current_time,
            last_updated: current_time,
        };

        self.concert_events.insert(event_id, &concert_event);
        self.update_concert_event_indexes(event_id, &concert_event);

        Ok(event_id)
    }

    /// Get concert event details
    pub fn get_concert_event(&self, event_id: u32) -> Option<ConcertEvent> {
        self.concert_events.get(event_id)
    }

    /// Search events by artist
    pub fn search_events_by_artist(&self, artist_id: u32) -> Vec<u32> {
        self.events_by_artist.get(artist_id).unwrap_or_default()
    }

    /// Search events by venue
    pub fn search_events_by_venue(&self, venue_id: u32) -> Vec<u32> {
        self.events_by_venue.get(venue_id).unwrap_or_default()
    }

    /// Search events by type
    pub fn search_events_by_type(&self, event_type: EventType) -> Vec<u32> {
        let type_hash = self.hash_event_type(event_type);
        self.events_by_type.get(type_hash).unwrap_or_default()
    }

    /// Update concert event indexes
    fn update_concert_event_indexes(&mut self, event_id: u32, event: &ConcertEvent) {
        // Update artist index (main artist)
        let mut artist_events = self.events_by_artist.get(event.artist_id).unwrap_or_default();
        if !artist_events.contains(&event_id) {
            artist_events.push(event_id);
            self.events_by_artist.insert(event.artist_id, &artist_events);
        }

        // Update artist index for supporting artists
        for supporting_artist_id in &event.supporting_artists {
            let mut supporting_events = self.events_by_artist.get(*supporting_artist_id).unwrap_or_default();
            if !supporting_events.contains(&event_id) {
                supporting_events.push(event_id);
                self.events_by_artist.insert(*supporting_artist_id, &supporting_events);
            }
        }

        // Update venue index
        let mut venue_events = self.events_by_venue.get(event.venue_id).unwrap_or_default();
        if !venue_events.contains(&event_id) {
            venue_events.push(event_id);
            self.events_by_venue.insert(event.venue_id, &venue_events);
        }

        // Update event type index
        let type_hash = self.hash_event_type(event.event_type);
        let mut type_events = self.events_by_type.get(type_hash).unwrap_or_default();
        if !type_events.contains(&event_id) {
            type_events.push(event_id);
            self.events_by_type.insert(type_hash, &type_events);
        }

        // Update date index
        let date_bucket = event.date / (24 * 60 * 60 * 1000);
        let mut date_events = self.events_by_date.get(date_bucket).unwrap_or_default();
        if !date_events.contains(&event_id) {
            date_events.push(event_id);
            self.events_by_date.insert(date_bucket, &date_events);
        }
    }

    /// Hash event type for indexing
    fn hash_event_type(&self, event_type: EventType) -> u32 {
        match event_type {
            EventType::Concert => 1, EventType::FestivalDay => 2, EventType::MeetAndGreet => 3,
            EventType::SoundCheck => 4, EventType::AlbumLaunch => 5, EventType::AcousticSession => 6,
            EventType::VirtualConcert => 7, EventType::PrivateEvent => 8, EventType::Masterclass => 9,
            EventType::ListeningParty => 10, EventType::UnpluggedSession => 11, EventType::CharityBenefit => 12,
            EventType::TributeConcert => 13, EventType::ResidencyShow => 14, EventType::PopupPerformance => 15,
        }
    }

    /// Get total event count
    pub fn total_concert_events(&self) -> u32 {
        self.next_concert_event_id.saturating_sub(1)
    }
}