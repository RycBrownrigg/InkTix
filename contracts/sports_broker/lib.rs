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

        // User Profile & Loyalty System (Step 6)
        user_profiles: ink::storage::Mapping<AccountId, UserProfile>,
        loyalty_points: ink::storage::Mapping<AccountId, u32>,
        team_fans: ink::storage::Mapping<u32, Vec<AccountId>>, // team_id -> fan list

        // Season Pass System (Step 7)
        season_passes: ink::storage::Mapping<u64, SeasonPass>,
        next_season_pass_id: u64,
        user_season_passes: ink::storage::Mapping<AccountId, Vec<u64>>,
        staking_rewards_pool: Balance,
        user_staked_amounts: ink::storage::Mapping<AccountId, Balance>,

        // NEW: Dynamic Pricing Engine (Step 8)
        team_performance: ink::storage::Mapping<u32, TeamPerformance>,
        pricing_multipliers: ink::storage::Mapping<u32, PricingMultiplier>,
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
        pub name: String, // "2024-25 NBA Season"
        pub sport_type: SportType,
        pub start_date: u64, // Unix timestamp
        pub end_date: u64,   // Unix timestamp
        pub regular_season_games: u32,
        pub active: bool,
        // Season pass pricing
        pub season_pass_base_price: Balance,
        pub early_bird_discount: u8, // percentage discount
        pub early_bird_deadline: u64,
    }

    /// Enhanced Event structure for sports
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SportsEvent {
        // Core fields
        pub id: u32,
        pub name: String,
        pub venue_id: u32,
        pub date: u64, // Unix timestamp
        pub capacity: u32,
        pub sold_tickets: u32,
        pub base_price: Balance, // Price in smallest unit (plancks for DOT)
        pub active: bool,
        
        // Sports-specific fields
        pub sport_type: SportType,
        pub home_team_id: u32,
        pub away_team_id: u32,
        pub season_id: u32,
        pub game_type: GameType,
        
        // Season pass benefits
        pub season_pass_discount: u8, // percentage discount for season pass holders

        // NEW: Dynamic pricing settings
        pub dynamic_pricing_enabled: bool,
        pub rivalry_multiplier: u32, // 10000 = 1.0x, 15000 = 1.5x (basis points)
    }

    /// Enhanced Ticket structure for sports
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SportsTicket {
        // Core fields
        pub id: u64,
        pub event_id: u32,
        pub owner: AccountId,
        pub purchase_price: Balance,
        pub purchase_date: u64,
        pub seat_number: u32,
        pub transferable: bool,
        
        // Sports-specific fields
        pub section: String,
        pub row: String,
        pub seat_type: SeatType,
        pub access_level: AccessLevel,

        // Loyalty integration
        pub loyalty_points_earned: u32,
        
        // Season pass integration
        pub season_pass_discount_applied: bool,
        pub is_season_pass_ticket: bool,

        // NEW: Dynamic pricing information
        pub dynamic_price_paid: Balance, // Final price after all multipliers
        pub performance_multiplier_applied: u32, // The multiplier that was used
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
    }

    /// Season pass for subscription management
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
        pub purchase_date: u64,
        pub games_included: u32,
        pub games_attended: u32,
        pub transferable: bool,
        pub includes_playoffs: bool,
        pub priority_level: u8, // Higher number = better seat access
        pub loyalty_tier_at_purchase: LoyaltyTier,
        pub staking_rewards_enabled: bool,
        pub staked_amount: Balance, // Amount earning staking rewards
        pub valid_until: u64, // Season end date
    }

    /// NEW: Team performance for dynamic pricing (Step 8)
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct TeamPerformance {
        pub team_id: u32,
        pub season_id: u32,
        pub wins: u32,
        pub losses: u32,
        pub win_percentage: u32, // 0-10000 (basis points, 7500 = 75%)
        pub streak: i32, // positive for wins, negative for losses
        pub playoff_probability: u32, // 0-10000 (basis points)
        pub last_updated: u64,
        pub performance_rank: u32, // 1 = best in league
        pub home_record_wins: u32,
        pub home_record_losses: u32,
        pub points_scored_avg: u32, // Average points per game (scaled by 100)
        pub points_allowed_avg: u32, // Average points allowed per game (scaled by 100)
    }

    /// NEW: Pricing multiplier based on various factors (Step 8)
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct PricingMultiplier {
        pub team_id: u32,
        pub base_multiplier: u32, // 10000 = 1.0x, 15000 = 1.5x (basis points)
        pub performance_multiplier: u32, // Based on win/loss record
        pub playoff_multiplier: u32, // Based on playoff probability
        pub streak_multiplier: u32, // Based on current win/loss streak
        pub rivalry_multiplier: u32, // Set per game for rivalry games
        pub demand_multiplier: u32, // Based on recent ticket sales
        pub final_multiplier: u32, // Calculated from all above factors
        pub last_updated: u64,
    }

    /// Sport types
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SportType {
        Basketball,
        Football,     // American Football
        Soccer,       // Football/Soccer
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
        FullSeason,      // All regular season games
        HalfSeason,      // Half of regular season games
        QuarterSeason,   // Quarter of regular season games
        Weekend,         // Weekend games only
        Weekday,         // Weekday games only
        Premium,         // Full season + playoffs + perks
        PlayoffsOnly,    // Playoff games only
        Package(u32),    // Custom number of games
    }

    /// Sports broker errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Team not found
        TeamNotFound,
        /// Venue not found
        VenueNotFound,
        /// Season not found
        SeasonNotFound,
        /// Event not found
        EventNotFound,
        /// Event not active
        EventNotActive,
        /// Event sold out
        EventSoldOut,
        /// Insufficient payment
        InsufficientPayment,
        /// Ticket not found
        TicketNotFound,
        /// Not ticket owner
        NotTicketOwner,
        /// Ticket not transferable
        TicketNotTransferable,
        /// Insufficient capacity
        InsufficientCapacity,
        /// ID overflow
        IdOverflow,
        /// Profile already exists
        ProfileAlreadyExists,
        /// Profile not found
        ProfileNotFound,
        /// Invalid favorite teams
        InvalidFavoriteTeams,
        // Season pass errors
        /// Season pass not found
        SeasonPassNotFound,
        /// Not season pass owner
        NotSeasonPassOwner,
        /// Season not active for passes
        SeasonNotActive,
        /// Season pass not transferable
        SeasonPassNotTransferable,
        /// Season pass expired
        SeasonPassExpired,
        /// Insufficient staking rewards
        InsufficientStakingRewards,
        /// Staking not enabled
        StakingNotEnabled,
        // NEW: Dynamic pricing errors
        /// Performance data not found
        PerformanceDataNotFound,
        /// Pricing data outdated
        PricingDataOutdated,
        /// Dynamic pricing disabled for event
        DynamicPricingDisabled,
        /// Invalid performance statistics
        InvalidPerformanceStats,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl SportsBroker {
        /// Creates a new Sports Broker contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
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
                // Step 6 storage
                user_profiles: ink::storage::Mapping::new(),
                loyalty_points: ink::storage::Mapping::new(),
                team_fans: ink::storage::Mapping::new(),
                // Step 7 storage
                season_passes: ink::storage::Mapping::new(),
                next_season_pass_id: 1,
                user_season_passes: ink::storage::Mapping::new(),
                staking_rewards_pool: 0,
                user_staked_amounts: ink::storage::Mapping::new(),
                // NEW: Step 8 storage initialization
                team_performance: ink::storage::Mapping::new(),
                pricing_multipliers: ink::storage::Mapping::new(),
            }
        }

        // ========================================================================
        // TEAM & VENUE MANAGEMENT (Steps 1-2)
        // ========================================================================

        /// Register a new sports team
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
            
            // Initialize empty fan list for new team
            self.team_fans.insert(team_id, &Vec::<AccountId>::new());
            
            // NEW: Initialize performance tracking for new team
            let performance = TeamPerformance {
                team_id,
                season_id: 0, // Will be updated when assigned to season
                wins: 0,
                losses: 0,
                win_percentage: 0,
                streak: 0,
                playoff_probability: 5000, // 50% default
                last_updated: self.env().block_timestamp(),
                performance_rank: 0,
                home_record_wins: 0,
                home_record_losses: 0,
                points_scored_avg: 10000, // 100.00 points average
                points_allowed_avg: 10000, // 100.00 points average
            };
            self.team_performance.insert(team_id, &performance);

            // NEW: Initialize pricing multiplier for new team
            let pricing = PricingMultiplier {
                team_id,
                base_multiplier: 10000, // 1.0x
                performance_multiplier: 10000, // 1.0x
                playoff_multiplier: 10000, // 1.0x
                streak_multiplier: 10000, // 1.0x
                rivalry_multiplier: 10000, // 1.0x
                demand_multiplier: 10000, // 1.0x
                final_multiplier: 10000, // 1.0x
                last_updated: self.env().block_timestamp(),
            };
            self.pricing_multipliers.insert(team_id, &pricing);
            
            Ok(team_id)
        }

        /// Register a new venue
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
            Ok(venue_id)
        }

        // ========================================================================
        // SEASON MANAGEMENT (Step 3 + Enhanced for Step 7)
        // ========================================================================

        /// Create a new sports season with season pass pricing
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
                // Season pass pricing defaults
                season_pass_base_price: 500_000_000_000_000, // 0.5 DOT
                early_bird_discount: 10, // 10% early bird discount
                early_bird_deadline: start_date, // Default to season start
            };

            self.seasons.insert(season_id, &season);
            Ok(season_id)
        }

        /// Update season pass pricing
        #[ink(message)]
        pub fn update_season_pass_pricing(
            &mut self,
            season_id: u32,
            base_price: Balance,
            early_bird_discount: u8,
            early_bird_deadline: u64,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;
            season.season_pass_base_price = base_price;
            season.early_bird_discount = early_bird_discount;
            season.early_bird_deadline = early_bird_deadline;
            
            self.seasons.insert(season_id, &season);
            Ok(())
        }

        // ========================================================================
        // SPORTS EVENT MANAGEMENT (Step 4 + Enhanced for Steps 7-8)
        // ========================================================================

        /// Create a sports event with dynamic pricing enabled
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

            // Validate venue exists
            let venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            
            // Validate home team exists
            let home_team = self.teams.get(home_team_id).ok_or(Error::TeamNotFound)?;
            
            // Validate away team exists
            let _away_team = self.teams.get(away_team_id).ok_or(Error::TeamNotFound)?;
            
            // Validate season exists
            let _season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;

            // Use venue capacity as default if provided capacity is 0
            let event_capacity = if capacity == 0 { venue.capacity } else { capacity };

            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            // NEW: Determine rivalry multiplier based on team matchup
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
                
                // Sports-specific fields
                sport_type: home_team.sport_type.clone(),
                home_team_id,
                away_team_id,
                season_id,
                game_type,
                
                // Season pass benefits
                season_pass_discount: 15, // 15% discount for season pass holders

                // NEW: Dynamic pricing settings
                dynamic_pricing_enabled: true,
                rivalry_multiplier,
            };

            self.events.insert(event_id, &sports_event);

            // NEW: Update pricing multipliers for both teams
            self.update_event_pricing_multipliers(home_team_id, away_team_id, &game_type, rivalry_multiplier);

            Ok(event_id)
        }

        // ========================================================================
        // ENHANCED TICKET PURCHASING (Step 5 + Enhanced for Steps 7-8)
        // ========================================================================

        /// Purchase a sports ticket with dynamic pricing
        #[ink(message, payable)]
        pub fn purchase_sports_ticket(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_type: SeatType,
        ) -> Result<u64> {
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

            // NEW: Calculate dynamic price with all factors
            let (final_price, season_pass_discount_applied, is_season_pass_ticket, performance_multiplier) = 
                self.calculate_comprehensive_ticket_price(buyer, &event, &seat_type)?;
            
            if payment < final_price {
                return Err(Error::InsufficientPayment);
            }

            // Calculate loyalty points for this purchase
            let loyalty_points_earned = self.calculate_loyalty_points(&seat_type, payment);

            // Create enhanced sports ticket
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
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
                
                // Sports-specific fields
                section,
                row,
                seat_type,
                access_level: self.determine_access_level(&seat_type),

                // Loyalty integration
                loyalty_points_earned,
                
                // Season pass integration
                season_pass_discount_applied,
                is_season_pass_ticket,

                // NEW: Dynamic pricing information
                dynamic_price_paid: final_price,
                performance_multiplier_applied: performance_multiplier,
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

            // Update user attendance and season pass usage
            self.update_user_attendance(buyer, event.home_team_id);
            if is_season_pass_ticket {
                self.update_season_pass_usage(buyer, event.season_id, event.home_team_id);
            }

            // NEW: Update demand multiplier based on sales
            self.update_demand_multiplier(event.home_team_id, event.sold_tickets, event.capacity);

            Ok(ticket_id)
        }

        /// Transfer a ticket to another user
        #[ink(message)]
        pub fn transfer_ticket(
            &mut self,
            ticket_id: u64,
            new_owner: AccountId,
        ) -> Result<()> {
            let caller = self.env().caller();
            
            let mut ticket = self.tickets.get(ticket_id).ok_or(Error::TicketNotFound)?;
            if ticket.owner != caller {
                return Err(Error::NotTicketOwner);
            }
            if !ticket.transferable {
                return Err(Error::TicketNotTransferable);
            }

            // Remove from current owner's list
            let mut current_owner_tickets = self.user_tickets.get(caller).unwrap_or_default();
            current_owner_tickets.retain(|&x| x != ticket_id);
            self.user_tickets.insert(caller, &current_owner_tickets);

            // Add to new owner's list
            let mut new_owner_tickets = self.user_tickets.get(new_owner).unwrap_or_default();
            new_owner_tickets.push(ticket_id);
            self.user_tickets.insert(new_owner, &new_owner_tickets);

            // Update ticket ownership
            ticket.owner = new_owner;
            self.tickets.insert(ticket_id, &ticket);

            Ok(())
        }

        // ========================================================================
        // USER PROFILE & LOYALTY SYSTEM (Step 6)
        // ========================================================================

        /// Create or update user profile
        #[ink(message)]
        pub fn create_user_profile(
            &mut self,
            favorite_teams: Vec<u32>,
            home_city: String,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Check if profile already exists
            if self.user_profiles.get(caller).is_some() {
                return Err(Error::ProfileAlreadyExists);
            }

            // Validate favorite teams exist
            for team_id in &favorite_teams {
                if self.teams.get(*team_id).is_none() {
                    return Err(Error::InvalidFavoriteTeams);
                }
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
            };

            self.user_profiles.insert(caller, &profile);

            // Add user to team fan lists
            for team_id in &favorite_teams {
                let mut fans = self.team_fans.get(*team_id).unwrap_or_default();
                if !fans.contains(&caller) {
                    fans.push(caller);
                    self.team_fans.insert(*team_id, &fans);
                }
            }

            Ok(())
        }

        /// Update user profile
        #[ink(message)]
        pub fn update_user_profile(
            &mut self,
            favorite_teams: Vec<u32>,
            home_city: String,
        ) -> Result<()> {
            let caller = self.env().caller();

            let mut profile = self.user_profiles.get(caller).ok_or(Error::ProfileNotFound)?;

            // Validate favorite teams exist
            for team_id in &favorite_teams {
                if self.teams.get(*team_id).is_none() {
                    return Err(Error::InvalidFavoriteTeams);
                }
            }

            // Remove from old team fan lists
            for old_team_id in &profile.favorite_teams {
                let mut fans = self.team_fans.get(*old_team_id).unwrap_or_default();
                fans.retain(|&x| x != caller);
                self.team_fans.insert(*old_team_id, &fans);
            }

            // Add to new team fan lists
            for team_id in &favorite_teams {
                let mut fans = self.team_fans.get(*team_id).unwrap_or_default();
                if !fans.contains(&caller) {
                    fans.push(caller);
                    self.team_fans.insert(*team_id, &fans);
                }
            }

            // Update profile
            profile.favorite_teams = favorite_teams;
            profile.home_city = home_city;
            self.user_profiles.insert(caller, &profile);

            Ok(())
        }

        /// Verify user as authentic fan (owner only)
        #[ink(message)]
        pub fn verify_user_fan(&mut self, user: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut profile = self.user_profiles.get(user).ok_or(Error::ProfileNotFound)?;
            profile.verified_fan = true;
            profile.anti_scalping_verified = true;
            self.user_profiles.insert(user, &profile);

            Ok(())
        }

        /// Award loyalty points to user
        #[ink(message)]
        pub fn award_loyalty_points(&mut self, user: AccountId, points: u32) {
            let current_points = self.loyalty_points.get(user).unwrap_or(0);
            let new_total = current_points.saturating_add(points);
            self.loyalty_points.insert(user, &new_total);

            // Update loyalty tier based on total points
            let new_tier = self.calculate_loyalty_tier(new_total);

            // Update user profile with new tier if profile exists
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.loyalty_tier = new_tier;
                self.user_profiles.insert(user, &profile);
            }
        }

        // ========================================================================
        // SEASON PASS SYSTEM (Step 7)
        // ========================================================================

        /// Purchase a season pass with optional staking rewards
        #[ink(message, payable)]
        pub fn purchase_season_pass(
            &mut self,
            season_id: u32,
            team_id: u32,
            pass_type: SeasonPassType,
            enable_staking: bool,
        ) -> Result<u64> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            // Validate season exists and is active
            let season = self.seasons.get(season_id).ok_or(Error::SeasonNotFound)?;
            if !season.active {
                return Err(Error::SeasonNotActive);
            }

            // Validate team exists
            let _team = self.teams.get(team_id).ok_or(Error::TeamNotFound)?;

            // Calculate season pass price
            let final_price = self.calculate_season_pass_price(&season, &pass_type, buyer)?;
            
            if payment < final_price {
                return Err(Error::InsufficientPayment);
            }

            // Create season pass
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
                purchase_date: self.env().block_timestamp(),
                games_included,
                games_attended: 0,
                transferable: true,
                includes_playoffs: matches!(pass_type, SeasonPassType::Premium),
                priority_level: self.calculate_priority_level(&loyalty_tier),
                loyalty_tier_at_purchase: loyalty_tier,
                staking_rewards_enabled: enable_staking,
                staked_amount: if enable_staking { payment } else { 0 },
                valid_until: season.end_date,
            };

            // Enable staking rewards if requested
            if enable_staking {
                self.staking_rewards_pool = self.staking_rewards_pool.saturating_add(payment);
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

            // Award loyalty points for season pass purchase
            let loyalty_points_earned = self.calculate_season_pass_loyalty_points(&pass_type);
            self.award_loyalty_points(buyer, loyalty_points_earned);

            Ok(pass_id)
        }

        /// Transfer season pass to another user
        #[ink(message)]
        pub fn transfer_season_pass(
            &mut self,
            pass_id: u64,
            new_owner: AccountId,
        ) -> Result<()> {
            let caller = self.env().caller();
            
            let mut season_pass = self.season_passes.get(pass_id).ok_or(Error::SeasonPassNotFound)?;
            if season_pass.owner != caller {
                return Err(Error::NotSeasonPassOwner);
            }
            if !season_pass.transferable {
                return Err(Error::SeasonPassNotTransferable);
            }

            // Check if season pass is still valid
            let current_time = self.env().block_timestamp();
            if current_time > season_pass.valid_until {
                return Err(Error::SeasonPassExpired);
            }

            // Remove from current owner's list
            let mut current_owner_passes = self.user_season_passes.get(caller).unwrap_or_default();
            current_owner_passes.retain(|&x| x != pass_id);
            self.user_season_passes.insert(caller, &current_owner_passes);

            // Add to new owner's list
            let mut new_owner_passes = self.user_season_passes.get(new_owner).unwrap_or_default();
            new_owner_passes.push(pass_id);
            self.user_season_passes.insert(new_owner, &new_owner_passes);

            // Update season pass ownership
            season_pass.owner = new_owner;
            self.season_passes.insert(pass_id, &season_pass);

            // Update user profiles
            self.update_user_profile_season_pass_status(caller);
            self.update_user_profile_for_season_pass(new_owner, season_pass.team_id);

            Ok(())
        }

        /// Claim staking rewards for season pass holders
        #[ink(message)]
        pub fn claim_staking_rewards(&mut self) -> Result<Balance> {
            let caller = self.env().caller();
            
            let staked_amount = self.user_staked_amounts.get(caller).unwrap_or(0);
            if staked_amount == 0 {
                return Err(Error::StakingNotEnabled);
            }

            // Calculate rewards (simplified staking simulation)
            let annual_rate: Balance = 800; // 8% APY in basis points
            let seconds_per_year: Balance = 31_536_000;
            
            // For demo: assume 30 days since last claim
            let time_elapsed: Balance = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds
            let rewards = (staked_amount * annual_rate * time_elapsed) / (10000 * seconds_per_year * 1000);

            if rewards > self.staking_rewards_pool {
                return Err(Error::InsufficientStakingRewards);
            }

            // In a real implementation, this would transfer tokens to the user
            self.staking_rewards_pool = self.staking_rewards_pool.saturating_sub(rewards);

            Ok(rewards)
        }

        // ========================================================================
        // NEW: DYNAMIC PRICING ENGINE (Step 8)
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
            streak: i32,
            points_scored_avg: u32,
            points_allowed_avg: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Validate team exists
            let _team = self.teams.get(team_id).ok_or(Error::TeamNotFound)?;

            // Validate performance statistics
            if playoff_probability > 10000 {
                return Err(Error::InvalidPerformanceStats);
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
                streak,
                playoff_probability,
                last_updated: self.env().block_timestamp(),
                performance_rank: 0, // Would be calculated relative to league
                home_record_wins: 0, // Simplified for now
                home_record_losses: 0,
                points_scored_avg,
                points_allowed_avg,
            };

            self.team_performance.insert(team_id, &performance);

            // Recalculate pricing multiplier based on new performance
            self.recalculate_performance_pricing_multiplier(team_id)?;

            Ok(())
        }

        /// Set rivalry multiplier for specific team matchups
        #[ink(message)]
        pub fn set_rivalry_multiplier(
            &mut self,
            team1_id: u32,
            team2_id: u32,
            multiplier: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Update rivalry multiplier for both teams when they play each other
            if let Some(mut pricing1) = self.pricing_multipliers.get(team1_id) {
                pricing1.rivalry_multiplier = multiplier;
                pricing1.last_updated = self.env().block_timestamp();
                self.pricing_multipliers.insert(team1_id, &pricing1);
            }

            if let Some(mut pricing2) = self.pricing_multipliers.get(team2_id) {
                pricing2.rivalry_multiplier = multiplier;
                pricing2.last_updated = self.env().block_timestamp();
                self.pricing_multipliers.insert(team2_id, &pricing2);
            }

            Ok(())
        }

        /// Enable or disable dynamic pricing for an event
        #[ink(message)]
        pub fn set_event_dynamic_pricing(
            &mut self,
            event_id: u32,
            enabled: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.events.get(event_id).ok_or(Error::EventNotFound)?;
            event.dynamic_pricing_enabled = enabled;
            self.events.insert(event_id, &event);

            Ok(())
        }

        // ========================================================================
        // HELPER METHODS (Enhanced for Dynamic Pricing)
        // ========================================================================

        /// Calculate comprehensive ticket price with all dynamic factors
        fn calculate_comprehensive_ticket_price(
            &self,
            buyer: AccountId,
            event: &SportsEvent,
            seat_type: &SeatType,
        ) -> Result<(Balance, bool, bool, u32)> {
            // Start with base seat price
            let mut final_price = self.calculate_seat_price(event.base_price, seat_type);
            let mut performance_multiplier = 10000; // 1.0x default

            // Apply dynamic pricing if enabled
            if event.dynamic_pricing_enabled {
                if let Some(pricing) = self.pricing_multipliers.get(event.home_team_id) {
                    performance_multiplier = pricing.final_multiplier;
                    final_price = (final_price * pricing.final_multiplier as Balance) / 10000;
                }
            }

            // Check for season pass benefits
            let (discounted_price, season_pass_discount_applied, is_season_pass_ticket) = 
                self.apply_season_pass_discount(buyer, event, final_price)?;

            Ok((discounted_price, season_pass_discount_applied, is_season_pass_ticket, performance_multiplier))
        }

        /// Calculate rivalry multiplier based on team matchup
        fn calculate_rivalry_multiplier(&self, home_team_id: u32, away_team_id: u32) -> u32 {
            // Simplified rivalry detection - in practice this would use a rivalry matrix
            // For now, same city teams or certain combinations get rivalry bonus
            if let (Some(home_team), Some(away_team)) = (self.teams.get(home_team_id), self.teams.get(away_team_id)) {
                if home_team.city == away_team.city {
                    return 12000; // 1.2x for same city rivals
                }
                
                // Specific rivalries could be hardcoded or configurable
                match (home_team.name.as_str(), away_team.name.as_str()) {
                    ("Lakers", "Celtics") | ("Celtics", "Lakers") => 15000, // 1.5x
                    ("Yankees", "Red Sox") | ("Red Sox", "Yankees") => 15000, // 1.5x
                    _ => 10000, // 1.0x default
                }
            } else {
                10000 // 1.0x default
            }
        }

        /// Update pricing multipliers for event creation
        fn update_event_pricing_multipliers(&mut self, home_team_id: u32, away_team_id: u32, game_type: &GameType, rivalry_multiplier: u32) {
            // Update home team pricing
            if let Some(mut home_pricing) = self.pricing_multipliers.get(home_team_id) {
                home_pricing.rivalry_multiplier = rivalry_multiplier;
                
                // Apply game type multiplier
                home_pricing.base_multiplier = match game_type {
                    GameType::RegularSeason => 10000,  // 1.0x
                    GameType::Playoff => 15000,        // 1.5x
                    GameType::Championship => 25000,   // 2.5x
                    GameType::AllStar => 20000,        // 2.0x
                    GameType::Preseason => 7500,       // 0.75x
                    GameType::Tournament => 18000,     // 1.8x
                    GameType::Exhibition => 8000,      // 0.8x
                };

                self.recalculate_final_multiplier(&mut home_pricing);
                self.pricing_multipliers.insert(home_team_id, &home_pricing);
            }

            // Update away team pricing (less impact since it's away game)
            if let Some(mut away_pricing) = self.pricing_multipliers.get(away_team_id) {
                away_pricing.rivalry_multiplier = (rivalry_multiplier + 10000) / 2; // Average with 1.0x
                self.recalculate_final_multiplier(&mut away_pricing);
                self.pricing_multipliers.insert(away_team_id, &away_pricing);
            }
        }

        /// Recalculate performance-based pricing multiplier
        fn recalculate_performance_pricing_multiplier(&mut self, team_id: u32) -> Result<()> {
            let mut pricing = self.pricing_multipliers.get(team_id).ok_or(Error::PerformanceDataNotFound)?;
            let performance = self.team_performance.get(team_id).ok_or(Error::PerformanceDataNotFound)?;

            // Performance-based multiplier (winning teams cost more)
            pricing.performance_multiplier = if performance.win_percentage > 7500 {
                12000 // Great team (>75% wins): 1.2x
            } else if performance.win_percentage > 6000 {
                11000 // Good team (60-75% wins): 1.1x
            } else if performance.win_percentage > 4000 {
                10000 // Average team (40-60% wins): 1.0x
            } else {
                9000  // Poor team (<40% wins): 0.9x
            };

            // Playoff probability multiplier
            pricing.playoff_multiplier = if performance.playoff_probability > 8000 {
                11500 // Very likely playoff team (>80%): 1.15x
            } else if performance.playoff_probability > 6000 {
                10500 // Likely playoff team (60-80%): 1.05x
            } else if performance.playoff_probability > 4000 {
                10000 // Bubble team (40-60%): 1.0x
            } else {
                9500  // Unlikely playoff team (<40%): 0.95x
            };

            // Streak multiplier (hot teams cost more)
            pricing.streak_multiplier = if performance.streak >= 5 {
                11000 // Hot streak (5+ wins): 1.1x
            } else if performance.streak >= 3 {
                10500 // Good streak (3-4 wins): 1.05x
            } else if performance.streak <= -5 {
                9000  // Cold streak (5+ losses): 0.9x
            } else if performance.streak <= -3 {
                9500  // Bad streak (3-4 losses): 0.95x
            } else {
                10000 // Neutral: 1.0x
            };

            self.recalculate_final_multiplier(&mut pricing);
            pricing.last_updated = self.env().block_timestamp();
            self.pricing_multipliers.insert(team_id, &pricing);

            Ok(())
        }

        /// Recalculate final multiplier from all components
        fn recalculate_final_multiplier(&self, pricing: &mut PricingMultiplier) {
            // Combine all multipliers (using safe math to avoid overflow)
            let temp1 = (pricing.base_multiplier as Balance * pricing.performance_multiplier as Balance) / 10000;
            let temp2 = (temp1 * pricing.playoff_multiplier as Balance) / 10000;
            let temp3 = (temp2 * pricing.streak_multiplier as Balance) / 10000;
            let temp4 = (temp3 * pricing.rivalry_multiplier as Balance) / 10000;
            let final_result = (temp4 * pricing.demand_multiplier as Balance) / 10000;
            
            pricing.final_multiplier = final_result as u32;
        }

        /// Update demand multiplier based on sales velocity
        fn update_demand_multiplier(&mut self, team_id: u32, sold_tickets: u32, capacity: u32) {
            if let Some(mut pricing) = self.pricing_multipliers.get(team_id) {
                let sell_through_percentage = (sold_tickets * 100) / capacity;
                
                pricing.demand_multiplier = match sell_through_percentage {
                    90.. => 13000,   // 90%+ sold: 1.3x (high demand)
                    75..=89 => 11500, // 75-89% sold: 1.15x
                    50..=74 => 10000, // 50-74% sold: 1.0x (normal)
                    25..=49 => 9500,  // 25-49% sold: 0.95x
                    _       => 9000,  // <25% sold: 0.9x (low demand)
                };

                self.recalculate_final_multiplier(&mut pricing);
                pricing.last_updated = self.env().block_timestamp();
                self.pricing_multipliers.insert(team_id, &pricing);
            }
        }

        /// Apply season pass discount to ticket purchase
        fn apply_season_pass_discount(
            &self,
            buyer: AccountId,
            event: &SportsEvent,
            base_price: Balance,
        ) -> Result<(Balance, bool, bool)> {
            // Check if user has valid season pass for this team/season
            if let Some(user_passes) = self.user_season_passes.get(buyer) {
                for pass_id in user_passes {
                    if let Some(season_pass) = self.season_passes.get(pass_id) {
                        if season_pass.team_id == event.home_team_id 
                            && season_pass.season_id == event.season_id
                            && season_pass.games_attended < season_pass.games_included
                            && self.env().block_timestamp() <= season_pass.valid_until {
                            
                            // Apply season pass discount
                            let discount_amount = (base_price * event.season_pass_discount as Balance) / 100;
                            let final_price = base_price.saturating_sub(discount_amount);
                            
                            return Ok((final_price, true, true));
                        }
                    }
                }
            }
            
            // No valid season pass found
            Ok((base_price, false, false))
        }

        /// Update user attendance count
        fn update_user_attendance(&mut self, user: AccountId, _team_id: u32) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.total_games_attended = profile.total_games_attended.saturating_add(1);
                self.user_profiles.insert(user, &profile);
            }
        }

        /// Update season pass usage when ticket is used
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

        /// Calculate season pass price with discounts
        fn calculate_season_pass_price(
            &self,
            season: &Season,
            pass_type: &SeasonPassType,
            buyer: AccountId,
        ) -> Result<Balance> {
            let mut final_price = season.season_pass_base_price;

            // Apply pass type multiplier
            let type_multiplier = match pass_type {
                SeasonPassType::FullSeason => 100,       // 1.0x
                SeasonPassType::HalfSeason => 55,        // 0.55x
                SeasonPassType::QuarterSeason => 30,     // 0.30x
                SeasonPassType::Weekend => 60,           // 0.60x
                SeasonPassType::Weekday => 45,           // 0.45x
                SeasonPassType::Premium => 150,          // 1.5x
                SeasonPassType::PlayoffsOnly => 80,      // 0.80x
                SeasonPassType::Package(_) => 75,        // 0.75x
            };
            
            final_price = (final_price * type_multiplier) / 100;

            // Apply early bird discount if applicable
            let current_time = self.env().block_timestamp();
            if current_time <= season.early_bird_deadline {
                let discount_amount = (final_price * season.early_bird_discount as Balance) / 100;
                final_price = final_price.saturating_sub(discount_amount);
            }

            // Apply loyalty tier discount
            let loyalty_tier = self.get_user_loyalty_tier(buyer);
            let loyalty_discount = match loyalty_tier {
                LoyaltyTier::Bronze => 0,
                LoyaltyTier::Silver => 5,    // 5% discount
                LoyaltyTier::Gold => 10,     // 10% discount
                LoyaltyTier::Platinum => 15, // 15% discount
                LoyaltyTier::Diamond => 20,  // 20% discount
            };

            if loyalty_discount > 0 {
                let discount_amount = (final_price * loyalty_discount) / 100;
                final_price = final_price.saturating_sub(discount_amount);
            }

            Ok(final_price)
        }

        /// Calculate games included in season pass
        fn calculate_games_included(&self, pass_type: &SeasonPassType, total_games: u32) -> u32 {
            match pass_type {
                SeasonPassType::FullSeason => total_games,
                SeasonPassType::HalfSeason => total_games / 2,
                SeasonPassType::QuarterSeason => total_games / 4,
                SeasonPassType::Weekend => total_games * 40 / 100, // ~40% are weekend games
                SeasonPassType::Weekday => total_games * 60 / 100,
                SeasonPassType::Premium => total_games + 4, // Includes estimated playoff games
                SeasonPassType::PlayoffsOnly => 16, // Estimated playoff games
                SeasonPassType::Package(games) => *games,
            }
        }

        /// Calculate loyalty points for season pass purchase
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

        /// Update user profile for season pass purchase
        fn update_user_profile_for_season_pass(&mut self, user: AccountId, team_id: u32) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                profile.season_pass_holder = true;
                if !profile.favorite_teams.contains(&team_id) {
                    profile.favorite_teams.push(team_id);
                }
                self.user_profiles.insert(user, &profile);
            }
        }

        /// Update user profile season pass status
        fn update_user_profile_season_pass_status(&mut self, user: AccountId) {
            if let Some(mut profile) = self.user_profiles.get(user) {
                // Check if user still has any valid season passes
                let has_valid_passes = if let Some(user_passes) = self.user_season_passes.get(user) {
                    let current_time = self.env().block_timestamp();
                    user_passes.iter().any(|&pass_id| {
                        if let Some(season_pass) = self.season_passes.get(pass_id) {
                            current_time <= season_pass.valid_until
                        } else {
                            false
                        }
                    })
                } else {
                    false
                };
                
                profile.season_pass_holder = has_valid_passes;
                self.user_profiles.insert(user, &profile);
            }
        }

        /// Get user loyalty tier
        fn get_user_loyalty_tier(&self, user: AccountId) -> LoyaltyTier {
            if let Some(profile) = self.user_profiles.get(user) {
                profile.loyalty_tier
            } else {
                LoyaltyTier::Bronze
            }
        }

        /// Calculate priority level based on loyalty tier
        fn calculate_priority_level(&self, tier: &LoyaltyTier) -> u8 {
            match tier {
                LoyaltyTier::Bronze => 1,
                LoyaltyTier::Silver => 2,
                LoyaltyTier::Gold => 3,
                LoyaltyTier::Platinum => 4,
                LoyaltyTier::Diamond => 5,
            }
        }

        /// Calculate loyalty tier based on points
        fn calculate_loyalty_tier(&self, total_points: u32) -> LoyaltyTier {
            match total_points {
                0..=999 => LoyaltyTier::Bronze,
                1000..=2999 => LoyaltyTier::Silver,
                3000..=6999 => LoyaltyTier::Gold,
                7000..=14999 => LoyaltyTier::Platinum,
                15000.. => LoyaltyTier::Diamond,
            }
        }

        /// Calculate loyalty points for ticket purchase
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

            // Bonus points based on ticket price (1 point per 0.01 DOT spent)
            let price_bonus = (price / 10_000_000_000) as u32; // Convert from plancks to 0.01 DOT units
            
            base_points + price_bonus
        }

        /// Calculate seat price based on seat type
        fn calculate_seat_price(&self, base_price: Balance, seat_type: &SeatType) -> Balance {
            let multiplier = match seat_type {
                SeatType::GeneralAdmission => 100,      // 1.0x
                SeatType::Reserved => 120,              // 1.2x
                SeatType::PremiumReserved => 150,       // 1.5x
                SeatType::Club => 200,                  // 2.0x
                SeatType::Suite => 500,                 // 5.0x
                SeatType::FieldLevel => 300,            // 3.0x
                SeatType::Courtside => 800,             // 8.0x
                SeatType::StudentSection => 50,         // 0.5x
            };
            
            (base_price * multiplier) / 100
        }

        /// Determine access level based on seat type
        fn determine_access_level(&self, seat_type: &SeatType) -> AccessLevel {
            match seat_type {
                SeatType::GeneralAdmission | SeatType::StudentSection => AccessLevel::Standard,
                SeatType::Reserved | SeatType::PremiumReserved => AccessLevel::Premium,
                SeatType::Club | SeatType::FieldLevel => AccessLevel::VIP,
                SeatType::Suite | SeatType::Courtside => AccessLevel::AllAccess,
            }
        }

        // ========================================================================
        // QUERY METHODS (Enhanced for Dynamic Pricing)
        // ========================================================================

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

        /// Get sports event information
        #[ink(message)]
        pub fn get_sports_event(&self, event_id: u32) -> Option<SportsEvent> {
            self.events.get(event_id)
        }

        /// Get sports ticket information
        #[ink(message)]
        pub fn get_sports_ticket(&self, ticket_id: u64) -> Option<SportsTicket> {
            self.tickets.get(ticket_id)
        }

        /// Get user's tickets
        #[ink(message)]
        pub fn get_user_tickets(&self, user: AccountId) -> Vec<u64> {
            self.user_tickets.get(user).unwrap_or_default()
        }

        /// Get current dynamic ticket price with all modifiers
        #[ink(message)]
        pub fn get_current_ticket_price(
            &self,
            event_id: u32,
            seat_type: SeatType,
            user: AccountId,
        ) -> Option<Balance> {
            let event = self.events.get(event_id)?;
            let (price, _, _, _) = self.calculate_comprehensive_ticket_price(user, &event, &seat_type).ok()?;
            Some(price)
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

        /// Get team fans list
        #[ink(message)]
        pub fn get_team_fans(&self, team_id: u32) -> Vec<AccountId> {
            self.team_fans.get(team_id).unwrap_or_default()
        }

        /// Get season pass information
        #[ink(message)]
        pub fn get_season_pass(&self, pass_id: u64) -> Option<SeasonPass> {
            self.season_passes.get(pass_id)
        }

        /// Get user's season passes
        #[ink(message)]
        pub fn get_user_season_passes(&self, user: AccountId) -> Vec<u64> {
            self.user_season_passes.get(user).unwrap_or_default()
        }

        /// Get season pass price quote
        #[ink(message)]
        pub fn get_season_pass_price(
            &self,
            season_id: u32,
            pass_type: SeasonPassType,
            user: AccountId,
        ) -> Option<Balance> {
            let season = self.seasons.get(season_id)?;
            self.calculate_season_pass_price(&season, &pass_type, user).ok()
        }

        /// Get user's staked amount
        #[ink(message)]
        pub fn get_user_staked_amount(&self, user: AccountId) -> Balance {
            self.user_staked_amounts.get(user).unwrap_or(0)
        }

        /// Check if user has valid season pass for team/season
        #[ink(message)]
        pub fn has_valid_season_pass(&self, user: AccountId, team_id: u32, season_id: u32) -> bool {
            if let Some(user_passes) = self.user_season_passes.get(user) {
                let current_time = self.env().block_timestamp();
                for pass_id in user_passes {
                    if let Some(season_pass) = self.season_passes.get(pass_id) {
                        if season_pass.team_id == team_id 
                            && season_pass.season_id == season_id
                            && current_time <= season_pass.valid_until {
                            return true;
                        }
                    }
                }
            }
            false
        }

        // NEW: Dynamic pricing queries

        /// Get team performance data
        #[ink(message)]
        pub fn get_team_performance(&self, team_id: u32) -> Option<TeamPerformance> {
            self.team_performance.get(team_id)
        }

        /// Get pricing multiplier data
        #[ink(message)]
        pub fn get_pricing_multiplier(&self, team_id: u32) -> Option<PricingMultiplier> {
            self.pricing_multipliers.get(team_id)
        }

        /// Get dynamic price breakdown for analysis
        #[ink(message)]
        pub fn get_price_breakdown(
            &self,
            event_id: u32,
            seat_type: SeatType,
            user: AccountId,
        ) -> Option<(Balance, Balance, u32, bool)> {
            let event = self.events.get(event_id)?;
            let base_price = self.calculate_seat_price(event.base_price, &seat_type);
            let (final_price, _, _, multiplier) = self.calculate_comprehensive_ticket_price(user, &event, &seat_type).ok()?;
            
            Some((base_price, final_price, multiplier, event.dynamic_pricing_enabled))
        }

        /// Get the owner of the contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total teams registered
        #[ink(message)]
        pub fn total_teams(&self) -> u32 {
            self.next_team_id.saturating_sub(1)
        }

        /// Get total venues registered
        #[ink(message)]
        pub fn total_venues(&self) -> u32 {
            self.next_venue_id.saturating_sub(1)
        }

        /// Get total seasons created
        #[ink(message)]
        pub fn total_seasons(&self) -> u32 {
            self.next_season_id.saturating_sub(1)
        }

        /// Get total events created
        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }

        /// Get total tickets sold
        #[ink(message)]
        pub fn total_tickets(&self) -> u64 {
            self.next_ticket_id.saturating_sub(1)
        }

        /// Get total season passes sold
        #[ink(message)]
        pub fn total_season_passes(&self) -> u64 {
            self.next_season_pass_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for SportsBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// COMPREHENSIVE TEST SUITE - Steps 2-8 Coverage
    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn setup_test_data(contract: &mut SportsBroker) -> (u32, u32, u32, u32, u32) {
            // Create venue
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                "New York".to_string(),
                20000,
            ).unwrap();

            // Create teams
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

            // Create season
            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000, // Oct 1, 2024
                1715644800000, // May 14, 2025
                82,
            ).unwrap();

            // Create event
            let event_id = contract.create_sports_event(
                "Knicks vs Celtics".to_string(),
                venue_id,
                1704067200000, // Jan 1, 2025
                18000,
                50_000_000_000_000, // 0.05 DOT base price
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            (venue_id, home_team_id, away_team_id, season_id, event_id)
        }

        // ========================================================================
        // EXISTING TESTS (Steps 1-7) - Selected Key Tests for Space
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
            assert_eq!(sports_broker.total_seasons(), 0);
            assert_eq!(sports_broker.total_events(), 0);
            assert_eq!(sports_broker.total_tickets(), 0);
            assert_eq!(sports_broker.total_season_passes(), 0);
        }

        #[ink::test]
        fn register_team_works() {
            let mut sports_broker = SportsBroker::new();
            
            let team_id = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();

            assert_eq!(team_id, 1);
            
            // NEW: Verify performance and pricing data initialized
            let performance = sports_broker.get_team_performance(team_id).unwrap();
            assert_eq!(performance.team_id, team_id);
            assert_eq!(performance.wins, 0);
            assert_eq!(performance.losses, 0);
            
            let pricing = sports_broker.get_pricing_multiplier(team_id).unwrap();
            assert_eq!(pricing.team_id, team_id);
            assert_eq!(pricing.final_multiplier, 10000); // 1.0x
        }

        #[ink::test]
        fn purchase_sports_ticket_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                50_000_000_000_000 // 0.05 DOT
            );

            let ticket_id = sports_broker.purchase_sports_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert_eq!(ticket.event_id, event_id);
            assert!(ticket.loyalty_points_earned > 0);
            // NEW: Verify dynamic pricing fields
            assert_eq!(ticket.dynamic_price_paid, 50_000_000_000_000);
            assert_eq!(ticket.performance_multiplier_applied, 10000); // 1.0x default
        }

        #[ink::test]
        fn purchase_season_pass_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                450_000_000_000_000 // 0.45 DOT (with early bird discount)
            );

            let pass_id = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false,
            ).unwrap();

            assert_eq!(pass_id, 1);
            let season_pass = sports_broker.get_season_pass(pass_id).unwrap();
            assert_eq!(season_pass.games_included, 82);
        }

        // ========================================================================
        // NEW: STEP 8 - DYNAMIC PRICING ENGINE TESTS
        // ========================================================================

        #[ink::test]
        fn update_team_performance_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);

            let result = sports_broker.update_team_performance(
                home_team_id,
                season_id,
                50, // wins
                20, // losses
                8500, // 85% playoff probability
                5, // 5 game win streak
                11200, // 112.00 points scored average
                10500, // 105.00 points allowed average
            );

            assert_eq!(result, Ok(()));
            
            let performance = sports_broker.get_team_performance(home_team_id).unwrap();
            assert_eq!(performance.wins, 50);
            assert_eq!(performance.losses, 20);
            assert_eq!(performance.playoff_probability, 8500);
            assert_eq!(performance.streak, 5);
            assert_eq!(performance.win_percentage, 7142); // 50/70 ≈ 71.42%

            // Verify pricing multiplier was updated
            let pricing = sports_broker.get_pricing_multiplier(home_team_id).unwrap();
            assert!(pricing.performance_multiplier > 10000); // Should be higher for winning team
            assert!(pricing.playoff_multiplier > 10000); // Should be higher for playoff team
            assert!(pricing.streak_multiplier > 10000); // Should be higher for win streak
        }

        #[ink::test]
        fn dynamic_pricing_affects_ticket_price() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Update team performance to make them expensive (great team)
            sports_broker.update_team_performance(
                home_team_id,
                season_id,
                60, // wins
                10, // losses (excellent record)
                9500, // 95% playoff probability
                7, // 7 game win streak
                12000, // High scoring
                9800, // Good defense
            ).unwrap();

            // Get price with dynamic pricing
            let dynamic_price = sports_broker.get_current_ticket_price(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
            ).unwrap();

            // Should be more than base price due to team performance
            let base_price = 50_000_000_000_000; // 0.05 DOT
            assert!(dynamic_price > base_price);

            // Verify price breakdown
            let (base, final_price, multiplier, enabled) = sports_broker.get_price_breakdown(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
            ).unwrap();
            
            assert_eq!(base, base_price);
            assert_eq!(final_price, dynamic_price);
            assert!(multiplier > 10000); // Should be higher than 1.0x
            assert!(enabled); // Dynamic pricing should be enabled
        }

        #[ink::test]
        fn poor_team_performance_reduces_prices() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Update team performance to make them cheap (poor team)
            sports_broker.update_team_performance(
                home_team_id,
                season_id,
                15, // wins
                55, // losses (terrible record)
                500, // 5% playoff probability
                -8, // 8 game losing streak
                9500, // Low scoring
                11500, // Poor defense
            ).unwrap();

            // Get price with dynamic pricing
            let dynamic_price = sports_broker.get_current_ticket_price(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
            ).unwrap();

            // Should be less than base price due to poor team performance
            let base_price = 50_000_000_000_000; // 0.05 DOT
            assert!(dynamic_price < base_price);

            // Verify multiplier is below 1.0x
            let (_, _, multiplier, _) = sports_broker.get_price_breakdown(
                event_id,
                SeatType::GeneralAdmission,
                accounts.alice,
            ).unwrap();
            
            assert!(multiplier < 10000); // Should be lower than 1.0x
        }

        #[ink::test]
        fn rivalry_multiplier_increases_prices() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, _, _, season_id, _) = setup_test_data(&mut sports_broker);

            // Create Lakers vs Celtics rivalry game
            let lakers_id = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();

            let celtics_id = sports_broker.register_team(
                "Celtics".to_string(),
                "Boston".to_string(),
                SportType::Basketball,
            ).unwrap();

            let rivalry_event_id = sports_broker.create_sports_event(
                "Lakers vs Celtics".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                lakers_id,
                celtics_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            let event = sports_broker.get_sports_event(rivalry_event_id).unwrap();
            assert!(event.rivalry_multiplier > 10000); // Should be higher for rivalry
        }

        #[ink::test]
        fn set_rivalry_multiplier_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, away_team_id, _, _) = setup_test_data(&mut sports_broker);

            let result = sports_broker.set_rivalry_multiplier(
                home_team_id,
                away_team_id,
                15000, // 1.5x multiplier
            );

            assert_eq!(result, Ok(()));

            // Verify both teams have updated rivalry multiplier
            let home_pricing = sports_broker.get_pricing_multiplier(home_team_id).unwrap();
            let away_pricing = sports_broker.get_pricing_multiplier(away_team_id).unwrap();
            
            assert_eq!(home_pricing.rivalry_multiplier, 15000);
            assert_eq!(away_pricing.rivalry_multiplier, 15000);
        }

        #[ink::test]
        fn disable_dynamic_pricing_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);

            // Disable dynamic pricing
            let result = sports_broker.set_event_dynamic_pricing(event_id, false);
            assert_eq!(result, Ok(()));

            let event = sports_broker.get_sports_event(event_id).unwrap();
            assert!(!event.dynamic_pricing_enabled);
        }

        #[ink::test]
        fn high_demand_increases_price() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Create a small capacity event for easier testing
            let small_event_id = sports_broker.create_sports_event(
                "Small Venue Game".to_string(),
                venue_id,
                1704067200000,
                10, // Small capacity of 10 tickets
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Simulate very high demand by buying 9 tickets (90% of 10)
            for i in 0..9 {
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                    50_000_000_000_000
                );

                let _ = sports_broker.purchase_sports_ticket(
                    small_event_id,
                    format!("Section {}", i),
                    "Row 1".to_string(),
                    SeatType::GeneralAdmission,
                );
            }

            // Check that demand multiplier increased significantly
            let pricing = sports_broker.get_pricing_multiplier(home_team_id).unwrap();
            assert!(pricing.demand_multiplier > 10000); // Should be higher due to high demand
            // Should be either 11500 (75-89%) or 13000 (90%+) - both indicate high demand
            assert!(pricing.demand_multiplier >= 11500);
        }

        #[ink::test]
        fn playoff_game_has_higher_base_multiplier() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            let _playoff_event_id = sports_broker.create_sports_event(
                "Playoff Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            // Playoff games should have higher base multiplier
            let pricing = sports_broker.get_pricing_multiplier(home_team_id).unwrap();
            assert_eq!(pricing.base_multiplier, 15000); // 1.5x for playoff games
        }

        #[ink::test]
        fn update_team_performance_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);

            // Try to update as non-owner
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.update_team_performance(
                home_team_id,
                season_id,
                50, 20, 8500, 5, 11200, 10500,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn invalid_performance_stats_rejected() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);

            // Try to set invalid playoff probability (>10000)
            let result = sports_broker.update_team_performance(
                home_team_id,
                season_id,
                50, 20, 
                15000, // Invalid: >10000 (>100%)
                5, 11200, 10500,
            );

            assert_eq!(result, Err(Error::InvalidPerformanceStats));
        }
    }
}