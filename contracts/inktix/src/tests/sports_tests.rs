//! Sports test helpers for contract verification.
//!
//! Provides reusable assertion helpers for sports-specific contract state.
//! Primary tests live inline in `lib.rs` under `#[cfg(test)]`.

/// Helper utilities for sports-related contract tests
pub struct SportsTestHelpers;

impl SportsTestHelpers {
    /// Helper to verify team registration
    pub fn verify_team_created(team_id: u32) {
        assert!(team_id > 0, "Team ID should be positive");
    }
}
