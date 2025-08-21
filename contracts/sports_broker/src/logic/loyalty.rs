use ink::prelude::*;
use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Loyalty and rewards logic
pub struct Loyalty;

impl Loyalty {
    /// Create a new loyalty profile for a user
    pub fn create_loyalty_profile(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
    ) -> Result<(), String> {
        let profile = LoyaltyProfile {
            user_id,
            total_points: 0,
            current_tier: LoyaltyTier::Bronze,
            points_earned_this_month: 0,
            points_earned_this_year: 0,
            total_tickets_purchased: 0,
            total_spent: 0,
            join_date: 0, // Will be set by caller
            last_activity: 0, // Will be set by caller
            streak_days: 0,
            referral_count: 0,
            referral_points: 0,
        };
        
        storage.loyalty_profiles.insert(user_id, &profile);
        
        Ok(())
    }
    
    /// Award points to a user
    pub fn award_points(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        points: u32,
        _reason: String,
    ) -> Result<(), String> {
        let mut profile = storage.loyalty_profiles.get(user_id)
            .ok_or("Loyalty profile not found")?;
        
        profile.total_points += points;
        profile.points_earned_this_month += points;
        profile.points_earned_this_year += points;
        profile.last_activity = 0; // Will be set by caller
        
        // Check for tier upgrade
        let new_tier = Self::calculate_tier(profile.total_points);
        if new_tier != profile.current_tier {
            profile.current_tier = new_tier;
        }
        
        storage.loyalty_profiles.insert(user_id, &profile);
        
        Ok(())
    }
    
    /// Deduct points from a user
    pub fn deduct_points(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        points: u32,
        _reason: String,
    ) -> Result<(), String> {
        let mut profile = storage.loyalty_profiles.get(user_id)
            .ok_or("Loyalty profile not found")?;
        
        if profile.total_points < points {
            return Err("Insufficient points".to_string());
        }
        
        profile.total_points -= points;
        profile.last_activity = 0; // Will be set by caller
        
        // Check for tier downgrade
        let new_tier = Self::calculate_tier(profile.total_points);
        if new_tier != profile.current_tier {
            profile.current_tier = new_tier;
        }
        
        storage.loyalty_profiles.insert(user_id, &profile);
        
        Ok(())
    }
    
    /// Claim a reward using loyalty points
    pub fn claim_reward(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        reward_type: RewardType,
        reward_value: u128,
    ) -> Result<(), String> {
        let mut profile = storage.loyalty_profiles.get(user_id)
            .ok_or("Loyalty profile not found")?;
        
        let required_points = Self::get_reward_cost(reward_type.clone(), reward_value);
        
        if profile.total_points < required_points {
            return Err("Insufficient points for this reward".to_string());
        }
        
        // Deduct points
        profile.total_points -= required_points;
        profile.last_activity = 0; // Will be set by caller
        
        storage.loyalty_profiles.insert(user_id, &profile);
        
        // Record the reward claim
        Self::record_reward_claim(storage, user_id, reward_type, reward_value, required_points);
        
        Ok(())
    }
    
    /// Process referral bonus for both referrer and referee
    pub fn process_referral_bonus(
        storage: &mut SportsBrokerStorage,
        referrer_id: AccountId,
        referee_id: AccountId,
    ) -> Result<(), String> {
        // Award bonus to referrer
        let referrer_bonus = Self::get_referral_bonus_points(LoyaltyTier::Bronze); // Default tier for new users
        Self::award_points(storage, referrer_id, referrer_bonus, "Referral bonus".to_string())?;
        
        // Award bonus to referee
        let referee_bonus = 100; // Welcome bonus
        Self::award_points(storage, referee_id, referee_bonus, "Welcome bonus".to_string())?;
        
        // Update referral counts
        if let Some(mut referrer_profile) = storage.loyalty_profiles.get(referrer_id) {
            referrer_profile.referral_count += 1;
            referrer_profile.referral_points += referrer_bonus;
            storage.loyalty_profiles.insert(referrer_id, &referrer_profile);
        }
        
        // Create referral record
        let referral = Referral {
            referrer_id,
            referred_id: referee_id,
            referral_date: 0, // Will be set by caller
            referrer_points_earned: referrer_bonus,
            referred_bonus_applied: true,
            referral_code: "REF".to_string(),
        };
        
        storage.referrals.insert(referrer_id, &referral);
        
        Ok(())
    }
    
