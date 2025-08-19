use ink::prelude::vec::Vec;
use crate::types::{SportType, CurrencyId};

/// Platform statistics for analytics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PlatformStats {
    pub total_users: u32,
    pub total_tickets_sold: u64,
    pub total_season_passes_sold: u64,
    pub total_events_created: u32,
    pub average_ticket_price: u128,
    pub most_popular_sport: SportType,
    pub most_popular_team_id: u32,
    pub most_popular_venue_id: u32,
    pub last_updated: u64,
}

/// Analytics report structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct AnalyticsReport {
    pub total_revenue: u128,
    pub revenue_by_currency: Vec<(CurrencyId, u128)>,
    pub top_teams_by_revenue: Vec<(u32, u128)>,
    pub top_venues_by_revenue: Vec<(u32, u128)>,
    pub average_ticket_price: u128,
    pub total_tickets_sold: u64,
    pub total_season_passes_sold: u64,
    pub user_count: u32,
    pub most_popular_sport: SportType,
    pub report_generated_at: u64,
}