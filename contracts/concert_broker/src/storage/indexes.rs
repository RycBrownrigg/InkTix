use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Search indexes for efficient event discovery
pub struct SearchIndexes {
    pub events_by_artist: Mapping<u32, Vec<u32>>,
    pub events_by_venue: Mapping<u32, Vec<u32>>,
    pub events_by_tour: Mapping<u32, Vec<u32>>,
    pub events_by_festival: Mapping<u32, Vec<u32>>,
    pub events_by_type: Mapping<u32, Vec<u32>>,
    pub events_by_date: Mapping<u64, Vec<u32>>,
}

impl SearchIndexes {
    pub fn new() -> Self {
        Self {
            events_by_artist: Mapping::new(),
            events_by_venue: Mapping::new(),
            events_by_tour: Mapping::new(),
            events_by_festival: Mapping::new(),
            events_by_type: Mapping::new(),
            events_by_date: Mapping::new(),
        }
    }

    /// Update all indexes for a new event
    pub fn update_event_indexes(&mut self, event_id: u32, event: &ConcertEvent) {
        // Update artist index (main artist)
        self.update_index(&mut self.events_by_artist, event.artist_id, event_id);

        // Update artist index for supporting artists
        for supporting_artist_id in &event.supporting_artists {
            self.update_index(&mut self.events_by_artist, *supporting_artist_id, event_id);
        }

        // Update venue index
        self.update_index(&mut self.events_by_venue, event.venue_id, event_id);

        // Update event type index
        let type_hash = self.hash_event_type(event.event_type);
        self.update_index(&mut self.events_by_type, type_hash, event_id);

        // Update date index
        let date_bucket = event.date / (24 * 60 * 60 * 1000);
        self.update_index(&mut self.events_by_date, date_bucket, event_id);

        // Update tour index if applicable
        if let Some(tour_id) = event.tour_id {
            self.update_index(&mut self.events_by_tour, tour_id, event_id);
        }

        // Update festival index if applicable
        if let Some(festival_id) = event.festival_id {
            self.update_index(&mut self.events_by_festival, festival_id, event_id);
        }
    }

    /// Helper function to update an index
    fn update_index(&mut self, index: &mut Mapping<u32, Vec<u32>>, key: u32, value: u32) {
        let mut values = index.get(key).unwrap_or_default();
        if !values.contains(&value) {
            values.push(value);
            index.insert(key, &values);
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
}