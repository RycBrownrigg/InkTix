use crate::storage::*;
use crate::types::*;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::vec;

/// Advanced team loyalty functionality
pub struct AdvancedTeamLoyalty;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl AdvancedTeamLoyalty {
    /// Create team loyalty profile
    pub fn create_team_loyalty_profile(
        storage: &mut SportsBrokerStorage,
        user: AccountId,
        team_id: u32,
    ) -> Result<u32, String> {
        let _team = storage.teams.get(team_id).ok_or("Team not found")?;
        
        let profile_id = storage.get_next_id("team_loyalty_profile");
        
        let profile = TeamLoyaltyProfile {
            user_id: user,
            team_id,
            loyalty_points: 0,
            loyalty_tier: TeamLoyaltyTier::Rookie,
            attendance_streak: 0,
            total_events_attended: 0,
            favorite_team_status: false,
            staked_amount: 0,
            staking_start_date: 0,
            last_attendance: 0,
            team_specific_benefits: vec![],
            created_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            last_updated: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
        };
        
        storage.team_loyalty_profiles.insert((user, team_id), &profile);
        Ok(profile_id)
    }

    /// Stake on favorite team
    pub fn stake_on_team(
        storage: &mut SportsBrokerStorage,
        user: AccountId,
        team_id: u32,
        amount: u128,
        currency: CurrencyId,
    ) -> Result<u32, String> {
        let _team = storage.teams.get(team_id).ok_or("Team not found")?;
        
        let stake_id = storage.get_next_id("team_stake");
        
        let stake = TeamStaking {
            user_id: user,
            team_id,
            staked_amount: amount,
            staking_start_date: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            staking_end_date: Some(ink::env::block_timestamp::<ink::env::DefaultEnvironment>() + 86400 * 30), // 30 days
            reward_multiplier: 10000, // 1.0x
            is_active: true,
            total_rewards_earned: 0,
        };
        
        storage.team_stakings.insert(stake_id, &stake);
        
        // Update loyalty profile
        if let Some(mut profile) = storage.team_loyalty_profiles.get((user, team_id)) {
            profile.staked_amount += amount;
            profile.loyalty_points += (amount / 1000000000000000000) as u32; // 1 point per DOT
            storage.team_loyalty_profiles.insert((user, team_id), &profile);
        }
        
        Ok(stake_id)
    }

    /// Record attendance for team loyalty
    pub fn record_attendance(
        storage: &mut SportsBrokerStorage,
        user: AccountId,
        team_id: u32,
        event_id: u32,
    ) -> Result<u32, String> {
        let _team = storage.teams.get(team_id).ok_or("Team not found")?;
        let _event = storage.events.get(event_id).ok_or("Event not found")?;
        
        let attendance_id = storage.get_next_id("attendance");
        
        let attendance = TeamAttendance {
            user_id: user,
            team_id,
            event_id,
            attendance_date: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            points_earned: 100, // 100 points per attendance
            streak_bonus: 0,
            total_streak: 0,
        };
        
        storage.team_attendance.insert(attendance_id, &attendance);
        
        // Update loyalty profile
        if let Some(mut profile) = storage.team_loyalty_profiles.get((user, team_id)) {
            profile.total_events_attended += 1;
            profile.attendance_streak += 1;
            profile.loyalty_points += 100;
            profile.last_attendance = attendance.attendance_date;
            
            // Update loyalty tier based on points
            profile.loyalty_tier = if profile.loyalty_points >= 10000 {
                TeamLoyaltyTier::LegendaryFan
            } else if profile.loyalty_points >= 2000 {
                TeamLoyaltyTier::UltraFan
            } else if profile.loyalty_points >= 500 {
                TeamLoyaltyTier::SuperFan
            } else if profile.loyalty_points >= 100 {
                TeamLoyaltyTier::Fan
            } else {
                TeamLoyaltyTier::Rookie
            };
            
            storage.team_loyalty_profiles.insert((user, team_id), &profile);
        }
        
        Ok(attendance_id)
    }
}