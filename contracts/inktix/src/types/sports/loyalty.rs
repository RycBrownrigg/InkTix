use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

/// Loyalty tier levels with increasing benefits
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum LoyaltyTier {
    Bronze, Silver, Gold, Platinum, Diamond,
}

/// User loyalty profile with points and tier
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct LoyaltyProfile {
    pub user_id: AccountId, pub total_points: u32, pub current_tier: LoyaltyTier,
    pub points_earned_this_month: u32, pub points_earned_this_year: u32,
    pub total_tickets_purchased: u32, pub total_spent: u128, pub join_date: u64,
    pub last_activity: u64, pub streak_days: u32, pub referral_count: u32,
    pub referral_points: u32, pub fantasy_sports_points: u32,
}

/// Reward types available in the system
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum RewardType {
    DiscountPercentage(u8), FreeTicket, VIPAccess, MerchandiseCredit(u128),
    EarlyAccess(u64), MeetAndGreet, ParkingPass, FoodCredit(u128),
    SeasonPassDiscount(u8), ExclusiveEvent,
}

/// Reward redemption record
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct RewardRedemption {
    pub id: u64, pub user_id: AccountId, pub reward_type: RewardType, pub points_cost: u32,
    pub redeemed_at: u64, pub expires_at: u64, pub is_used: bool, pub event_id: Option<u32>,
}

/// Points earning rules and multipliers
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PointsRule {
    pub id: u32, pub name: String, pub base_points: u32, pub multiplier: u32,
    pub tier_bonus: bool, pub weekend_bonus: bool, pub new_user_bonus: bool, pub active: bool,
}

/// Special promotion with bonus points
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Promotion {
    pub id: u32, pub name: String, pub description: String, pub points_multiplier: u32,
    pub start_time: u64, pub end_time: u64, pub applicable_events: Vec<u32>,
    pub applicable_tiers: Vec<LoyaltyTier>, pub active: bool,
}

/// Referral program tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Referral {
    pub referrer_id: AccountId, pub referred_id: AccountId, pub referral_date: u64,
    pub referrer_points_earned: u32, pub referred_bonus_applied: bool, pub referral_code: String,
}

/// Team loyalty profile for a specific user-team relationship
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyProfile {
    pub user_id: AccountId, pub team_id: u32, pub loyalty_points: u32,
    pub loyalty_tier: TeamLoyaltyTier, pub attendance_streak: u32, pub total_events_attended: u32,
    pub favorite_team_status: bool, pub staked_amount: u128, pub staking_start_date: u64,
    pub last_attendance: u64, pub team_specific_benefits: Vec<TeamBenefit>,
    pub created_at: u64, pub last_updated: u64,
}

/// Team-specific loyalty tiers
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamLoyaltyTier {
    Rookie, Fan, SuperFan, UltraFan, LegendaryFan,
}

/// Team-specific benefits and perks
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamBenefit {
    PrioritySeating, MeetAndGreetAccess, ExclusiveMerchandise, EarlyTicketAccess,
    ParkingPass, ConcessionDiscounts, BackstageAccess, TeamPracticeAccess,
    ChampionshipRing, AlumniAssociation,
}

/// Team staking information for loyalty rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamStaking {
    pub user_id: AccountId, pub team_id: u32, pub staked_amount: u128,
    pub staking_start_date: u64, pub staking_end_date: Option<u64>,
    pub reward_multiplier: u32, pub is_active: bool, pub total_rewards_earned: u128,
}

/// Team attendance tracking for streak rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamAttendance {
    pub user_id: AccountId, pub team_id: u32, pub event_id: u32,
    pub attendance_date: u64, pub points_earned: u32, pub streak_bonus: u32, pub total_streak: u32,
}

/// Team performance-based loyalty rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamPerformanceReward {
    pub id: u32, pub team_id: u32, pub reward_type: TeamPerformanceRewardType,
    pub points_multiplier: u32, pub start_date: u64, pub end_date: u64, pub is_active: bool,
}

/// Types of team performance rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamPerformanceRewardType {
    PlayoffAppearance, ChampionshipWin, RegularSeasonChampion, DivisionChampion,
    ConferenceChampion, RecordBreakingSeason, ComebackVictory, RivalryGameVictory,
}

/// Team loyalty analytics and statistics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyAnalytics {
    pub team_id: u32, pub total_fans: u32, pub total_loyalty_points: u32,
    pub average_loyalty_tier: TeamLoyaltyTier, pub total_staked_amount: u128,
    pub total_attendance: u32, pub longest_attendance_streak: u32,
    pub most_loyal_fan: Option<AccountId>, pub last_updated: u64,
}

/// Team loyalty challenge for engagement
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyChallenge {
    pub id: u32, pub team_id: u32, pub name: String, pub description: String,
    pub challenge_type: TeamChallengeType, pub points_reward: u32,
    pub start_date: u64, pub end_date: u64, pub is_active: bool,
    pub participants: Vec<AccountId>, pub completion_criteria: String,
}

/// Types of team loyalty challenges
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamChallengeType {
    AttendanceStreak, SocialMediaEngagement, MerchandisePurchase, ReferralChallenge,
    GamePrediction, FanArtSubmission, CommunityParticipation, SeasonPassHolder,
}

impl Default for TeamLoyaltyProfile {
    fn default() -> Self {
        Self { user_id: AccountId::from([0u8; 32]), team_id: 0, loyalty_points: 0,
               loyalty_tier: TeamLoyaltyTier::Rookie, attendance_streak: 0,
               total_events_attended: 0, favorite_team_status: false, staked_amount: 0,
               staking_start_date: 0, last_attendance: 0, team_specific_benefits: Vec::new(),
               created_at: 0, last_updated: 0 }
    }
}

impl Default for TeamLoyaltyTier { fn default() -> Self { Self::Rookie } }

impl Default for TeamStaking {
    fn default() -> Self {
        Self { user_id: AccountId::from([0u8; 32]), team_id: 0, staked_amount: 0,
               staking_start_date: 0, staking_end_date: None, reward_multiplier: 10000,
               is_active: false, total_rewards_earned: 0 }
    }
}

impl Default for TeamLoyaltyAnalytics {
    fn default() -> Self {
        Self { team_id: 0, total_fans: 0, total_loyalty_points: 0,
               average_loyalty_tier: TeamLoyaltyTier::Rookie, total_staked_amount: 0,
               total_attendance: 0, longest_attendance_streak: 0, most_loyal_fan: None, last_updated: 0 }
    }
}
