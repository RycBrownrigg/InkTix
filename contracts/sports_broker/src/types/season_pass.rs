
use ink::primitives::AccountId;

/// Season pass types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeasonPassType {
    FullSeason,      // All regular season games
    HalfSeason,      // Half of regular season games
    PlayoffOnly,     // Playoff games only
    Premium,         // Full season + playoffs + special events
    Corporate,       // Corporate season passes with special benefits
    Alumni,          // Alumni association passes
}

/// Season pass status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeasonPassStatus {
    Active,
    Suspended,
    Expired,
    Cancelled,
    PendingActivation,
}

/// Season pass benefits and perks
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPassBenefits {
    pub priority_access: bool,           // Priority access to high-demand games
    pub exclusive_events: bool,          // Access to exclusive events
    pub merchandise_discount: u8,        // Discount percentage on merchandise
    pub parking_included: bool,          // Parking pass included
    pub concession_credits: u128,        // Concession credits per game
    pub vip_upgrades: bool,             // VIP upgrade opportunities
    pub meet_greet_access: bool,         // Meet and greet access
    pub backstage_tours: bool,          // Backstage tour access
    pub loyalty_multiplier: u32,        // Loyalty points multiplier (10000 = 1.0x)
    pub staking_rewards: bool,          // Eligible for DeFi staking rewards
}

/// Season pass structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPass {
    pub id: u32,
    pub owner: AccountId,
    pub team_id: u32,
    pub season_id: u32,
    pub pass_type: SeasonPassType,
    pub status: SeasonPassStatus,
    pub purchase_date: u64,
    pub activation_date: u64,
    pub expiry_date: u64,
    pub total_games: u32,
    pub games_attended: u32,
    pub games_remaining: u32,
    pub purchase_price: u128,
    pub purchase_currency: super::currency::CurrencyId,
    pub benefits: SeasonPassBenefits,
    pub staking_amount: u128,           // Amount staked for DeFi rewards
    pub staking_rewards_earned: u128,   // Total rewards earned from staking
    pub last_staking_update: u64,       // Last time staking rewards were calculated
    pub transferable: bool,             // Whether the pass can be transferred
    pub transfer_cooldown_until: u64,   // Cooldown period for transfers
}

/// Season pass package configuration
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPassPackage {
    pub id: u32,
    pub team_id: u32,
    pub season_id: u32,
    pub package_name: String,
    pub pass_type: SeasonPassType,
    pub total_games: u32,
    pub base_price: u128,
    pub currency: super::currency::CurrencyId,
    pub max_quantity: u32,
    pub sold_quantity: u32,
    pub benefits: SeasonPassBenefits,
    pub staking_required: bool,         // Whether staking is required
    pub min_staking_amount: u128,       // Minimum staking amount
    pub staking_reward_rate: u32,       // Annual reward rate (basis points, 500 = 5%)
    pub active: bool,
    pub sale_start_date: u64,
    pub sale_end_date: u64,
}

/// Season pass usage tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPassUsage {
    pub pass_id: u32,
    pub event_id: u32,
    pub usage_date: u64,
    pub entry_time: u64,
    pub exit_time: Option<u64>,
    pub benefits_used: Vec<String>,     // List of benefits used at this event
    pub loyalty_points_earned: u32,
    pub staking_rewards_earned: u128,
}

/// Season pass renewal options
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct RenewalOption {
    pub current_pass_id: u32,
    pub renewal_package_id: u32,
    pub discount_percentage: u8,
    pub early_bird_discount: u8,
    pub early_bird_deadline: u64,
    pub loyalty_tier_discount: u8,      // Additional discount based on loyalty tier
    pub staking_bonus: u32,             // Additional staking rewards for renewal
}

/// Season pass analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPassAnalytics {
    pub total_passes_sold: u32,
    pub total_revenue: u128,
    pub average_attendance_rate: u32,   // Percentage of games attended
    pub total_staking_amount: u128,
    pub total_staking_rewards: u128,
    pub most_popular_package: u32,      // Package ID with highest sales
    pub renewal_rate: u32,              // Percentage of passes renewed
    pub customer_satisfaction_score: u32, // 1-100 score
    pub last_updated: u64,
}

impl Default for SeasonPassBenefits {
    fn default() -> Self {
        Self {
            priority_access: false,
            exclusive_events: false,
            merchandise_discount: 0,
            parking_included: false,
            concession_credits: 0,
            vip_upgrades: false,
            meet_greet_access: false,
            backstage_tours: false,
            loyalty_multiplier: 10000, // 1.0x
            staking_rewards: false,
        }
    }
}

impl Default for SeasonPass {
    fn default() -> Self {
        Self {
            id: 0,
            owner: AccountId::from([0u8; 32]),
            team_id: 0,
            season_id: 0,
            pass_type: SeasonPassType::FullSeason,
            status: SeasonPassStatus::PendingActivation,
            purchase_date: 0,
            activation_date: 0,
            expiry_date: 0,
            total_games: 0,
            games_attended: 0,
            games_remaining: 0,
            purchase_price: 0,
            purchase_currency: super::currency::CurrencyId::DOT,
            benefits: SeasonPassBenefits::default(),
            staking_amount: 0,
            staking_rewards_earned: 0,
            last_staking_update: 0,
            transferable: true,
            transfer_cooldown_until: 0,
        }
    }
}

impl Default for SeasonPassPackage {
    fn default() -> Self {
        Self {
            id: 0,
            team_id: 0,
            season_id: 0,
            package_name: String::new(),
            pass_type: SeasonPassType::FullSeason,
            total_games: 0,
            base_price: 0,
            currency: super::currency::CurrencyId::DOT,
            max_quantity: 0,
            sold_quantity: 0,
            benefits: SeasonPassBenefits::default(),
            staking_required: false,
            min_staking_amount: 0,
            staking_reward_rate: 0,
            active: false,
            sale_start_date: 0,
            sale_end_date: 0,
        }
    }
}