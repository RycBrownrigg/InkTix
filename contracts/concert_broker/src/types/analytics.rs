use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Revenue analytics storage structure
pub struct RevenueAnalytics {
    pub total_revenue: u128,
    pub artist_revenue: Mapping<u32, u128>,
    pub venue_revenue: Mapping<u32, u128>,
    pub currency_revenue: Mapping<CurrencyId, u128>,
    pub merchandise_revenue: u128,
    pub vip_revenue: u128,
}

impl RevenueAnalytics {
    pub fn new() -> Self {
        Self {
            total_revenue: 0,
            artist_revenue: Mapping::new(),
            venue_revenue: Mapping::new(),
            currency_revenue: Mapping::new(),
            merchandise_revenue: 0,
            vip_revenue: 0,
        }
    }

    pub fn update_total_revenue(&mut self, amount: u128) {
        self.total_revenue = self.total_revenue.saturating_add(amount);
    }

    pub fn update_artist_revenue(&mut self, artist_id: u32, amount: u128) {
        let current = self.artist_revenue.get(artist_id).unwrap_or(0);
        self.artist_revenue.insert(artist_id, &(current.saturating_add(amount)));
    }

    pub fn update_venue_revenue(&mut self, venue_id: u32, amount: u128) {
        let current = self.venue_revenue.get(venue_id).unwrap_or(0);
        self.venue_revenue.insert(venue_id, &(current.saturating_add(amount)));
    }

    pub fn update_currency_revenue(&mut self, currency: CurrencyId, amount: u128) {
        let current = self.currency_revenue.get(currency).unwrap_or(0);
        self.currency_revenue.insert(currency, &(current.saturating_add(amount)));
    }

    pub fn update_merchandise_revenue(&mut self, amount: u128) {
        self.merchandise_revenue = self.merchandise_revenue.saturating_add(amount);
    }

    pub fn update_vip_revenue(&mut self, amount: u128) {
        self.vip_revenue = self.vip_revenue.saturating_add(amount);
    }
}