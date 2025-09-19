use crate::types::*;
use ink::prelude::*;
use ink::primitives::AccountId;
use ink::storage::Mapping;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;

#[allow(clippy::arithmetic_side_effects)]

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
    pub total_season_passes: u32,
    pub total_season_pass_packages: u32,
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

    // Season pass management
    pub season_passes: Mapping<u32, SeasonPass>,
    pub season_pass_packages: Mapping<u32, SeasonPassPackage>,
    pub season_pass_usage: Mapping<u32, SeasonPassUsage>,
    pub renewal_options: Mapping<u32, RenewalOption>,
    pub season_pass_analytics: Mapping<u32, SeasonPassAnalytics>,
    pub user_season_passes: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub team_season_passes: Mapping<u32, Vec<u32>>,

    // Fantasy sports management
    pub total_fantasy_leagues: u32,
    pub total_fantasy_teams: u32,
    pub total_fantasy_participations: u32,
    pub total_fantasy_rewards: u32,
    pub total_fantasy_game_weeks: u32,
    pub total_fantasy_transfers: u32,
    pub fantasy_leagues: Mapping<u32, FantasyLeague>,
    pub fantasy_teams: Mapping<u32, FantasyTeam>,
    pub fantasy_participations: Mapping<u32, FantasyLeagueParticipation>,
    pub fantasy_leaderboards: Mapping<u32, FantasyLeaderboard>,
    pub fantasy_rewards: Mapping<u32, FantasyRewards>,
    pub fantasy_game_weeks: Mapping<u32, FantasyGameWeek>,
    pub fantasy_transfers: Mapping<u32, FantasyTransfer>,
    pub fantasy_settings: Mapping<u32, FantasySettings>,
    pub user_fantasy_leagues: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub user_fantasy_teams: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub league_participants: Mapping<u32, Vec<ink::primitives::AccountId>>,

    // Advanced team loyalty management
    pub total_team_loyalty_profiles: u32,
    pub total_team_stakings: u32,
    pub total_team_attendance: u32,
    pub total_team_performance_rewards: u32,
    pub total_team_loyalty_challenges: u32,
    pub team_loyalty_profiles: Mapping<(ink::primitives::AccountId, u32), TeamLoyaltyProfile>,
    pub team_stakings: Mapping<u32, TeamStaking>,
    pub team_attendance: Mapping<u32, TeamAttendance>,
    pub team_attendances: Mapping<u32, TeamAttendance>,
    pub team_performance_rewards: Mapping<u32, TeamPerformanceReward>,
    pub team_loyalty_challenges: Mapping<u32, TeamLoyaltyChallenge>,
    pub team_loyalty_analytics: Mapping<u32, TeamLoyaltyAnalytics>,
    pub user_team_loyalty: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub team_fans: Mapping<u32, Vec<ink::primitives::AccountId>>,

    // Venue-specific management
    pub total_parking_passes: u32,
    pub total_concession_credits: u32,
    pub total_merchandise_bundles: u32,
    pub total_bundle_purchases: u32,
    pub total_capacity_reservations: u32,
    pub parking_passes: Mapping<u32, crate::types::venue::ParkingPass>,
    pub concession_credits: Mapping<u32, crate::types::venue::ConcessionCredits>,
    pub merchandise_bundles: Mapping<u32, crate::types::venue::MerchandiseBundle>,
    pub bundle_purchases: Mapping<u32, crate::types::venue::MerchandiseBundlePurchase>,
    pub capacity_reservations: Mapping<u32, crate::types::venue::CapacityReservation>,
    pub user_parking_passes: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub user_concession_credits: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub user_merchandise_bundles: Mapping<ink::primitives::AccountId, Vec<u32>>,
    pub venue_parking_passes: Mapping<u32, Vec<u32>>,
    pub venue_concession_credits: Mapping<u32, Vec<u32>>,
    pub venue_merchandise_bundles: Mapping<u32, Vec<u32>>,

    // Cross-chain functionality
    pub total_cross_chain_events: u32,
    pub total_cross_chain_requests: u32,
    pub total_cross_chain_transactions: u32,
    pub total_connected_chains: u32,
    pub cross_chain_events: Mapping<u32, CrossChainEvent>,
    pub cross_chain_requests: Mapping<u32, CrossChainTicketRequest>,
    pub cross_chain_transactions: Mapping<u32, CrossChainTransaction>,
    pub chain_connectivity: Mapping<BlockchainNetwork, ChainConnectivityStatus>,
    pub user_cross_chain_requests: Mapping<AccountId, Vec<u32>>,
    pub user_cross_chain_transactions: Mapping<AccountId, Vec<u32>>,
    pub chain_events: Mapping<BlockchainNetwork, Vec<u32>>,

    // XCM Management
    pub total_xcm_messages_sent: u64,
    pub total_xcm_messages_received: u64,
    pub next_xcm_message_id: u64,
    pub next_nonce: u64,
    pub xcm_messages: Mapping<u64, XcmMessage>,
    pub xcm_chain_connectivity: Mapping<String, XcmConnectivityStatus>,
    pub chain_xcm_messages: Mapping<String, Vec<u64>>,
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
            total_season_passes: 0,
            total_season_pass_packages: 0,
            total_fantasy_leagues: 0,
            total_fantasy_teams: 0,
            total_fantasy_participations: 0,
            total_fantasy_rewards: 0,
            total_fantasy_game_weeks: 0,
            total_fantasy_transfers: 0,
            total_team_loyalty_profiles: 0,
            total_team_stakings: 0,
            total_team_attendance: 0,
            total_team_performance_rewards: 0,
            total_team_loyalty_challenges: 0,
            total_parking_passes: 0,
            total_concession_credits: 0,
            total_merchandise_bundles: 0,
            total_bundle_purchases: 0,
            total_capacity_reservations: 0,
            next_report_id: 1,
            analytics_enabled: true,

            teams: Mapping::default(),
            venues: Mapping::default(),
            seasons: Mapping::default(),
            events: Mapping::default(),
            tickets: Mapping::default(),
            user_tickets: Mapping::default(),

            supported_currencies: vec![
                CurrencyId::DOT,
                CurrencyId::ACA,
                CurrencyId::AUSD,
                CurrencyId::LDOT,
                CurrencyId::KSM,
            ],
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

            // Season pass management
            season_passes: Mapping::default(),
            season_pass_packages: Mapping::default(),
            season_pass_usage: Mapping::default(),
            renewal_options: Mapping::default(),
            season_pass_analytics: Mapping::default(),
            user_season_passes: Mapping::default(),
            team_season_passes: Mapping::default(),

            // Fantasy sports management
            fantasy_leagues: Mapping::default(),
            fantasy_teams: Mapping::default(),
            fantasy_participations: Mapping::default(),
            fantasy_leaderboards: Mapping::default(),
            fantasy_rewards: Mapping::default(),
            fantasy_game_weeks: Mapping::default(),
            fantasy_transfers: Mapping::default(),
            fantasy_settings: Mapping::default(),
            user_fantasy_leagues: Mapping::default(),
            user_fantasy_teams: Mapping::default(),
            league_participants: Mapping::default(),

            // Advanced team loyalty management
            team_loyalty_profiles: Mapping::default(),
            team_stakings: Mapping::default(),
            team_attendance: Mapping::default(),
            team_attendances: Mapping::default(),
            team_performance_rewards: Mapping::default(),
            team_loyalty_challenges: Mapping::default(),
            team_loyalty_analytics: Mapping::default(),
            user_team_loyalty: Mapping::default(),
            team_fans: Mapping::default(),

            // Venue-specific management
            parking_passes: Mapping::default(),
            concession_credits: Mapping::default(),
            merchandise_bundles: Mapping::default(),
            bundle_purchases: Mapping::default(),
            capacity_reservations: Mapping::default(),
            user_parking_passes: Mapping::default(),
            user_concession_credits: Mapping::default(),
            user_merchandise_bundles: Mapping::default(),
            venue_parking_passes: Mapping::default(),
            venue_concession_credits: Mapping::default(),
            venue_merchandise_bundles: Mapping::default(),

            // Cross-chain functionality
            total_cross_chain_events: 0,
            total_cross_chain_requests: 0,
            total_cross_chain_transactions: 0,
            total_connected_chains: 0,
            cross_chain_events: Mapping::default(),
            cross_chain_requests: Mapping::default(),
            cross_chain_transactions: Mapping::default(),
            chain_connectivity: Mapping::default(),
            user_cross_chain_requests: Mapping::default(),
            user_cross_chain_transactions: Mapping::default(),
            chain_events: Mapping::default(),

            // XCM Management
            total_xcm_messages_sent: 0,
            total_xcm_messages_received: 0,
            next_xcm_message_id: 1,
            next_nonce: 1,
            xcm_messages: Mapping::default(),
            xcm_chain_connectivity: Mapping::default(),
            chain_xcm_messages: Mapping::default(),
        }
    }
}

