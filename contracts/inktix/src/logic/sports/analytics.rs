use crate::storage::contract_storage::InkTixStorage;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::vec;
use crate::types::*;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
pub struct Analytics;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl Analytics {
    pub fn generate_analytics_report(storage: &mut InkTixStorage, report_type: ReportType, start_date: u64, end_date: u64) -> Result<u32, String> {
        let report_id = storage.get_next_report_id();
        let platform_stats = Self::update_platform_stats(storage);
        let mut event_analytics = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.date >= start_date && event.date <= end_date {
                    if let Some(analytics) = storage.event_analytics.get(event_id) { event_analytics.push(analytics); }
                }
            }
        }
        let mut team_analytics = Vec::new();
        for team_id in 1..=storage.total_teams {
            if let Some(analytics) = storage.team_analytics.get(team_id) { team_analytics.push(analytics); }
        }
        let report = AnalyticsReport {
            report_id, report_type, time_period: TimePeriod::AllTime, platform_stats,
            event_analytics, team_analytics, top_performing_events: Vec::new(),
            top_performing_teams: Vec::new(), revenue_trends: Vec::new(),
            attendance_trends: Vec::new(),
            generated_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
        };
        storage.analytics_reports.insert(report_id, &report);
        Ok(report_id)
    }

    pub fn update_platform_stats(storage: &mut InkTixStorage) -> PlatformStats {
        let mut stats = PlatformStats {
            total_revenue: 0, total_tickets_sold: storage.total_tickets as u32,
            total_events: storage.total_events, total_users: 0, average_ticket_price: 0,
            total_season_passes: 0, currency_distribution: vec![],
            sport_type_distribution: vec![], last_updated: 0,
        };
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                stats.total_revenue = stats.total_revenue.saturating_add(event.revenue_generated);
            }
        }
        if stats.total_tickets_sold > 0 { stats.average_ticket_price = stats.total_revenue / stats.total_tickets_sold as u128; }
        storage.platform_stats = stats.clone();
        stats
    }
}

/// User engagement metrics
#[derive(Debug, Clone, PartialEq)]
pub struct UserEngagementMetrics {
    pub total_users: u32, pub active_users: u32, pub engagement_rate: u32, pub average_tickets_per_user: u32,
}
