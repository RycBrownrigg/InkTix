use ink::prelude::*;
use ink::storage::Mapping;
use crate::types::*;

/// Main contract storage structure
#[derive(Debug)]
#[ink::storage_item]
pub struct SportsBrokerStorage {
    // Contract state
    pub owner: ink::primitives::AccountId,
    pub total_teams: u32,
    pub total_venues: u32,
    pub total_events: u32,
    pub total_tickets: u64,
    pub total_seasons: u32,
    pub next_report_id: u32,
    pub analytics_enabled: bool,

    // Core data mappings
    pub teams: Mapping<u32, Team>,
    pub venues: Mapping<u32, Venue>,
    pub seasons: Mapping<u32, Season>,
    pub events: Mapping<u32, SportsEvent>,
    pub tickets: Mapping<u64, SportsTicket>,
    pub user_tickets: Mapping<ink::primitives::AccountId, Vec<u64>>,

    // Currency and pricing
    pub supported_currencies: Vec<CurrencyId>,
    pub currency_rates: Mapping<CurrencyId, u128>,
    pub currency_revenue: Mapping<CurrencyId, u128>,

    // Team performance and pricing
    pub team_performance: Mapping<u32, TeamPerformance>,
    pub pricing_multipliers: Mapping<u32, PricingMultiplier>,

    // Analytics storage
    pub platform_stats: PlatformStats,
    pub event_analytics: Mapping<u32, EventAnalytics>,
    pub team_analytics: Mapping<u32, TeamAnalytics>,
    pub user_analytics: Mapping<ink::primitives::AccountId, UserAnalytics>,
    pub analytics_reports: Mapping<u32, AnalyticsReport>,

    // Anti-scalping
    pub anti_scalping_configs: Mapping<u32, AntiScalpingConfig>,
    pub ticket_transfer_history: Mapping<u64, TicketTransferHistory>,
    pub user_behavior_profiles: Mapping<ink::primitives::AccountId, UserBehaviorProfile>,
    pub resale_listings: Mapping<u64, ResaleListing>,

    // Loyalty and rewards
    pub loyalty_profiles: Mapping<ink::primitives::AccountId, LoyaltyProfile>,
    pub reward_redemptions: Mapping<u64, RewardRedemption>,
    pub points_rules: Mapping<u32, PointsRule>,
    pub promotions: Mapping<u32, Promotion>,
    pub referrals: Mapping<ink::primitives::AccountId, Referral>,
}

impl Default for SportsBrokerStorage {
    fn default() -> Self {
        Self {
            owner: ink::primitives::AccountId::from([0u8; 32]),
            total_teams: 0,
            total_venues: 0,
            total_events: 0,
            total_tickets: 0,
            total_seasons: 0,
            next_report_id: 1,
            analytics_enabled: true,

            teams: Mapping::default(),
            venues: Mapping::default(),
            seasons: Mapping::default(),
            events: Mapping::default(),
            tickets: Mapping::default(),
            user_tickets: Mapping::default(),

            supported_currencies: vec![CurrencyId::DOT, CurrencyId::ACA, CurrencyId::AUSD, CurrencyId::LDOT, CurrencyId::KSM],
            currency_rates: Mapping::default(),
            currency_revenue: Mapping::default(),

            team_performance: Mapping::default(),
            pricing_multipliers: Mapping::default(),

            platform_stats: PlatformStats {
                total_events: 0,
                total_tickets_sold: 0,
                total_revenue: 0,
                total_users: 0,
                average_ticket_price: 0,
                total_season_passes: 0,
                currency_distribution: vec![],
                sport_type_distribution: vec![],
                last_updated: 0,
            },
            event_analytics: Mapping::default(),
            team_analytics: Mapping::default(),
            user_analytics: Mapping::default(),
            analytics_reports: Mapping::default(),

            anti_scalping_configs: Mapping::default(),
            ticket_transfer_history: Mapping::default(),
            user_behavior_profiles: Mapping::default(),
            resale_listings: Mapping::default(),

            loyalty_profiles: Mapping::default(),
            reward_redemptions: Mapping::default(),
            points_rules: Mapping::default(),
            promotions: Mapping::default(),
            referrals: Mapping::default(),
        }
    }
}

impl SportsBrokerStorage {
    /// Initialize currency rates with default values
    pub fn initialize_currency_rates(&mut self) {
        self.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000_000_000);
        self.currency_rates.insert(CurrencyId::ACA, &50_000_000_000_000_000);
        self.currency_rates.insert(CurrencyId::AUSD, &1_000_000_000_000_000_000);
        self.currency_rates.insert(CurrencyId::LDOT, &1_000_000_000_000_000_000);
        self.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000_000_000);
    }

    /// Get the next available ID for a given entity type
    pub fn get_next_id(&mut self, entity_type: &str) -> u32 {
        match entity_type {
            "team" => {
                self.total_teams += 1;
                self.total_teams
            }
            "venue" => {
                self.total_venues += 1;
                self.total_venues
            }
            "event" => {
                self.total_events += 1;
                self.total_events
            }
            "season" => {
                self.total_seasons += 1;
                self.total_seasons
            }
            _ => 0,
        }
    }

    /// Get the next ticket ID
    pub fn get_next_ticket_id(&mut self) -> u64 {
        self.total_tickets += 1;
        self.total_tickets
    }

    /// Get the next report ID
    pub fn get_next_report_id(&mut self) -> u32 {
        let id = self.next_report_id;
        self.next_report_id += 1;
        id
    }
}
