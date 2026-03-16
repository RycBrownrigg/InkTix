/// Sports-specific tests are included inline in lib.rs under #[cfg(test)]
/// This module provides additional sports test helpers.

pub struct SportsTestHelpers;

impl SportsTestHelpers {
    /// Helper to verify team registration
    pub fn verify_team_created(team_id: u32) {
        assert!(team_id > 0, "Team ID should be positive");
    }
}
