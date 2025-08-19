

/// Team performance for dynamic pricing
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamPerformance {
    pub team_id: u32,
    pub season_id: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_percentage: u32,
    pub streak: i32,
    pub playoff_probability: u32,
    pub last_updated: u64,
    pub performance_rank: u32,
    pub home_record_wins: u32,
    pub home_record_losses: u32,
    pub points_scored_avg: u32,
    pub points_allowed_avg: u32,
}

/// Pricing multiplier based on various factors
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PricingMultiplier {
    pub team_id: u32,
    pub base_multiplier: u32,
    pub performance_multiplier: u32,
    pub playoff_multiplier: u32,
    pub streak_multiplier: u32,
    pub rivalry_multiplier: u32,
    pub demand_multiplier: u32,
    pub final_multiplier: u32,
    pub last_updated: u64,
}