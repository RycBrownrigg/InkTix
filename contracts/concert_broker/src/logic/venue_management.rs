use crate::types::{Venue, VenueAddress, VenueType, AgeRestriction, InkTixError, InkTixResult};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Venue management business logic
pub struct VenueManager {
    pub venues: Mapping<u32, Venue>,
    pub next_venue_id: u32,
    pub venues_by_type: Mapping<u32, Vec<u32>>,
    pub venues_by_city: Mapping<u32, Vec<u32>>,
}

impl VenueManager {
    pub fn new() -> Self {
        Self {
            venues: Mapping::new(),
            next_venue_id: 1,
            venues_by_type: Mapping::new(),
            venues_by_city: Mapping::new(),
        }
    }

    /// Register a comprehensive venue profile
    pub fn register_venue(
        &mut self,
        name: String,
        address: VenueAddress,
        venue_type: VenueType,
        capacity: u32,
        standing_capacity: Option<u32>,
        acoustic_rating: u8,
        amenities: Vec<VenueAmenity>,
        age_restrictions: AgeRestriction,
        parking_available: bool,
        parking_capacity: Option<u32>,
        public_transport_access: bool,
        accessibility_features: Vec<AccessibilityFeature>,
        sound_system_quality: SoundSystemRating,
        lighting_capabilities: LightingCapabilities,
        security_level: SecurityLevel,
        established_year: Option<u32>,
        booking_contact: Option<String>,
        website: Option<String>,
    ) -> InkTixResult<u32> {
        if name.is_empty() || address.city.is_empty() || capacity == 0 || acoustic_rating > 10 {
            return Err(InkTixError::InvalidData);
        }

        let venue_id = self.next_venue_id;
        self.next_venue_id = self.next_venue_id
            .checked_add(1)
            .ok_or(InkTixError::IdOverflow)?;

        let current_time = ink::env::block_timestamp();

        let venue = Venue {
            id: venue_id,
            name,
            address: address.clone(),
            venue_type,
            capacity,
            standing_capacity,
            acoustic_rating,
            amenities,
            age_restrictions,
            parking_available,
            parking_capacity,
            public_transport_access,
            accessibility_features,
            sound_system_quality,
            lighting_capabilities,
            backstage_facilities: false,
            merchandise_space: false,
            food_and_beverage: false,
            alcohol_license: false,
            security_level,
            established_year,
            notable_past_performers: Vec::new(),
            average_ticket_price: 0,
            booking_contact,
            website,
            verified: false,
            created_at: current_time,
        };

        self.venues.insert(venue_id, &venue);
        
        self.update_venue_type_index(venue_id, venue_type);
        self.update_venue_city_index(venue_id, &address.city);

        Ok(venue_id)
    }

    /// Get venue details
    pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
        self.venues.get(venue_id)
    }

    /// Search venues by type
    pub fn search_venues_by_type(&self, venue_type: VenueType) -> Vec<u32> {
        let type_hash = self.hash_venue_type(venue_type);
        self.venues_by_type.get(type_hash).unwrap_or_default()
    }

    /// Search venues by city
    pub fn search_venues_by_city(&self, city: &str) -> Vec<u32> {
        let city_hash = self.hash_string(city);
        self.venues_by_city.get(city_hash).unwrap_or_default()
    }

    /// Update venue type index
    fn update_venue_type_index(&mut self, venue_id: u32, venue_type: VenueType) {
        let type_hash = self.hash_venue_type(venue_type);
        let mut venues_of_type = self.venues_by_type.get(type_hash).unwrap_or_default();
        if !venues_of_type.contains(&venue_id) {
            venues_of_type.push(venue_id);
            self.venues_by_type.insert(type_hash, &venues_of_type);
        }
    }

    /// Update venue city index
    fn update_venue_city_index(&mut self, venue_id: u32, city: &str) {
        let city_hash = self.hash_string(city);
        let mut venues_in_city = self.venues_by_city.get(city_hash).unwrap_or_default();
        if !venues_in_city.contains(&venue_id) {
            venues_in_city.push(venue_id);
            self.venues_by_city.insert(city_hash, &venues_in_city);
        }
    }

    /// Hash venue type for indexing
    fn hash_venue_type(&self, venue_type: VenueType) -> u32 {
        match venue_type {
            VenueType::Arena => 1, VenueType::Stadium => 2, VenueType::Theater => 3,
            VenueType::Club => 4, VenueType::Bar => 5, VenueType::ConcertHall => 6,
            VenueType::Amphitheater => 7, VenueType::FestivalGround => 8, VenueType::OperaHouse => 9,
            VenueType::JazzClub => 10, VenueType::ComedyClub => 11, VenueType::MultiPurpose => 12,
            VenueType::OutdoorVenue => 13, VenueType::PrivateVenue => 14, VenueType::RecordingStudio => 15,
            VenueType::Rooftop => 16, VenueType::Warehouse => 17, VenueType::Church => 18,
            VenueType::University => 19, VenueType::Other => 99,
        }
    }

    /// Hash string for indexing
    fn hash_string(&self, s: &str) -> u32 {
        let mut hash: u32 = 0;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }

    /// Get total venue count
    pub fn total_venues(&self) -> u32 {
        self.next_venue_id.saturating_sub(1)
    }
}