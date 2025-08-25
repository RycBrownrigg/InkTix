use crate::types::*;
use crate::storage::*;
use ink::primitives::AccountId;


/// Advanced Team Loyalty Programs functionality
pub struct AdvancedTeamLoyalty;

impl AdvancedTeamLoyalty {
    /// Create a team loyalty profile for a user
    pub fn create_team_loyalty_profile(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        team_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        // Verify team exists
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        // Check if profile already exists
        let profile_key = (user_id, team_id);
        if storage.team_loyalty_profiles.get(profile_key).is_some() {
            return Err("Team loyalty profile already exists".to_string());
        }

        let profile = TeamLoyaltyProfile {
            user_id,
            team_id,
            loyalty_points: 0,
            loyalty_tier: TeamLoyaltyTier::Rookie,
            attendance_streak: 0,
            total_events_attended: 0,
            favorite_team_status: false,
            staked_amount: 0,
            staking_start_date: 0,
            last_attendance: 0,
            team_specific_benefits: Vec::new(),
            created_at: current_time,
            last_updated: current_time,
        };

        // Store profile
        storage.team_loyalty_profiles.insert(profile_key, &profile);

        // Update user's team loyalty list
        let mut user_teams = storage.user_team_loyalty.get(user_id).unwrap_or_default();
        user_teams.push(team_id);
        storage.user_team_loyalty.insert(user_id, &user_teams);

        // Update team's fan list
        let mut team_fans = storage.team_fans.get(team_id).unwrap_or_default();
        team_fans.push(user_id);
        storage.team_fans.insert(team_id, &team_fans);

        // Update analytics
        Self::update_team_loyalty_analytics(storage, team_id, current_time)?;

        Ok(())
    }

    /// Stake on a favorite team for loyalty rewards
    pub fn stake_on_team(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        team_id: u32,
        amount: u128,
        current_time: u64,
    ) -> Result<u32, String> {
        // Verify team exists
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        // Check if user has team loyalty profile
        let profile_key = (user_id, team_id);
        let mut profile = storage.team_loyalty_profiles.get(profile_key)
            .ok_or("Team loyalty profile not found. Create one first.".to_string())?;

        // Validate staking amount
        if amount == 0 {
            return Err("Staking amount must be greater than 0".to_string());
        }

        // Check if already staking
        if profile.staked_amount > 0 {
            return Err("Already staking on this team".to_string());
        }

        // Create staking record
        let staking_id = storage.get_next_id("team_staking");
        let staking = TeamStaking {
            user_id,
            team_id,
            staked_amount: amount,
            staking_start_date: current_time,
            staking_end_date: None,
            reward_multiplier: Self::calculate_staking_multiplier(amount),
            is_active: true,
            total_rewards_earned: 0,
        };

        storage.team_stakings.insert(staking_id, &staking);

        // Update profile
        profile.staked_amount = amount;
        profile.staking_start_date = current_time;
        profile.favorite_team_status = true;
        profile.last_updated = current_time;
        storage.team_loyalty_profiles.insert(profile_key, &profile);

        // Update analytics
        Self::update_team_loyalty_analytics(storage, team_id, current_time)?;

        Ok(staking_id)
    }

    /// Unstake from a team
    pub fn unstake_from_team(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        team_id: u32,
        current_time: u64,
    ) -> Result<u128, String> {
        // Check if user has team loyalty profile
        let profile_key = (user_id, team_id);
        let mut profile = storage.team_loyalty_profiles.get(profile_key)
            .ok_or("Team loyalty profile not found".to_string())?;

        if profile.staked_amount == 0 {
            return Err("No active staking on this team".to_string());
        }

        let staked_amount = profile.staked_amount;

        // Find and update staking record
        for staking_id in 1..=storage.total_team_stakings {
            if let Some(mut staking) = storage.team_stakings.get(staking_id) {
                if staking.user_id == user_id && staking.team_id == team_id && staking.is_active {
                    staking.is_active = false;
                    staking.staking_end_date = Some(current_time);
                    storage.team_stakings.insert(staking_id, &staking);
                    break;
                }
            }
        }

        // Update profile
        profile.staked_amount = 0;
        profile.staking_start_date = 0;
        profile.favorite_team_status = false;
        profile.last_updated = current_time;
        storage.team_loyalty_profiles.insert(profile_key, &profile);

        // Update analytics
        Self::update_team_loyalty_analytics(storage, team_id, current_time)?;

        Ok(staked_amount)
    }

