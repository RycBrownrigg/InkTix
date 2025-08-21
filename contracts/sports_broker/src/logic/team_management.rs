use ink::prelude::*;
use crate::types::*;
use crate::storage::*;

/// Team management logic
pub struct TeamManagement;

impl TeamManagement {
    /// Register a new team
    pub fn register_team(
        storage: &mut SportsBrokerStorage,
        name: String,
        sport: String,
        city: String,
    ) -> u32 {
        let team_id = storage.get_next_id("team");
        
        let team = Team {
            id: team_id,
            name,
            sport_type: match sport.as_str() {
                "Basketball" => SportType::Basketball,
                "Football" => SportType::Football,
                "Baseball" => SportType::Baseball,
                "Soccer" => SportType::Soccer,
                "Hockey" => SportType::Hockey,
                _ => SportType::Basketball,
            },
            city,
            verified: false,
        };

        storage.teams.insert(team_id, &team);
        team_id
    }

    /// Get team information
    pub fn get_team(storage: &SportsBrokerStorage, team_id: u32) -> Option<Team> {
        storage.teams.get(team_id)
    }

    /// Update team performance
    pub fn update_team_performance(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        season_id: u32,
        wins: u32,
        losses: u32,
        points_scored: u32,
        playoff_rounds: u32,
        points_allowed: u32,
        total_games: u32,
    ) -> Result<(), String> {
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        let win_percentage = if wins + losses > 0 {
            ((wins as u64 * 10000) / (wins + losses) as u64) as u32
        } else {
            0
        };

        let performance = TeamPerformance {
            team_id,
            season_id,
            wins,
            losses,
            win_percentage,
            streak: 0,
            playoff_probability: 5000, // 50% default
            last_updated: 0,
            performance_rank: 0,
            home_record_wins: 0,
            home_record_losses: 0,
            points_scored_avg: points_scored,
            points_allowed_avg: points_allowed,
        };

        storage.team_performance.insert(team_id, &performance);

        // Update pricing multiplier based on performance
        Self::update_pricing_multiplier(storage, team_id, win_percentage);

        Ok(())
    }

    /// Get team performance
    pub fn get_team_performance(storage: &SportsBrokerStorage, team_id: u32) -> Option<TeamPerformance> {
        storage.team_performance.get(team_id)
    }

    /// Get pricing multiplier for a team
    pub fn get_pricing_multiplier(storage: &SportsBrokerStorage, team_id: u32) -> Option<PricingMultiplier> {
        storage.pricing_multipliers.get(team_id)
    }

    /// Update pricing multiplier based on team performance
    fn update_pricing_multiplier(storage: &mut SportsBrokerStorage, team_id: u32, win_percentage: u32) {
        let performance_multiplier = if win_percentage >= 7500 {
            12000 // 1.2x for 75%+ win rate
        } else if win_percentage >= 6000 {
            11000 // 1.1x for 60%+ win rate
        } else if win_percentage >= 5000 {
            10000 // 1.0x for 50%+ win rate
        } else if win_percentage >= 4000 {
            9000  // 0.9x for 40%+ win rate
        } else {
            8000  // 0.8x for <40% win rate
        };

        let multiplier = PricingMultiplier {
            team_id,
            base_multiplier: 10000,   // 1.0x base
            performance_multiplier,
            playoff_multiplier: 10000, // 1.0x base
            streak_multiplier: 10000,  // 1.0x base
            rivalry_multiplier: 10000, // 1.0x base
            demand_multiplier: 10000,  // 1.0x base
            final_multiplier: performance_multiplier, // For simplicity
            last_updated: 0,
        };

        storage.pricing_multipliers.insert(team_id, &multiplier);
    }
}