    /// Create a new promotion
    pub fn create_promotion(
        storage: &mut SportsBrokerStorage,
        name: String,
        description: String,
        points_multiplier: u32,
        start_time: u64,
        end_time: u64,
        applicable_events: Vec<u32>,
        applicable_tiers: Vec<LoyaltyTier>,
    ) -> Result<u32, String> {
        let promotion_id = storage.get_next_id("promotion");
        
        let promotion = Promotion {
            id: promotion_id,
            name,
            description,
            points_multiplier,
            start_time,
            end_time,
            applicable_events,
            applicable_tiers,
            active: true,
        };
        
        storage.promotions.insert(promotion_id, &promotion);
        
        Ok(promotion_id)
    }
    
    /// Redeem a promotion
    pub fn redeem_promotion(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        promotion_id: u32,
    ) -> Result<(), String> {
        let promotion = storage.promotions.get(promotion_id)
            .ok_or("Promotion not found")?;
        
        if !promotion.active {
            return Err("Promotion is not active".to_string());
        }
        
        let current_time = 0; // Will be set by caller
        if current_time < promotion.start_time || current_time > promotion.end_time {
            return Err("Promotion is not available at this time".to_string());
        }
        
        let profile = storage.loyalty_profiles.get(user_id)
            .ok_or("Loyalty profile not found")?;
        
        // Check if user's tier is applicable
        if !promotion.applicable_tiers.contains(&profile.current_tier) {
            return Err("Promotion not available for your loyalty tier".to_string());
        }
        
        // Award bonus points
        let bonus_points = 100 * promotion.points_multiplier; // Base 100 points
        Self::award_points(storage, user_id, bonus_points, "Promotion bonus".to_string())?;
        
        // Update user activity
        if let Some(mut profile) = storage.loyalty_profiles.get(user_id) {
            profile.last_activity = current_time;
            storage.loyalty_profiles.insert(user_id, &profile);
        }
        
        Ok(())
    }
    