    /// Record team attendance for streak tracking
    pub fn record_team_attendance(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        team_id: u32,
        event_id: u32,
        current_time: u64,
    ) -> Result<u32, String> {
        // Verify team exists
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        // Check if user has team loyalty profile
        let profile_key = (user_id, team_id);
        let mut profile = storage.team_loyalty_profiles.get(profile_key)
            .ok_or("Team loyalty profile not found. Create one first.".to_string())?;

        // Create attendance record
        let attendance_id = storage.get_next_id("team_attendance");
        let mut attendance = TeamAttendance {
            user_id,
            team_id,
            event_id,
            attendance_date: current_time,
            points_earned: 0,
            streak_bonus: 0,
            total_streak: 0,
        };

        // Calculate attendance streak
        let days_since_last = if profile.last_attendance > 0 {
            (current_time - profile.last_attendance) / (24 * 60 * 60 * 1000)
        } else {
            0
        };

        if days_since_last <= 7 { // Within a week
            profile.attendance_streak += 1;
            attendance.streak_bonus = Self::calculate_streak_bonus(profile.attendance_streak);
        } else {
            profile.attendance_streak = 1;
        }

        // Calculate points earned
        let base_points = 10; // Base points for attendance
        let streak_points = attendance.streak_bonus;
        let staking_bonus = if profile.staked_amount > 0 { 5 } else { 0 }; // Bonus for stakers
        
        attendance.points_earned = base_points + streak_points + staking_bonus;
        attendance.total_streak = profile.attendance_streak;

        // Update profile
        profile.loyalty_points += attendance.points_earned;
        profile.total_events_attended += 1;
        profile.last_attendance = current_time;
        profile.last_updated = current_time;

        // Check for tier upgrade
        let new_tier = Self::calculate_team_loyalty_tier(profile.loyalty_points);
        if new_tier != profile.loyalty_tier {
            profile.loyalty_tier = new_tier.clone();
            profile.team_specific_benefits = Self::get_benefits_for_tier(new_tier);
        }

        // Store attendance record
        storage.team_attendance.insert(attendance_id, &attendance);

        // Update profile
        storage.team_loyalty_profiles.insert(profile_key, &profile);

        // Update analytics
        Self::update_team_loyalty_analytics(storage, team_id, current_time)?;

        Ok(attendance_id)
    }

    /// Award performance-based rewards for team achievements
    pub fn award_team_performance_reward(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        reward_type: TeamPerformanceRewardType,
        points_multiplier: u32,
        start_date: u64,
        end_date: u64,
        current_time: u64,
    ) -> Result<u32, String> {
        // Verify team exists
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        // Create performance reward
        let reward_id = storage.get_next_id("team_performance_reward");
        let reward = TeamPerformanceReward {
            id: reward_id,
            team_id,
            reward_type,
            points_multiplier,
            start_date,
            end_date,
            is_active: true,
        };

        storage.team_performance_rewards.insert(reward_id, &reward);

        // Award points to all team fans
        if let Some(team_fans) = storage.team_fans.get(team_id) {
            for fan_id in team_fans {
                if let Some(mut profile) = storage.team_loyalty_profiles.get((fan_id, team_id)) {
                    let bonus_points = points_multiplier * 10; // Base 10 points * multiplier
                    profile.loyalty_points += bonus_points;
                    profile.last_updated = current_time;

                    // Check for tier upgrade
                    let new_tier = Self::calculate_team_loyalty_tier(profile.loyalty_points);
                    if new_tier != profile.loyalty_tier {
                        profile.loyalty_tier = new_tier.clone();
                        profile.team_specific_benefits = Self::get_benefits_for_tier(new_tier);
                    }

                    storage.team_loyalty_profiles.insert((fan_id, team_id), &profile);
                }
            }
        }

        // Update analytics
        Self::update_team_loyalty_analytics(storage, team_id, current_time)?;

        Ok(reward_id)
    }

    /// Create a team loyalty challenge
    pub fn create_team_loyalty_challenge(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        name: String,
        description: String,
        challenge_type: TeamChallengeType,
        points_reward: u32,
        start_date: u64,
        end_date: u64,
        completion_criteria: String,
    ) -> Result<u32, String> {
        // Verify team exists
        if !storage.teams.contains(team_id) {
            return Err("Team not found".to_string());
        }

        // Validate dates
        if start_date >= end_date {
            return Err("Start date must be before end date".to_string());
        }

        // Create challenge
        let challenge_id = storage.get_next_id("team_loyalty_challenge");
        let challenge = TeamLoyaltyChallenge {
            id: challenge_id,
            team_id,
            name,
            description,
            challenge_type,
            points_reward,
            start_date,
            end_date,
            is_active: true,
            participants: Vec::new(),
            completion_criteria,
        };

        storage.team_loyalty_challenges.insert(challenge_id, &challenge);

        Ok(challenge_id)
    }

