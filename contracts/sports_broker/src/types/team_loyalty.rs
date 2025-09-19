use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

/// Team loyalty profile for a specific user-team relationship
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyProfile {
    pub user_id: AccountId,
    pub team_id: u32,
    pub loyalty_points: u32,
    pub loyalty_tier: TeamLoyaltyTier,
    pub attendance_streak: u32,
    pub total_events_attended: u32,
    pub favorite_team_status: bool,
    pub staked_amount: u128,
    pub staking_start_date: u64,
    pub last_attendance: u64,
    pub team_specific_benefits: Vec<TeamBenefit>,
    pub created_at: u64,
    pub last_updated: u64,
}

/// Team-specific loyalty tiers with unique benefits
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamLoyaltyTier {
    Rookie,       // 0-99 points - Basic benefits
    Fan,          // 100-499 points - Enhanced benefits
    SuperFan,     // 500-1999 points - Premium benefits
    UltraFan,     // 2000-9999 points - VIP benefits
    LegendaryFan, // 10000+ points - Exclusive benefits
}

/// Team-specific benefits and perks
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamBenefit {
    PrioritySeating,      // Priority access to best seats
    MeetAndGreetAccess,   // Access to player meet and greets
    ExclusiveMerchandise, // Access to team-exclusive merchandise
    EarlyTicketAccess,    // Early access to ticket sales
    ParkingPass,          // Free or discounted parking
    ConcessionDiscounts,  // Discounts on food and drinks
    BackstageAccess,      // Limited backstage access
    TeamPracticeAccess,   // Access to team practices
    ChampionshipRing,     // Special championship rewards
    AlumniAssociation,    // Access to alumni events
}

/// Team staking information for loyalty rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamStaking {
    pub user_id: AccountId,
    pub team_id: u32,
    pub staked_amount: u128,
    pub staking_start_date: u64,
    pub staking_end_date: Option<u64>,
    pub reward_multiplier: u32, // Points multiplier based on staking
    pub is_active: bool,
    pub total_rewards_earned: u128,
}

/// Team attendance tracking for streak rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamAttendance {
    pub user_id: AccountId,
    pub team_id: u32,
    pub event_id: u32,
    pub attendance_date: u64,
    pub points_earned: u32,
    pub streak_bonus: u32,
    pub total_streak: u32,
}

/// Team performance-based loyalty rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamPerformanceReward {
    pub id: u32,
    pub team_id: u32,
    pub reward_type: TeamPerformanceRewardType,
    pub points_multiplier: u32,
    pub start_date: u64,
    pub end_date: u64,
    pub is_active: bool,
}

/// Types of team performance rewards
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamPerformanceRewardType {
    PlayoffAppearance,     // Team made playoffs
    ChampionshipWin,       // Team won championship
    RegularSeasonChampion, // Team won regular season
    DivisionChampion,      // Team won division
    ConferenceChampion,    // Team won conference
    RecordBreakingSeason,  // Team broke records
    ComebackVictory,       // Team had comeback wins
    RivalryGameVictory,    // Team won rivalry games
}

/// Team loyalty analytics and statistics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyAnalytics {
    pub team_id: u32,
    pub total_fans: u32,
    pub total_loyalty_points: u32,
    pub average_loyalty_tier: TeamLoyaltyTier,
    pub total_staked_amount: u128,
    pub total_attendance: u32,
    pub longest_attendance_streak: u32,
    pub most_loyal_fan: Option<AccountId>,
    pub last_updated: u64,
}

/// Team loyalty challenge for engagement
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamLoyaltyChallenge {
    pub id: u32,
    pub team_id: u32,
    pub name: String,
    pub description: String,
    pub challenge_type: TeamChallengeType,
    pub points_reward: u32,
    pub start_date: u64,
    pub end_date: u64,
    pub is_active: bool,
    pub participants: Vec<AccountId>,
    pub completion_criteria: String,
}

/// Types of team loyalty challenges
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TeamChallengeType {
    AttendanceStreak,       // Attend X consecutive games
    SocialMediaEngagement,  // Engage with team on social media
    MerchandisePurchase,    // Purchase team merchandise
    ReferralChallenge,      // Refer new fans to the team
    GamePrediction,         // Correctly predict game outcomes
    FanArtSubmission,       // Submit fan art or content
    CommunityParticipation, // Participate in community events
    SeasonPassHolder,       // Hold season pass for full season
}

impl Default for TeamLoyaltyProfile {
    fn default() -> Self {
        Self {
            user_id: AccountId::from([0u8; 32]),
            team_id: 0,
            loyalty_points: 0,
            loyalty_tier: TeamLoyaltyTier::Rookie,
            attendance_streak: 0,
            total_events_attended: 0,
            favorite_team_status: false,
            staked_amount: 0,
            staking_start_date: 0,
            last_attendance: 0,
            team_specific_benefits: Vec::new(),
            created_at: 0,
            last_updated: 0,
        }
    }
}

impl Default for TeamLoyaltyTier {
    fn default() -> Self {
        Self::Rookie
    }
}

impl Default for TeamStaking {
    fn default() -> Self {
        Self {
            user_id: AccountId::from([0u8; 32]),
            team_id: 0,
            staked_amount: 0,
            staking_start_date: 0,
            staking_end_date: None,
            reward_multiplier: 10000, // 1.0x
            is_active: false,
            total_rewards_earned: 0,
        }
    }
}

impl Default for TeamLoyaltyAnalytics {
    fn default() -> Self {
        Self {
            team_id: 0,
            total_fans: 0,
            total_loyalty_points: 0,
            average_loyalty_tier: TeamLoyaltyTier::Rookie,
            total_staked_amount: 0,
            total_attendance: 0,
            longest_attendance_streak: 0,
            most_loyal_fan: None,
            last_updated: 0,
        }
    }
}
