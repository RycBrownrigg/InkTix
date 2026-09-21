//! Concert test helpers for contract verification.
//!
//! Provides reusable assertion helpers for concert-specific contract state.
//! Primary tests live inline in `lib.rs` under `#[cfg(test)]`.

/// Helper utilities for concert-related contract tests
// Not yet called by an inline `lib.rs` test; kept available for future use.
#[allow(dead_code)]
pub struct ConcertTestHelpers;

#[allow(dead_code)]
impl ConcertTestHelpers {
    /// Helper to verify artist registration
    pub fn verify_artist_created(artist_id: u32) {
        assert!(artist_id > 0, "Artist ID should be positive");
    }
}