    /// Join a team loyalty challenge
    pub fn join_team_challenge(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        challenge_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        // Get challenge
        let mut challenge = storage.team_loyalty_challenges.get(challenge_id)
            .ok_or("Challenge not found".to_string())?;

        if !challenge.is_active {
            return Err("Challenge is not active".to_string());
        }

        if current_time < challenge.start_date || current_time > challenge.end_date {
            return Err("Challenge is not currently running".to_string());
        }

        // Check if user is already participating
        if challenge.participants.contains(&user_id) {
            return Err("Already participating in this challenge".to_string());
        }

        // Add user to participants
        challenge.participants.push(user_id);
        storage.team_loyalty_challenges.insert(challenge_id, &challenge);

        Ok(())
    }

    /// Complete a team loyalty challenge
    pub fn complete_team_challenge(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        challenge_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        // Get challenge
        let challenge = storage.team_loyalty_challenges.get(challenge_id)
            .ok_or("Challenge not found".to_string())?;

        if !challenge.is_active {
            return Err("Challenge is not active".to_string());
        }

        if current_time < challenge.start_date || current_time > challenge.end_date {
            return Err("Challenge is not currently running".to_string());
        }

        // Check if user is participating
        if !challenge.participants.contains(&user_id) {
            return Err("Not participating in this challenge".to_string());
        }

            // Award points to user for all their team loyalty profiles
            if let Some(user_teams) = storage.user_team_loyalty.get(user_id) {
                for team_id in user_teams {
                    if let Some(mut profile) = storage.team_loyalty_profiles.get((user_id, team_id)) {
                        profile.loyalty_points += challenge.points_reward;
                        profile.last_updated = current_time;

                        // Check for tier upgrade
                        let new_tier = Self::calculate_team_loyalty_tier(profile.loyalty_points);
                        if new_tier != profile.loyalty_tier {
                            profile.loyalty_tier = new_tier.clone();
                            profile.team_specific_benefits = Self::get_benefits_for_tier(new_tier);
                        }

                        storage.team_loyalty_profiles.insert((user_id, team_id), &profile);
                    }
                }
            }

        Ok(())
    }

    /// Get team loyalty profile for a user
    pub fn get_team_loyalty_profile(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
        team_id: u32,
    ) -> Option<TeamLoyaltyProfile> {
        let profile_key = (user_id, team_id);
        storage.team_loyalty_profiles.get(profile_key)
    }

