
use ink::primitives::AccountId;
use crate::types::*;
use crate::storage::contract_storage::SportsBrokerStorage;

pub struct FantasySportsManagement;

impl FantasySportsManagement {
    /// Create a new fantasy league
    pub fn create_fantasy_league(
        storage: &mut SportsBrokerStorage,
        name: String,
        description: String,
        league_type: FantasyLeagueType,
        max_teams: u32,
        entry_fee: u128,
        created_by: AccountId,
        start_date: u64,
        end_date: u64,
        season_id: u32,
        sport_type: String,
        rules: String,
        scoring_system: String,
        current_time: u64,
    ) -> Result<u32, String> {
        if name.is_empty() {
            return Err("League name cannot be empty".to_string());
        }
        if max_teams < 2 || max_teams > 100 {
            return Err("Max teams must be between 2 and 100".to_string());
        }
        if start_date >= end_date {
            return Err("Start date must be before end date".to_string());
        }
        if current_time >= start_date {
            return Err("Start date must be in the future".to_string());
        }

        let league_id = storage.get_next_fantasy_league_id();
        let league = FantasyLeague {
            league_id,
            name: name.clone(),
            description,
            league_type,
            status: FantasyLeagueStatus::Open,
            max_teams,
            entry_fee,
            prize_pool: entry_fee * max_teams as u128,
            created_by,
            created_at: current_time,
            start_date,
            end_date,
            season_id,
            sport_type,
            rules,
            scoring_system,
        };

        storage.fantasy_leagues.insert(league_id, &league);
        
        // Add creator to user's fantasy leagues
        let mut user_leagues = storage.user_fantasy_leagues.get(created_by).unwrap_or_default();
        user_leagues.push(league_id);
        storage.user_fantasy_leagues.insert(created_by, &user_leagues);

        Ok(league_id)
    }

    /// Join a fantasy league
    pub fn join_fantasy_league(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        league_id: u32,
        team_name: String,
        ticket_id: u64,
        current_time: u64,
    ) -> Result<u32, String> {
        let league = storage.fantasy_leagues.get(league_id)
            .ok_or("Fantasy league not found")?;

        if league.status != FantasyLeagueStatus::Open {
            return Err("League is not open for registration".to_string());
        }

        // Check if user already has a team in this league
        let user_teams = storage.user_fantasy_teams.get(user_id).unwrap_or_default();
        for team_id in &user_teams {
            let team = storage.fantasy_teams.get(*team_id).unwrap_or_default();
            if team.league_id == league_id {
                return Err("User already has a team in this league".to_string());
            }
        }

        // Check if league is full
        let participants = storage.league_participants.get(league_id).unwrap_or_default();
        if participants.len() >= league.max_teams as usize {
            return Err("League is full".to_string());
        }

        // Verify ticket ownership
        let ticket = storage.tickets.get(ticket_id)
            .ok_or("Ticket not found")?;
        if ticket.owner != user_id {
            return Err("User does not own this ticket".to_string());
        }

        // Create fantasy team
        let team_id = storage.get_next_fantasy_team_id();
        let team = FantasyTeam {
            team_id,
            league_id,
            owner: user_id,
            name: team_name.clone(),
            status: FantasyTeamStatus::Active,
            total_points: 0,
            rank: 0,
            created_at: current_time,
            last_updated: current_time,
            players: Vec::new(),
            captain_id: None,
            vice_captain_id: None,
        };

        storage.fantasy_teams.insert(team_id, &team);

        // Create participation record
        let participation_id = storage.get_next_fantasy_participation_id();
        let participation = FantasyLeagueParticipation {
            participation_id,
            user_id,
            league_id,
            team_id,
            ticket_id: ticket_id as u32, // Convert u64 to u32
            joined_at: current_time,
            is_active: true,
            bonus_points: 0,
            loyalty_multiplier: 1,
        };

        storage.fantasy_participations.insert(participation_id, &participation);

        // Update user's fantasy teams
        let mut user_teams = storage.user_fantasy_teams.get(user_id).unwrap_or_default();
        user_teams.push(team_id);
        storage.user_fantasy_teams.insert(user_id, &user_teams);

        // Update league participants
        let mut participants = storage.league_participants.get(league_id).unwrap_or_default();
        participants.push(team_id);
        storage.league_participants.insert(league_id, &participants);

        // Initialize leaderboard entry
        let mut leaderboard = storage.fantasy_leaderboards.get(league_id).unwrap_or_default();
        let entry = FantasyLeaderboardEntry {
            user_id,
            team_name,
            total_points: 0,
            rank: 0,
            games_played: 0,
            win_streak: 0,
            bonus_points: 0,
            loyalty_multiplier: 1,
        };
        leaderboard.entries.push(entry);
        leaderboard.last_updated = current_time;
        storage.fantasy_leaderboards.insert(league_id, &leaderboard);

        Ok(team_id)
    }

