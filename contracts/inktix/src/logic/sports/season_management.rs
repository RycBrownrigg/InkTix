//! Season creation and management logic.
//!
//! Creates seasons with sport type mapping, date ranges, and early-bird pricing.
//!
//! # Functions
//! - `create_season` -- registers a new season with default game counts and pricing

use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;

/// Season management logic
pub struct SeasonManagement;

impl SeasonManagement {
    /// Create a new season with sport type, date range, and early-bird pricing defaults
    pub fn create_season(storage: &mut InkTixStorage, name: String, sport: String, _year: u32, start_date: u64, end_date: u64) -> u32 {
        let season_id = storage.get_next_id("season");
        let sport_type = match sport.as_str() {
            "Basketball" => SportType::Basketball, "Football" => SportType::Football,
            "Baseball" => SportType::Baseball, "Soccer" => SportType::Soccer,
            "Hockey" => SportType::Hockey, _ => SportType::Basketball,
        };
        let season = Season {
            id: season_id, name, sport_type, start_date, end_date,
            regular_season_games: 82, active: true,
            season_pass_base_price: 1000_000_000_000_000_000, early_bird_discount: 20,
            early_bird_deadline: start_date.saturating_sub(30 * 24 * 60 * 60),
        };
        storage.seasons.insert(season_id, &season);
        season_id
    }
}