    /// Get all team loyalty profiles for a user
    pub fn get_user_team_loyalty_profiles(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Vec<TeamLoyaltyProfile> {
        if let Some(team_ids) = storage.user_team_loyalty.get(user_id) {
            team_ids.iter()
                .filter_map(|&team_id| {
                    let profile_key = (user_id, team_id);
                    storage.team_loyalty_profiles.get(profile_key)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get team loyalty analytics
    pub fn get_team_loyalty_analytics(
        storage: &SportsBrokerStorage,
        team_id: u32,
    ) -> Option<TeamLoyaltyAnalytics> {
        storage.team_loyalty_analytics.get(team_id)
    }

    // Helper methods
    fn calculate_staking_multiplier(staked_amount: u128) -> u32 {
        // Higher staking amounts get higher multipliers
        match staked_amount {
            0..=1_000_000_000_000_000_000 => 10000,        // 1.0x for 0-1 DOT
            1_000_000_000_000_000_001..=10_000_000_000_000_000_000 => 12000, // 1.2x for 1-10 DOT
            10_000_000_000_000_000_001..=100_000_000_000_000_000_000 => 15000, // 1.5x for 10-100 DOT
            _ => 20000, // 2.0x for 100+ DOT
        }
    }

    fn calculate_streak_bonus(streak: u32) -> u32 {
        match streak {
            0..=2 => 0,      // No bonus for 0-2 games
            3..=5 => 5,      // 5 bonus points for 3-5 games
            6..=10 => 10,    // 10 bonus points for 6-10 games
            11..=20 => 20,   // 20 bonus points for 11-20 games
            _ => 30,         // 30 bonus points for 20+ games
        }
    }

    fn calculate_team_loyalty_tier(points: u32) -> TeamLoyaltyTier {
        match points {
            0..=99 => TeamLoyaltyTier::Rookie,
            100..=499 => TeamLoyaltyTier::Fan,
            500..=1999 => TeamLoyaltyTier::SuperFan,
            2000..=9999 => TeamLoyaltyTier::UltraFan,
            _ => TeamLoyaltyTier::LegendaryFan,
        }
    }

    fn get_benefits_for_tier(tier: TeamLoyaltyTier) -> Vec<TeamBenefit> {
        match tier {
            TeamLoyaltyTier::Rookie => vec![
                TeamBenefit::EarlyTicketAccess,
            ],
            TeamLoyaltyTier::Fan => vec![
                TeamBenefit::EarlyTicketAccess,
                TeamBenefit::ConcessionDiscounts,
            ],
            TeamLoyaltyTier::SuperFan => vec![
                TeamBenefit::EarlyTicketAccess,
                TeamBenefit::ConcessionDiscounts,
                TeamBenefit::ParkingPass,
                TeamBenefit::ExclusiveMerchandise,
            ],
            TeamLoyaltyTier::UltraFan => vec![
                TeamBenefit::EarlyTicketAccess,
                TeamBenefit::ConcessionDiscounts,
                TeamBenefit::ParkingPass,
                TeamBenefit::ExclusiveMerchandise,
                TeamBenefit::PrioritySeating,
                TeamBenefit::MeetAndGreetAccess,
            ],
            TeamLoyaltyTier::LegendaryFan => vec![
                TeamBenefit::EarlyTicketAccess,
                TeamBenefit::ConcessionDiscounts,
                TeamBenefit::ParkingPass,
                TeamBenefit::ExclusiveMerchandise,
                TeamBenefit::PrioritySeating,
                TeamBenefit::MeetAndGreetAccess,
                TeamBenefit::BackstageAccess,
                TeamBenefit::TeamPracticeAccess,
                TeamBenefit::ChampionshipRing,
                TeamBenefit::AlumniAssociation,
            ],
        }
    }

    fn update_team_loyalty_analytics(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        current_time: u64,
    ) -> Result<(), String> {
        let mut analytics = storage.team_loyalty_analytics.get(team_id)
            .unwrap_or_default();

        // Get team fans
        let team_fans = storage.team_fans.get(team_id).unwrap_or_default();
        analytics.total_fans = team_fans.len() as u32;

        // Calculate total loyalty points and find most loyal fan
        let mut total_points = 0;
        let mut most_loyal_fan = None;
        let mut highest_points = 0;

        for &fan_id in &team_fans {
            if let Some(profile) = storage.team_loyalty_profiles.get((fan_id, team_id)) {
                total_points += profile.loyalty_points;
                if profile.loyalty_points > highest_points {
                    highest_points = profile.loyalty_points;
                    most_loyal_fan = Some(fan_id);
                }
            }
        }

        analytics.total_loyalty_points = total_points;
        analytics.most_loyal_fan = most_loyal_fan;

        // Calculate average loyalty tier
        let mut tier_sum = 0;
        let mut fan_count = 0;
        for &fan_id in &team_fans {
            if let Some(profile) = storage.team_loyalty_profiles.get((fan_id, team_id)) {
                tier_sum += profile.loyalty_tier as u32;
                fan_count += 1;
            }
        }

        if fan_count > 0 {
            let average_tier_value = tier_sum / fan_count;
            analytics.average_loyalty_tier = match average_tier_value {
                0 => TeamLoyaltyTier::Rookie,
                1 => TeamLoyaltyTier::Fan,
                2 => TeamLoyaltyTier::SuperFan,
                3 => TeamLoyaltyTier::UltraFan,
                _ => TeamLoyaltyTier::LegendaryFan,
            };
        }

        // Calculate total staked amount
        let mut total_staked = 0;
        for staking_id in 1..=storage.total_team_stakings {
            if let Some(staking) = storage.team_stakings.get(staking_id) {
                if staking.team_id == team_id && staking.is_active {
                    total_staked += staking.staked_amount;
                }
            }
        }
        analytics.total_staked_amount = total_staked;

        // Calculate total attendance and longest streak
        let mut total_attendance = 0;
        let mut longest_streak = 0;
        for attendance_id in 1..=storage.total_team_attendance {
            if let Some(attendance) = storage.team_attendance.get(attendance_id) {
                if attendance.team_id == team_id {
                    total_attendance += 1;
                    if attendance.total_streak > longest_streak {
                        longest_streak = attendance.total_streak;
                    }
                }
            }
        }
        analytics.total_attendance = total_attendance;
        analytics.longest_attendance_streak = longest_streak;

        analytics.last_updated = current_time;
        storage.team_loyalty_analytics.insert(team_id, &analytics);

        Ok(())
    }
}