    /// Get loyalty profile for a user
    pub fn get_loyalty_profile(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Option<LoyaltyProfile> {
        storage.loyalty_profiles.get(user_id)
    }
    
    /// Get available promotions for a user
    pub fn get_available_promotions(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Vec<Promotion> {
        let mut available_promotions = Vec::new();
        
        if let Some(profile) = storage.loyalty_profiles.get(user_id) {
            for promotion_id in 1..=1000 { // Arbitrary limit, should be tracked in storage
                if let Some(promotion) = storage.promotions.get(promotion_id) {
                    if promotion.active && promotion.applicable_tiers.contains(&profile.current_tier) {
                        let current_time = 0; // Will be set by caller
                        if current_time >= promotion.start_time && current_time <= promotion.end_time {
                            available_promotions.push(promotion);
                        }
                    }
                }
            }
        }
        
        available_promotions
    }
    
    /// Get available rewards for a user
    pub fn get_user_available_rewards(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Vec<RewardType> {
        let mut available_rewards = Vec::new();
        
        if let Some(profile) = storage.loyalty_profiles.get(user_id) {
            let tier_rewards = Self::get_tier_rewards(profile.current_tier);
            for reward in tier_rewards {
                let cost = Self::get_reward_cost(reward.clone(), 0);
                if profile.total_points >= cost {
                    available_rewards.push(reward);
                }
            }
        }
        
        available_rewards
    }
    
    // Helper methods
    fn calculate_tier(points: u32) -> LoyaltyTier {
        match points {
            0..=999 => LoyaltyTier::Bronze,
            1000..=4999 => LoyaltyTier::Silver,
            5000..=19999 => LoyaltyTier::Gold,
            20000..=49999 => LoyaltyTier::Platinum,
            50000..=99999 => LoyaltyTier::Diamond,
            _ => LoyaltyTier::Diamond,
        }
    }
    
    fn get_reward_cost(reward_type: RewardType, _reward_value: u128) -> u32 {
        match reward_type {
            RewardType::DiscountPercentage(_) => 500,
            RewardType::FreeTicket => 1000,
            RewardType::VIPAccess => 2000,
            RewardType::MerchandiseCredit(_) => 300,
            RewardType::EarlyAccess(_) => 400,
            RewardType::MeetAndGreet => 5000,
            RewardType::ParkingPass => 200,
            RewardType::FoodCredit(_) => 250,
            RewardType::SeasonPassDiscount(_) => 1500,
            RewardType::ExclusiveEvent => 3000,
        }
    }
    
    fn get_referral_bonus_points(tier: LoyaltyTier) -> u32 {
        match tier {
            LoyaltyTier::Bronze => 100,
            LoyaltyTier::Silver => 150,
            LoyaltyTier::Gold => 200,
            LoyaltyTier::Platinum => 300,
            LoyaltyTier::Diamond => 500,
        }
    }
    
    fn get_tier_rewards(tier: LoyaltyTier) -> Vec<RewardType> {
        match tier {
            LoyaltyTier::Bronze => vec![
                RewardType::DiscountPercentage(5),
                RewardType::MerchandiseCredit(1000),
            ],
            LoyaltyTier::Silver => vec![
                RewardType::DiscountPercentage(10),
                RewardType::MerchandiseCredit(2000),
                RewardType::EarlyAccess(3600), // 1 hour
                RewardType::ParkingPass,
            ],
            LoyaltyTier::Gold => vec![
                RewardType::DiscountPercentage(15),
                RewardType::MerchandiseCredit(5000),
                RewardType::EarlyAccess(7200), // 2 hours
                RewardType::FoodCredit(3000),
                RewardType::SeasonPassDiscount(10),
            ],
            LoyaltyTier::Platinum => vec![
                RewardType::DiscountPercentage(20),
                RewardType::MerchandiseCredit(10000),
                RewardType::EarlyAccess(10800), // 3 hours
                RewardType::FoodCredit(5000),
                RewardType::SeasonPassDiscount(15),
                RewardType::VIPAccess,
            ],
            LoyaltyTier::Diamond => vec![
                RewardType::DiscountPercentage(25),
                RewardType::MerchandiseCredit(20000),
                RewardType::EarlyAccess(14400), // 4 hours
                RewardType::FoodCredit(10000),
                RewardType::SeasonPassDiscount(20),
                RewardType::VIPAccess,
                RewardType::MeetAndGreet,
                RewardType::ExclusiveEvent,
            ],
        }
    }
    
    fn record_reward_claim(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        _reward_type: RewardType,
        _reward_value: u128,
        _points_spent: u32,
    ) {
        // For now, just update the user's last activity
        // In a full implementation, this would create a detailed record
        if let Some(mut profile) = storage.loyalty_profiles.get(user_id) {
            profile.last_activity = 0; // Will be set by caller
            storage.loyalty_profiles.insert(user_id, &profile);
        }
    }
    
    fn record_points_transaction(
        _storage: &mut SportsBrokerStorage,
        _user_id: AccountId,
        _points: u32,
        _reason: String,
    ) {
        // Placeholder implementation
    }
    
    fn record_promotion_redemption(
        _storage: &mut SportsBrokerStorage,
        _user_id: AccountId,
        _promotion_id: u32,
    ) {
        // Placeholder implementation
    }
}

/// Available reward structure
#[derive(Debug, Clone, PartialEq)]
pub struct AvailableReward {
    pub reward_type: RewardType,
    pub reward_value: u128,
    pub required_points: u32,
    pub source: String,
    pub source_id: Option<u64>,
}