    /// Add player to fantasy team
    pub fn add_player_to_team(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        user_id: AccountId,
        player_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        let team = storage.fantasy_teams.get(team_id)
            .ok_or("Fantasy team not found")?;

        if team.owner != user_id {
            return Err("User does not own this team".to_string());
        }

        if team.status != FantasyTeamStatus::Active {
            return Err("Team is not active".to_string());
        }

        // Check if player is already in team
        if team.players.contains(&player_id) {
            return Err("Player is already in team".to_string());
        }

        // For now, we'll skip position validation since player_stats storage is not implemented
        // In a real implementation, this would validate player positions
        
        // Check position limits (simplified for now)
        let position_count = 0;
        // This would normally check against player_stats storage
        // For now, we'll allow any number of players

        let settings = storage.fantasy_settings.get(team.league_id).unwrap_or_default();
        if position_count >= settings.max_players_per_position {
            return Err("Position limit reached for this player type".to_string());
        }

        // Add player to team
        let mut updated_team = team.clone();
        updated_team.players.push(player_id);
        updated_team.last_updated = current_time;
        storage.fantasy_teams.insert(team_id, &updated_team);

        Ok(())
    }

    /// Set team captain and vice-captain
    pub fn set_team_captains(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        user_id: AccountId,
        captain_id: u32,
        vice_captain_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        let team = storage.fantasy_teams.get(team_id)
            .ok_or("Fantasy team not found")?;

        if team.owner != user_id {
            return Err("User does not own this team".to_string());
        }

        if team.status != FantasyTeamStatus::Active {
            return Err("Team is not active".to_string());
        }

        // Verify both players are in the team
        if !team.players.contains(&captain_id) {
            return Err("Captain must be in the team".to_string());
        }
        if !team.players.contains(&vice_captain_id) {
            return Err("Vice-captain must be in the team".to_string());
        }
        if captain_id == vice_captain_id {
            return Err("Captain and vice-captain must be different players".to_string());
        }

        let mut updated_team = team.clone();
        updated_team.captain_id = Some(captain_id);
        updated_team.vice_captain_id = Some(vice_captain_id);
        updated_team.last_updated = current_time;
        storage.fantasy_teams.insert(team_id, &updated_team);

        Ok(())
    }

