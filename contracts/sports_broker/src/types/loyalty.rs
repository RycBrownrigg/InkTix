use ink::primitives::AccountId;
use ink::prelude::vec::Vec;
use ink::prelude::string::String;

/// Loyalty tier levels with increasing benefits
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum LoyaltyTier {
    Bronze,     // 0-999 points
    Silver,     // 1000-4999 points
    Gold,       // 5000-19999 points
    Platinum,   // 20000-99999 points
    Diamond,    // 100000+ points
}

/// User loyalty profile with points and tier
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct LoyaltyProfile {
    pub user_id: AccountId,
    pub total_points: u32,
    pub current_tier: LoyaltyTier,
    pub points_earned_this_month: u32,
    pub points_earned_this_year: u32,
    pub total_tickets_purchased: u32,
    pub total_spent: u128,
    pub join_date: u64,
    pub last_activity: u64,
    pub streak_days: u32,
    pub referral_count: u32,
    pub referral_points: u32,
}

/// Reward types available in the system
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum RewardType {
    DiscountPercentage(u8),      // 5%, 10%, 15%, 20%
    FreeTicket,                  // Free ticket to any event
    VIPAccess,                   // VIP seating or access
    MerchandiseCredit(u128),     // Credit for team merchandise
    EarlyAccess(u64),            // Early access to sales (timestamp)
    MeetAndGreet,                // Meet team/artist
    ParkingPass,                 // Free parking
    FoodCredit(u128),            // Credit for concessions
    SeasonPassDiscount(u8),      // Discount on season passes
    ExclusiveEvent,              // Access to exclusive events
}

/// Reward redemption record
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct RewardRedemption {
    pub id: u64,
    pub user_id: AccountId,
    pub reward_type: RewardType,
    pub points_cost: u32,
    pub redeemed_at: u64,
    pub expires_at: u64,
    pub is_used: bool,
    pub event_id: Option<u32>, // For event-specific rewards
}

/// Points earning rules and multipliers
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PointsRule {
    pub id: u32,
    pub name: String,
    pub base_points: u32,
    pub multiplier: u32, // Changed from u8 to u32
    pub tier_bonus: bool, // Whether higher tiers get bonus points
    pub weekend_bonus: bool, // Whether weekend purchases get bonus
    pub new_user_bonus: bool, // Whether new users get bonus
    pub active: bool,
}

/// Special promotion with bonus points
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Promotion {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub points_multiplier: u32, // Changed from u8 to u32
    pub start_time: u64,        // Changed from start_date
    pub end_time: u64,          // Changed from end_date
    pub applicable_events: Vec<u32>, // Empty = all events
    pub applicable_tiers: Vec<LoyaltyTier>, // Empty = all tiers
    pub active: bool,
}

/// Referral program tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Referral {
    pub referrer_id: AccountId,
    pub referred_id: AccountId,
    pub referral_date: u64,
    pub referrer_points_earned: u32,
    pub referred_bonus_applied: bool,
    pub referral_code: String,
}