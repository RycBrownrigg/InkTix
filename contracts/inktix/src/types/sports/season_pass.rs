//! Season pass type definitions.
//!
//! Covers season pass types, statuses, benefits, packages, usage tracking,
//! renewal options, and analytics for recurring ticket holders.

use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;

/// Season pass types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeasonPassType {
    FullSeason,
    HalfSeason,
    PlayoffOnly,
    Premium,
    Corporate,
    Alumni,
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
    pub priority_access: bool,
    pub exclusive_events: bool,
    pub merchandise_discount: u8,
    pub parking_included: bool,
    pub concession_credits: u128,
    pub vip_upgrades: bool,
    pub meet_greet_access: bool,
    pub backstage_tours: bool,
    pub loyalty_multiplier: u32,
    pub staking_rewards: bool,
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
    pub purchase_currency: crate::types::core::currency::CurrencyId,
    pub benefits: SeasonPassBenefits,
    pub staking_amount: u128,
    pub staking_rewards_earned: u128,
    pub last_staking_update: u64,
    pub transferable: bool,
    pub transfer_cooldown_until: u64,
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
    pub currency: crate::types::core::currency::CurrencyId,
    pub max_quantity: u32,
    pub sold_quantity: u32,
    pub benefits: SeasonPassBenefits,
    pub staking_required: bool,
    pub min_staking_amount: u128,
    pub staking_reward_rate: u32,
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
    pub benefits_used: Vec<String>,
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
    pub loyalty_tier_discount: u8,
    pub staking_bonus: u32,
}

/// Season pass analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPassAnalytics {
    pub total_passes_sold: u32,
    pub total_revenue: u128,
    pub average_attendance_rate: u32,
    pub total_staking_amount: u128,
    pub total_staking_rewards: u128,
    pub most_popular_package: u32,
    pub renewal_rate: u32,
    pub customer_satisfaction_score: u32,
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
            loyalty_multiplier: 10000,
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
            purchase_currency: crate::types::core::currency::CurrencyId::DOT,
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
            currency: crate::types::core::currency::CurrencyId::DOT,
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
