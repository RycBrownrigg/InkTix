use ink::primitives::AccountId;
use crate::types::{LoyaltyTier, CurrencyId};

/// Types of season passes available
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeasonPassType {
    FullSeason,
    HalfSeason,
    QuarterSeason,
    PlayoffOnly,
    Premium,
    VIP,
    Student,
    Senior,
    Military,
    Family,
}

/// Season pass for subscription management with multi-currency support
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeasonPass {
    pub id: u64,
    pub owner: AccountId,
    pub season_id: u32,
    pub team_id: u32,
    pub pass_type: SeasonPassType,
    pub purchase_price: u128,
    pub purchase_currency: CurrencyId,
    pub purchase_date: u64,
    pub games_included: u32,
    pub games_attended: u32,
    pub transferable: bool,
    pub includes_playoffs: bool,
    pub priority_level: u8,
    pub loyalty_tier_at_purchase: LoyaltyTier,
    pub staking_rewards_enabled: bool,
    pub staked_amount: u128,
    pub valid_until: u64,
    pub dot_equivalent_paid: u128,
}