#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// InkTix Sports Broker - Specialized Sports & Athletic Events Marketplace
/// 
/// Built on InkTix Core Foundation - Deployed on Acala Parachain
/// 
/// Sports-Specific Features:
/// - Season pass management with liquid staking rewards
/// - Team loyalty programs and fan verification
/// - Dynamic pricing based on team performance  
/// - Fantasy sports integration
/// - Multi-currency payments (DOT, ACA, aUSD, LDOT)
/// - DeFi yield generation on purchases
/// - Venue partnerships and parking/concession bundles
/// 
/// Acala Integration Benefits:
/// - Native multi-currency support
/// - Liquid staking rewards for season passes
/// - DEX integration for automatic currency conversion
/// - Lending protocol integration for payment plans

#[ink::contract]
mod sports_broker {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};

    // ============================================================================
    // ENHANCED STORAGE & STATE
    // ============================================================================

    #[ink(storage)]
    pub struct SportsBroker {
        // Core marketplace (inherited from InkTix Core)
        events: Mapping<u32, SportsEvent>,
        next_event_id: u32,
        tickets: Mapping<u64, SportsTicket>,
        user_tickets: Mapping<AccountId, Vec<u64>>,
        next_ticket_id: u64,
        owner: AccountId,

        // Sports-specific storage
        teams: Mapping<u32, Team>,
        next_team_id: u32,
        venues: Mapping<u32, Venue>, 
        next_venue_id: u32,
        seasons: Mapping<u32, Season>,
        next_season_id: u32,
        
        // User management
        user_profiles: Mapping<AccountId, UserProfile>,
        team_fans: Mapping<u32, Vec<AccountId>>, // team_id -> fan list
        loyalty_points: Mapping<AccountId, u32>,
        
        // Season passes and subscriptions
        season_passes: Mapping<u64, SeasonPass>,
        next_season_pass_id: u64,
        user_season_passes: Mapping<AccountId, Vec<u64>>,
        
        // Dynamic pricing
        team_performance: Mapping<u32, TeamPerformance>,
        pricing_multipliers: Mapping<u32, PricingMultiplier>, // team_id -> multiplier
        
        // Multi-currency support (Acala integration)
        supported_currencies: Vec<CurrencyId>,
        currency_rates: Mapping<CurrencyId, Balance>, // Rate relative to DOT
        
        // DeFi integration
        staking_rewards_pool: Balance,
        total_staked_amount: Balance,
        user_staked_amounts: Mapping<AccountId, Balance>,
    }

    // ============================================================================
    // SPORTS-SPECIFIC DATA TYPES
    // ============================================================================

    /// Enhanced Event structure for sports
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct SportsEvent {
        // Core fields (from foundation)
        pub id: u32,
        pub name: String,
        pub venue_id: u32,
        pub date: u64,
        pub capacity: u32,
        pub sold_tickets: u32,
        pub base_price: Balance,
        pub active: bool,
        
        // Sports-specific fields
        pub sport_type: SportType,
        pub home_team_id: u32,
        pub away_team_id: u32,
        pub season_id: u32,
        pub game_type: GameType,
        pub playoff_round: Option<u8>,
        pub fantasy_eligible: bool,
        pub dynamic_pricing_enabled: bool,
        pub loyalty_discount_percentage: u8, // 0-100
        pub includes_parking: bool,
        pub includes_concessions: bool,
        pub broadcast_nationally: bool,
    }

    /// Enhanced Ticket structure for sports
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct SportsTicket {
        // Core fields (from foundation)
        pub id: u64,
        pub event_id: u32,
        pub owner: AccountId,
        pub purchase_price: Balance,
        pub purchase_currency: CurrencyId,
        pub purchase_date: u64,
        pub seat_number: u32,
        pub transferable: bool,
        
        // Sports-specific fields
        pub section: String,
        pub row: String,
        pub seat_type: SeatType,
        pub season_pass_discount_applied: bool,
        pub loyalty_points_earned: u32,
        pub fantasy_points_bonus: u32,
        pub includes_parking_pass: bool,
        pub includes_concession_credit: Balance,
        pub team_merchandise_included: bool,
        pub is_season_pass_ticket: bool,
        pub access_level: AccessLevel,
    }

    /// Team information
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Team {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub sport_type: SportType,
        pub league: String,
        pub conference: Option<String>,
        pub division: Option<String>,
        pub logo_url: String,
        pub established_year: u32,
        pub home_venue_id: u32,
        pub verified: bool,
        pub fan_token_contract: Option<AccountId>,
    }

    /// Venue information with sports-specific features
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Venue {
        pub id: u32,
        pub name: String,
        pub city: String,
        pub state_province: String,
        pub country: String,
        pub capacity: u32,
        pub venue_type: VenueType,
        pub parking_spots: u32,
        pub concession_stands: u32,
        pub premium_seating_available: bool,
        pub accessibility_features: Vec<AccessibilityFeature>,
        pub partner_restaurants: Vec<String>,
        pub partner_hotels: Vec<String>,
    }

    /// Season information for subscription management
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Season {
        pub id: u32,
        pub name: String, // "2024-25 NBA Season"
        pub sport_type: SportType,
        pub start_date: u64,
        pub end_date: u64,
        pub regular_season_games: u32,
        pub playoff_games_estimate: u32,
        pub active: bool,
        pub season_pass_base_price: Balance,
        pub early_bird_discount: u8, // percentage
        pub early_bird_deadline: u64,
    }

    /// Season pass for subscription management
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
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
        pub priority_level: u8, // Higher number = better seats
        pub loyalty_tier: LoyaltyTier,
        pub staking_rewards_enabled: bool,
        pub staked_amount: Balance, // Amount earning liquid staking rewards
    }

    /// User profile for fan management
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct UserProfile {
        pub verified_fan: bool,
        pub favorite_teams: Vec<u32>,
        pub home_city: String,
        pub loyalty_tier: LoyaltyTier,
        pub total_games_attended: u32,
        pub account_creation_date: u64,
        pub preferred_payment_currency: CurrencyId,
        pub anti_scalping_verified: bool,
        pub social_media_verified: bool,
        pub season_pass_holder: bool,
    }

    /// Team performance for dynamic pricing
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct TeamPerformance {
        pub team_id: u32,
        pub season_id: u32,
        pub wins: u32,
        pub losses: u32,
        pub win_percentage: u32, // 0-10000 (basis points)
        pub streak: i32, // positive for wins, negative for losses
        pub playoff_probability: u32, // 0-10000 (basis points)
        pub last_updated: u64,
        pub performance_rank: u32, // 1 = best in league
        pub home_record_wins: u32,
        pub home_record_losses: u32,
    }

    /// Pricing multiplier based on various factors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct PricingMultiplier {
        pub team_id: u32,
        pub base_multiplier: u32, // 10000 = 1.0x, 15000 = 1.5x
        pub performance_multiplier: u32,
        pub rivalry_multiplier: u32,
        pub playoff_multiplier: u32,
        pub demand_multiplier: u32,
        pub final_multiplier: u32, // Calculated from above
        pub last_updated: u64,
    }

    // ============================================================================
    // ENUMS AND CONSTANTS
    // ============================================================================

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum SportType {
        Basketball,
        Football,     // American Football
        Soccer,       // Football/Soccer
        Baseball,
        Hockey,
        Tennis,
        Golf,
        Racing,       // NASCAR, F1, etc.
        Boxing,
        MMA,
        Wrestling,
        Other(String),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum GameType {
        RegularSeason,
        Playoff,
        Championship,
        AllStar,
        Preseason,
        Tournament,
        Exhibition,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum SeatType {
        GeneralAdmission,
        Reserved,
        PremiumReserved,
        Club,
        Suite,
        FieldLevel,
        Courtside,
        StudentSection,
        SeasonTicketHolder,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum AccessLevel {
        Standard,
        Premium,
        VIP,
        AllAccess,
    }

    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum VenueType {
        Stadium,
        Arena,
        Ballpark,
        Field,
        Court,
        Track,
        Other(String),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum SeasonPassType {
        FullSeason,
        HalfSeason,
        QuarterSeason,
        Weekend,
        Weekday,
        Premium,
        Playoffs,
        Package(u32), // Custom number of games
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum LoyaltyTier {
        Bronze,
        Silver,
        Gold,
        Platinum,
        Diamond,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum AccessibilityFeature {
        WheelchairAccessible,
        AudioAssistance,
        VisualAssistance,
        ServiceAnimalFriendly,
        ElevatorAccess,
        AccessibleParking,
        AccessibleRestrooms,
    }

    /// Multi-currency support for Acala integration
    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum CurrencyId {
        DOT,  // Polkadot native token
        ACA,  // Acala native token
        AUSD, // Acala USD stablecoin
        LDOT, // Liquid DOT from Acala staking
        KSM,  // Kusama (for cross-chain payments)
    }

    /// Enhanced error types for sports-specific functionality
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        // Core errors (inherited from foundation)
        EventNotFound,
        Unauthorized,
        EventNotActive,
        InsufficientCapacity,
        EventIdOverflow,
        InsufficientPayment,
        EventSoldOut,
        TicketNotFound,
        NotTicketOwner,
        TicketNotTransferable,
        TicketIdOverflow,
        
        // Sports-specific errors
        TeamNotFound,
        VenueNotFound,
        SeasonNotFound,
        SeasonNotActive,
        SeasonPassNotFound,
        NotSeasonPassOwner,
        InvalidSportType,
        UserNotVerified,
        LoyaltyTierTooLow,
        SeasonPassRequired,
        GameNotEligibleForFantasy,
        PricingDataOutdated,
        DynamicPricingDisabled,
        UnsupportedCurrency,
        CurrencyConversionFailed,
        InsufficientStakingRewards,
        StakingNotEnabled,
        TeamIdOverflow,
        VenueIdOverflow,
        SeasonIdOverflow,
        SeasonPassIdOverflow,
        InvalidLoyaltyDiscount,
        ParkingNotAvailable,
        ConcessionsNotAvailable,
    }

    // ============================================================================
    // SPORTS BROKER IMPLEMENTATION
    // ============================================================================

    impl SportsBroker {
        /// Initialize the sports marketplace with Acala multi-currency support
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut supported_currencies = Vec::new();
            supported_currencies.push(CurrencyId::DOT);
            supported_currencies.push(CurrencyId::ACA);
            supported_currencies.push(CurrencyId::AUSD);
            supported_currencies.push(CurrencyId::LDOT);

            Self {
                events: Mapping::new(),
                next_event_id: 1,
                tickets: Mapping::new(),
                user_tickets: Mapping::new(),
                next_ticket_id: 1,
                owner: Self::env().caller(),
                
                teams: Mapping::new(),
                next_team_id: 1,
                venues: Mapping::new(),
                next_venue_id: 1,
                seasons: Mapping::new(),
                next_season_id: 1,
                
                user_profiles: Mapping::new(),
                team_fans: Mapping::new(),
                loyalty_points: Mapping::new(),
                
                season_passes: Mapping::new(),
                next_season_pass_id: 1,
                user_season_passes: Mapping::new(),
                
                team_performance: Mapping::new(),
                pricing_multipliers: Mapping::new(),
                
                supported_currencies,
                currency_rates: Mapping::new(),
                
                staking_rewards_pool: 0,
                total_staked_amount: 0,
                user_staked_amounts: Mapping::new(),
            }
        }

        // ========================================================================
        // TEAM & VENUE MANAGEMENT
        // ========================================================================

        /// Register a new sports team
        #[ink(message)]
        pub fn register_team(
            &mut self,
            name: String,
            city: String,
            sport_type: SportType,
            league: String,
            home_venue_id: u32,
        ) -> Result<u32, Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }

            let team_id = self.next_team_id;
            self.next_team_id = self.next_team_id
                .checked_add(1)
                .ok_or(Error::TeamIdOverflow)?;

            let team = Team {
                id: team_id,
                name: name.clone(),
                city,
                sport_type: sport_type.clone(),
                league,
                conference: None,
                division: None,
                logo_url: String::new(),
                established_year: 0,
                home_venue_id,
                verified: true,
                fan_token_contract: None,
            };

            self.teams.insert(team_id, &team);
            
            // Initialize empty fan list
            self.team_fans.insert(team_id, &Vec::<AccountId>::new());
            
            // Initialize performance tracking
            let performance = TeamPerformance {
                team_id,
                season_id: 0,
                wins: 0,
                losses: 0,
                win_percentage: 0,
                streak: 0,
                playoff_probability: 0,
                last_updated: self.env().block_timestamp(),
                performance_rank: 0,
                home_record_wins: 0,
                home_record_losses: 0,
            };
            self.team_performance.insert(team_id, &performance);

            self.env().emit_event(TeamRegistered {
                team_id,
                name,
                sport_type,
            });

            Ok(team_id)
        }

        /// Register a new venue
        #[ink(message)]
        pub fn register_venue(
            &mut self,
            name: String,
            city: String,
            state_province: String,
            country: String,
            capacity: u32,
            venue_type: VenueType,
        ) -> Result<u32, Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }

            let venue_id = self.next_venue_id;
            self.next_venue_id = self.next_venue_id
                .checked_add(1)
                .ok_or(Error::VenueIdOverflow)?;

            let venue = Venue {
                id: venue_id,
                name: name.clone(),
                city,
                state_province,
                country,
                capacity,
                venue_type,
                parking_spots: 0,
                concession_stands: 0,
                premium_seating_available: false,
                accessibility_features: Vec::new(),
                partner_restaurants: Vec::new(),
                partner_hotels: Vec::new(),
            };

            self.venues.insert(venue_id, &venue);

            self.env().emit_event(VenueRegistered {
                venue_id,
                name,
                capacity,
            });

            Ok(venue_id)
        }

        // ========================================================================
        // SEASON & SUBSCRIPTION MANAGEMENT
        // ========================================================================

        /// Create a new sports season
        #[ink(message)]
        pub fn create_season(
            &mut self,
            name: String,
            sport_type: SportType,
            start_date: u64,
            end_date: u64,
            regular_season_games: u32,
            season_pass_base_price: Balance,
        ) -> Result<u32, Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }

            let season_id = self.next_season_id;
            self.next_season_id = self.next_season_id
                .checked_add(1)
                .ok_or(Error::SeasonIdOverflow)?;

            let season = Season {
                id: season_id,
                name: name.clone(),
                sport_type: sport_type.clone(),
                start_date,
                end_date,
                regular_season_games,
                playoff_games_estimate: 0,
                active: true,
                season_pass_base_price,
                early_bird_discount: 0,
                early_bird_deadline: start_date,
            };

            self.seasons.insert(season_id, &season);

            self.env().emit_event(SeasonCreated {
                season_id,
                name,
                sport_type,
            });

            Ok(season_id)
        }

        /// Purchase a season pass with staking rewards
        #[ink(message, payable)]
        pub fn purchase_season_pass(
            &mut self,
            season_id: u32,
            team_id: u32,
            pass_type: SeasonPassType,
            currency: CurrencyId,
            enable_staking: bool,
        ) -> Result<u64, Error> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            // Validate season and team
            let season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;
            if !season.active {
                return Err(Error::SeasonNotActive);
            }
            
            let _team = self.teams.get(team_id).ok_or(Error::TeamNotFound)?;

            // Calculate season pass price with dynamic factors
            let base_price = season.season_pass_base_price;
            let final_price = self.calculate_season_pass_price(base_price, team_id, &pass_type)?;

            // Convert payment to DOT equivalent if needed
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            
            if payment_in_dot < final_price {
                return Err(Error::InsufficientPayment);
            }

            // Create season pass
            let pass_id = self.next_season_pass_id;
            self.next_season_pass_id = self.next_season_pass_id
                .checked_add(1)
                .ok_or(Error::SeasonPassIdOverflow)?;

            let games_included = self.calculate_games_included(&pass_type, season.regular_season_games);
            let loyalty_tier = self.get_user_loyalty_tier(buyer);

            let season_pass = SeasonPass {
                id: pass_id,
                owner: buyer,
                season_id,
                team_id,
                pass_type: pass_type.clone(),
                purchase_price: payment,
                purchase_currency: currency,
                purchase_date: self.env().block_timestamp(),
                games_included,
                games_attended: 0,
                transferable: true,
                includes_playoffs: false,
                priority_level: self.calculate_priority_level(&loyalty_tier),
                loyalty_tier,
                staking_rewards_enabled: enable_staking,
                staked_amount: if enable_staking { payment } else { 0 },
            };

            // Enable staking rewards if requested
            if enable_staking {
                self.total_staked_amount = self.total_staked_amount
                    .checked_add(payment)
                    .ok_or(Error::InsufficientStakingRewards)?;
                
                let current_staked = self.user_staked_amounts.get(buyer).unwrap_or(0);
                self.user_staked_amounts.insert(buyer, &(current_staked + payment));
            }

            // Store season pass
            self.season_passes.insert(pass_id, &season_pass);

            // Update user season passes
            let mut user_passes = self.user_season_passes.get(buyer).unwrap_or_default();
            user_passes.push(pass_id);
            self.user_season_passes.insert(buyer, &user_passes);

            // Update user profile
            self.update_user_profile_for_season_pass(buyer, team_id);

            // Award loyalty points
            let loyalty_points_earned = self.calculate_loyalty_points_for_season_pass(&pass_type);
            self.award_loyalty_points(buyer, loyalty_points_earned);

            self.env().emit_event(SeasonPassPurchased {
                pass_id,
                season_id,
                team_id,
                buyer,
                price: payment,
                staking_enabled: enable_staking,
            });

            Ok(pass_id)
        }

        // ========================================================================
        // ENHANCED SPORTS EVENT CREATION
        // ========================================================================

        /// Create a sports event with team information and dynamic pricing
        #[ink(message)]
        pub fn create_sports_event(
            &mut self,
            name: String,
            venue_id: u32,
            date: u64,
            capacity: u32,
            base_price: Balance,
            sport_type: SportType,
            home_team_id: u32,
            away_team_id: u32,
            season_id: u32,
            game_type: GameType,
        ) -> Result<u32, Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }

            // Validate venue, teams, and season
            let _venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            let _home_team = self.teams.get(home_team_id).ok_or(Error::TeamNotFound)?;
            let _away_team = self.teams.get(away_team_id).ok_or(Error::TeamNotFound)?;
            let _season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;

            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::EventIdOverflow)?;

            let sports_event = SportsEvent {
                id: event_id,
                name: name.clone(),
                venue_id,
                date,
                capacity,
                sold_tickets: 0,
                base_price,
                active: true,
                
                // Sports-specific fields
                sport_type: sport_type.clone(),
                home_team_id,
                away_team_id,
                season_id,
                game_type: game_type.clone(),
                playoff_round: None,
                fantasy_eligible: true,
                dynamic_pricing_enabled: true,
                loyalty_discount_percentage: 10, // 10% discount for loyal fans
                includes_parking: false,
                includes_concessions: false,
                broadcast_nationally: false,
            };

            self.events.insert(event_id, &sports_event);

            // Update pricing multipliers based on teams
            self.update_pricing_multipliers(home_team_id, away_team_id, &game_type);

            self.env().emit_event(SportsEventCreated {
                event_id,
                name,
                home_team_id,
                away_team_id,
                date,
            });

            Ok(event_id)
        }

        // ========================================================================
        // ENHANCED SPORTS TICKET PURCHASING
        // ========================================================================

        /// Purchase a sports ticket with multi-currency support and loyalty benefits
        #[ink(message, payable)]
        pub fn purchase_sports_ticket(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_type: SeatType,
            currency: CurrencyId,
            use_loyalty_discount: bool,
        ) -> Result<u64, Error> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            // Get and validate event
            let mut event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            if !event.active {
                return Err(Error::EventNotActive);
            }
            if event.sold_tickets >= event.capacity {
                return Err(Error::EventSoldOut);
            }

            // Calculate final price with all modifiers
            let final_price = self.calculate_dynamic_ticket_price(
                event_id,
                &event,
                &seat_type,
                buyer,
                use_loyalty_discount,
            )?;

            // Convert payment to DOT equivalent
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            
            if payment_in_dot < final_price {
                return Err(Error::InsufficientPayment);
            }

            // Check if user has season pass benefits
            let season_pass_discount = self.check_season_pass_benefits(buyer, event.team_id())?;
            let loyalty_points_earned = self.calculate_loyalty_points(&seat_type, final_price);

            // Create enhanced sports ticket
            let ticket_id = self.next_ticket_id;
            self.next_ticket_id = self.next_ticket_id
                .checked_add(1)
                .ok_or(Error::TicketIdOverflow)?;

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
                
                // Sports-specific fields
                section,
                row,
                seat_type: seat_type.clone(),
                season_pass_discount_applied: season_pass_discount,
                loyalty_points_earned,
                fantasy_points_bonus: if event.fantasy_eligible { 100 } else { 0 },
                includes_parking_pass: event.includes_parking,
                includes_concession_credit: if event.includes_concessions { 2_000_000_000_000 } else { 0 }, // 0.002 DOT
                team_merchandise_included: false,
                is_season_pass_ticket: season_pass_discount,
                access_level: self.determine_access_level(&seat_type),
            };

            // Store ticket
            self.tickets.insert(ticket_id, &sports_ticket);

            // Update user tickets
            let mut user_ticket_list = self.user_tickets.get(buyer).unwrap_or_default();
            user_ticket_list.push(ticket_id);
            self.user_tickets.insert(buyer, &user_ticket_list);

            // Update event sold count
            event.sold_tickets = seat_number;
            self.events.insert(event_id, &event);

            // Award loyalty points
            self.award_loyalty_points(buyer, loyalty_points_earned);

            // Update user profile with attendance
            self.update_user_attendance(buyer, event.home_team_id);

            self.env().emit_event(SportsTicketPurchased {
                ticket_id,
                event_id,
                buyer,
                price: payment,
                currency,
                seat_type: seat_type.clone(),
                loyalty_points_earned,
            });

            if event.sold_tickets == event.capacity {
                self.env().emit_event(SportsEventSoldOut { event_id });
            }

            Ok(ticket_id)
        }

        // ========================================================================
        // DYNAMIC PRICING ENGINE
        // ========================================================================

        /// Update team performance data for dynamic pricing
        #[ink(message)]
        pub fn update_team_performance(
            &mut self,
            team_id: u32,
            season_id: u32,
            wins: u32,
            losses: u32,
            playoff_probability: u32,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }

            let win_percentage = if wins + losses > 0 {
                (wins * 10000) / (wins + losses)
            } else {
                0
            };

            let performance = TeamPerformance {
                team_id,
                season_id,
                wins,
                losses,
                win_percentage,
                streak: 0, // Would be calculated from recent games
                playoff_probability,
                last_updated: self.env().block_timestamp(),
                performance_rank: 0, // Would be calculated relative to league
                home_record_wins: 0,
                home_record_losses: 0,
            };

            self.team_performance.insert(team_id, &performance);

            // Update pricing multipliers based on new performance
            self.recalculate_pricing_multiplier(team_id);

            self.env().emit_event(TeamPerformanceUpdated {
                team_id,
                wins,
                losses,
                playoff_probability,
            });

            Ok(())
        }

        /// Calculate dynamic ticket price based on multiple factors
        fn calculate_dynamic_ticket_price(
            &self,
            _event_id: u32,
            event: &SportsEvent,
            seat_type: &SeatType,
            buyer: AccountId,
            use_loyalty_discount: bool,
        ) -> Result<Balance, Error> {
            let mut final_price = event.base_price;

            // Apply seat type multiplier
            let seat_multiplier: Balance = match seat_type {
                SeatType::GeneralAdmission => 10000,      // 1.0x
                SeatType::Reserved => 12000,              // 1.2x
                SeatType::PremiumReserved => 15000,       // 1.5x
                SeatType::Club => 20000,                  // 2.0x
                SeatType::Suite => 50000,                 // 5.0x
                SeatType::FieldLevel => 30000,            // 3.0x
                SeatType::Courtside => 80000,             // 8.0x
                SeatType::StudentSection => 5000,         // 0.5x
                SeatType::SeasonTicketHolder => 8000,     // 0.8x
            };
            
            final_price = (final_price * seat_multiplier) / 10000;

            // Apply dynamic pricing if enabled
            if event.dynamic_pricing_enabled {
                if let Some(multiplier) = self.pricing_multipliers.get(event.home_team_id) {
                    final_price = (final_price * multiplier.final_multiplier as Balance) / 10000;
                }
            }

            // Apply loyalty discount if eligible and requested
            if use_loyalty_discount && event.loyalty_discount_percentage > 0 {
                let user_tier = self.get_user_loyalty_tier(buyer);
                let discount = match user_tier {
                    LoyaltyTier::Bronze => event.loyalty_discount_percentage / 2,
                    LoyaltyTier::Silver => event.loyalty_discount_percentage,
                    LoyaltyTier::Gold => event.loyalty_discount_percentage + 5,
                    LoyaltyTier::Platinum => event.loyalty_discount_percentage + 10,
                    LoyaltyTier::Diamond => event.loyalty_discount_percentage + 15,
                };
                
                let discount_amount = (final_price * discount as Balance) / 100;
                final_price = final_price.saturating_sub(discount_amount);
            }

            Ok(final_price)
        }

        // ========================================================================
        // DEFI INTEGRATION (ACALA-SPECIFIC)
        // ========================================================================

        /// Convert payment amount to DOT equivalent using Acala DEX rates
        fn convert_to_dot_equivalent(
            &self,
            amount: Balance,
            currency: CurrencyId,
        ) -> Result<Balance, Error> {
            match currency {
                CurrencyId::DOT => Ok(amount),
                CurrencyId::ACA => {
                    // Get ACA/DOT rate from Acala DEX
                    let rate = self.currency_rates.get(CurrencyId::ACA).unwrap_or(500_000_000_000); // 0.0005 DOT per ACA
                    Ok((amount * rate) / 1_000_000_000_000)
                },
                CurrencyId::AUSD => {
                    // aUSD is designed to be stable at ~$6-8 per DOT
                    let rate = self.currency_rates.get(CurrencyId::AUSD).unwrap_or(150_000_000_000); // ~0.15 DOT per aUSD
                    Ok((amount * rate) / 1_000_000_000_000)
                },
                CurrencyId::LDOT => {
                    // LDOT should be close to 1:1 with DOT
                    let rate = self.currency_rates.get(CurrencyId::LDOT).unwrap_or(950_000_000_000); // ~0.95 DOT per LDOT
                    Ok((amount * rate) / 1_000_000_000_000)
                },
                CurrencyId::KSM => {
                    // KSM/DOT rate varies but typically ~1:10 to 1:20
                    let rate = self.currency_rates.get(CurrencyId::KSM).unwrap_or(15_000_000_000_000); // ~15 DOT per KSM
                    Ok((amount * rate) / 1_000_000_000_000)
                },
            }
        }

        /// Claim staking rewards for season pass holders
        #[ink(message)]
        pub fn claim_staking_rewards(&mut self) -> Result<Balance, Error> {
            let caller = self.env().caller();
            
            let staked_amount = self.user_staked_amounts.get(caller).unwrap_or(0);
            if staked_amount == 0 {
                return Err(Error::StakingNotEnabled);
            }

            // Calculate rewards (simplified - in practice would integrate with Acala liquid staking)
            // Assuming 8% APY for LDOT staking
            let annual_rate: Balance = 800; // 8% in basis points
            let seconds_per_year: Balance = 31_536_000;
            
            // For demo purposes, assume last claim was 30 days ago
            let time_elapsed: Balance = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds
            let rewards = (staked_amount * annual_rate * time_elapsed) / (10000 * seconds_per_year * 1000);

            if rewards > self.staking_rewards_pool {
                return Err(Error::InsufficientStakingRewards);
            }

            // Transfer rewards to user (in practice would be done through Acala runtime)
            self.staking_rewards_pool = self.staking_rewards_pool.saturating_sub(rewards);

            self.env().emit_event(StakingRewardsClaimed {
                user: caller,
                amount: rewards,
            });

            Ok(rewards)
        }

        // ========================================================================
        // USER MANAGEMENT & LOYALTY SYSTEM
        // ========================================================================

        /// Create or update user profile
        #[ink(message)]
        pub fn create_user_profile(
            &mut self,
            favorite_teams: Vec<u32>,
            home_city: String,
            preferred_currency: CurrencyId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();

            let profile = UserProfile {
                verified_fan: false,
                favorite_teams,
                home_city,
                loyalty_tier: LoyaltyTier::Bronze,
                total_games_attended: 0,
                account_creation_date: self.env().block_timestamp(),
                preferred_payment_currency: preferred_currency,
                anti_scalping_verified: false,
                social_media_verified: false,
                season_pass_holder: false,
            };

            self.user_profiles.insert(caller, &profile);

            // Add user to team fan lists
            for team_id in &profile.favorite_teams {
                let mut fans = self.team_fans.get(*team_id).unwrap_or_default();
                if !fans.contains(&caller) {
                    fans.push(caller);
                    self.team_fans.insert(*team_id, &fans);
                }
            }

            self.env().emit_event(UserProfileCreated {
                user: caller,
                favorite_teams: profile.favorite_teams,
            });

            Ok(())
        }

        /// Award loyalty points to user
        fn award_loyalty_points(&mut self, user: AccountId, points: u32) {
            let current_points = self.loyalty_points.get(user).unwrap_or(0);
            let new_total = current_points.saturating_add(points);
            self.loyalty_points.insert(user, &new_total);

            // Update loyalty tier based on total points
            let new_tier = match new_total {
                0..=999 => LoyaltyTier::Bronze,
                1000..=2999 => LoyaltyTier::Silver,
                3000..=6999 => LoyaltyTier::Gold,
                7000..=14999 => LoyaltyTier::Platinum,
                15000.. => LoyaltyTier::Diamond,
            };

            // Update user profile with new tier
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.loyalty_tier = new_tier;
                self.user_profiles.insert(user, &profile);
            }
        }

        // ========================================================================
        // QUERY METHODS (ENHANCED FOR SPORTS)
        // ========================================================================

        /// Get sports event details
        #[ink(message)]
        pub fn get_sports_event(&self, event_id: u32) -> Option<SportsEvent> {
            self.events.get(event_id)
        }

        /// Get sports ticket details
        #[ink(message)]
        pub fn get_sports_ticket(&self, ticket_id: u64) -> Option<SportsTicket> {
            self.tickets.get(ticket_id)
        }

        /// Get team information
        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.teams.get(team_id)
        }

        /// Get venue information
        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        /// Get season information
        #[ink(message)]
        pub fn get_season(&self, season_id: u32) -> Option<Season> {
            self.seasons.get(season_id)
        }

        /// Get season pass details
        #[ink(message)]
        pub fn get_season_pass(&self, pass_id: u64) -> Option<SeasonPass> {
            self.season_passes.get(pass_id)
        }

        /// Get user profile
        #[ink(message)]
        pub fn get_user_profile(&self, user: AccountId) -> Option<UserProfile> {
            self.user_profiles.get(user)
        }

        /// Get user loyalty points
        #[ink(message)]
        pub fn get_user_loyalty_points(&self, user: AccountId) -> u32 {
            self.loyalty_points.get(user).unwrap_or(0)
        }

        /// Get user season passes
        #[ink(message)]
        pub fn get_user_season_passes(&self, user: AccountId) -> Vec<u64> {
            self.user_season_passes.get(user).unwrap_or_default()
        }

        /// Search events by team
        #[ink(message)]
        pub fn search_events_by_team(&self, team_id: u32) -> Vec<u32> {
            let mut results = Vec::new();
            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    if (event.home_team_id == team_id || event.away_team_id == team_id) && event.active {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Search events by sport type
        #[ink(message)]
        pub fn search_events_by_sport(&self, sport_type: SportType) -> Vec<u32> {
            let mut results = Vec::new();
            for event_id in 1..self.next_event_id {
                if let Some(event) = self.events.get(event_id) {
                    if event.sport_type == sport_type && event.active {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Get current ticket price with all modifiers
        #[ink(message)]
        pub fn get_current_ticket_price(
            &self,
            event_id: u32,
            seat_type: SeatType,
            user: AccountId,
        ) -> Result<Balance, Error> {
            let event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            self.calculate_dynamic_ticket_price(event_id, &event, &seat_type, user, true)
        }

        /// Get team performance
        #[ink(message)]
        pub fn get_team_performance(&self, team_id: u32) -> Option<TeamPerformance> {
            self.team_performance.get(team_id)
        }

        /// Get supported currencies
        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.supported_currencies.clone()
        }

        /// Get user's staked amount
        #[ink(message)]
        pub fn get_user_staked_amount(&self, user: AccountId) -> Balance {
            self.user_staked_amounts.get(user).unwrap_or(0)
        }

        // ========================================================================
        // HELPER METHODS (PRIVATE)
        // ========================================================================

        fn calculate_season_pass_price(
            &self,
            base_price: Balance,
            team_id: u32,
            pass_type: &SeasonPassType,
        ) -> Result<Balance, Error> {
            let type_multiplier = match pass_type {
                SeasonPassType::FullSeason => 10000,      // 1.0x
                SeasonPassType::HalfSeason => 5500,       // 0.55x
                SeasonPassType::QuarterSeason => 3000,    // 0.30x
                SeasonPassType::Weekend => 6000,          // 0.60x
                SeasonPassType::Weekday => 4500,          // 0.45x
                SeasonPassType::Premium => 15000,         // 1.5x
                SeasonPassType::Playoffs => 8000,         // 0.80x
                SeasonPassType::Package(_) => 7500,       // 0.75x
            };

            let mut final_price = (base_price * type_multiplier) / 10000;

            // Apply team popularity multiplier
            if let Some(performance) = self.team_performance.get(team_id) {
                let popularity_multiplier = if performance.playoff_probability > 7500 {
                    12000 // Popular team: 1.2x
                } else if performance.playoff_probability > 5000 {
                    10000 // Average team: 1.0x
                } else {
                    8500  // Rebuilding team: 0.85x
                };
                final_price = (final_price * popularity_multiplier) / 10000;
            }

            Ok(final_price)
        }

        fn calculate_games_included(&self, pass_type: &SeasonPassType, total_games: u32) -> u32 {
            match pass_type {
                SeasonPassType::FullSeason => total_games,
                SeasonPassType::HalfSeason => total_games / 2,
                SeasonPassType::QuarterSeason => total_games / 4,
                SeasonPassType::Weekend => total_games * 40 / 100, // ~40% of games are weekends
                SeasonPassType::Weekday => total_games * 60 / 100,
                SeasonPassType::Premium => total_games + 4, // Includes potential playoff games
                SeasonPassType::Playoffs => 16, // Estimated playoff games
                SeasonPassType::Package(games) => *games,
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

        fn calculate_loyalty_points_for_season_pass(&self, pass_type: &SeasonPassType) -> u32 {
            match pass_type {
                SeasonPassType::FullSeason => 1000,
                SeasonPassType::HalfSeason => 500,
                SeasonPassType::QuarterSeason => 250,
                SeasonPassType::Weekend => 300,
                SeasonPassType::Weekday => 200,
                SeasonPassType::Premium => 1500,
                SeasonPassType::Playoffs => 400,
                SeasonPassType::Package(games) => *games * 10,
            }
        }

        fn check_season_pass_benefits(&self, user: AccountId, team_id: u32) -> Result<bool, Error> {
            if let Some(user_passes) = self.user_season_passes.get(user) {
                for pass_id in user_passes {
                    if let Some(pass) = self.season_passes.get(pass_id) {
                        if pass.team_id == team_id {
                            return Ok(true);
                        }
                    }
                }
            }
            Ok(false)
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
                SeatType::SeasonTicketHolder => 20,
            };

            // Bonus points based on ticket price (1 point per 0.01 DOT spent)
            let price_bonus = (price / 10_000_000_000) as u32; // Convert from plancks to 0.01 DOT units
            
            base_points + price_bonus
        }

        fn determine_access_level(&self, seat_type: &SeatType) -> AccessLevel {
            match seat_type {
                SeatType::GeneralAdmission | SeatType::StudentSection => AccessLevel::Standard,
                SeatType::Reserved | SeatType::PremiumReserved => AccessLevel::Premium,
                SeatType::Club | SeatType::FieldLevel => AccessLevel::VIP,
                SeatType::Suite | SeatType::Courtside => AccessLevel::AllAccess,
                SeatType::SeasonTicketHolder => AccessLevel::Premium,
            }
        }

        fn update_user_attendance(&mut self, user: AccountId, _team_id: u32) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.total_games_attended = profile.total_games_attended.saturating_add(1);
                self.user_profiles.insert(user, &profile);
            }
        }

        fn update_pricing_multipliers(&mut self, home_team_id: u32, _away_team_id: u32, game_type: &GameType) {
            // Update pricing for home team
            let base_multiplier = match game_type {
                GameType::RegularSeason => 10000,  // 1.0x
                GameType::Playoff => 15000,        // 1.5x
                GameType::Championship => 25000,   // 2.5x
                GameType::AllStar => 20000,        // 2.0x
                GameType::Preseason => 7500,       // 0.75x
                GameType::Tournament => 18000,     // 1.8x
                GameType::Exhibition => 8000,      // 0.8x
            };

            let multiplier = PricingMultiplier {
                team_id: home_team_id,
                base_multiplier,
                performance_multiplier: 10000,
                rivalry_multiplier: 10000,
                playoff_multiplier: 10000,
                demand_multiplier: 10000,
                final_multiplier: base_multiplier,
                last_updated: self.env().block_timestamp(),
            };

            self.pricing_multipliers.insert(home_team_id, &multiplier);
        }

        fn recalculate_pricing_multiplier(&mut self, team_id: u32) {
            if let Some(mut multiplier) = self.pricing_multipliers.get(team_id) {
                if let Some(performance) = self.team_performance.get(team_id) {
                    // Performance-based multiplier (winning teams cost more)
                    multiplier.performance_multiplier = if performance.win_percentage > 7500 {
                        12000 // Great team: 1.2x
                    } else if performance.win_percentage > 5000 {
                        10000 // Average team: 1.0x
                    } else {
                        8500  // Poor team: 0.85x
                    };

                    // Playoff probability multiplier
                    multiplier.playoff_multiplier = if performance.playoff_probability > 8000 {
                        11000 // Likely playoff team: 1.1x
                    } else if performance.playoff_probability > 5000 {
                        10000 // Bubble team: 1.0x
                    } else {
                        9000  // Unlikely playoff team: 0.9x
                    };

                    // Calculate final multiplier (avoid overflow by using Balance and step-by-step division)
                    let temp = (multiplier.base_multiplier as Balance * 
                        multiplier.performance_multiplier as Balance) / 10000;
                    let temp2 = (temp * multiplier.rivalry_multiplier as Balance) / 10000;
                    let temp3 = (temp2 * multiplier.playoff_multiplier as Balance) / 10000;
                    multiplier.final_multiplier = ((temp3 * multiplier.demand_multiplier as Balance) / 10000) as u32;

                    multiplier.last_updated = self.env().block_timestamp();
                    self.pricing_multipliers.insert(team_id, &multiplier);
                }
            }
        }
    }

    impl SportsEvent {
        pub fn team_id(&self) -> u32 {
            self.home_team_id
        }
    }

    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    // ============================================================================
    // SPORTS-SPECIFIC BLOCKCHAIN EVENTS
    // ============================================================================

    #[ink(event)]
    pub struct TeamRegistered {
        #[ink(topic)]
        team_id: u32,
        #[ink(topic)]
        name: String,
        sport_type: SportType,
    }

    #[ink(event)]
    pub struct VenueRegistered {
        #[ink(topic)]
        venue_id: u32,
        #[ink(topic)]
        name: String,
        capacity: u32,
    }

    #[ink(event)]
    pub struct SeasonCreated {
        #[ink(topic)]
        season_id: u32,
        #[ink(topic)]
        name: String,
        sport_type: SportType,
    }

    #[ink(event)]
    pub struct SeasonPassPurchased {
        #[ink(topic)]
        pass_id: u64,
        #[ink(topic)]
        season_id: u32,
        #[ink(topic)]
        team_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        price: Balance,
        staking_enabled: bool,
    }

    #[ink(event)]
    pub struct SportsEventCreated {
        #[ink(topic)]
        event_id: u32,
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        home_team_id: u32,
        #[ink(topic)]
        away_team_id: u32,
        date: u64,
    }

    #[ink(event)]
    pub struct SportsTicketPurchased {
        #[ink(topic)]
        ticket_id: u64,
        #[ink(topic)]
        event_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        price: Balance,
        currency: CurrencyId,
        seat_type: SeatType,
        loyalty_points_earned: u32,
    }

    #[ink(event)]
    pub struct SportsEventSoldOut {
        #[ink(topic)]
        event_id: u32,
    }

    #[ink(event)]
    pub struct TeamPerformanceUpdated {
        #[ink(topic)]
        team_id: u32,
        wins: u32,
        losses: u32,
        playoff_probability: u32,
    }

    #[ink(event)]
    pub struct StakingRewardsClaimed {
        #[ink(topic)]
        user: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct UserProfileCreated {
        #[ink(topic)]
        user: AccountId,
        favorite_teams: Vec<u32>,
    }

    // ============================================================================
    // COMPREHENSIVE SPORTS-SPECIFIC TEST SUITE
    // ============================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn setup_contract() -> SportsBroker {
            SportsBroker::new()
        }

        fn setup_test_data(contract: &mut SportsBroker) -> (u32, u32, u32, u32) {
            // Create venue
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                "New York".to_string(),
                "NY".to_string(),
                "USA".to_string(),
                20000,
                VenueType::Arena,
            ).unwrap();

            // Create teams
            let home_team_id = contract.register_team(
                "New York Knicks".to_string(),
                "New York".to_string(),
                SportType::Basketball,
                "NBA".to_string(),
                venue_id,
            ).unwrap();

            let away_team_id = contract.register_team(
                "Boston Celtics".to_string(),
                "Boston".to_string(),
                SportType::Basketball,
                "NBA".to_string(),
                venue_id,
            ).unwrap();

            // Create season
            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000, // Oct 1, 2024
                1715644800000, // May 14, 2025
                82,
                500_000_000_000_000, // 0.5 DOT season pass
            ).unwrap();

            (venue_id, home_team_id, away_team_id, season_id)
        }

        // ========================================================================
        // INITIALIZATION TESTS
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let contract = SportsBroker::new();
            assert_eq!(contract.next_event_id, 1);
            assert_eq!(contract.next_team_id, 1);
            assert_eq!(contract.next_venue_id, 1);
            assert_eq!(contract.next_season_id, 1);
            assert_eq!(contract.supported_currencies.len(), 4); // DOT, ACA, aUSD, LDOT
        }

        // ========================================================================
        // TEAM & VENUE TESTS
        // ========================================================================

        #[ink::test]
        fn register_team_works() {
            let mut contract = setup_contract();
            let (venue_id, _, _, _) = setup_test_data(&mut contract);

            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
                "NBA".to_string(),
                venue_id,
            ).unwrap();

            assert_eq!(team_id, 3); // Should be 3rd team (after test setup)
            
            let team = contract.get_team(team_id).unwrap();
            assert_eq!(team.name, "Lakers");
            assert_eq!(team.sport_type, SportType::Basketball);
            assert!(team.verified);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut contract = setup_contract();

            let venue_id = contract.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                "CA".to_string(),
                "USA".to_string(),
                20000,
                VenueType::Arena,
            ).unwrap();

            assert_eq!(venue_id, 1);
            
            let venue = contract.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.capacity, 20000);
        }

        // ========================================================================
        // SEASON & SEASON PASS TESTS
        // ========================================================================

        #[ink::test]
        fn create_season_works() {
            let mut contract = setup_contract();

            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
                500_000_000_000_000,
            ).unwrap();

            assert_eq!(season_id, 1);
            
            let season = contract.get_season(season_id).unwrap();
            assert_eq!(season.name, "2024-25 NBA Season");
            assert_eq!(season.regular_season_games, 82);
            assert!(season.active);
        }

        #[ink::test]
        fn purchase_season_pass_insufficient_payment() {
            let mut contract = setup_contract();
            let (_, home_team_id, _, season_id) = setup_test_data(&mut contract);

            // Try to purchase without enough payment
            let result = contract.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                CurrencyId::DOT,
                false,
            );

            assert_eq!(result, Err(Error::InsufficientPayment));
        }

        // ========================================================================
        // SPORTS EVENT TESTS
        // ========================================================================

        #[ink::test]
        fn create_sports_event_works() {
            let mut contract = setup_contract();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut contract);

            let event_id = contract.create_sports_event(
                "Knicks vs Celtics".to_string(),
                venue_id,
                1704067200000, // Jan 1, 2024
                18000,
                50_000_000_000_000, // 0.05 DOT
                SportType::Basketball,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            assert_eq!(event_id, 1);
            
            let event = contract.get_sports_event(event_id).unwrap();
            assert_eq!(event.name, "Knicks vs Celtics");
            assert_eq!(event.home_team_id, home_team_id);
            assert_eq!(event.away_team_id, away_team_id);
            assert_eq!(event.sport_type, SportType::Basketball);
            assert!(event.fantasy_eligible);
            assert!(event.dynamic_pricing_enabled);
        }

        #[ink::test]
        fn create_sports_event_invalid_team() {
            let mut contract = setup_contract();
            let (venue_id, _, _, season_id) = setup_test_data(&mut contract);

            let result = contract.create_sports_event(
                "Invalid Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                SportType::Basketball,
                999, // Invalid team ID
                1,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::TeamNotFound));
        }

        // ========================================================================
        // SPORTS TICKET PURCHASING TESTS
        // ========================================================================

        #[ink::test]
        fn purchase_sports_ticket_insufficient_payment() {
            let mut contract = setup_contract();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut contract);

            contract.create_sports_event(
                "Test Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                SportType::Basketball,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let result = contract.purchase_sports_ticket(
                1,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::Reserved,
                CurrencyId::DOT,
                false,
            );

            assert_eq!(result, Err(Error::InsufficientPayment));
        }

        // ========================================================================
        // DYNAMIC PRICING TESTS
        // ========================================================================

        #[ink::test]
        fn update_team_performance_works() {
            let mut contract = setup_contract();
            let (_, home_team_id, _, season_id) = setup_test_data(&mut contract);

            let result = contract.update_team_performance(
                home_team_id,
                season_id,
                45, // wins
                15, // losses
                8500, // 85% playoff probability
            );

            assert_eq!(result, Ok(()));
            
            let performance = contract.get_team_performance(home_team_id).unwrap();
            assert_eq!(performance.wins, 45);
            assert_eq!(performance.losses, 15);
            assert_eq!(performance.playoff_probability, 8500);
            assert_eq!(performance.win_percentage, 7500); // 45/60 = 75%
        }

        // ========================================================================
        // MULTI-CURRENCY TESTS
        // ========================================================================

        #[ink::test]
        fn get_supported_currencies_works() {
            let contract = setup_contract();
            let currencies = contract.get_supported_currencies();
            
            assert_eq!(currencies.len(), 4);
            assert!(currencies.contains(&CurrencyId::DOT));
            assert!(currencies.contains(&CurrencyId::ACA));
            assert!(currencies.contains(&CurrencyId::AUSD));
            assert!(currencies.contains(&CurrencyId::LDOT));
        }

        // ========================================================================
        // USER PROFILE TESTS
        // ========================================================================

        #[ink::test]
        fn create_user_profile_works() {
            let mut contract = setup_contract();
            let (_, home_team_id, _, _) = setup_test_data(&mut contract);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            
            let favorite_teams = vec![home_team_id];
            let result = contract.create_user_profile(
                favorite_teams.clone(),
                "New York".to_string(),
                CurrencyId::DOT,
            );

            assert_eq!(result, Ok(()));
            
            let profile = contract.get_user_profile(accounts.alice).unwrap();
            assert_eq!(profile.favorite_teams, favorite_teams);
            assert_eq!(profile.home_city, "New York");
            assert_eq!(profile.loyalty_tier, LoyaltyTier::Bronze);
            assert!(!profile.verified_fan);
        }

        // ========================================================================
        // SEARCH FUNCTIONALITY TESTS
        // ========================================================================

        #[ink::test]
        fn search_events_by_team_works() {
            let mut contract = setup_contract();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut contract);

            // Create event with home team
            contract.create_sports_event(
                "Knicks Home Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                SportType::Basketball,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Create another event with away team as home
            contract.create_sports_event(
                "Celtics Home Game".to_string(),
                venue_id,
                1704153600000,
                18000,
                50_000_000_000_000,
                SportType::Basketball,
                away_team_id,
                home_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let knicks_games = contract.search_events_by_team(home_team_id);
            assert_eq!(knicks_games.len(), 2); // Should find both games (home and away)

            let celtics_games = contract.search_events_by_team(away_team_id);
            assert_eq!(celtics_games.len(), 2); // Should find both games (home and away)
        }

        #[ink::test]
        fn search_events_by_sport_works() {
            let mut contract = setup_contract();
            let (venue_id, home_team_id, away_team_id, season_id) = setup_test_data(&mut contract);

            contract.create_sports_event(
                "Basketball Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                SportType::Basketball,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let basketball_events = contract.search_events_by_sport(SportType::Basketball);
            assert_eq!(basketball_events.len(), 1);

            let football_events = contract.search_events_by_sport(SportType::Football);
            assert_eq!(football_events.len(), 0);
        }

        // ========================================================================
        // ERROR HANDLING TESTS
        // ========================================================================

        #[ink::test]
        fn comprehensive_error_handling() {
            let mut contract = setup_contract();
            
            // Test team not found
            assert_eq!(contract.get_team(999), None);
            
            // Test venue not found
            assert_eq!(contract.get_venue(999), None);
            
            // Test season not found
            assert_eq!(contract.get_season(999), None);
            
            // Test season pass not found
            assert_eq!(contract.get_season_pass(999), None);
            
            // Test unauthorized team registration
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(get_accounts().bob);
            let result = contract.register_team(
                "Unauthorized Team".to_string(),
                "City".to_string(),
                SportType::Basketball,
                "League".to_string(),
                1,
            );
            assert_eq!(result, Err(Error::Unauthorized));
        }

        // ========================================================================
        // LOYALTY SYSTEM TESTS
        // ========================================================================

        #[ink::test]
        fn loyalty_points_system_works() {
            let mut contract = setup_contract();
            let accounts = get_accounts();
            
            // Initially zero points
            assert_eq!(contract.get_user_loyalty_points(accounts.alice), 0);
            
            // Award some points
            contract.award_loyalty_points(accounts.alice, 500);
            assert_eq!(contract.get_user_loyalty_points(accounts.alice), 500);
            
            // Award more points to trigger tier upgrade
            contract.award_loyalty_points(accounts.alice, 1500);
            assert_eq!(contract.get_user_loyalty_points(accounts.alice), 2000);
            
            // Check if profile was updated with new tier
            if let Some(profile) = contract.get_user_profile(accounts.alice) {
                assert_eq!(profile.loyalty_tier, LoyaltyTier::Silver);
            }
        }
    }
}