    /// Transfer players in fantasy team
    pub fn transfer_players(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        user_id: AccountId,
        player_out: u32,
        player_in: u32,
        week_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        let team = storage.fantasy_teams.get(team_id)
            .ok_or("Fantasy team not found")?;

        if team.owner != user_id {
            return Err("User does not own this team".to_string());
        }

        if team.status != FantasyTeamStatus::Active {
            return Err("Team is not active".to_string());
        }

        // Check transfer limits (simplified - we'll implement proper tracking later)
        // For now, we'll allow transfers without strict weekly limits
        // In a real implementation, this would track transfers per week using a separate counter

        // Verify player_out is in team
        if !team.players.contains(&player_out) {
            return Err("Player to remove is not in team".to_string());
        }

        // Verify player_in is not in team
        if team.players.contains(&player_in) {
            return Err("Player to add is already in team".to_string());
        }

        // For now, we'll skip position validation since player_stats storage is not implemented
        // In a real implementation, this would validate player positions
        
        // Check position limits (simplified for now)
        let position_count = 0;
        // This would normally check against player_stats storage
        // For now, we'll allow any number of players
        
        // Get settings for position validation
        let settings = storage.fantasy_settings.get(team.league_id).unwrap_or_default();
        if position_count >= settings.max_players_per_position {
            return Err("Position limit reached for this player type".to_string());
        }

        // Perform transfer
        let mut updated_team = team.clone();
        updated_team.players.retain(|&pid| pid != player_out);
        updated_team.players.push(player_in);
        updated_team.last_updated = current_time;

        // Update captain/vice-captain if needed
        if updated_team.captain_id == Some(player_out) {
            updated_team.captain_id = None;
        }
        if updated_team.vice_captain_id == Some(player_out) {
            updated_team.vice_captain_id = None;
        }

        storage.fantasy_teams.insert(team_id, &updated_team);

        // Record transfer
        let transfer_id = storage.get_next_fantasy_transfer_id();
        let transfer = FantasyTransfer {
            transfer_id,
            team_id,
            user_id,
            player_out,
            player_in,
            transfer_cost: 0, // Could implement transfer costs later
            transfer_time: current_time,
            week_id,
        };
        storage.fantasy_transfers.insert(transfer_id, &transfer);

        Ok(())
    }

    /// Update player stats
    pub fn update_player_stats(
        _storage: &mut SportsBrokerStorage,
        _player_id: u32,
        _points: u32,
        _touchdowns: u32,
        _yards: u32,
        _completion_percentage: Option<u32>,
        _field_goal_percentage: Option<u32>,
        _current_time: u64,
    ) -> Result<(), String> {
        // For now, we'll skip updating player stats since player_stats storage is not implemented
        // In a real implementation, this would update the player_stats storage
        
        // This is a placeholder implementation
        Ok(())
    }

    /// Calculate fantasy team points for a game week
    pub fn calculate_team_points(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        _week_id: u32,
        _current_time: u64,
    ) -> Result<u32, String> {
        let team = storage.fantasy_teams.get(team_id)
            .ok_or("Fantasy team not found")?;

        let settings = storage.fantasy_settings.get(team.league_id).unwrap_or_default();
        let mut total_points = 0;

        // For now, we'll use a simplified scoring system since player_stats storage is not implemented
        // In a real implementation, this would calculate points based on actual player performance
        
        // Simplified scoring: each player gets 10 points, captain gets 2x, vice-captain gets 1.5x
        for &player_id in &team.players {
            let mut player_points = 10; // Base points per player

            // Apply captain multiplier
            if team.captain_id == Some(player_id) {
                player_points *= settings.captain_multiplier;
            } else if team.vice_captain_id == Some(player_id) {
                player_points *= settings.vice_captain_multiplier;
            }

            total_points += player_points;
        }

        // Update team total points
        let mut updated_team = team.clone();
        updated_team.total_points += total_points;
        updated_team.last_updated = current_time;
        storage.fantasy_teams.insert(team_id, &updated_team);

        // Update leaderboard
        let mut leaderboard = storage.fantasy_leaderboards.get(team.league_id).unwrap_or_default();
        for entry in &mut leaderboard.entries {
            if entry.user_id == team.owner {
                entry.total_points = updated_team.total_points;
                entry.games_played += 1;
                break;
            }
        }
        leaderboard.last_updated = current_time;
        storage.fantasy_leaderboards.insert(team.league_id, &leaderboard);

        Ok(total_points)
    }

    /// Get fantasy league leaderboard
    pub fn get_league_leaderboard(
        storage: &SportsBrokerStorage,
        league_id: u32,
    ) -> Result<FantasyLeaderboard, String> {
        storage.fantasy_leaderboards.get(league_id)
            .ok_or("Leaderboard not found".to_string())
    }

