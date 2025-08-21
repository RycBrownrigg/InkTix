use ink::prelude::*;
use crate::types::*;
use crate::storage::*;

/// Season management logic
pub struct SeasonManagement;

impl SeasonManagement {
    /// Create a new season
    pub fn create_season(
        storage: &mut SportsBrokerStorage,
        name: String,
        sport: String,
        _year: u32,
        start_date: u64,
        end_date: u64,
    ) -> u32 {
        let season_id = storage.get_next_id("season");
        
        let sport_type = match sport.as_str() {
            "Basketball" => SportType::Basketball,
            "Football" => SportType::Football,
            "Baseball" => SportType::Baseball,
            "Soccer" => SportType::Soccer,
            "Hockey" => SportType::Hockey,
            _ => SportType::Basketball,
        };

        let season = Season {
            id: season_id,
            name,
            sport_type,
            start_date,
            end_date,
            regular_season_games: 82, // Default for most sports
            active: true,
            season_pass_base_price: 1000_000_000_000_000_000, // Default price
            early_bird_discount: 20, // 20% discount
            early_bird_deadline: start_date.saturating_sub(30 * 24 * 60 * 60), // 30 days before
        };

        storage.seasons.insert(season_id, &season);
        season_id
    }
}
