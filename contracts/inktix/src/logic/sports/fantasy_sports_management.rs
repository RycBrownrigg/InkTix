use crate::storage::*;
use crate::types::*;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;
use ink::prelude::vec;

pub struct FantasySportsManagement;

#[allow(clippy::arithmetic_side_effects)]
impl FantasySportsManagement {
    pub fn create_fantasy_league(storage: &mut InkTixStorage, user: AccountId, name: String, description: String, max_participants: u32, entry_fee: u128, _currency: CurrencyId) -> Result<u32, String> {
        let league_id = storage.get_next_id("fantasy_league");
        let league = FantasyLeague {
            league_id, name, description, league_type: FantasyLeagueType::SeasonLong,
            status: FantasyLeagueStatus::Open, max_teams: max_participants, entry_fee,
            prize_pool: entry_fee, created_by: user,
            created_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            start_date: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            end_date: ink::env::block_timestamp::<ink::env::DefaultEnvironment>() + 86400 * 365,
            season_id: 1, sport_type: "Basketball".to_string(),
            rules: "Standard fantasy rules".to_string(), scoring_system: "Standard scoring".to_string(),
        };
        storage.fantasy_leagues.insert(league_id, &league);
        Ok(league_id)
    }

    pub fn join_fantasy_league(storage: &mut InkTixStorage, _user: AccountId, league_id: u32) -> Result<u32, String> {
        let _league = storage.fantasy_leagues.get(league_id).ok_or("League not found")?;
        Ok(league_id)
    }

    pub fn create_fantasy_team(storage: &mut InkTixStorage, user: AccountId, league_id: u32, name: String) -> Result<u32, String> {
        let _league = storage.fantasy_leagues.get(league_id).ok_or("League not found")?;
        let team_id = storage.get_next_id("fantasy_team");
        let team = FantasyTeam {
            team_id, league_id, owner: user, name, status: FantasyTeamStatus::Active,
            total_points: 0, rank: 0,
            created_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            last_updated: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            players: vec![], captain_id: None, vice_captain_id: None,
        };
        storage.fantasy_teams.insert(team_id, &team);
        let mut user_teams = storage.user_fantasy_teams.get(&user).unwrap_or_default();
        user_teams.push(team_id);
        storage.user_fantasy_teams.insert(&user, &user_teams);
        Ok(team_id)
    }
}