    /// Get user's fantasy teams
    pub fn get_user_fantasy_teams(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Vec<FantasyTeam> {
        let team_ids = storage.user_fantasy_teams.get(user_id).unwrap_or_default();
        team_ids.into_iter()
            .filter_map(|id| storage.fantasy_teams.get(id))
            .collect()
    }

    /// Get user's fantasy leagues
    pub fn get_user_fantasy_leagues(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Vec<FantasyLeague> {
        let league_ids = storage.user_fantasy_leagues.get(user_id).unwrap_or_default();
        league_ids.into_iter()
            .filter_map(|id| storage.fantasy_leagues.get(id))
            .collect()
    }

    /// Award fantasy sports loyalty points
    pub fn award_fantasy_loyalty_points(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        league_id: u32,
        points: u32,
        current_time: u64,
    ) -> Result<(), String> {
        // Get user's loyalty profile
        let mut loyalty_profile = storage.loyalty_profiles.get(user_id)
            .ok_or("Loyalty profile not found")?;

        // Add fantasy sports bonus points
        loyalty_profile.fantasy_sports_points += points;
        loyalty_profile.total_points += points;

        storage.loyalty_profiles.insert(user_id, &loyalty_profile);

        // Create fantasy reward record
        let reward_id = storage.get_next_fantasy_reward_id();
        let reward = FantasyRewards {
            reward_id,
            user_id,
            league_id,
            season_id: 0, // Will be set when season is known
            reward_type: "Fantasy Sports Loyalty".to_string(),
            reward_amount: 0,
            reward_currency: "Points".to_string(),
            loyalty_points: points,
            claimed_at: Some(current_time),
            is_claimed: true,
        };

        storage.fantasy_rewards.insert(reward_id, &reward);

        Ok(())
    }

    /// Create fantasy game week
    pub fn create_fantasy_game_week(
        storage: &mut SportsBrokerStorage,
        league_id: u32,
        season_id: u32,
        start_date: u64,
        end_date: u64,
        games: Vec<u32>,
        transfer_deadline: u64,
        captain_selection_deadline: u64,
        current_time: u64,
    ) -> Result<u32, String> {
        if start_date >= end_date {
            return Err("Start date must be before end date".to_string());
        }
        if transfer_deadline >= start_date {
            return Err("Transfer deadline must be before start date".to_string());
        }
        if captain_selection_deadline >= start_date {
            return Err("Captain selection deadline must be before start date".to_string());
        }

        let week_id = storage.get_next_fantasy_game_week_id();
        let game_week = FantasyGameWeek {
            week_id,
            league_id,
            season_id,
            start_date,
            end_date,
            games,
            is_active: false,
            transfer_deadline,
            captain_selection_deadline,
        };

        storage.fantasy_game_weeks.insert(week_id, &game_week);

        Ok(week_id)
    }

    /// Activate fantasy game week
    pub fn activate_fantasy_game_week(
        storage: &mut SportsBrokerStorage,
        week_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        let mut game_week = storage.fantasy_game_weeks.get(week_id)
            .ok_or("Game week not found")?;

        if current_time < game_week.start_date {
            return Err("Game week has not started yet".to_string());
        }

        game_week.is_active = true;
        storage.fantasy_game_weeks.insert(week_id, &game_week);

        Ok(())
    }

    /// Get fantasy league settings
    pub fn get_fantasy_settings(
        storage: &SportsBrokerStorage,
        league_id: u32,
    ) -> Result<FantasySettings, String> {
        storage.fantasy_settings.get(league_id)
            .ok_or("Fantasy settings not found".to_string())
    }

    /// Update fantasy league settings
    pub fn update_fantasy_settings(
        storage: &mut SportsBrokerStorage,
        league_id: u32,
        settings: FantasySettings,
    ) -> Result<(), String> {
        storage.fantasy_settings.insert(league_id, &settings);
        Ok(())
    }
}
