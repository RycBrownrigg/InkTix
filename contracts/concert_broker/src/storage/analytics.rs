use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Revenue analytics tracking
pub struct RevenueAnalytics {
    pub total_revenue: u128,
    pub revenue_by_currency: Mapping<u32, u128>, // CurrencyId -> amount
    pub revenue_by_event: Mapping<u32, u128>,    // EventId -> amount
}

impl RevenueAnalytics {
    pub fn new() -> Self {
        Self {
            total_revenue: 0,
            revenue_by_currency: Mapping::new(),
            revenue_by_event: Mapping::new(),
        }
    }
}