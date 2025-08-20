use ink::primitives::AccountId;
use ink::prelude::vec::Vec;
use crate::types::{CurrencyId, SportType};

/// Platform-wide statistics and metrics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PlatformStats {
    pub total_revenue: u128,
    pub total_tickets_sold: u32,
    pub total_events: u32,
    pub total_users: u32,
    pub average_ticket_price: u128,
    pub total_season_passes: u32,
    pub currency_distribution: Vec<(CurrencyId, u128)>,
    pub sport_type_distribution: Vec<(SportType, u32)>,
    pub last_updated: u64,
}

/// Event-specific analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct EventAnalytics {
    pub event_id: u32,
    pub tickets_sold: u32,
    pub revenue_generated: u128,
    pub average_price: u128,
    pub sellout_percentage: u8, // 0-100
    pub currency_breakdown: Vec<(CurrencyId, u128)>,
    pub attendance_forecast: u32,
    pub revenue_forecast: u128,
    pub last_updated: u64,
}

/// Team performance analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TeamAnalytics {
    pub team_id: u32,
    pub total_events: u32,
    pub total_revenue: u128,
    pub average_attendance: u32,
    pub ticket_price_trend: Vec<(u64, u128)>, // timestamp, price
    pub performance_correlation: u8, // 0-100, how much performance affects pricing
    pub fan_engagement_score: u8, // 0-100
    pub last_updated: u64,
}

/// User behavior analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct UserAnalytics {
    pub user_id: AccountId,
    pub total_tickets_purchased: u32,
    pub total_spent: u128,
    pub favorite_sport: SportType,
    pub average_ticket_price: u128,
    pub loyalty_tier: u8,
    pub last_purchase_date: u64,
    pub purchase_frequency: u8, // days between purchases
}

/// Comprehensive analytics report
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct AnalyticsReport {
    pub report_id: u32,
    pub report_type: ReportType,
    pub time_period: TimePeriod,
    pub platform_stats: PlatformStats,
    pub event_analytics: Vec<EventAnalytics>,
    pub team_analytics: Vec<TeamAnalytics>,
    pub top_performing_events: Vec<u32>,
    pub top_performing_teams: Vec<u32>,
    pub revenue_trends: Vec<(u64, u128)>,
    pub attendance_trends: Vec<(u64, u32)>,
    pub generated_at: u64,
}

/// Types of analytics reports
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum ReportType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom(u64, u64), // start_timestamp, end_timestamp
}

/// Time periods for analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TimePeriod {
    Last24Hours,
    Last7Days,
    Last30Days,
    Last90Days,
    Last365Days,
    AllTime,
}