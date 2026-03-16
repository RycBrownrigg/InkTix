/// Concert-specific tests are included inline in lib.rs under #[cfg(test)]
/// This module provides additional concert test helpers.

pub struct ConcertTestHelpers;

impl ConcertTestHelpers {
    /// Helper to verify artist registration
    pub fn verify_artist_created(artist_id: u32) {
        assert!(artist_id > 0, "Artist ID should be positive");
    }
}
