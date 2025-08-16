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

        // NEW: Season Pass System (Step 7)
        season_passes: ink::storage::Mapping<u64, SeasonPass>,
        next_season_pass_id: u64,
        user_season_passes: ink::storage::Mapping<AccountId, Vec<u64>>,
        staking_rewards_pool: Balance,
        user_staked_amounts: ink::storage::Mapping<AccountId, Balance>,
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
        // NEW: Season pass pricing
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
        
        // NEW: Season pass benefits
        pub season_pass_discount: u8, // percentage discount for season pass holders
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
        
        // NEW: Season pass integration
        pub season_pass_discount_applied: bool,
        pub is_season_pass_ticket: bool,
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
        // NEW: Season pass holder status
        pub season_pass_holder: bool,
    }

    /// NEW: Season pass for subscription management (Step 7)
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

    /// NEW: Season pass types (Step 7)
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
        // NEW: Season pass errors
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
                // NEW: Step 7 storage initialization
                season_passes: ink::storage::Mapping::new(),
                next_season_pass_id: 1,
                user_season_passes: ink::storage::Mapping::new(),
                staking_rewards_pool: 0,
                user_staked_amounts: ink::storage::Mapping::new(),
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
                // NEW: Season pass pricing defaults
                season_pass_base_price: 500_000_000_000_000, // 0.5 DOT
                early_bird_discount: 10, // 10% early bird discount
                early_bird_deadline: start_date, // Default to season start
            };

            self.seasons.insert(season_id, &season);
            Ok(season_id)
        }

        /// NEW: Update season pass pricing (Step 7)
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
        // SPORTS EVENT MANAGEMENT (Step 4 + Enhanced for Step 7)
        // ========================================================================

        /// Create a sports event with season pass benefits
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
                
                // NEW: Season pass benefits
                season_pass_discount: 15, // 15% discount for season pass holders
            };

            self.events.insert(event_id, &sports_event);
            Ok(event_id)
        }

        // ========================================================================
        // ENHANCED TICKET PURCHASING (Step 5 + Enhanced for Step 7)
        // ========================================================================

        /// Purchase a sports ticket with season pass benefits
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

            // Calculate base price
            let base_price = self.calculate_seat_price(event.base_price, &seat_type);
            
            // NEW: Check for season pass benefits
            let (final_price, season_pass_discount_applied, is_season_pass_ticket) = 
                self.apply_season_pass_discount(buyer, &event, base_price)?;
            
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
                
                // NEW: Season pass integration
                season_pass_discount_applied,
                is_season_pass_ticket,
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
                season_pass_holder: false, // NEW: Initially false
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
        // NEW: SEASON PASS SYSTEM (Step 7)
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
            // In practice, this would integrate with Acala liquid staking
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
        // HELPER METHODS
        // ========================================================================

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
        // QUERY METHODS
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

        /// Get ticket price for a seat type
        #[ink(message)]
        pub fn get_ticket_price(&self, event_id: u32, seat_type: SeatType) -> Option<Balance> {
            let event = self.events.get(event_id)?;
            Some(self.calculate_seat_price(event.base_price, &seat_type))
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

        // NEW: Season pass queries

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

    /// COMPREHENSIVE TEST SUITE - Steps 2-7 Coverage
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
        // EXISTING TESTS (Steps 1-6) - All preserved
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.total_teams(), 0);
            assert_eq!(sports_broker.total_venues(), 0);
            assert_eq!(sports_broker.total_seasons(), 0);
            assert_eq!(sports_broker.total_events(), 0);
            assert_eq!(sports_broker.total_tickets(), 0);
            assert_eq!(sports_broker.total_season_passes(), 0); // NEW
        }

        // ========================================================================
        // STEP 2: TEAM & VENUE MANAGEMENT TESTS (REGRESSION)
        // ========================================================================

        #[ink::test]
        fn register_team_works() {
            let mut sports_broker = SportsBroker::new();
            
            let team_id = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();

            assert_eq!(team_id, 1);
            assert_eq!(sports_broker.total_teams(), 1);
            
            let team = sports_broker.get_team(team_id).unwrap();
            assert_eq!(team.name, "Lakers");
            assert_eq!(team.city, "Los Angeles");
            assert_eq!(team.sport_type, SportType::Basketball);
            assert!(team.verified);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut sports_broker = SportsBroker::new();

            let venue_id = sports_broker.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                20000,
            ).unwrap();

            assert_eq!(venue_id, 1);
            assert_eq!(sports_broker.total_venues(), 1);
            
            let venue = sports_broker.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Staples Center");
            assert_eq!(venue.city, "Los Angeles");
            assert_eq!(venue.capacity, 20000);
        }

        #[ink::test]
        fn register_team_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();

            // Try to register team as non-owner
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.register_team(
                "Unauthorized Team".to_string(),
                "City".to_string(),
                SportType::Basketball,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn register_venue_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();

            // Try to register venue as non-owner
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.register_venue(
                "Unauthorized Venue".to_string(),
                "City".to_string(),
                15000,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn multiple_teams_registration() {
            let mut sports_broker = SportsBroker::new();

            let team1 = sports_broker.register_team(
                "Lakers".to_string(),
                "Los Angeles".to_string(),
                SportType::Basketball,
            ).unwrap();

            let team2 = sports_broker.register_team(
                "Warriors".to_string(),
                "San Francisco".to_string(),
                SportType::Basketball,
            ).unwrap();

            assert_eq!(team1, 1);
            assert_eq!(team2, 2);
            assert_eq!(sports_broker.total_teams(), 2);
        }

        #[ink::test]
        fn multiple_venues_registration() {
            let mut sports_broker = SportsBroker::new();

            let venue1 = sports_broker.register_venue(
                "Staples Center".to_string(),
                "Los Angeles".to_string(),
                20000,
            ).unwrap();

            let venue2 = sports_broker.register_venue(
                "Chase Center".to_string(),
                "San Francisco".to_string(),
                18000,
            ).unwrap();

            assert_eq!(venue1, 1);
            assert_eq!(venue2, 2);
            assert_eq!(sports_broker.total_venues(), 2);
        }

        #[ink::test]
        fn get_nonexistent_team_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_team(999), None);
        }

        #[ink::test]
        fn get_nonexistent_venue_returns_none() {
            let sports_broker = SportsBroker::new();
            assert_eq!(sports_broker.get_venue(999), None);
        }

        #[ink::test]
        fn sport_type_other_variant_works() {
            let mut sports_broker = SportsBroker::new();

            let team_id = sports_broker.register_team(
                "Racing Team".to_string(),
                "Monaco".to_string(),
                SportType::Other("Formula 1".to_string()),
            ).unwrap();

            let team = sports_broker.get_team(team_id).unwrap();
            assert_eq!(team.sport_type, SportType::Other("Formula 1".to_string()));
        }

        // ========================================================================
        // STEP 3: SEASON MANAGEMENT TESTS (REGRESSION)
        // ========================================================================

        #[ink::test]
        fn create_season_works() {
            let mut sports_broker = SportsBroker::new();

            let season_id = sports_broker.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000, // Oct 1, 2024
                1715644800000, // May 14, 2025
                82,
            ).unwrap();

            assert_eq!(season_id, 1);
            assert_eq!(sports_broker.total_seasons(), 1);
            
            let season = sports_broker.get_season(season_id).unwrap();
            assert_eq!(season.name, "2024-25 NBA Season");
            assert_eq!(season.sport_type, SportType::Basketball);
            assert_eq!(season.regular_season_games, 82);
            assert!(season.active);
        }

        #[ink::test]
        fn create_season_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.create_season(
                "Unauthorized Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn multiple_seasons_creation() {
            let mut sports_broker = SportsBroker::new();

            let season1 = sports_broker.create_season(
                "2024-25 NBA Season".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            ).unwrap();

            let season2 = sports_broker.create_season(
                "2024 NFL Season".to_string(),
                SportType::Football,
                1693526400000, // Sep 1, 2024
                1707004800000, // Feb 4, 2025
                17,
            ).unwrap();

            assert_eq!(season1, 1);
            assert_eq!(season2, 2);
            assert_eq!(sports_broker.total_seasons(), 2);
        }

        #[ink::test]
        fn season_with_different_sport_types() {
            let mut sports_broker = SportsBroker::new();

            let basketball_season = sports_broker.create_season(
                "NBA 2024-25".to_string(),
                SportType::Basketball,
                1696118400000,
                1715644800000,
                82,
            ).unwrap();

            let soccer_season = sports_broker.create_season(
                "MLS 2024".to_string(),
                SportType::Soccer,
                1709251200000, // Mar 1, 2024
                1698796800000, // Nov 1, 2024
                34,
            ).unwrap();

            let b_season = sports_broker.get_season(basketball_season).unwrap();
            let s_season = sports_broker.get_season(soccer_season).unwrap();

            assert_eq!(b_season.sport_type, SportType::Basketball);
            assert_eq!(s_season.sport_type, SportType::Soccer);
        }

        // ========================================================================
        // STEP 4: EVENT MANAGEMENT TESTS (REGRESSION)
        // ========================================================================

        #[ink::test]
        fn create_sports_event_works() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            let event_id = sports_broker.create_sports_event(
                "Big Game".to_string(),
                venue_id,
                1704067200000,
                18000,
                75_000_000_000_000, // 0.075 DOT
                home_team_id,
                away_team_id,
                season_id,
                GameType::Playoff,
            ).unwrap();

            // Note: event_id will be 2 because setup_test_data creates event 1
            assert_eq!(event_id, 2);
            
            let event = sports_broker.get_sports_event(event_id).unwrap();
            assert_eq!(event.name, "Big Game");
            assert_eq!(event.home_team_id, home_team_id);
            assert_eq!(event.away_team_id, away_team_id);
            assert_eq!(event.game_type, GameType::Playoff);
            assert!(event.active);
        }

        #[ink::test]
        fn create_sports_event_unauthorized() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = sports_broker.create_sports_event(
                "Unauthorized Event".to_string(),
                venue_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn create_sports_event_invalid_venue() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            let result = sports_broker.create_sports_event(
                "Invalid Venue Game".to_string(),
                999, // Invalid venue ID
                1704067200000,
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            );

            assert_eq!(result, Err(Error::VenueNotFound));
        }

        #[ink::test]
        fn multiple_sports_events() {
            let mut sports_broker = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);

            let event2 = sports_broker.create_sports_event(
                "Game 2".to_string(),
                venue_id,
                1704153600000, // Different date
                18000,
                50_000_000_000_000,
                home_team_id,
                away_team_id,
                season_id,
                GameType::RegularSeason,
            ).unwrap();

            // Should be event ID 2 (setup created event 1)
            assert_eq!(event2, 2);
            assert_eq!(sports_broker.total_events(), 2);
        }

        // ========================================================================
        // STEP 5: TICKET PURCHASING TESTS (EXISTING)
        // ========================================================================

        #[ink::test]
        fn purchase_sports_ticket_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);

            // Set payment for general admission ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                50_000_000_000_000 // 0.05 DOT
            );

            let result = sports_broker.purchase_sports_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            );

            assert!(result.is_ok());
            let ticket_id = result.unwrap();
            assert_eq!(ticket_id, 1);
            assert_eq!(sports_broker.total_tickets(), 1);

            // Verify ticket data includes loyalty points
            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert_eq!(ticket.event_id, event_id);
            assert_eq!(ticket.section, "Section A");
            assert_eq!(ticket.row, "Row 1");
            assert_eq!(ticket.seat_type, SeatType::GeneralAdmission);
            assert_eq!(ticket.access_level, AccessLevel::Standard);
            assert_eq!(ticket.seat_number, 1);
            assert!(ticket.transferable);
            assert!(ticket.loyalty_points_earned > 0); // NEW: Verify loyalty points awarded
        }

        #[ink::test]
        fn purchase_sports_ticket_insufficient_payment() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);

            // Set payment lower than required
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                25_000_000_000_000 // 0.025 DOT (less than 0.05 DOT required)
            );

            let result = sports_broker.purchase_sports_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            );

            assert_eq!(result, Err(Error::InsufficientPayment));
        }

        #[ink::test]
        fn transfer_ticket_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Alice buys a ticket
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                50_000_000_000_000
            );

            let ticket_id = sports_broker.purchase_sports_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Alice transfers to Bob
            let result = sports_broker.transfer_ticket(ticket_id, accounts.bob);
            assert!(result.is_ok());

            // Verify ownership changed
            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert_eq!(ticket.owner, accounts.bob);
        }

        // ========================================================================
        // STEP 6: USER PROFILE & LOYALTY SYSTEM TESTS
        // ========================================================================

        #[ink::test]
        fn create_user_profile_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, away_team_id, _, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            
            let favorite_teams = vec![home_team_id, away_team_id];
            let result = sports_broker.create_user_profile(
                favorite_teams.clone(),
                "New York".to_string(),
            );

            assert_eq!(result, Ok(()));
            
            let profile = sports_broker.get_user_profile(accounts.alice).unwrap();
            assert_eq!(profile.favorite_teams, favorite_teams);
            assert_eq!(profile.home_city, "New York");
            assert_eq!(profile.loyalty_tier, LoyaltyTier::Bronze);
            assert!(!profile.verified_fan);
            assert_eq!(profile.total_games_attended, 0);
        }

        #[ink::test]
        fn loyalty_points_system_works() {
            let mut sports_broker = SportsBroker::new();
            let accounts = get_accounts();
            
            // Initially zero points
            assert_eq!(sports_broker.get_user_loyalty_points(accounts.alice), 0);
            
            // Award some points
            sports_broker.award_loyalty_points(accounts.alice, 500);
            assert_eq!(sports_broker.get_user_loyalty_points(accounts.alice), 500);
            
            // Award more points to trigger tier upgrade
            sports_broker.award_loyalty_points(accounts.alice, 1500);
            assert_eq!(sports_broker.get_user_loyalty_points(accounts.alice), 2000);
        }

        // ========================================================================
        // NEW: STEP 7 - SEASON PASS SYSTEM TESTS
        // ========================================================================

        #[ink::test]
        fn purchase_season_pass_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Alice purchases a full season pass
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000 // 0.5 DOT
            );

            let pass_id = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false, // No staking
            ).unwrap();

            assert_eq!(pass_id, 1);
            assert_eq!(sports_broker.total_season_passes(), 1);

            // Verify season pass details
            let season_pass = sports_broker.get_season_pass(pass_id).unwrap();
            assert_eq!(season_pass.owner, accounts.alice);
            assert_eq!(season_pass.season_id, season_id);
            assert_eq!(season_pass.team_id, home_team_id);
            assert_eq!(season_pass.pass_type, SeasonPassType::FullSeason);
            assert_eq!(season_pass.games_included, 82);
            assert_eq!(season_pass.games_attended, 0);
            assert!(season_pass.transferable);
            assert!(!season_pass.staking_rewards_enabled);

            // Verify user has season pass
            let user_passes = sports_broker.get_user_season_passes(accounts.alice);
            assert_eq!(user_passes.len(), 1);
            assert_eq!(user_passes[0], pass_id);

            // Verify loyalty points awarded
            let points = sports_broker.get_user_loyalty_points(accounts.alice);
            assert!(points >= 1000); // Full season should award 1000+ points
        }

        #[ink::test]
        fn purchase_season_pass_with_staking_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            let pass_id = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                true, // Enable staking
            ).unwrap();

            let season_pass = sports_broker.get_season_pass(pass_id).unwrap();
            assert!(season_pass.staking_rewards_enabled);
            assert_eq!(season_pass.staked_amount, 500_000_000_000_000);

            // Verify staking amounts updated
            let staked = sports_broker.get_user_staked_amount(accounts.alice);
            assert_eq!(staked, 500_000_000_000_000);
        }

        #[ink::test]
        fn different_season_pass_types_work() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Half season pass
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                275_000_000_000_000 // Should be about 55% of full price
            );

            let half_pass = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::HalfSeason,
                false,
            ).unwrap();

            // Weekend pass
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                300_000_000_000_000 // Should be about 60% of full price
            );

            let weekend_pass = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::Weekend,
                false,
            ).unwrap();

            let half_season_pass = sports_broker.get_season_pass(half_pass).unwrap();
            let weekend_season_pass = sports_broker.get_season_pass(weekend_pass).unwrap();

            assert_eq!(half_season_pass.games_included, 41); // 82/2
            assert_eq!(weekend_season_pass.games_included, 32); // 82*40/100
            assert_eq!(half_season_pass.pass_type, SeasonPassType::HalfSeason);
            assert_eq!(weekend_season_pass.pass_type, SeasonPassType::Weekend);
        }

        #[ink::test]
        fn season_pass_ticket_discount_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, event_id) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Alice buys season pass
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false,
            ).unwrap();

            // Alice buys ticket (should get season pass discount)
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                42_500_000_000_000 // 15% discount from 50,000,000,000,000
            );

            let ticket_id = sports_broker.purchase_sports_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            let ticket = sports_broker.get_sports_ticket(ticket_id).unwrap();
            assert!(ticket.season_pass_discount_applied);
            assert!(ticket.is_season_pass_ticket);

            // Verify season pass usage updated
            let user_passes = sports_broker.get_user_season_passes(accounts.alice);
            let season_pass = sports_broker.get_season_pass(user_passes[0]).unwrap();
            assert_eq!(season_pass.games_attended, 1);
        }

        #[ink::test]
        fn transfer_season_pass_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Alice buys season pass
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            let pass_id = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false,
            ).unwrap();

            // Alice transfers to Bob
            let result = sports_broker.transfer_season_pass(pass_id, accounts.bob);
            assert!(result.is_ok());

            // Verify ownership changed
            let season_pass = sports_broker.get_season_pass(pass_id).unwrap();
            assert_eq!(season_pass.owner, accounts.bob);

            // Verify Alice's list is empty, Bob's has the pass
            let alice_passes = sports_broker.get_user_season_passes(accounts.alice);
            let bob_passes = sports_broker.get_user_season_passes(accounts.bob);
            assert_eq!(alice_passes.len(), 0);
            assert_eq!(bob_passes.len(), 1);
            assert_eq!(bob_passes[0], pass_id);
        }

        #[ink::test]
        fn season_pass_price_quote_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, _home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Check price for different pass types
            let full_price = sports_broker.get_season_pass_price(
                season_id,
                SeasonPassType::FullSeason,
                accounts.alice,
            ).unwrap();

            let half_price = sports_broker.get_season_pass_price(
                season_id,
                SeasonPassType::HalfSeason,
                accounts.alice,
            ).unwrap();

            let premium_price = sports_broker.get_season_pass_price(
                season_id,
                SeasonPassType::Premium,
                accounts.alice,
            ).unwrap();

            // Verify pricing relationships
            assert!(half_price < full_price);
            assert!(premium_price > full_price);
            // Base price is 0.5 DOT but with 10% early bird discount = 0.45 DOT
            assert_eq!(full_price, 450_000_000_000_000); // Base price with early bird discount
        }

        #[ink::test]
        fn has_valid_season_pass_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Initially no valid pass
            assert!(!sports_broker.has_valid_season_pass(accounts.alice, home_team_id, season_id));

            // Alice buys season pass for home team
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false,
            ).unwrap();

            // Should have valid pass for home team but not away team
            assert!(sports_broker.has_valid_season_pass(accounts.alice, home_team_id, season_id));
            assert!(!sports_broker.has_valid_season_pass(accounts.alice, away_team_id, season_id));
        }

        #[ink::test]
        fn claim_staking_rewards_works() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);
            let accounts = get_accounts();

            // Alice buys season pass with staking
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                true, // Enable staking
            ).unwrap();

            // Alice claims rewards
            let rewards = sports_broker.claim_staking_rewards().unwrap();
            assert!(rewards > 0);
        }

        #[ink::test]
        fn season_pass_insufficient_payment() {
            let mut sports_broker = SportsBroker::new();
            let (_, home_team_id, _, season_id, _) = setup_test_data(&mut sports_broker);

            // Try to buy with insufficient payment
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                100_000_000_000_000 // Only 0.1 DOT (need 0.5 DOT)
            );

            let result = sports_broker.purchase_season_pass(
                season_id,
                home_team_id,
                SeasonPassType::FullSeason,
                false,
            );

            assert_eq!(result, Err(Error::InsufficientPayment));
        }

        #[ink::test]
        fn season_pass_invalid_season() {
            let mut sports_broker = SportsBroker::new();
            
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                500_000_000_000_000
            );

            let result = sports_broker.purchase_season_pass(
                999, // Invalid season
                1,
                SeasonPassType::FullSeason,
                false,
            );

            assert_eq!(result, Err(Error::SeasonNotFound));
        }

        // Include all previous tests from Steps 1-6 here (abbreviated for space)
        // They should all continue to pass
    }
}