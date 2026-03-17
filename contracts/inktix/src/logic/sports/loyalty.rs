//! Global loyalty profile and points management.
//!
//! Creates loyalty profiles, awards points based on activity, and
//! automatically promotes users through tier levels based on accumulated points.
//!
//! # Functions
//! - `create_loyalty_profile` -- initializes a loyalty profile for a user
//! - `award_points` -- grants loyalty points and recalculates the user's tier

use crate::storage::*;
use crate::types::*;
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;
use ink::prelude::vec;

/// Loyalty and rewards system functionality
pub struct Loyalty;

#[allow(clippy::arithmetic_side_effects)]
impl Loyalty {
    /// Create a new loyalty profile for a user
    pub fn create_loyalty_profile(storage: &mut InkTixStorage, user: AccountId) -> Result<(), String> {
        if storage.loyalty_profiles.get(user).is_some() { return Err("Loyalty profile already exists".to_string()); }
        let profile = LoyaltyProfile {
            user_id: user, total_points: 0, current_tier: LoyaltyTier::Bronze,
            points_earned_this_month: 0, points_earned_this_year: 0,
            total_tickets_purchased: 0, total_spent: 0,
            join_date: ink::env::block_timestamp::<DefaultEnvironment>(),
            last_activity: ink::env::block_timestamp::<DefaultEnvironment>(),
            streak_days: 0, referral_count: 0, referral_points: 0, fantasy_sports_points: 0,
        };
        storage.loyalty_profiles.insert(user, &profile);
        Ok(())
    }

    /// Award loyalty points to a user and recalculate their tier
    pub fn award_points(storage: &mut InkTixStorage, user: AccountId, points: u32, _reason: String) -> Result<(), String> {
        let mut profile = storage.loyalty_profiles.get(user).ok_or("Loyalty profile not found")?;
        profile.total_points += points;
        profile.points_earned_this_month += points;
        profile.points_earned_this_year += points;
        profile.last_activity = ink::env::block_timestamp::<DefaultEnvironment>();
        let new_tier = Self::calculate_tier(profile.total_points);
        if new_tier != profile.current_tier { profile.current_tier = new_tier; }
        storage.loyalty_profiles.insert(user, &profile);
        Ok(())
    }

    fn calculate_tier(total_points: u32) -> LoyaltyTier {
        match total_points {
            0..=999 => LoyaltyTier::Bronze, 1000..=4999 => LoyaltyTier::Silver,
            5000..=19999 => LoyaltyTier::Gold, 20000..=99999 => LoyaltyTier::Platinum,
            _ => LoyaltyTier::Diamond,
        }
    }
}
