#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sports_broker {
    use ink::prelude::{string::String, vec::Vec};

    /// The Sports Broker contract storage.
    #[ink(storage)]
    pub struct SportsBroker {
        /// The contract owner
        owner: AccountId,
        
        // Team management
        teams: ink::storage::Mapping<u32, Team>,
        next_team_id: u32,
        
        // Venue management  
        venues: ink::storage::Mapping<u32, Venue>,
        next_venue_id: u32,

        // Season management
        seasons: ink::storage::Mapping<u32, Season>,
        next_season_id: u32,

        // Sports Event management
        events: ink::storage::Mapping<u32, SportsEvent>,
        next_event_id: u32,

        // Enhanced Ticket management
        tickets: ink::storage::Mapping<u64, SportsTicket>,
        next_ticket_id: u64,
        user_tickets: ink::storage::Mapping<AccountId, Vec<u64>>,

        // User Profile & Loyalty System
        user_profiles: ink::storage::Mapping<AccountId, UserProfile>,
        loyalty_points: ink::storage::Mapping<AccountId, u32>,
        team_fans: ink::storage::Mapping<u32, Vec<AccountId>>,

        // Season Pass System
        season_passes: ink::storage::Mapping<u64, SeasonPass>,
        next_season_pass_id: u64,
        user_season_passes: ink::storage::Mapping<AccountId, Vec<u64>>,

        // Dynamic Pricing Engine
        team_performance: ink::storage::Mapping<u32, TeamPerformance>,
        pricing_multipliers: ink::storage::Mapping<u32, PricingMultiplier>,

        // Multi-Currency Support
        supported_currencies: Vec<CurrencyId>,
        currency_rates: ink::storage::Mapping<CurrencyId, Balance>,
        staking_rewards_pool: Balance,
        user_staked_amounts: ink::storage::Mapping<AccountId, Balance>,
        total_staked_amount: Balance,
        last_staking_update: ink::storage::Mapping<AccountId, u64>,

        // NEW: Analytics & Search (Step 10)
        event_search_index: ink::storage::Mapping<u32, Vec<u32>>, // sport_type_hash -> event_ids
        team_event_index: ink::storage::Mapping<u32, Vec<u32>>,   // team_id -> event_ids
        venue_event_index: ink::storage::Mapping<u32, Vec<u32>>,  // venue_id -> event_ids
        date_event_index: ink::storage::Mapping<u64, Vec<u32>>,   // date_bucket -> event_ids
        
        // Revenue and analytics tracking
        total_revenue: Balance,
        currency_revenue: ink::storage::Mapping<CurrencyId, Balance>,
        team_revenue: ink::storage::Mapping<u32, Balance>,
        venue_revenue: ink::storage::Mapping<u32, Balance>,
        
        // Performance metrics
        platform_stats: PlatformStats,
    }

    /// Team information
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Team {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub sport_type: SportType,
        pub verified: bool,
    }

    /// Venue information
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Venue {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub capacity: u32,
    }

    /// Season information for subscription management
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Season {
        pub id: u32,
        pub name: String,
        pub sport_type: SportType,
        pub start_date: u64,
        pub end_date: u64,
        pub regular_season_games: u32,
        pub active: bool,
        pub season_pass_base_price: Balance,
        pub early_bird_discount: u8,
        pub early_bird_deadline: u64,
    }

    /// Enhanced Event structure for sports
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SportsEvent {
        pub id: u32,
        pub name: String,
        pub venue_id: u32,
        pub date: u64,
        pub capacity: u32,
        pub sold_tickets: u32,
        pub base_price: Balance,
        pub active: bool,
        pub sport_type: SportType,
        pub home_team_id: u32,
        pub away_team_id: u32,
        pub season_id: u32,
        pub game_type: GameType,
        pub season_pass_discount: u8,
        pub dynamic_pricing_enabled: bool,
        pub rivalry_multiplier: u32,
        pub revenue_generated: Balance, // NEW: Track revenue per event
    }

    /// Enhanced Ticket structure for sports with multi-currency support
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SportsTicket {
        pub id: u64,
        pub event_id: u32,
        pub owner: AccountId,
        pub purchase_price: Balance,
        pub purchase_currency: CurrencyId,
        pub purchase_date: u64,
        pub seat_number: u32,
        pub transferable: bool,
        pub section: String,
        pub row: String,
        pub seat_type: SeatType,
        pub access_level: AccessLevel,
        pub loyalty_points_earned: u32,
        pub season_pass_discount_applied: bool,
        pub is_season_pass_ticket: bool,
        pub dynamic_price_paid: Balance,
        pub performance_multiplier_applied: u32,
        pub dot_equivalent_paid: Balance,
    }

    /// User profile for fan management
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct UserProfile {
        pub verified_fan: bool,
        pub favorite_teams: Vec<u32>,
        pub home_city: String,
        pub loyalty_tier: LoyaltyTier,
        pub total_games_attended: u32,
        pub account_creation_date: u64,
        pub anti_scalping_verified: bool,
        pub social_media_verified: bool,
        pub season_pass_holder: bool,
        pub preferred_currency: CurrencyId,
        pub total_spent: Balance, // NEW: Track user spending
    }

    /// Season pass for subscription management with multi-currency support
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SeasonPass {
        pub id: u64,
        pub owner: AccountId,
        pub season_id: u32,
        pub team_id: u32,
        pub pass_type: SeasonPassType,
        pub purchase_price: Balance,
        pub purchase_currency: CurrencyId,
        pub purchase_date: u64,
        pub games_included: u32,
        pub games_attended: u32,
        pub transferable: bool,
        pub includes_playoffs: bool,
        pub priority_level: u8,
        pub loyalty_tier_at_purchase: LoyaltyTier,
        pub staking_rewards_enabled: bool,
        pub staked_amount: Balance,
        pub valid_until: u64,
        pub dot_equivalent_paid: Balance,
    }

    /// Team performance for dynamic pricing
    #[derive(Debug, PartialEq, Eq)]
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
    #[derive(Debug, PartialEq, Eq)]
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

    /// NEW: Platform statistics for analytics
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct PlatformStats {
        pub total_users: u32,
        pub total_tickets_sold: u64,
        pub total_season_passes_sold: u64,
        pub total_events_created: u32,
        pub average_ticket_price: Balance,
        pub most_popular_sport: SportType,
        pub most_popular_team_id: u32,
        pub most_popular_venue_id: u32,
        pub last_updated: u64,
    }

    /// NEW: Search filters for advanced event discovery
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct EventSearchFilter {
        pub sport_type: Option<SportType>,
        pub team_id: Option<u32>,
        pub venue_id: Option<u32>,
        pub min_date: Option<u64>,
        pub max_date: Option<u64>,
        pub game_type: Option<GameType>,
        pub max_price: Option<Balance>,
        pub min_availability: Option<u32>, // Minimum available tickets
        pub active_only: bool,
    }

    /// NEW: Analytics report structure
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct AnalyticsReport {
        pub total_revenue: Balance,
        pub revenue_by_currency: Vec<(CurrencyId, Balance)>,
        pub top_teams_by_revenue: Vec<(u32, Balance)>,
        pub top_venues_by_revenue: Vec<(u32, Balance)>,
        pub average_ticket_price: Balance,
        pub total_tickets_sold: u64,
        pub total_season_passes_sold: u64,
        pub user_count: u32,
        pub most_popular_sport: SportType,
        pub report_generated_at: u64,
    }

    /// Multi-currency support for Acala integration
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum CurrencyId {
        DOT,
        ACA,
        AUSD,
        LDOT,
        KSM,
    }

    /// Sport types
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SportType {
        Basketball,
        Football,
        Soccer,
        Baseball,
        Hockey,
        Tennis,
        Other(String),
    }

    /// Game/Event types
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum GameType {
        RegularSeason,
        Playoff,
        Championship,
        AllStar,
        Preseason,
        Tournament,
        Exhibition,
    }

    /// Seat types for sports venues
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SeatType {
        GeneralAdmission,
        Reserved,
        PremiumReserved,
        Club,
        Suite,
        FieldLevel,
        Courtside,
        StudentSection,
    }

    /// Access levels for different ticket types
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AccessLevel {
        Standard,
        Premium,
        VIP,
        AllAccess,
    }

    /// Loyalty tiers
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum LoyaltyTier {
        Bronze,
        Silver,
        Gold,
        Platinum,
        Diamond,
    }

    /// Season pass types
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SeasonPassType {
        FullSeason,
        HalfSeason,
        QuarterSeason,
        Weekend,
        Weekday,
        Premium,
        PlayoffsOnly,
        Package(u32),
    }

    /// Sports broker errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
        TeamNotFound,
        VenueNotFound,
        SeasonNotFound,
        EventNotFound,
        EventNotActive,
        EventSoldOut,
        InsufficientPayment,
        TicketNotFound,
        NotTicketOwner,
        TicketNotTransferable,
        InsufficientCapacity,
        IdOverflow,
        ProfileAlreadyExists,
        ProfileNotFound,
        InvalidFavoriteTeams,
        SeasonPassNotFound,
        NotSeasonPassOwner,
        SeasonNotActive,
        SeasonPassNotTransferable,
        SeasonPassExpired,
        InsufficientStakingRewards,
        StakingNotEnabled,
        PerformanceDataNotFound,
        PricingDataOutdated,
        DynamicPricingDisabled,
        InvalidPerformanceStats,
        UnsupportedCurrency,
        CurrencyConversionFailed,
        InvalidCurrencyRate,
        StakingRewardsNotReady,
        // NEW: Search and analytics errors
        InvalidSearchParameters,
        NoSearchResults,
        InvalidDateRange,
        AnalyticsNotReady,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl SportsBroker {
        /// Creates a new Sports Broker contract with full functionality.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut supported_currencies = Vec::new();
            supported_currencies.push(CurrencyId::DOT);
            supported_currencies.push(CurrencyId::ACA);
            supported_currencies.push(CurrencyId::AUSD);
            supported_currencies.push(CurrencyId::LDOT);
            supported_currencies.push(CurrencyId::KSM);

            let platform_stats = PlatformStats {
                total_users: 0,
                total_tickets_sold: 0,
                total_season_passes_sold: 0,
                total_events_created: 0,
                average_ticket_price: 0,
                most_popular_sport: SportType::Basketball,
                most_popular_team_id: 0,
                most_popular_venue_id: 0,
                last_updated: 0,
            };

            let mut contract = Self {
                owner: Self::env().caller(),
                teams: ink::storage::Mapping::new(),
                next_team_id: 1,
                venues: ink::storage::Mapping::new(),
                next_venue_id: 1,
                seasons: ink::storage::Mapping::new(),
                next_season_id: 1,
                events: ink::storage::Mapping::new(),
                next_event_id: 1,
                tickets: ink::storage::Mapping::new(),
                next_ticket_id: 1,
                user_tickets: ink::storage::Mapping::new(),
                user_profiles: ink::storage::Mapping::new(),
                loyalty_points: ink::storage::Mapping::new(),
                team_fans: ink::storage::Mapping::new(),
                season_passes: ink::storage::Mapping::new(),
                next_season_pass_id: 1,
                user_season_passes: ink::storage::Mapping::new(),
                team_performance: ink::storage::Mapping::new(),
                pricing_multipliers: ink::storage::Mapping::new(),
                supported_currencies,
                currency_rates: ink::storage::Mapping::new(),
                staking_rewards_pool: 100_000_000_000_000_000,
                user_staked_amounts: ink::storage::Mapping::new(),
                total_staked_amount: 0,
                last_staking_update: ink::storage::Mapping::new(),
                // NEW: Initialize search and analytics storage
                event_search_index: ink::storage::Mapping::new(),
                team_event_index: ink::storage::Mapping::new(),
                venue_event_index: ink::storage::Mapping::new(),
                date_event_index: ink::storage::Mapping::new(),
                total_revenue: 0,
                currency_revenue: ink::storage::Mapping::new(),
                team_revenue: ink::storage::Mapping::new(),
                venue_revenue: ink::storage::Mapping::new(),
                platform_stats,
            };

            // Set default currency rates
            contract.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000);
            contract.currency_rates.insert(CurrencyId::ACA, &50_000_000_000);
            contract.currency_rates.insert(CurrencyId::AUSD, &150_000_000_000);
            contract.currency_rates.insert(CurrencyId::LDOT, &950_000_000_000);
            contract.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000);

            // Initialize currency revenue tracking
            for currency in &contract.supported_currencies {
                contract.currency_revenue.insert(*currency, &0);
            }

            contract
        }

        // ========================================================================
        // TEAM & VENUE MANAGEMENT (Steps 1-2)
        // ========================================================================

        #[ink(message)]
        pub fn register_team(
            &mut self,
            name: String,
            city: String,
            sport_type: SportType,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let team_id = self.next_team_id;
            self.next_team_id = self.next_team_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let team = Team {
                id: team_id,
                name,
                city,
                sport_type,
                verified: true,
            };

            self.teams.insert(team_id, &team);
            self.team_fans.insert(team_id, &Vec::<AccountId>::new());
            self.team_event_index.insert(team_id, &Vec::<u32>::new()); // NEW: Initialize event index
            self.team_revenue.insert(team_id, &0); // NEW: Initialize revenue tracking
            
            // Initialize performance tracking
            let performance = TeamPerformance {
                team_id,
                season_id: 0,
                wins: 0,
                losses: 0,
                win_percentage: 0,
                streak: 0,
                playoff_probability: 5000,
                last_updated: self.env().block_timestamp(),
                performance_rank: 0,
                home_record_wins: 0,
                home_record_losses: 0,
                points_scored_avg: 10000,
                points_allowed_avg: 10000,
            };
            self.team_performance.insert(team_id, &performance);

            // Initialize pricing multiplier
            let pricing = PricingMultiplier {
                team_id,
                base_multiplier: 10000,
                performance_multiplier: 10000,
                playoff_multiplier: 10000,
                streak_multiplier: 10000,
                rivalry_multiplier: 10000,
                demand_multiplier: 10000,
                final_multiplier: 10000,
                last_updated: self.env().block_timestamp(),
            };
            self.pricing_multipliers.insert(team_id, &pricing);
            
            Ok(team_id)
        }

        #[ink(message)]
        pub fn register_venue(
            &mut self,
            name: String,
            city: String,
            capacity: u32,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let venue_id = self.next_venue_id;
            self.next_venue_id = self.next_venue_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let venue = Venue {
                id: venue_id,
                name,
                city,
                capacity,
            };

            self.venues.insert(venue_id, &venue);
            self.venue_event_index.insert(venue_id, &Vec::<u32>::new()); // NEW: Initialize event index
            self.venue_revenue.insert(venue_id, &0); // NEW: Initialize revenue tracking
            
            Ok(venue_id)
        }

        // ========================================================================
        // SEASON MANAGEMENT (Step 3)
        // ========================================================================

        #[ink(message)]
        pub fn create_season(
            &mut self,
            name: String,
            sport_type: SportType,
            start_date: u64,
            end_date: u64,
            regular_season_games: u32,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let season_id = self.next_season_id;
            self.next_season_id = self.next_season_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let season = Season {
                id: season_id,
                name,
                sport_type,
                start_date,
                end_date,
                regular_season_games,
                active: true,
                season_pass_base_price: 500_000_000_000_000,
                early_bird_discount: 10,
                early_bird_deadline: start_date,
            };

            self.seasons.insert(season_id, &season);
            Ok(season_id)
        }

        // ========================================================================
        // SPORTS EVENT MANAGEMENT WITH SEARCH INDEXING (Step 4 + 10)
        // ========================================================================

        #[ink(message)]
        pub fn create_sports_event(
            &mut self,
            name: String,
            venue_id: u32,
            date: u64,
            capacity: u32,
            base_price: Balance,
            home_team_id: u32,
            away_team_id: u32,
            season_id: u32,
            game_type: GameType,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            let home_team = self.teams.get(home_team_id).ok_or(Error::TeamNotFound)?;
            let _away_team = self.teams.get(away_team_id).ok_or(Error::TeamNotFound)?;
            let _season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;

            let event_capacity = if capacity == 0 { venue.capacity } else { capacity };

            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let rivalry_multiplier = self.calculate_rivalry_multiplier(home_team_id, away_team_id);

            let sports_event = SportsEvent {
                id: event_id,
                name,
                venue_id,
                date,
                capacity: event_capacity,
                sold_tickets: 0,
                base_price,
                active: true,
                sport_type: home_team.sport_type.clone(),
                home_team_id,
                away_team_id,
                season_id,
                game_type,
                season_pass_discount: 15,
                dynamic_pricing_enabled: true,
                rivalry_multiplier,
                revenue_generated: 0, // NEW: Initialize revenue tracking
            };

            self.events.insert(event_id, &sports_event);
            self.update_event_pricing_multipliers(home_team_id, away_team_id, &game_type, rivalry_multiplier);

            // NEW: Update search indexes
            self.update_search_indexes(event_id, &sports_event);
            
            // NEW: Update platform stats
            self.platform_stats.total_events_created = self.platform_stats.total_events_created.saturating_add(1);

            Ok(event_id)
        }

        // ========================================================================
        // ENHANCED TICKET PURCHASING WITH REVENUE TRACKING (Step 5 + 9 + 10)
        // ========================================================================

        #[ink(message, payable)]
        pub fn purchase_sports_ticket_with_currency(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_type: SeatType,
            currency: CurrencyId,
        ) -> Result<u64> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            if !self.supported_currencies.contains(&currency) {
                return Err(Error::UnsupportedCurrency);
            }

            let mut event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            if !event.active {
                return Err(Error::EventNotActive);
            }
            if event.sold_tickets >= event.capacity {
                return Err(Error::EventSoldOut);
            }

            let (final_price_dot, season_pass_discount_applied, is_season_pass_ticket, performance_multiplier) = 
                self.calculate_comprehensive_ticket_price(buyer, &event, &seat_type)?;
            
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            
            if payment_in_dot < final_price_dot {
                return Err(Error::InsufficientPayment);
            }

            let loyalty_points_earned = self.calculate_loyalty_points(&seat_type, payment_in_dot);

            let ticket_id = self.next_ticket_id;
            self.next_ticket_id = self.next_ticket_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let seat_number = event.sold_tickets
                .checked_add(1)
                .ok_or(Error::InsufficientCapacity)?;

            let sports_ticket = SportsTicket {
                id: ticket_id,
                event_id,
                owner: buyer,
                purchase_price: payment,
                purchase_currency: currency,
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
                section,
                row,
                seat_type,
                access_level: self.determine_access_level(&seat_type),
                loyalty_points_earned,
                season_pass_discount_applied,
                is_season_pass_ticket,
                dynamic_price_paid: final_price_dot,
                performance_multiplier_applied: performance_multiplier,
                dot_equivalent_paid: payment_in_dot,
            };

            self.tickets.insert(ticket_id, &sports_ticket);

            let mut user_ticket_list = self.user_tickets.get(buyer).unwrap_or_default();
            user_ticket_list.push(ticket_id);
            self.user_tickets.insert(buyer, &user_ticket_list);

            // Update event sold count and revenue
            event.sold_tickets = seat_number;
            event.revenue_generated = event.revenue_generated.saturating_add(payment_in_dot);
            self.events.insert(event_id, &event);

            // NEW: Update revenue analytics
            self.update_revenue_analytics(currency, payment_in_dot, event.home_team_id, event.venue_id);
            
            // NEW: Update user spending analytics
            self.update_user_spending(buyer, payment_in_dot);

            self.award_loyalty_points(buyer, loyalty_points_earned);
            self.update_user_attendance(buyer, event.home_team_id);

            if is_season_pass_ticket {
                self.update_season_pass_usage(buyer, event.season_id, event.home_team_id);
            }

            self.update_demand_multiplier(event.home_team_id, event.sold_tickets, event.capacity);
            
            // NEW: Update platform statistics
            self.update_platform_stats_for_ticket_sale(payment_in_dot);

            Ok(ticket_id)
        }

        #[ink(message, payable)]
        pub fn purchase_sports_ticket(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_type: SeatType,
        ) -> Result<u64> {
            self.purchase_sports_ticket_with_currency(event_id, section, row, seat_type, CurrencyId::DOT)
        }

        // ========================================================================
        // SEASON PASS SYSTEM WITH REVENUE TRACKING (Step 7 + 9 + 10)
        // ========================================================================

        #[ink(message, payable)]
        pub fn purchase_season_pass_with_currency(
            &mut self,
            season_id: u32,
            team_id: u32,
            pass_type: SeasonPassType,
            currency: CurrencyId,
            enable_staking: bool,
        ) -> Result<u64> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            if !self.supported_currencies.contains(&currency) {
                return Err(Error::UnsupportedCurrency);
            }

            let season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;
            if !season.active {
                return Err(Error::SeasonNotActive);
            }

            let _team = self.teams.get(team_id).ok_or(Error::TeamNotFound)?;

            let final_price_dot = self.calculate_season_pass_price(&season, &pass_type, buyer)?;
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            
            if payment_in_dot < final_price_dot {
                return Err(Error::InsufficientPayment);
            }

            let pass_id = self.next_season_pass_id;
            self.next_season_pass_id = self.next_season_pass_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let games_included = self.calculate_games_included(&pass_type, season.regular_season_games);
            let loyalty_tier = self.get_user_loyalty_tier(buyer);

            let season_pass = SeasonPass {
                id: pass_id,
                owner: buyer,
                season_id,
                team_id,
                pass_type,
                purchase_price: payment,
                purchase_currency: currency,
                purchase_date: self.env().block_timestamp(),
                games_included,
                games_attended: 0,
                transferable: true,
                includes_playoffs: matches!(pass_type, SeasonPassType::Premium),
                priority_level: self.calculate_priority_level(&loyalty_tier),
                loyalty_tier_at_purchase: loyalty_tier,
                staking_rewards_enabled: enable_staking,
                staked_amount: if enable_staking { payment_in_dot } else { 0 },
                valid_until: season.end_date,
                dot_equivalent_paid: payment_in_dot,
            };

            if enable_staking {
                self.total_staked_amount = self.total_staked_amount.saturating_add(payment_in_dot);
                let current_staked = self.user_staked_amounts.get(buyer).unwrap_or(0);
                self.user_staked_amounts.insert(buyer, &(current_staked + payment_in_dot));
                self.last_staking_update.insert(buyer, &self.env().block_timestamp());
            }

            self.season_passes.insert(pass_id, &season_pass);

            let mut user_passes = self.user_season_passes.get(buyer).unwrap_or_default();
            user_passes.push(pass_id);
            self.user_season_passes.insert(buyer, &user_passes);

            // NEW: Update revenue analytics for season pass
            self.update_revenue_analytics(currency, payment_in_dot, team_id, 0);
            self.update_user_spending(buyer, payment_in_dot);

            self.update_user_profile_for_season_pass(buyer, team_id);
            let loyalty_points_earned = self.calculate_season_pass_loyalty_points(&pass_type);
            self.award_loyalty_points(buyer, loyalty_points_earned);

            // NEW: Update platform stats
            self.platform_stats.total_season_passes_sold = self.platform_stats.total_season_passes_sold.saturating_add(1);

            Ok(pass_id)
        }

        #[ink(message, payable)]
        pub fn purchase_season_pass(
            &mut self,
            season_id: u32,
            team_id: u32,
            pass_type: SeasonPassType,
            enable_staking: bool,
        ) -> Result<u64> {
            self.purchase_season_pass_with_currency(season_id, team_id, pass_type, CurrencyId::DOT, enable_staking)
        }

        // ========================================================================
        // MULTI-CURRENCY MANAGEMENT (Step 9)
        // ========================================================================

        #[ink(message)]
        pub fn update_currency_rate(
            &mut self,
            currency: CurrencyId,
            rate_to_dot: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if rate_to_dot == 0 {
                return Err(Error::InvalidCurrencyRate);
            }

            self.currency_rates.insert(currency, &rate_to_dot);
            Ok(())
        }

        #[ink(message)]
        pub fn add_supported_currency(
            &mut self,
            currency: CurrencyId,
            rate_to_dot: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if !self.supported_currencies.contains(&currency) {
                self.supported_currencies.push(currency);
                self.currency_rates.insert(currency, &rate_to_dot);
                self.currency_revenue.insert(currency, &0); // NEW: Initialize revenue tracking
            }

            Ok(())
        }

        fn convert_to_dot_equivalent(
            &self,
            amount: Balance,
            currency: CurrencyId,
        ) -> Result<Balance> {
            match currency {
                CurrencyId::DOT => Ok(amount),
                _ => {
                    let rate = self.currency_rates.get(currency)
                        .ok_or(Error::UnsupportedCurrency)?;
                    
                    let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000;
                    if dot_amount == 0 && amount > 0 {
                        return Err(Error::CurrencyConversionFailed);
                    }
                    Ok(dot_amount)
                }
            }
        }

        fn convert_from_dot_equivalent(
            &self,
            dot_amount: Balance,
            target_currency: CurrencyId,
        ) -> Result<Balance> {
            match target_currency {
                CurrencyId::DOT => Ok(dot_amount),
                _ => {
                    let rate = self.currency_rates.get(target_currency)
                        .ok_or(Error::UnsupportedCurrency)?;
                    
                    if rate == 0 {
                        return Err(Error::CurrencyConversionFailed);
                    }
                    
                    let target_amount = dot_amount.saturating_mul(1_000_000_000_000) / rate;
                    Ok(target_amount)
                }
            }
        }

        // ========================================================================
        // STAKING REWARDS SYSTEM (Step 9)
        // ========================================================================

        #[ink(message)]
        pub fn claim_staking_rewards(&mut self) -> Result<Balance> {
            let caller = self.env().caller();
            
            let staked_amount = self.user_staked_amounts.get(caller).unwrap_or(0);
            if staked_amount == 0 {
                return Err(Error::StakingNotEnabled);
            }

            let last_claim = self.last_staking_update.get(caller).unwrap_or(0);
            let current_time = self.env().block_timestamp();
            
            if current_time < last_claim + (24 * 60 * 60 * 1000) {
                return Err(Error::StakingRewardsNotReady);
            }

            let annual_rate: Balance = 800;
            let seconds_per_year: Balance = 31_536_000_000;
            
            let time_elapsed = current_time - last_claim;
            let rewards = (staked_amount * annual_rate * time_elapsed as Balance) / (10000 * seconds_per_year);

            if rewards > self.staking_rewards_pool {
                return Err(Error::InsufficientStakingRewards);
            }

            self.staking_rewards_pool = self.staking_rewards_pool.saturating_sub(rewards);
            self.last_staking_update.insert(caller, &current_time);

            Ok(rewards)
        }

        #[ink(message, payable)]
        pub fn add_staking_rewards(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let amount = self.env().transferred_value();
            self.staking_rewards_pool = self.staking_rewards_pool.saturating_add(amount);
            Ok(())
        }

        #[ink(message)]
        pub fn get_estimated_staking_rewards(&self, user: AccountId) -> Balance {
            let staked_amount = self.user_staked_amounts.get(user).unwrap_or(0);
            if staked_amount == 0 {
                return 0;
            }

            let last_claim = self.last_staking_update.get(user).unwrap_or(0);
            let current_time = self.env().block_timestamp();
            
            if current_time <= last_claim {
                return 0;
            }

            let annual_rate: Balance = 800;
            let seconds_per_year: Balance = 31_536_000_000;
            let time_elapsed = current_time - last_claim;
            
            (staked_amount * annual_rate * time_elapsed as Balance) / (10000 * seconds_per_year)
        }

        // ========================================================================
        // NEW: ADVANCED SEARCH & DISCOVERY FUNCTIONS (Step 10)
        // ========================================================================

        /// Search events by sport type
        #[ink(message)]
        pub fn search_events_by_sport(&self, sport_type: SportType) -> Vec<u32> {
            let sport_hash = self.hash_sport_type(&sport_type);
            self.event_search_index.get(sport_hash).unwrap_or_default()
        }

        /// Search events by team (home or away)
        #[ink(message)]
        pub fn search_events_by_team(&self, team_id: u32) -> Vec<u32> {
            self.team_event_index.get(team_id).unwrap_or_default()
        }

        /// Search events by venue
        #[ink(message)]
        pub fn search_events_by_venue(&self, venue_id: u32) -> Vec<u32> {
            self.venue_event_index.get(venue_id).unwrap_or_default()
        }

        /// Search events by date range
        #[ink(message)]
        pub fn search_events_by_date_range(&self, start_date: u64, end_date: u64) -> Result<Vec<u32>> {
            if start_date > end_date {
                return Err(Error::InvalidDateRange);
            }

            let mut results = Vec::new();
            
            // Simple linear search through events (in production, would use more efficient indexing)
            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    if event.date >= start_date && event.date <= end_date {
                        results.push(event_id);
                    }
                }
            }

            Ok(results)
        }

        /// Advanced search with multiple filters
        #[ink(message)]
        pub fn search_events_advanced(&self, filter: EventSearchFilter) -> Result<Vec<u32>> {
            let mut results = Vec::new();

            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    // Apply filters
                    if filter.active_only && !event.active {
                        continue;
                    }

                    if let Some(sport) = &filter.sport_type {
                        if &event.sport_type != sport {
                            continue;
                        }
                    }

                    if let Some(team_id) = filter.team_id {
                        if event.home_team_id != team_id && event.away_team_id != team_id {
                            continue;
                        }
                    }

                    if let Some(venue_id) = filter.venue_id {
                        if event.venue_id != venue_id {
                            continue;
                        }
                    }

                    if let Some(min_date) = filter.min_date {
                        if event.date < min_date {
                            continue;
                        }
                    }

                    if let Some(max_date) = filter.max_date {
                        if event.date > max_date {
                            continue;
                        }
                    }

                    if let Some(game_type) = filter.game_type {
                        if event.game_type != game_type {
                            continue;
                        }
                    }

                    if let Some(max_price) = filter.max_price {
                        if event.base_price > max_price {
                            continue;
                        }
                    }

                    if let Some(min_availability) = filter.min_availability {
                        let available = event.capacity.saturating_sub(event.sold_tickets);
                        if available < min_availability {
                            continue;
                        }
                    }

                    results.push(event_id);
                }
            }

            if results.is_empty() {
                return Err(Error::NoSearchResults);
            }

            Ok(results)
        }

        /// Get recommended events for user based on preferences
        #[ink(message)]
        pub fn get_recommended_events(&self, user: AccountId, limit: u32) -> Vec<u32> {
            let mut recommendations = Vec::new();

            if let Some(profile) = self.user_profiles.get(user) {
                // Find events for user's favorite teams
                for team_id in &profile.favorite_teams {
                    let team_events = self.search_events_by_team(*team_id);
                    for event_id in team_events {
                        if let Some(event) = self.events.get(event_id) {
                            if event.active && recommendations.len() < limit as usize {
                                recommendations.push(event_id);
                            }
                        }
                    }
                }
            }

            recommendations
        }

        // ========================================================================
        // NEW: ANALYTICS & REPORTING FUNCTIONS (Step 10)
        // ========================================================================

        /// Generate comprehensive analytics report
        #[ink(message)]
        pub fn generate_analytics_report(&self) -> AnalyticsReport {
            let mut revenue_by_currency = Vec::new();
            for currency in &self.supported_currencies {
                let revenue = self.currency_revenue.get(*currency).unwrap_or(0);
                revenue_by_currency.push((*currency, revenue));
            }

            let mut top_teams_by_revenue = Vec::new();
            for team_id in 1..self.next_team_id {
                let revenue = self.team_revenue.get(team_id).unwrap_or(0);
                if revenue > 0 {
                    top_teams_by_revenue.push((team_id, revenue));
                }
            }
            
            // Sort by revenue (descending)
            top_teams_by_revenue.sort_by(|a, b| b.1.cmp(&a.1));
            top_teams_by_revenue.truncate(10); // Top 10

            let mut top_venues_by_revenue = Vec::new();
            for venue_id in 1..self.next_venue_id {
                let revenue = self.venue_revenue.get(venue_id).unwrap_or(0);
                if revenue > 0 {
                    top_venues_by_revenue.push((venue_id, revenue));
                }
            }
            
            // Sort by revenue (descending)
            top_venues_by_revenue.sort_by(|a, b| b.1.cmp(&a.1));
            top_venues_by_revenue.truncate(10); // Top 10

            AnalyticsReport {
                total_revenue: self.total_revenue,
                revenue_by_currency,
                top_teams_by_revenue,
                top_venues_by_revenue,
                average_ticket_price: self.platform_stats.average_ticket_price,
                total_tickets_sold: self.platform_stats.total_tickets_sold,
                total_season_passes_sold: self.platform_stats.total_season_passes_sold,
                user_count: self.platform_stats.total_users,
                most_popular_sport: self.platform_stats.most_popular_sport.clone(),
                report_generated_at: self.env().block_timestamp(),
            }
        }

        /// Get revenue analytics for specific team
        #[ink(message)]
        pub fn get_team_revenue_analytics(&self, team_id: u32) -> Option<Balance> {
            self.team_revenue.get(team_id)
        }

        /// Get revenue analytics for specific venue
        #[ink(message)]
        pub fn get_venue_revenue_analytics(&self, venue_id: u32) -> Option<Balance> {
            self.venue_revenue.get(venue_id)
        }

        /// Get platform performance statistics
        #[ink(message)]
        pub fn get_platform_stats(&self) -> PlatformStats {
            self.platform_stats.clone()
        }

        /// Get user spending analytics
        #[ink(message)]
        pub fn get_user_spending_analytics(&self, user: AccountId) -> Balance {
            if let Some(profile) = self.user_profiles.get(user) {
                profile.total_spent
            } else {
                0
            }
        }

        /// Get event attendance rate
        #[ink(message)]
        pub fn get_event_attendance_rate(&self, event_id: u32) -> Option<u32> {
            if let Some(event) = self.events.get(event_id) {
                if event.capacity > 0 {
                    Some((event.sold_tickets * 100) / event.capacity)
                } else {
                    Some(0)
                }
            } else {
                None
            }
        }

        /// Get top performing events by revenue
        #[ink(message)]
        pub fn get_top_events_by_revenue(&self, limit: u32) -> Vec<(u32, Balance)> {
            let mut events_revenue = Vec::new();
            
            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    if event.revenue_generated > 0 {
                        events_revenue.push((event_id, event.revenue_generated));
                    }
                }
            }
            
            // Sort by revenue (descending)
            events_revenue.sort_by(|a, b| b.1.cmp(&a.1));
            events_revenue.truncate(limit as usize);
            
            events_revenue
        }

        // ========================================================================
        // NEW: HELPER METHODS FOR SEARCH & ANALYTICS (Step 10)
        // ========================================================================

        /// Update search indexes when creating events
        fn update_search_indexes(&mut self, event_id: u32, event: &SportsEvent) {
            // Update sport type index
            let sport_hash = self.hash_sport_type(&event.sport_type);
            let mut sport_events = self.event_search_index.get(sport_hash).unwrap_or_default();
            sport_events.push(event_id);
            self.event_search_index.insert(sport_hash, &sport_events);

            // Update team indexes (both home and away)
            let mut home_team_events = self.team_event_index.get(event.home_team_id).unwrap_or_default();
            home_team_events.push(event_id);
            self.team_event_index.insert(event.home_team_id, &home_team_events);

            let mut away_team_events = self.team_event_index.get(event.away_team_id).unwrap_or_default();
            away_team_events.push(event_id);
            self.team_event_index.insert(event.away_team_id, &away_team_events);

            // Update venue index
            let mut venue_events = self.venue_event_index.get(event.venue_id).unwrap_or_default();
            venue_events.push(event_id);
            self.venue_event_index.insert(event.venue_id, &venue_events);

            // Update date index (bucket by day)
            let date_bucket = event.date / (24 * 60 * 60 * 1000); // Days since epoch
            let mut date_events = self.date_event_index.get(date_bucket).unwrap_or_default();
            date_events.push(event_id);
            self.date_event_index.insert(date_bucket, &date_events);
        }

        /// Update revenue analytics across the platform
        fn update_revenue_analytics(&mut self, currency: CurrencyId, amount_dot: Balance, team_id: u32, venue_id: u32) {
            // Update total revenue
            self.total_revenue = self.total_revenue.saturating_add(amount_dot);

            // Update currency-specific revenue
            let current_currency_revenue = self.currency_revenue.get(currency).unwrap_or(0);
            self.currency_revenue.insert(currency, &(current_currency_revenue + amount_dot));

            // Update team revenue
            let current_team_revenue = self.team_revenue.get(team_id).unwrap_or(0);
            self.team_revenue.insert(team_id, &(current_team_revenue + amount_dot));

            // Update venue revenue (if venue_id > 0)
            if venue_id > 0 {
                let current_venue_revenue = self.venue_revenue.get(venue_id).unwrap_or(0);
                self.venue_revenue.insert(venue_id, &(current_venue_revenue + amount_dot));
            }
        }

        /// Update user spending analytics
        fn update_user_spending(&mut self, user: AccountId, amount_dot: Balance) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.total_spent = profile.total_spent.saturating_add(amount_dot);
                self.user_profiles.insert(user, &profile);
            }
        }

        /// Update platform statistics for ticket sales
        fn update_platform_stats_for_ticket_sale(&mut self, price_dot: Balance) {
            self.platform_stats.total_tickets_sold = self.platform_stats.total_tickets_sold.saturating_add(1);
            
            // Update average ticket price
            if self.platform_stats.total_tickets_sold > 0 {
                let total_revenue = self.platform_stats.average_ticket_price * (self.platform_stats.total_tickets_sold - 1) as Balance;
                self.platform_stats.average_ticket_price = (total_revenue + price_dot) / self.platform_stats.total_tickets_sold as Balance;
            } else {
                self.platform_stats.average_ticket_price = price_dot;
            }
            
            self.platform_stats.last_updated = self.env().block_timestamp();
        }

        /// Simple hash function for sport types (for indexing)
        fn hash_sport_type(&self, sport_type: &SportType) -> u32 {
            match sport_type {
                SportType::Basketball => 1,
                SportType::Football => 2,
                SportType::Soccer => 3,
                SportType::Baseball => 4,
                SportType::Hockey => 5,
                SportType::Tennis => 6,
                SportType::Other(_) => 7,
            }
        }

        // ========================================================================
        // USER PROFILE MANAGEMENT WITH ANALYTICS (Step 6 + 9 + 10)
        // ========================================================================

        #[ink(message)]
        pub fn create_user_profile_with_currency(
            &mut self,
            favorite_teams: Vec<u32>,
            home_city: String,
            preferred_currency: CurrencyId,
        ) -> Result<()> {
            let caller = self.env().caller();

            if self.user_profiles.get(caller).is_some() {
                return Err(Error::ProfileAlreadyExists);
            }

            for team_id in &favorite_teams {
                if self.teams.get(*team_id).is_none() {
                    return Err(Error::InvalidFavoriteTeams);
                }
            }

            if !self.supported_currencies.contains(&preferred_currency) {
                return Err(Error::UnsupportedCurrency);
            }

            let profile = UserProfile {
                verified_fan: false,
                favorite_teams: favorite_teams.clone(),
                home_city,
                loyalty_tier: LoyaltyTier::Bronze,
                total_games_attended: 0,
                account_creation_date: self.env().block_timestamp(),
                anti_scalping_verified: false,
                social_media_verified: false,
                season_pass_holder: false,
                preferred_currency,
                total_spent: 0, // NEW: Initialize spending tracking
            };

            self.user_profiles.insert(caller, &profile);

            for team_id in &favorite_teams {
                let mut fans = self.team_fans.get(*team_id).unwrap_or_default();
                if !fans.contains(&caller) {
                    fans.push(caller);
                    self.team_fans.insert(*team_id, &fans);
                }
            }

            // NEW: Update platform stats
            self.platform_stats.total_users = self.platform_stats.total_users.saturating_add(1);

            Ok(())
        }

        #[ink(message)]
        pub fn create_user_profile(
            &mut self,
            favorite_teams: Vec<u32>,
            home_city: String,
        ) -> Result<()> {
            self.create_user_profile_with_currency(favorite_teams, home_city, CurrencyId::DOT)
        }

        #[ink(message)]
        pub fn update_preferred_currency(&mut self, currency: CurrencyId) -> Result<()> {
            let caller = self.env().caller();
            
            if !self.supported_currencies.contains(&currency) {
                return Err(Error::UnsupportedCurrency);
            }

            let mut profile = self.user_profiles.get(caller).ok_or(Error::ProfileNotFound)?;
            profile.preferred_currency = currency;
            self.user_profiles.insert(caller, &profile);

            Ok(())
        }

        // ========================================================================
        // HELPER METHODS (Enhanced for All Features)
        // ========================================================================

        fn calculate_comprehensive_ticket_price(
            &self,
            buyer: AccountId,
            event: &SportsEvent,
            seat_type: &SeatType,
        ) -> Result<(Balance, bool, bool, u32)> {
            let mut final_price = self.calculate_seat_price(event.base_price, seat_type);
            let mut performance_multiplier = 10000;

            if event.dynamic_pricing_enabled {
                if let Some(pricing) = self.pricing_multipliers.get(event.home_team_id) {
                    performance_multiplier = pricing.final_multiplier;
                    final_price = (final_price * pricing.final_multiplier as Balance) / 10000;
                }
            }

            let (discounted_price, season_pass_discount_applied, is_season_pass_ticket) = 
                self.apply_season_pass_discount(buyer, event, final_price)?;

            Ok((discounted_price, season_pass_discount_applied, is_season_pass_ticket, performance_multiplier))
        }

        fn calculate_rivalry_multiplier(&self, home_team_id: u32, away_team_id: u32) -> u32 {
            if let (Some(home_team), Some(away_team)) = (self.teams.get(home_team_id), self.teams.get(away_team_id)) {
                if home_team.city == away_team.city {
                    return 12000;
                }
                
                match (home_team.name.as_str(), away_team.name.as_str()) {
                    ("Lakers", "Celtics") | ("Celtics", "Lakers") => 15000,
                    ("Yankees", "Red Sox") | ("Red Sox", "Yankees") => 15000,
                    _ => 10000,
                }
            } else {
                10000
            }
        }

        fn calculate_season_pass_price(
            &self,
            season: &Season,
            pass_type: &SeasonPassType,
            buyer: AccountId,
        ) -> Result<Balance> {
            let mut final_price = season.season_pass_base_price;

            let type_multiplier = match pass_type {
                SeasonPassType::FullSeason => 100,
                SeasonPassType::HalfSeason => 55,
                SeasonPassType::QuarterSeason => 30,
                SeasonPassType::Weekend => 60,
                SeasonPassType::Weekday => 45,
                SeasonPassType::Premium => 150,
                SeasonPassType::PlayoffsOnly => 80,
                SeasonPassType::Package(_) => 75,
            };
            
            final_price = (final_price * type_multiplier) / 100;

            let current_time = self.env().block_timestamp();
            if current_time <= season.early_bird_deadline {
                let discount_amount = (final_price * season.early_bird_discount as Balance) / 100;
                final_price = final_price.saturating_sub(discount_amount);
            }

            let loyalty_tier = self.get_user_loyalty_tier(buyer);
            let loyalty_discount = match loyalty_tier {
                LoyaltyTier::Bronze => 0,
                LoyaltyTier::Silver => 5,
                LoyaltyTier::Gold => 10,
                LoyaltyTier::Platinum => 15,
                LoyaltyTier::Diamond => 20,
            };

            if loyalty_discount > 0 {
                let discount_amount = (final_price * loyalty_discount) / 100;
                final_price = final_price.saturating_sub(discount_amount);
            }

            Ok(final_price)
        }

        fn apply_season_pass_discount(
            &self,
            buyer: AccountId,
            event: &SportsEvent,
            base_price: Balance,
        ) -> Result<(Balance, bool, bool)> {
            if let Some(user_passes) = self.user_season_passes.get(buyer) {
                for pass_id in user_passes {
                    if let Some(season_pass) = self.season_passes.get(pass_id) {
                        if season_pass.team_id == event.home_team_id 
                            && season_pass.season_id == event.season_id
                            && season_pass.games_attended < season_pass.games_included
                            && self.env().block_timestamp() <= season_pass.valid_until {
                            
                            let discount_amount = (base_price * event.season_pass_discount as Balance) / 100;
                            let final_price = base_price.saturating_sub(discount_amount);
                            
                            return Ok((final_price, true, true));
                        }
                    }
                }
            }
            
            Ok((base_price, false, false))
        }

        fn calculate_seat_price(&self, base_price: Balance, seat_type: &SeatType) -> Balance {
            let multiplier = match seat_type {
                SeatType::GeneralAdmission => 100,
                SeatType::Reserved => 120,
                SeatType::PremiumReserved => 150,
                SeatType::Club => 200,
                SeatType::Suite => 500,
                SeatType::FieldLevel => 300,
                SeatType::Courtside => 800,
                SeatType::StudentSection => 50,
            };
            
            (base_price * multiplier) / 100
        }

        fn determine_access_level(&self, seat_type: &SeatType) -> AccessLevel {
            match seat_type {
                SeatType::GeneralAdmission | SeatType::StudentSection => AccessLevel::Standard,
                SeatType::Reserved | SeatType::PremiumReserved => AccessLevel::Premium,
                SeatType::Club | SeatType::FieldLevel => AccessLevel::VIP,
                SeatType::Suite | SeatType::Courtside => AccessLevel::AllAccess,
            }
        }

        fn calculate_loyalty_points(&self, seat_type: &SeatType, price: Balance) -> u32 {
            let base_points = match seat_type {
                SeatType::GeneralAdmission => 10,
                SeatType::Reserved => 15,
                SeatType::PremiumReserved => 25,
                SeatType::Club => 50,
                SeatType::Suite => 100,
                SeatType::FieldLevel => 75,
                SeatType::Courtside => 200,
                SeatType::StudentSection => 5,
            };

            let price_bonus = (price / 10_000_000_000) as u32;
            base_points + price_bonus
        }

        fn calculate_games_included(&self, pass_type: &SeasonPassType, total_games: u32) -> u32 {
            match pass_type {
                SeasonPassType::FullSeason => total_games,
                SeasonPassType::HalfSeason => total_games / 2,
                SeasonPassType::QuarterSeason => total_games / 4,
                SeasonPassType::Weekend => total_games * 40 / 100,
                SeasonPassType::Weekday => total_games * 60 / 100,
                SeasonPassType::Premium => total_games + 4,
                SeasonPassType::PlayoffsOnly => 16,
                SeasonPassType::Package(games) => *games,
            }
        }

        fn calculate_season_pass_loyalty_points(&self, pass_type: &SeasonPassType) -> u32 {
            match pass_type {
                SeasonPassType::FullSeason => 1000,
                SeasonPassType::HalfSeason => 500,
                SeasonPassType::QuarterSeason => 250,
                SeasonPassType::Weekend => 300,
                SeasonPassType::Weekday => 200,
                SeasonPassType::Premium => 1500,
                SeasonPassType::PlayoffsOnly => 400,
                SeasonPassType::Package(games) => *games * 10,
            }
        }

        fn get_user_loyalty_tier(&self, user: AccountId) -> LoyaltyTier {
            if let Some(profile) = self.user_profiles.get(user) {
                profile.loyalty_tier
            } else {
                LoyaltyTier::Bronze
            }
        }

        fn calculate_priority_level(&self, tier: &LoyaltyTier) -> u8 {
            match tier {
                LoyaltyTier::Bronze => 1,
                LoyaltyTier::Silver => 2,
                LoyaltyTier::Gold => 3,
                LoyaltyTier::Platinum => 4,
                LoyaltyTier::Diamond => 5,
            }
        }

        fn update_user_profile_for_season_pass(&mut self, user: AccountId, team_id: u32) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.season_pass_holder = true;
                if !profile.favorite_teams.contains(&team_id) {
                    profile.favorite_teams.push(team_id);
                }
                self.user_profiles.insert(user, &profile);
            }
        }

        fn award_loyalty_points(&mut self, user: AccountId, points: u32) {
            let current_points = self.loyalty_points.get(user).unwrap_or(0);
            let new_total = current_points.saturating_add(points);
            self.loyalty_points.insert(user, &new_total);

            let new_tier = self.calculate_loyalty_tier(new_total);

            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.loyalty_tier = new_tier;
                self.user_profiles.insert(user, &profile);
            }
        }

        fn calculate_loyalty_tier(&self, total_points: u32) -> LoyaltyTier {
            match total_points {
                0..=999 => LoyaltyTier::Bronze,
                1000..=2999 => LoyaltyTier::Silver,
                3000..=6999 => LoyaltyTier::Gold,
                7000..=14999 => LoyaltyTier::Platinum,
                15000.. => LoyaltyTier::Diamond,
            }
        }

        fn update_user_attendance(&mut self, user: AccountId, _team_id: u32) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.total_games_attended = profile.total_games_attended.saturating_add(1);
                self.user_profiles.insert(user, &profile);
            }
        }

        fn update_season_pass_usage(&mut self, user: AccountId, season_id: u32, team_id: u32) {
            if let Some(user_passes) = self.user_season_passes.get(user) {
                for pass_id in user_passes {
                    if let Some(mut season_pass) = self.season_passes.get(pass_id) {
                        if season_pass.team_id == team_id && season_pass.season_id == season_id {
                            season_pass.games_attended = season_pass.games_attended.saturating_add(1);
                            self.season_passes.insert(pass_id, &season_pass);
                            break;
                        }
                    }
                }
            }
        }

        fn update_event_pricing_multipliers(&mut self, home_team_id: u32, away_team_id: u32, game_type: &GameType, rivalry_multiplier: u32) {
            if let Some(mut home_pricing) = self.pricing_multipliers.get(home_team_id) {
                home_pricing.rivalry_multiplier = rivalry_multiplier;
                
                home_pricing.base_multiplier = match game_type {
                    GameType::RegularSeason => 10000,
                    GameType::Playoff => 15000,
                    GameType::Championship => 25000,
                    GameType::AllStar => 20000,
                    GameType::Preseason => 7500,
                    GameType::Tournament => 18000,
                    GameType::Exhibition => 8000,
                };

                self.recalculate_final_multiplier(&mut home_pricing);
                self.pricing_multipliers.insert(home_team_id, &home_pricing);
            }

            if let Some(mut away_pricing) = self.pricing_multipliers.get(away_team_id) {
                away_pricing.rivalry_multiplier = (rivalry_multiplier + 10000) / 2;
                self.recalculate_final_multiplier(&mut away_pricing);
                self.pricing_multipliers.insert(away_team_id, &away_pricing);
            }
        }

        fn recalculate_final_multiplier(&self, pricing: &mut PricingMultiplier) {
            let temp1 = (pricing.base_multiplier as Balance * pricing.performance_multiplier as Balance) / 10000;
            let temp2 = (temp1 * pricing.playoff_multiplier as Balance) / 10000;
            let temp3 = (temp2 * pricing.streak_multiplier as Balance) / 10000;
            let temp4 = (temp3 * pricing.rivalry_multiplier as Balance) / 10000;
            let final_result = (temp4 * pricing.demand_multiplier as Balance) / 10000;
            
            pricing.final_multiplier = final_result as u32;
        }

        fn update_demand_multiplier(&mut self, team_id: u32, sold_tickets: u32, capacity: u32) {
            if let Some(mut pricing) = self.pricing_multipliers.get(team_id) {
                let sell_through_percentage = (sold_tickets * 100) / capacity;
                
                pricing.demand_multiplier = match sell_through_percentage {
                    90.. => 13000,
                    75..=89 => 11500,
                    50..=74 => 10000,
                    25..=49 => 9500,
                    _       => 9000,
                };

                self.recalculate_final_multiplier(&mut pricing);
                pricing.last_updated = self.env().block_timestamp();
                self.pricing_multipliers.insert(team_id, &pricing);
            }
        }

        // ========================================================================
        // ENHANCED QUERY METHODS (All Steps Combined)
        // ========================================================================

        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.supported_currencies.clone()
        }

        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<Balance> {
            self.currency_rates.get(currency)
        }

        #[ink(message)]
        pub fn get_user_staked_amount(&self, user: AccountId) -> Balance {
            self.user_staked_amounts.get(user).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_staking_rewards_pool(&self) -> Balance {
            self.staking_rewards_pool
        }

        #[ink(message)]
        pub fn get_total_staked_amount(&self) -> Balance {
            self.total_staked_amount
        }

        #[ink(message)]
        pub fn get_current_ticket_price_in_currency(
            &self,
            event_id: u32,
            seat_type: SeatType,
            user: AccountId,
            currency: CurrencyId,
        ) -> Result<Balance> {
            let event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            let (price_dot, _, _, _) = self.calculate_comprehensive_ticket_price(user, &event, &seat_type)?;
            self.convert_from_dot_equivalent(price_dot, currency)
        }

        #[ink(message)]
        pub fn get_season_pass_price_in_currency(
            &self,
            season_id: u32,
            pass_type: SeasonPassType,
            user: AccountId,
            currency: CurrencyId,
        ) -> Result<Balance> {
            let season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;
            let price_dot = self.calculate_season_pass_price(&season, &pass_type, user)?;
            self.convert_from_dot_equivalent(price_dot, currency)
        }

        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.teams.get(team_id)
        }

        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        #[ink(message)]
        pub fn get_season(&self, season_id: u32) -> Option<Season> {
            self.seasons.get(season_id)
        }

        #[ink(message)]
        pub fn get_sports_event(&self, event_id: u32) -> Option<SportsEvent> {
            self.events.get(event_id)
        }

        #[ink(message)]
        pub fn get_sports_ticket(&self, ticket_id: u64) -> Option<SportsTicket> {
            self.tickets.get(ticket_id)
        }

        #[ink(message)]
        pub fn get_season_pass(&self, pass_id: u64) -> Option<SeasonPass> {
            self.season_passes.get(pass_id)
        }

        #[ink(message)]
        pub fn get_user_profile(&self, user: AccountId) -> Option<UserProfile> {
            self.user_profiles.get(user)
        }

        #[ink(message)]
        pub fn get_user_tickets(&self, user: AccountId) -> Vec<u64> {
            self.user_tickets.get(user).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_user_season_passes(&self, user: AccountId) -> Vec<u64> {
            self.user_season_passes.get(user).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_user_loyalty_points(&self, user: AccountId) -> u32 {
            self.loyalty_points.get(user).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_team_performance(&self, team_id: u32) -> Option<TeamPerformance> {
            self.team_performance.get(team_id)
        }

        #[ink(message)]
        pub fn get_pricing_multiplier(&self, team_id: u32) -> Option<PricingMultiplier> {
            self.pricing_multipliers.get(team_id)
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn total_teams(&self) -> u32 {
            self.next_team_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_venues(&self) -> u32 {
            self.next_venue_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_seasons(&self) -> u32 {
            self.next_season_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_tickets(&self) -> u64 {
            self.next_ticket_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_season_passes(&self) -> u64 {
            self.next_season_pass_id.saturating_sub(1)
        }

        /// NEW: Get total platform revenue
        #[ink(message)]
        pub fn get_total_revenue(&self) -> Balance {
            self.total_revenue
        }
    }

    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    // ========================================================================
    // COMPREHENSIVE TEST SUITE - STEP 10 SEARCH & ANALYTICS
    // ========================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn setup_test_data(contract: &mut SportsBroker) -> (u32, u32, u32, u32, u32) {
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                "New York".to_string(),
                20000,
            ).unwrap();

            let home_team_id = contract.register_team(
                "New York Knicks".to_string(),
                "New York".to_string(),
                SportType::Basketball,
            ).unwrap();

            let away_team_id = contract.register_team(
                "Boston Celtics".to_string(),
                "Boston".to_string(),
                SportType::Basketball,
            ).unwrap();

            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            ).unwrap();

            let event_id = contract.create_sports_event(
                "Knicks vs Celtics".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            (venue_id, home_team_id, away_team_id, season_id, event_id)
        }

        // ========================================================================
        // REGRESSION TESTS (Steps 1-9 Still Work)
        // ========================================================================

        #[ink::test]
        fn complete_platform_initialization_works() {
            let sports_broker = SportsBroker::new();
            
            // Core functionality
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
            
            // Multi-currency support
            let currencies = sports_broker.get_supported_currencies();
            assert_eq!(currencies.len(), 5);
            
            // NEW: Analytics initialization
            let platform_stats = sports_broker.get_platform_stats();
            assert_eq!(platform_stats.total_users, 0);
            assert_eq!(platform_stats.total_tickets_sold, 0);
            assert_eq!(platform_stats.total_events_created, 0);
            assert_eq!(sports_broker.get_total_revenue(), 0);
        }

        #[ink::test]
        fn multi_currency_functionality_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);

            let ticket_id = sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert_eq!(ticket.purchase_currency, CurrencyId::ACA);
            assert!(ticket.dot_equivalent_paid > 0);
            
            // NEW: Verify revenue tracking
            assert!(sports_broker.get_total_revenue() > 0);
        }

        // ========================================================================
        // NEW: STEP 10 - SEARCH & DISCOVERY TESTS
        // ========================================================================

        #[ink::test]
        fn search_events_by_sport_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            // Create multiple basketball events
            sports_broker.create_sports_event(
                "Basketball Game 1".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            sports_broker.create_sports_event(
                "Basketball Game 2".to_string(),
                venue_id,
                1704153600000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Create a football team and event
            let football_team_id = sports_broker.register_team(
                "Giants".to_string(),
                "New York".to_string(),
                SportType::Football,
            ).unwrap();

            sports_broker.create_sports_event(
                "Football Game".to_string(),
                venue_id,
                1704240000000,
                18000,
                60_000_000_000_000,
                football_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Search for basketball events
            let basketball_events = sports_broker.search_events_by_sport(SportType::Basketball);
            assert_eq!(basketball_events.len(), 3); // Initial + 2 new

            // Search for football events
            let football_events = sports_broker.search_events_by_sport(SportType::Football);
            assert_eq!(football_events.len(), 1);
        }

        #[ink::test]
        fn search_events_by_team_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            // Create multiple events for the home team
            sports_broker.create_sports_event(
                "Home Game 1".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            sports_broker.create_sports_event(
                "Away Game".to_string(),
                venue_id,
                1704153600000,
                18000,
                50_000_000_000_000,
                away_team_id,
                home_team_id, // home_team as away team
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let team_events = sports_broker.search_events_by_team(home_team_id);
            assert_eq!(team_events.len(), 3); // Initial + home + away games
        }

        #[ink::test]
        fn search_events_by_venue_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            // Create multiple events at the same venue
            sports_broker.create_sports_event(
                "Venue Game 1".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            sports_broker.create_sports_event(
                "Venue Game 2".to_string(),
                venue_id,
                1704153600000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            let venue_events = sports_broker.search_events_by_venue(venue_id);
            assert_eq!(venue_events.len(), 3); // Initial + 2 new
        }

        #[ink::test]
        fn search_events_by_date_range_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            // Create events on different dates
            sports_broker.create_sports_event(
                "Early Game".to_string(),
                venue_id,
                1704067200000, // Early date
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            sports_broker.create_sports_event(
                "Late Game".to_string(),
                venue_id,
                1704326400000, // Later date
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Search in date range
            let events_in_range = sports_broker.search_events_by_date_range(
                1704000000000, // Start range
                1704200000000, // End range
            ).unwrap();

            assert!(events_in_range.len() >= 2);
        }

        #[ink::test]
        fn search_events_by_date_range_invalid() {
            let sports_broker = SportsBroker::new();

            // Invalid date range (start > end)
            let result = sports_broker.search_events_by_date_range(
                1704200000000, // Later date
                1704000000000, // Earlier date
            );

            assert_eq!(result, Err(Error::InvalidDateRange));
        }

        #[ink::test]
        fn advanced_search_with_filters_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            // Create diverse events
            sports_broker.create_sports_event(
                "Playoff Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                100_000_000_000_000, // High price
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            sports_broker.create_sports_event(
                "Regular Game".to_string(),
                venue_id,
                1704153600000,
                18000,
                30_000_000_000_000, // Lower price
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Search with filters
            let filter = EventSearchFilter {
                sport_type: Some(SportType::Basketball),
                team_id: Some(home_team_id),
                venue_id: None,
                min_date: Some(1704000000000),
                max_date: Some(1704200000000),
                game_type: Some(GameType::Playoff),
                max_price: Some(150_000_000_000_000),
                min_availability: Some(1000),
                active_only: true,
            };

            let filtered_events = sports_broker.search_events_advanced(filter).unwrap();
            assert!(filtered_events.len() >= 1);
        }

        #[ink::test]
        fn get_recommended_events_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, _, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Create user profile with favorite team
            sports_broker.create_user_profile(
                vec![home_team_id],
                "New York".to_string(),
            ).unwrap();

            // Get recommendations
            let recommendations = sports_broker.get_recommended_events(accounts.alice, 10);
            assert!(recommendations.len() >= 1); // Should find events for favorite team
        }

        // ========================================================================
        // NEW: STEP 10 - ANALYTICS & REPORTING TESTS
        // ========================================================================

        #[ink::test]
        fn revenue_tracking_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);

            // Initial revenue should be 0
            assert_eq!(sports_broker.get_total_revenue(), 0);

            // Purchase ticket
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            // Revenue should be updated
            assert!(sports_broker.get_total_revenue() > 0);
        }

        #[ink::test]
        fn generate_analytics_report_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Create user profile
            sports_broker.create_user_profile(
                vec![home_team_id],
                "New York".to_string(),
            ).unwrap();

            // Purchase ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            // Purchase season pass
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10_000_000_000_000_000);
            sports_broker.purchase_season_pass_with_currency(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                CurrencyId::LDOT,
                false,
            ).unwrap();

            // Generate analytics report
            let report = sports_broker.generate_analytics_report();
            
            assert!(report.total_revenue > 0);
            assert!(report.total_tickets_sold > 0);
            assert!(report.total_season_passes_sold > 0);
            assert!(report.user_count > 0);
            assert!(!report.revenue_by_currency.is_empty());
            assert!(!report.top_teams_by_revenue.is_empty());
        }

        #[ink::test]
        fn team_revenue_analytics_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);

            // Initially no revenue
            assert_eq!(sports_broker.get_team_revenue_analytics(home_team_id), Some(0));

            // Purchase ticket
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            // Should have revenue now
            let team_revenue = sports_broker.get_team_revenue_analytics(home_team_id).unwrap();
            assert!(team_revenue > 0);
        }

        #[ink::test]
        fn platform_stats_tracking_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            let initial_stats = sports_broker.get_platform_stats();
            assert_eq!(initial_stats.total_users, 0);
            assert_eq!(initial_stats.total_tickets_sold, 0);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Create user profile
            sports_broker.create_user_profile(
                vec![home_team_id],
                "New York".to_string(),
            ).unwrap();

            // Purchase ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            let updated_stats = sports_broker.get_platform_stats();
            assert_eq!(updated_stats.total_users, 1);
            assert_eq!(updated_stats.total_tickets_sold, 1);
            assert!(updated_stats.average_ticket_price > 0);
            assert_eq!(updated_stats.total_events_created, 1);
        }

        #[ink::test]
        fn user_spending_analytics_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Create user profile
            sports_broker.create_user_profile(
                vec![home_team_id],
                "New York".to_string(),
            ).unwrap();

            // Initially no spending
            assert_eq!(sports_broker.get_user_spending_analytics(accounts.alice), 0);

            // Purchase ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1_000_000_000_000_000);
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            // Should have spending tracked
            let user_spending = sports_broker.get_user_spending_analytics(accounts.alice);
            assert!(user_spending > 0);
        }

        #[ink::test]
        fn event_attendance_rate_works() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();

            // Create a smaller event for this test
            let venue_id = sports_broker.register_venue(
                "Small Venue".to_string(),
                "Test City".to_string(),
                100, // Small capacity for percentage calculation
            ).unwrap();

            let home_team_id = sports_broker.register_team(
                "Test Team".to_string(),
                "Test City".to_string(),
                SportType::Basketball,
            ).unwrap();

            let away_team_id = sports_broker.register_team(
                "Away Team".to_string(),
                "Away City".to_string(),
                SportType::Basketball,
            ).unwrap();

            let season_id = sports_broker.create_season(
                "Test Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                50,
            ).unwrap();

            let event_id = sports_broker.create_sports_event(
                "Small Event".to_string(),
                venue_id,
                1704067200000,
                100, // Small capacity
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Initially 0% attendance
            assert_eq!(sports_broker.get_event_attendance_rate(event_id), Some(0));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10_000_000_000_000_000); // 10 ACA

            // Purchase ticket
            sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            // Should have 1% attendance rate now (1/100 * 100 = 1%)
            let attendance_rate = sports_broker.get_event_attendance_rate(event_id).unwrap();
            assert!(attendance_rate > 0);
            assert!(attendance_rate <= 100);
            assert_eq!(attendance_rate, 1); // Should be exactly 1%
        }

        #[ink::test]
        fn top_events_by_revenue_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Create multiple events
            let event1 = sports_broker.create_sports_event(
                "High Revenue Event".to_string(),
                venue_id,
                1704067200000,
                18000,
                100_000_000_000_000, // High price
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            let event2 = sports_broker.create_sports_event(
                "Low Revenue Event".to_string(),
                venue_id,
                1704153600000,
                18000,
                20_000_000_000_000, // Lower price
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Purchase tickets for both events with massive amounts
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(10_000_000_000_000_000); // 10 ACA
            sports_broker.purchase_sports_ticket_with_currency(
                event1,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(5_000_000_000_000_000); // 5 ACA
            sports_broker.purchase_sports_ticket_with_currency(
                event2,
                "Section B".to_string(),
                "Row 2".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::ACA,
            ).unwrap();

            let top_events = sports_broker.get_top_events_by_revenue(10);
            assert!(top_events.len() >= 2);
            
            // First event should have higher revenue
            if top_events.len() >= 2 {
                assert!(top_events[0].1 >= top_events[1].1);
            }
        }

        // ========================================================================
        // COMPREHENSIVE INTEGRATION TEST
        // ========================================================================

        #[ink::test]
        fn complete_platform_workflow_with_analytics() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();

            // Step 1: Platform setup
            let venue_id = sports_broker.register_venue(
                "Complete Arena".to_string(),
                "Test City".to_string(),
                25000,
            ).unwrap();

            let team1_id = sports_broker.register_team(
                "Team Alpha".to_string(),
                "Test City".to_string(),
                SportType::Basketball,
            ).unwrap();

            let team2_id = sports_broker.register_team(
                "Team Beta".to_string(),
                "Other City".to_string(),
                SportType::Basketball,
            ).unwrap();

            let season_id = sports_broker.create_season(
                "Test Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                100,
            ).unwrap();

            let event_id = sports_broker.create_sports_event(
                "Championship Game".to_string(),
                venue_id,
                1704067200000,
                20000,
                75_000_000_000_000,
                team1_id,
                team2_id,
                season_id,
                GameType::Championship,
            ).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Step 2: User onboarding with analytics
            sports_broker.create_user_profile_with_currency(
                vec![team1_id],
                "Test City".to_string(),
                CurrencyId::AUSD,
            ).unwrap();

            // Step 3: Season pass purchase with staking
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(50_000_000_000_000_000); // 50 LDOT
            let pass_id = sports_broker.purchase_season_pass_with_currency(
                season_id,
                team1_id,
                SeasonPassType::Premium,
                CurrencyId::LDOT,
                true,
            ).unwrap();

            // Step 4: Ticket purchase with multi-currency
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(20_000_000_000_000_000); // 20 aUSD
            let ticket_id = sports_broker.purchase_sports_ticket_with_currency(
                event_id,
                "VIP Section".to_string(),
                "Row 1".to_string(),
                SeatType::Suite,
                CurrencyId::AUSD,
            ).unwrap();

            // Step 5: Search and discovery
            let basketball_events = sports_broker.search_events_by_sport(SportType::Basketball);
            assert!(basketball_events.len() >= 1);

            let team_events = sports_broker.search_events_by_team(team1_id);
            assert!(team_events.len() >= 1);

            let recommendations = sports_broker.get_recommended_events(accounts.alice, 5);
            assert!(!recommendations.is_empty());

            // Step 6: Analytics verification
            let analytics_report = sports_broker.generate_analytics_report();
            assert!(analytics_report.total_revenue > 0);
            assert_eq!(analytics_report.total_tickets_sold, 1);
            assert_eq!(analytics_report.total_season_passes_sold, 1);
            assert_eq!(analytics_report.user_count, 1);

            let platform_stats = sports_broker.get_platform_stats();
            assert_eq!(platform_stats.total_users, 1);
            assert_eq!(platform_stats.total_tickets_sold, 1);
            assert_eq!(platform_stats.total_season_passes_sold, 1);
            assert_eq!(platform_stats.total_events_created, 1);

            // Step 7: Revenue analytics
            let team_revenue = sports_broker.get_team_revenue_analytics(team1_id).unwrap();
            assert!(team_revenue > 0);

            let venue_revenue = sports_broker.get_venue_revenue_analytics(venue_id).unwrap();
            assert!(venue_revenue > 0);

            let user_spending = sports_broker.get_user_spending_analytics(accounts.alice);
            assert!(user_spending > 0);

            // Step 8: Staking rewards
            let staked_amount = sports_broker.get_user_staked_amount(accounts.alice);
            assert!(staked_amount > 0);

            let estimated_rewards = sports_broker.get_estimated_staking_rewards(accounts.alice);
            assert_eq!(estimated_rewards, 0); // No time passed yet

            // Step 9: Verify all objects created correctly
            let team = sports_broker.get_team(team1_id).unwrap();
            assert_eq!(team.name, "Team Alpha");

            let venue = sports_broker.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Complete Arena");

            let season = sports_broker.get_season(season_id).unwrap();
            assert_eq!(season.name, "Test Season");

            let event = sports_broker.get_sports_event(event_id).unwrap();
            assert_eq!(event.name, "Championship Game");
            assert!(event.revenue_generated > 0);

            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert_eq!(ticket.purchase_currency, CurrencyId::AUSD);
            assert!(ticket.season_pass_discount_applied);

            let season_pass = sports_broker.get_season_pass(pass_id).unwrap();
            assert_eq!(season_pass.purchase_currency, CurrencyId::LDOT);
            assert!(season_pass.staking_rewards_enabled);

            let profile = sports_broker.get_user_profile(accounts.alice).unwrap();
            assert_eq!(profile.preferred_currency, CurrencyId::AUSD);
            assert!(profile.season_pass_holder);
            assert!(profile.total_spent > 0);

            // Step 10: Multi-currency queries
            let price_in_dot = sports_broker.get_current_ticket_price_in_currency(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
                CurrencyId::DOT,
            ).unwrap();

            let price_in_aca = sports_broker.get_current_ticket_price_in_currency(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
                CurrencyId::ACA,
            ).unwrap();

            assert!(price_in_aca > price_in_dot); // ACA should require more tokens

            // Final verification: Complete InkTix Sports Broker is working!
            assert_eq!(sports_broker.total_teams(), 2);
            assert_eq!(sports_broker.total_venues(), 1);
            assert_eq!(sports_broker.total_seasons(), 1);
            assert_eq!(sports_broker.total_events(), 1);
            assert_eq!(sports_broker.total_tickets(), 1);
            assert_eq!(sports_broker.total_season_passes(), 1);
            assert!(sports_broker.get_total_revenue() > 0);
        }
    }
}