#[allow(clippy::arithmetic_side_effects)]
impl SportsBrokerStorage {
    /// Initialize currency rates with default values
    pub fn initialize_currency_rates(&mut self) {
        self.currency_rates
            .insert(CurrencyId::DOT, &1_000_000_000_000_000_000);
        self.currency_rates
            .insert(CurrencyId::ACA, &50_000_000_000_000_000);
        self.currency_rates
            .insert(CurrencyId::AUSD, &1_000_000_000_000_000_000);
        self.currency_rates
            .insert(CurrencyId::LDOT, &1_000_000_000_000_000_000);
        self.currency_rates
            .insert(CurrencyId::KSM, &15_000_000_000_000_000_000);
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
            "team_loyalty_profile" => {
                self.total_team_loyalty_profiles += 1;
                self.total_team_loyalty_profiles
            }
            "team_staking" => {
                self.total_team_stakings += 1;
                self.total_team_stakings
            }
            "team_attendance" => {
                self.total_team_attendance += 1;
                self.total_team_attendance
            }
            "team_performance_reward" => {
                self.total_team_performance_rewards += 1;
                self.total_team_performance_rewards
            }
            "team_loyalty_challenge" => {
                self.total_team_loyalty_challenges += 1;
                self.total_team_loyalty_challenges
            }
            "parking_pass" => {
                self.total_parking_passes += 1;
                self.total_parking_passes
            }
            "concession_credits" => {
                self.total_concession_credits += 1;
                self.total_concession_credits
            }
            "merchandise_bundle" => {
                self.total_merchandise_bundles += 1;
                self.total_merchandise_bundles
            }
            "bundle_purchase" => {
                self.total_bundle_purchases += 1;
                self.total_bundle_purchases
            }
            "capacity_reservation" => {
                self.total_capacity_reservations += 1;
                self.total_capacity_reservations
            }
            "cross_chain_event" => {
                self.total_cross_chain_events += 1;
                self.total_cross_chain_events
            }
            "cross_chain_request" => {
                self.total_cross_chain_requests += 1;
                self.total_cross_chain_requests
            }
            "cross_chain_transaction" => {
                self.total_cross_chain_transactions += 1;
                self.total_cross_chain_transactions
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

    /// Get the next season pass ID
    pub fn get_next_season_pass_id(&mut self) -> u32 {
        let id = self.total_season_passes + 1;
        self.total_season_passes = id;
        id
    }

    /// Get the next season pass package ID
    pub fn get_next_season_pass_package_id(&mut self) -> u32 {
        // We'll use a simple counter for packages
        // In a real implementation, this might be more sophisticated
        let id = self.next_report_id;
        self.next_report_id += 1;
        id
    }

    /// Get the next fantasy league ID
    pub fn get_next_fantasy_league_id(&mut self) -> u32 {
        self.total_fantasy_leagues += 1;
        self.total_fantasy_leagues
    }

    /// Get the next fantasy team ID
    pub fn get_next_fantasy_team_id(&mut self) -> u32 {
        self.total_fantasy_teams += 1;
        self.total_fantasy_teams
    }

    /// Get the next fantasy participation ID
    pub fn get_next_fantasy_participation_id(&mut self) -> u32 {
        self.total_fantasy_participations += 1;
        self.total_fantasy_participations
    }

    /// Get the next fantasy reward ID
    pub fn get_next_fantasy_reward_id(&mut self) -> u32 {
        self.total_fantasy_rewards += 1;
        self.total_fantasy_rewards
    }

    /// Get the next fantasy game week ID
    pub fn get_next_fantasy_game_week_id(&mut self) -> u32 {
        self.total_fantasy_game_weeks += 1;
        self.total_fantasy_game_weeks
    }

    /// Get the next fantasy transfer ID
    pub fn get_next_fantasy_transfer_id(&mut self) -> u32 {
        self.total_fantasy_transfers += 1;
        self.total_fantasy_transfers
    }

    /// Get current timestamp
    pub fn get_current_timestamp(&self) -> u64 {
        ink::env::block_timestamp::<ink::env::DefaultEnvironment>()
    }
}
