use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;

use crate::types::*;

/// Analytics logic
pub struct Analytics;

impl Analytics {
    /// Generate a comprehensive platform report
    pub fn generate_platform_report(
        storage: &mut SportsBrokerStorage,
        report_type: ReportType,
        time_period: TimePeriod,
        start_date: u64,
        end_date: u64,
    ) -> Result<AnalyticsReport, String> {
        let report_id = storage.get_next_report_id();
        
        // Collect platform statistics
        let platform_stats = Self::update_platform_stats(storage);
        
        // Collect event analytics for the time period
        let mut event_analytics = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.date >= start_date && event.date <= end_date {
                    if let Some(analytics) = storage.event_analytics.get(event_id) {
                        event_analytics.push(analytics);
                    }
                }
            }
        }
        
        // Collect team analytics
        let mut team_analytics = Vec::new();
        for team_id in 1..=storage.total_teams {
            if let Some(analytics) = storage.team_analytics.get(team_id) {
                team_analytics.push(analytics);
            }
        }
        
        // Get top performing events and teams
        let top_events = Self::get_top_performing_events(storage);
        let top_teams = Self::get_top_performing_teams(storage);
        
        // Calculate revenue and attendance trends
        let revenue_trends = Self::calculate_revenue_trends(storage, start_date, end_date);
        let attendance_trends = Self::calculate_attendance_trends(storage, start_date, end_date);
        
        let report = AnalyticsReport {
            report_id,
            report_type,
            time_period,
            platform_stats,
            event_analytics,
            team_analytics,
            top_performing_events: top_events,
            top_performing_teams: top_teams,
            revenue_trends,
            attendance_trends,
            generated_at: 0, // Will be set by caller
        };
        
        // Store the report
        storage.analytics_reports.insert(report_id, &report);
        
        Ok(report)
    }
    
    /// Generate team-specific analytics
    pub fn generate_team_analytics(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        _season_id: u32,
    ) -> Result<TeamAnalytics, String> {
        let team_events = Self::get_team_events(storage, team_id);
        
        let analytics = TeamAnalytics {
            team_id,
            total_events: team_events.len() as u32,
            total_revenue: Self::calculate_team_revenue(&team_events),
            average_attendance: Self::calculate_average_attendance(&team_events),
            ticket_price_trend: Self::calculate_ticket_price_trend(&team_events),
            performance_correlation: Self::calculate_performance_correlation(team_id, storage),
            fan_engagement_score: Self::calculate_fan_engagement_score(team_id, storage),
            last_updated: 0, // Will be set by caller
        };
        
        // Store the analytics
        storage.team_analytics.insert(team_id, &analytics);
        
        Ok(analytics)
    }
    
    /// Generate user-specific analytics
    pub fn generate_user_analytics(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
    ) -> Result<UserAnalytics, String> {
        let user_tickets = Self::get_user_tickets(storage, user_id);
        let total_spent = Self::calculate_total_spent(&user_tickets);
        let favorite_sport = Self::get_user_favorite_sport(storage, user_id);
        let loyalty_tier = Self::get_user_loyalty_tier(storage, user_id);
        
        let analytics = UserAnalytics {
            user_id,
            total_tickets_purchased: user_tickets.len() as u32,
            total_spent,
            favorite_sport: favorite_sport.unwrap_or(SportType::Football),
            average_ticket_price: if user_tickets.len() > 0 {
                total_spent / user_tickets.len() as u128
            } else {
                0
            },
            loyalty_tier: loyalty_tier.unwrap_or(0),
            last_purchase_date: Self::get_last_purchase_date(&user_tickets).unwrap_or(0),
            purchase_frequency: Self::calculate_purchase_frequency(&user_tickets),
        };
        
        // Store the analytics
        storage.user_analytics.insert(user_id, &analytics);
        
        Ok(analytics)
    }
    
    /// Update platform statistics
    pub fn update_platform_stats(storage: &mut SportsBrokerStorage) -> PlatformStats {
        let mut stats = PlatformStats {
            total_revenue: 0,
            total_tickets_sold: storage.total_tickets as u32,
            total_events: storage.total_events,
            total_users: 0, // Calculate from user_tickets mapping
            average_ticket_price: 0,
            total_season_passes: 0,
            currency_distribution: vec![],
            sport_type_distribution: vec![],
            last_updated: 0,
        };
        
        // Calculate total revenue from all events
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                stats.total_revenue = stats.total_revenue.saturating_add(event.revenue_generated);
            }
        }
        
        // Calculate average ticket price
        if stats.total_tickets_sold > 0 {
            stats.average_ticket_price = stats.total_revenue / stats.total_tickets_sold as u128;
        }
        
        // Calculate currency distribution
        for currency in &storage.supported_currencies {
            if let Some(revenue) = storage.currency_revenue.get(*currency) {
                stats.currency_distribution.push((*currency, revenue));
            }
        }
        
        // Calculate sport type distribution using Vec instead of HashMap
        let mut sport_counts = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                let mut found = false;
                for (sport_type, count) in &mut sport_counts {
                    if *sport_type == event.sport_type {
                        *count += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    sport_counts.push((event.sport_type, 1));
                }
            }
        }
        stats.sport_type_distribution = sport_counts;
        
        // Update the stored stats
        storage.platform_stats = stats.clone();
        
        stats
    }
    
    // Helper methods
    fn get_team_events(storage: &SportsBrokerStorage, team_id: u32) -> Vec<SportsEvent> {
        let mut events = Vec::new();
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                if event.home_team_id == team_id || event.away_team_id == team_id {
                    events.push(event);
                }
            }
        }
        events
    }
    
    fn calculate_team_revenue(events: &[SportsEvent]) -> u128 {
        events.iter().map(|event| event.revenue_generated).sum()
    }
    
    fn calculate_average_attendance(events: &[SportsEvent]) -> u32 {
        if events.is_empty() {
            return 0;
        }
        let total_attendance: u32 = events.iter().map(|event| event.sold_tickets).sum();
        total_attendance / events.len() as u32
    }
    
    fn calculate_ticket_price_trend(events: &[SportsEvent]) -> Vec<(u64, u128)> {
        events.iter()
            .map(|event| (event.date, event.base_price))
            .collect()
    }
    
    fn calculate_performance_correlation(_team_id: u32, _storage: &SportsBrokerStorage) -> u8 {
        // Placeholder implementation
        75
    }
    
    fn calculate_fan_engagement_score(_team_id: u32, _storage: &SportsBrokerStorage) -> u8 {
        // Placeholder implementation
        80
    }
    
    fn get_user_tickets(storage: &SportsBrokerStorage, user_id: AccountId) -> Vec<SportsTicket> {
        if let Some(ticket_ids) = storage.user_tickets.get(user_id) {
            ticket_ids.iter()
                .filter_map(|&ticket_id| storage.tickets.get(ticket_id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn calculate_total_spent(tickets: &[SportsTicket]) -> u128 {
        tickets.iter().map(|ticket| ticket.purchase_price).sum()
    }
    
    fn get_user_favorite_sport(_storage: &SportsBrokerStorage, _user_id: AccountId) -> Option<SportType> {
        // Placeholder implementation
        Some(SportType::Football)
    }
    
    fn get_user_loyalty_tier(storage: &SportsBrokerStorage, user_id: AccountId) -> Option<u8> {
        if let Some(profile) = storage.loyalty_profiles.get(user_id) {
            Some(profile.current_tier as u8)
        } else {
            None
        }
    }
    
    fn get_last_purchase_date(tickets: &[SportsTicket]) -> Option<u64> {
        tickets.iter()
            .map(|ticket| ticket.purchase_date)
            .max()
    }
    
    fn calculate_purchase_frequency(tickets: &[SportsTicket]) -> u8 {
        if tickets.len() < 2 {
            return 0;
        }
        
        let mut dates: Vec<u64> = tickets.iter()
            .map(|ticket| ticket.purchase_date)
            .collect();
        dates.sort();
        
        let total_days = dates.last().unwrap() - dates.first().unwrap();
        let avg_days_between = total_days / (dates.len() - 1) as u64;
        
        avg_days_between as u8
    }
    
    fn get_top_performing_events(storage: &SportsBrokerStorage) -> Vec<u32> {
        let mut event_performance: Vec<(u32, u128)> = Vec::new();
        
        for event_id in 1..=storage.total_events {
            if let Some(event) = storage.events.get(event_id) {
                let performance_score = event.revenue_generated + (event.sold_tickets as u128 * 1000);
                event_performance.push((event_id, performance_score));
            }
        }
        
        event_performance.sort_by(|a, b| b.1.cmp(&a.1));
        event_performance.into_iter()
            .take(10)
            .map(|(event_id, _)| event_id)
            .collect()
    }
    
    fn get_top_performing_teams(storage: &SportsBrokerStorage) -> Vec<u32> {
        let mut team_performance: Vec<(u32, u128)> = Vec::new();
        
        for team_id in 1..=storage.total_teams {
            let events = Self::get_team_events(storage, team_id);
            let total_revenue: u128 = events.iter().map(|event| event.revenue_generated).sum();
            let total_tickets: u32 = events.iter().map(|event| event.sold_tickets).sum();
            let performance_score = total_revenue + (total_tickets as u128 * 1000);
            team_performance.push((team_id, performance_score));
        }
        
        team_performance.sort_by(|a, b| b.1.cmp(&a.1));
        team_performance.into_iter()
            .take(10)
            .map(|(team_id, _)| team_id)
            .collect()
    }
    
    fn calculate_revenue_trends(storage: &SportsBrokerStorage, start_date: u64, end_date: u64) -> Vec<(u64, u128)> {
        let mut trends = Vec::new();
        let time_span = end_date - start_date;
        let interval = time_span / 10; // 10 data points
        
        for i in 0..10 {
            let timestamp = start_date + (i * interval);
            let mut revenue = 0;
            
            for event_id in 1..=storage.total_events {
                if let Some(event) = storage.events.get(event_id) {
                    if event.date >= timestamp && event.date < timestamp + interval {
                        revenue += event.revenue_generated;
                    }
                }
            }
            
            trends.push((timestamp, revenue));
        }
        
        trends
    }
    
    fn calculate_attendance_trends(storage: &SportsBrokerStorage, start_date: u64, end_date: u64) -> Vec<(u64, u32)> {
        let mut trends = Vec::new();
        let time_span = end_date - start_date;
        let interval = time_span / 10; // 10 data points
        
        for i in 0..10 {
            let timestamp = start_date + (i * interval);
            let mut attendance = 0;
            
            for event_id in 1..=storage.total_events {
                if let Some(event) = storage.events.get(event_id) {
                    if event.date >= timestamp && event.date < timestamp + interval {
                        attendance += event.sold_tickets;
                    }
                }
            }
            
            trends.push((timestamp, attendance));
        }
        
        trends
    }
    

}

/// User engagement metrics
#[derive(Debug, Clone, PartialEq)]
pub struct UserEngagementMetrics {
    pub total_users: u32,
    pub active_users: u32,
    pub engagement_rate: u32,
    pub average_tickets_per_user: u32,
}
