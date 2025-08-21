use ink::prelude::*;
use crate::types::*;
use crate::storage::*;

/// Venue management logic
pub struct VenueManagement;

impl VenueManagement {
    /// Register a new venue
    pub fn register_venue(
        storage: &mut SportsBrokerStorage,
        name: String,
        capacity: u32,
        address: String,
        _sport_type: String,
    ) -> u32 {
        let venue_id = storage.get_next_id("venue");
        
        let venue = Venue {
            id: venue_id,
            name,
            capacity,
            city: address, // Use address as city
        };

        storage.venues.insert(venue_id, &venue);
        venue_id
    }

    /// Get venue information
    pub fn get_venue(storage: &SportsBrokerStorage, venue_id: u32) -> Option<Venue> {
        storage.venues.get(venue_id)
    }
}
