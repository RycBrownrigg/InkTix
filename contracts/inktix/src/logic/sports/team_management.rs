use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;

/// Team management logic
pub struct TeamManagement;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl TeamManagement {
    pub fn register_team(storage: &mut InkTixStorage, name: String, city: String, sport_type: SportType) -> Result<u32, String> {
        let team_id = storage.get_next_id("team");
        let team = Team { id: team_id, name, sport_type, city, verified: false };
        storage.teams.insert(team_id, &team);
        Ok(team_id)
    }

    pub fn get_team(storage: &InkTixStorage, team_id: u32) -> Option<Team> { storage.teams.get(team_id) }

    pub fn update_team_performance(
        storage: &mut InkTixStorage, team_id: u32, season_id: u32,
        wins: u32, losses: u32, points_scored: u32, _playoff_rounds: u32,
        points_allowed: u32, _total_games: u32,
    ) -> Result<(), String> {
        if !storage.teams.contains(team_id) { return Err("Team not found".to_string()); }
        let win_percentage = if wins + losses > 0 { ((wins as u64 * 10000) / (wins + losses) as u64) as u32 } else { 0 };
        let performance = TeamPerformance {
            team_id, season_id, wins, losses, win_percentage, streak: 0,
            playoff_probability: 5000, last_updated: 0, performance_rank: 0,
            home_record_wins: 0, home_record_losses: 0,
            points_scored_avg: points_scored, points_allowed_avg: points_allowed,
        };
        storage.team_performance.insert(team_id, &performance);
        Self::update_pricing_multiplier(storage, team_id, win_percentage);
        Ok(())
    }

    pub fn get_all_teams(storage: &InkTixStorage) -> Vec<Team> {
        let mut teams = Vec::new();
        for team_id in 1..=storage.total_teams {
            if let Some(team) = storage.teams.get(team_id) { teams.push(team); }
        }
        teams
    }

    pub fn update_team(storage: &mut InkTixStorage, team_id: u32, name: Option<String>, city: Option<String>, sport_type: Option<SportType>) -> Result<(), String> {
        let mut team = storage.teams.get(team_id).ok_or("Team not found")?;
        if let Some(n) = name { team.name = n; }
        if let Some(c) = city { team.city = c; }
        if let Some(s) = sport_type { team.sport_type = s; }
        storage.teams.insert(team_id, &team);
        Ok(())
    }

    fn update_pricing_multiplier(storage: &mut InkTixStorage, team_id: u32, win_percentage: u32) {
        let performance_multiplier = if win_percentage >= 7500 { 12000 }
            else if win_percentage >= 6000 { 11000 }
            else if win_percentage >= 5000 { 10000 }
            else if win_percentage >= 4000 { 9000 }
            else { 8000 };
        let multiplier = PricingMultiplier {
            team_id, base_multiplier: 10000, performance_multiplier,
            playoff_multiplier: 10000, streak_multiplier: 10000, rivalry_multiplier: 10000,
            demand_multiplier: 10000, final_multiplier: performance_multiplier, last_updated: 0,
        };
        storage.pricing_multipliers.insert(team_id, &multiplier);
    }
}
