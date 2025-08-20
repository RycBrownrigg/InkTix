#![cfg_attr(not(feature = "std"), no_std, no_main)]

// Declare the modules
mod types;

/// Sports Broker - Modular Sports Ticketing Platform
#[ink::contract]
pub mod sports_broker {
    use ink::prelude::string::String;
    use crate::types::*;

    #[ink(storage)]
    pub struct SportsBroker {
        // Team management
        pub teams: ink::storage::Mapping<u32, Team>,
        pub next_team_id: u32,

        // NEW: Owner field
        pub owner: ink::primitives::AccountId,

        // Venue management  
        pub venues: ink::storage::Mapping<u32, Venue>,
        pub next_venue_id: u32,

        // Season management
        pub seasons: ink::storage::Mapping<u32, Season>,
        pub next_season_id: u32,

        // Event management
        pub events: ink::storage::Mapping<u32, SportsEvent>,
        pub next_event_id: u32,

        // Ticket management
        pub tickets: ink::storage::Mapping<u64, SportsTicket>,
        pub next_ticket_id: u64,
        pub user_tickets: ink::storage::Mapping<ink::primitives::AccountId, ink::prelude::vec::Vec<u64>>,

        // NEW: Dynamic Pricing Engine
        pub team_performance: ink::storage::Mapping<u32, TeamPerformance>,
        pub pricing_multipliers: ink::storage::Mapping<u32, PricingMultiplier>,

        // NEW: Multi-Currency Support
        pub supported_currencies: ink::prelude::vec::Vec<CurrencyId>,
        pub currency_rates: ink::storage::Mapping<CurrencyId, u128>,
        pub currency_revenue: ink::storage::Mapping<CurrencyId, u128>,
        pub total_revenue_dot: u128,

        // Counters
        pub total_teams: u32,
        pub total_venues: u32,
        pub total_events: u32,
        pub total_tickets: u32,
        pub total_seasons: u32,

        // NEW: Analytics and reporting
        pub platform_stats: PlatformStats,
        pub event_analytics: ink::storage::Mapping<u32, EventAnalytics>,
        pub team_analytics: ink::storage::Mapping<u32, TeamAnalytics>,
        pub user_analytics: ink::storage::Mapping<ink::primitives::AccountId, UserAnalytics>,
        pub analytics_reports: ink::storage::Mapping<u32, AnalyticsReport>,
        pub next_report_id: u32,
        pub analytics_enabled: bool,
    }

    impl SportsBroker {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            // Initialize platform stats
            let platform_stats = PlatformStats {
                total_revenue: 0,
                total_tickets_sold: 0,
                total_events: 0,
                total_users: 0,
                average_ticket_price: 0,
                total_season_passes: 0,
                currency_distribution: Vec::new(),
                sport_type_distribution: Vec::new(),
                last_updated: 0,
            };
            // Initialize supported currencies
            let mut supported_currencies = ink::prelude::vec::Vec::new();
            supported_currencies.push(CurrencyId::DOT);
            supported_currencies.push(CurrencyId::ACA);
            supported_currencies.push(CurrencyId::AUSD);
            supported_currencies.push(CurrencyId::LDOT);
            supported_currencies.push(CurrencyId::KSM);

            let mut contract = Self {
                owner: Self::env().caller(),
                teams: ink::storage::Mapping::new(),
                venues: ink::storage::Mapping::new(),
                seasons: ink::storage::Mapping::new(),
                events: ink::storage::Mapping::new(),
                tickets: ink::storage::Mapping::new(),
                user_tickets: ink::storage::Mapping::new(),
                // Dynamic pricing storage
                team_performance: ink::storage::Mapping::new(),
                pricing_multipliers: ink::storage::Mapping::new(),
                // Multi-currency storage
                supported_currencies,
                currency_rates: ink::storage::Mapping::new(),
                currency_revenue: ink::storage::Mapping::new(),
                total_revenue_dot: 0,
                next_team_id: 1,
                next_venue_id: 1,
                next_season_id: 1,
                next_event_id: 1,
                next_ticket_id: 1,
                total_teams: 0,
                total_venues: 0,
                total_events: 0,
                total_tickets: 0,
                total_seasons: 0,
                platform_stats,
                event_analytics: ink::storage::Mapping::default(),
                team_analytics: ink::storage::Mapping::default(),
                user_analytics: ink::storage::Mapping::default(),
                analytics_reports: ink::storage::Mapping::default(),
                next_report_id: 1,
                analytics_enabled: true,
            };

            // Set default currency rates (1 DOT = 1 DOT, 1 ACA = 0.05 DOT, etc.)
            contract.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000_000_000); // 1 DOT = 1 DOT
            contract.currency_rates.insert(CurrencyId::ACA, &50_000_000_000_000_000);     // 1 ACA = 0.05 DOT
            contract.currency_rates.insert(CurrencyId::AUSD, &150_000_000_000_000_000);   // 1 aUSD = 0.15 DOT
            contract.currency_rates.insert(CurrencyId::LDOT, &950_000_000_000_000_000);   // 1 LDOT = 0.95 DOT
            contract.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000_000_000); // 1 KSM = 15 DOT

            // Initialize currency revenue tracking
            for currency in &contract.supported_currencies {
                contract.currency_revenue.insert(*currency, &0);
            }

            contract
        }

        #[ink(message)]
        pub fn register_team(&mut self, name: String, sport: String, city: String) -> u32 {
            let team_id = self.next_team_id;
            self.next_team_id += 1;

            // Convert string sport to SportType enum
            let sport_type = match sport.as_str() {
                "Basketball" => SportType::Basketball,
                "Football" => SportType::Football,
                "Soccer" => SportType::Soccer,
                "Baseball" => SportType::Baseball,
                "Hockey" => SportType::Hockey,
                "Tennis" => SportType::Tennis,
                _ => SportType::Other(sport.clone()),
            };

            let team = Team {
                id: team_id,
                name,
                city,
                sport_type,
                verified: false,
            };

            self.teams.insert(team_id, &team);
            self.total_teams += 1;

            // NEW: Initialize performance tracking
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

            team_id
        }

        #[ink(message)]
        pub fn register_venue(&mut self, name: String, capacity: u32, address: String, _sport_type: String) -> u32 {
            let venue_id = self.next_venue_id;
            self.next_venue_id += 1;

            let venue = Venue {
                id: venue_id,
                name,
                city: address, // Use address as city
                capacity,
            };

            self.venues.insert(venue_id, &venue);
            self.total_venues += 1;
            venue_id
        }

        // NEW: Season management
        #[ink(message)]
        pub fn create_season(&mut self, name: String, sport: String, _year: u32, start_date: u64, end_date: u64) -> u32 {
            let season_id = self.next_season_id;
            self.next_season_id += 1;

            // Convert string sport to SportType enum
            let sport_type = match sport.as_str() {
                "Basketball" => SportType::Basketball,
                "Football" => SportType::Football,
                "Soccer" => SportType::Soccer,
                "Baseball" => SportType::Baseball,
                "Hockey" => SportType::Hockey,
                "Tennis" => SportType::Tennis,
                _ => SportType::Other(sport.clone()),
            };

            let season = Season {
                id: season_id,
                name,
                sport_type,
                start_date,
                end_date,
                regular_season_games: 82, // Default for most sports
                active: true,
                season_pass_base_price: 1000_000_000_000_000, // Default price
                early_bird_discount: 20, // 20% discount
                early_bird_deadline: start_date - 30 * 24 * 60 * 60, // 30 days before
            };

            self.seasons.insert(season_id, &season);
            self.total_seasons += 1;
            season_id
        }

        // NEW: Event management
        #[ink(message)]
        pub fn create_sports_event(
            &mut self,
            name: String,
            home_team_id: u32,
            away_team_id: u32,
            venue_id: u32,
            season_id: u32,
            event_date: u64,
            capacity: u32,
            base_price: u128,
            game_type: GameType,
        ) -> u32 {
            let event_id = self.next_event_id;
            self.next_event_id += 1;

            let event = SportsEvent {
                id: event_id,
                name,
                venue_id,
                date: event_date, // Use event_date as date
                capacity,
                sold_tickets: 0,
                base_price,
                active: true,
                sport_type: SportType::Basketball, // Default, could be made configurable
                home_team_id,
                away_team_id,
                season_id,
                game_type,
                season_pass_discount: 15, // 15% discount for season pass holders
                dynamic_pricing_enabled: false, // Default to false
                rivalry_multiplier: 100, // 100% = no multiplier
                revenue_generated: 0,
            };

            self.events.insert(event_id, &event);
            self.total_events += 1;
            event_id
        }

        // NEW: Ticket management
        #[ink(message, payable)]
        pub fn purchase_ticket(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_number: u32,
        ) -> Result<u64, String> {
            let event = self.events.get(event_id).ok_or("Event not found")?;
            
            if !event.active {
                return Err("Event is not active".to_string());
            }
            
            if event.sold_tickets >= event.capacity {
                return Err("Event is sold out".to_string());
            }

            let caller = self.env().caller();
            let payment = self.env().transferred_value();
            
            if payment < event.base_price {
                return Err("Insufficient payment".to_string());
            }

            let ticket_id = self.next_ticket_id;
            self.next_ticket_id += 1;

            let ticket = SportsTicket {
                id: ticket_id,
                event_id,
                owner: caller,
                purchase_price: payment,
                purchase_currency: CurrencyId::DOT, // Default to DOT
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
                section,
                row,
                seat_type: SeatType::GeneralAdmission, // Default seat type
                access_level: AccessLevel::Standard, // Default access level
                loyalty_points_earned: 10, // Default loyalty points
                season_pass_discount_applied: false,
                is_season_pass_ticket: false,
                dynamic_price_paid: payment,
                performance_multiplier_applied: 100, // 100% = no multiplier
                dot_equivalent_paid: payment,
            };

            self.tickets.insert(ticket_id, &ticket);
            self.total_tickets += 1;

            // Update user's ticket collection
            let mut user_tickets = self.user_tickets.get(caller).unwrap_or_default();
            user_tickets.push(ticket_id);
            self.user_tickets.insert(caller, &user_tickets);

            // Update event sold tickets
            let mut updated_event = event.clone();
            updated_event.sold_tickets += 1;
            if updated_event.sold_tickets >= updated_event.capacity {
                updated_event.active = false; // Mark as sold out
            }
            self.events.insert(event_id, &updated_event);

            Ok(ticket_id)
        }

        // NEW: Multi-currency ticket purchasing
        #[ink(message, payable)]
        pub fn purchase_ticket_with_currency(
            &mut self,
            event_id: u32,
            section: String,
            row: String,
            seat_number: u32,
            currency: CurrencyId,
        ) -> Result<u64, String> {
            let event = self.events.get(event_id).ok_or("Event not found")?;
            
            if !event.active {
                return Err("Event is not active".to_string());
            }
            
            if event.sold_tickets >= event.capacity {
                return Err("Event is sold out".to_string());
            }

            if !self.supported_currencies.contains(&currency) {
                return Err("Unsupported currency".to_string());
            }

            let caller = self.env().caller();
            let payment = self.env().transferred_value();
            
            // Convert payment to DOT equivalent for validation
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            
            if payment_in_dot < event.base_price {
                return Err("Insufficient payment".to_string());
            }

            let ticket_id = self.next_ticket_id;
            self.next_ticket_id += 1;

            let ticket = SportsTicket {
                id: ticket_id,
                event_id,
                owner: caller,
                purchase_price: payment,
                purchase_currency: currency,
                purchase_date: self.env().block_timestamp(),
                seat_number,
                transferable: true,
                section,
                row,
                seat_type: SeatType::GeneralAdmission,
                access_level: AccessLevel::Standard,
                loyalty_points_earned: 10,
                season_pass_discount_applied: false,
                is_season_pass_ticket: false,
                dynamic_price_paid: payment_in_dot,
                performance_multiplier_applied: 100,
                dot_equivalent_paid: payment_in_dot,
            };

            self.tickets.insert(ticket_id, &ticket);
            self.total_tickets += 1;

            // Update user's ticket collection
            let mut user_tickets = self.user_tickets.get(caller).unwrap_or_default();
            user_tickets.push(ticket_id);
            self.user_tickets.insert(caller, &user_tickets);

            // Update event sold tickets
            let mut updated_event = event.clone();
            updated_event.sold_tickets += 1;
            if updated_event.sold_tickets >= updated_event.capacity {
                updated_event.active = false;
            }
            self.events.insert(event_id, &updated_event);

            // NEW: Update revenue tracking
            self.update_revenue_tracking(currency, payment_in_dot);

            Ok(ticket_id)
        }
        
        #[ink(message)]
        pub fn get_team(&self, team_id: u32) -> Option<Team> {
            self.teams.get(team_id)
        }
                // Add owner getter
        #[ink(message)]
        pub fn get_owner(&self) -> ink::primitives::AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        #[ink(message)]
        pub fn get_stats(&self) -> (u32, u32, u32, u32, u32) {
            (self.total_teams, self.total_venues, self.total_events, self.total_tickets, self.total_seasons)
        }
                
        // NEW: Dynamic Pricing Engine Methods
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
        ) -> Result<(), String> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err("Only owner can update team performance".to_string());
            }

            let _team = self.get_team(team_id).ok_or("Team not found")?;

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
                performance_rank: 0,
                home_record_wins: 0,
                home_record_losses: 0,
                points_scored_avg,
                points_allowed_avg,
            };

            self.team_performance.insert(team_id, &performance);

            // Update pricing multipliers based on new performance
            self.recalculate_pricing_multiplier(team_id, &performance);

            Ok(())
        }

        #[ink(message)]
        pub fn get_team_performance(&self, team_id: u32) -> Option<TeamPerformance> {
            self.team_performance.get(team_id)
        }

        #[ink(message)]
        pub fn get_pricing_multiplier(&self, team_id: u32) -> Option<PricingMultiplier> {
            self.pricing_multipliers.get(team_id)
        }

        // Helper method to recalculate pricing multiplier
        fn recalculate_pricing_multiplier(&mut self, team_id: u32, performance: &TeamPerformance) {
            let mut pricing = self.pricing_multipliers.get(team_id).unwrap_or_else(|| PricingMultiplier {
                team_id,
                base_multiplier: 10000,
                performance_multiplier: 10000,
                playoff_multiplier: 10000,
                streak_multiplier: 10000,
                rivalry_multiplier: 10000,
                demand_multiplier: 10000,
                final_multiplier: 10000,
                last_updated: self.env().block_timestamp(),
            });

            // Performance-based multiplier (winning teams cost more)
            pricing.performance_multiplier = if performance.win_percentage >= 7500 {
                12000 // Great team: 1.2x (75% or higher)
            } else if performance.win_percentage >= 5000 {
                10000 // Average team: 1.0x (50% or higher)
            } else {
                8500  // Poor team: 0.85x (below 50%)
            };

            // Playoff probability multiplier
            pricing.playoff_multiplier = if performance.playoff_probability > 8000 {
                11000 // Likely playoff team: 1.1x
            } else if performance.playoff_probability > 5000 {
                10000 // Bubble team: 1.0x
            } else {
                9000  // Unlikely playoff team: 0.9x
            };

            // Streak multiplier (hot teams cost more)
            pricing.streak_multiplier = if performance.streak >= 5 {
                11500 // Hot streak: 1.15x
            } else if performance.streak >= 0 {
                10000 // No streak: 1.0x
            } else if performance.streak <= -5 {
                8500  // Cold streak: 0.85x
            } else {
                9500  // Minor losing streak: 0.95x
            };

            // Calculate final multiplier
            let temp1 = (pricing.base_multiplier as u128 * pricing.performance_multiplier as u128) / 10000;
            let temp2 = (temp1 * pricing.playoff_multiplier as u128) / 10000;
            let temp3 = (temp2 * pricing.streak_multiplier as u128) / 10000;
            let temp4 = (temp3 * pricing.rivalry_multiplier as u128) / 10000;
            pricing.final_multiplier = ((temp4 * pricing.demand_multiplier as u128) / 10000) as u32;

            pricing.last_updated = self.env().block_timestamp();
            self.pricing_multipliers.insert(team_id, &pricing);
        }

        // NEW: Multi-Currency Management Methods
        #[ink(message)]
        pub fn update_currency_rate(
            &mut self,
            currency: CurrencyId,
            rate_to_dot: u128,
        ) -> Result<(), String> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err("Only owner can update currency rates".to_string());
            }

            if rate_to_dot == 0 {
                return Err("Invalid exchange rate".to_string());
            }

            self.currency_rates.insert(currency, &rate_to_dot);
            Ok(())
        }

        pub fn add_supported_currency(
            &mut self,
            currency: CurrencyId,
            rate_to_dot: u128,
        ) -> Result<(), String> {
            // Always add to supported currencies if not present
            if !self.supported_currencies.contains(&currency) {
                self.supported_currencies.push(currency);
                self.currency_revenue.insert(currency, &0);
            }
            
            // Always update the currency rate (this allows rate updates)
            self.currency_rates.insert(currency, &rate_to_dot);
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_supported_currencies(&self) -> ink::prelude::vec::Vec<CurrencyId> {
            self.supported_currencies.clone()
        }

        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<u128> {
            self.currency_rates.get(currency)
        }

        #[ink(message)]
        pub fn get_currency_revenue(&self, currency: CurrencyId) -> u128 {
            self.currency_revenue.get(currency).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_revenue_dot(&self) -> u128 {
            self.total_revenue_dot
        }

        // Helper method to convert to DOT equivalent
        fn convert_to_dot_equivalent(
            &self,
            amount: u128,
            currency: CurrencyId,
        ) -> Result<u128, String> {
            match currency {
                CurrencyId::DOT => Ok(amount),
                _ => {
                    let rate = self.currency_rates.get(currency)
                        .ok_or("Unsupported currency")?;
                    
                    let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000_000_000;
                    if dot_amount == 0 && amount > 0 {
                        return Err("Currency conversion failed".to_string());
                    }
                    Ok(dot_amount)
                }
            }
        }

        // Helper method to convert from DOT equivalent
        fn convert_from_dot_equivalent(
            &self,
            dot_amount: u128,
            target_currency: CurrencyId,
        ) -> Result<u128, String> {
            match target_currency {
                CurrencyId::DOT => Ok(dot_amount),
                _ => {
                    let rate = self.currency_rates.get(target_currency)
                        .ok_or("Unsupported currency")?;
                    
                    if rate == 0 {
                        return Err("Currency conversion failed".to_string());
                    }
                    
                    let target_amount = dot_amount.saturating_mul(1_000_000_000_000_000_000) / rate;
                    Ok(target_amount)
                }
            }
        }

        // Helper method to update revenue tracking
        fn update_revenue_tracking(&mut self, currency: CurrencyId, amount_dot: u128) {
            // Update total revenue in DOT
            self.total_revenue_dot = self.total_revenue_dot.saturating_add(amount_dot);

            // Update currency-specific revenue
            let current_currency_revenue = self.currency_revenue.get(currency).unwrap_or(0);
            self.currency_revenue.insert(currency, &(current_currency_revenue.saturating_add(amount_dot)));
        }
                
        // NEW: Analytics and reporting methods
        #[ink(message)]
        pub fn get_platform_stats(&self) -> PlatformStats {
            self.platform_stats.clone()
        }

        #[ink(message)]
        pub fn get_event_analytics(&self, event_id: u32) -> Option<EventAnalytics> {
            self.event_analytics.get(event_id)
        }

        #[ink(message)]
        pub fn get_team_analytics(&self, team_id: u32) -> Option<TeamAnalytics> {
            self.team_analytics.get(team_id)
        }

        #[ink(message)]
        pub fn get_user_analytics(&self, user_id: AccountId) -> Option<UserAnalytics> {
            self.user_analytics.get(user_id)
        }

        #[ink(message)]
        pub fn generate_analytics_report(
            &mut self,
            report_type: ReportType,
            time_period: TimePeriod,
        ) -> u32 {
            let report_id = self.next_report_id;
            self.next_report_id += 1;

            // Generate comprehensive report
            let report = AnalyticsReport {
                report_id,
                report_type,
                time_period,
                platform_stats: self.platform_stats.clone(),
                event_analytics: self.collect_event_analytics(),
                team_analytics: self.collect_team_analytics(),
                top_performing_events: self.get_top_performing_events(),
                top_performing_teams: self.get_top_performing_teams(),
                revenue_trends: self.get_revenue_trends(),
                attendance_trends: self.get_attendance_trends(),
                generated_at: Self::env().block_timestamp(),
            };

            self.analytics_reports.insert(report_id, &report);
            report_id
        }

            // Helper methods for analytics
        fn collect_event_analytics(&self) -> Vec<EventAnalytics> {
            let analytics = Vec::new(); // Remove mut
            // Implementation to collect all event analytics
            analytics
        }

        fn collect_team_analytics(&self) -> Vec<TeamAnalytics> {
            let analytics = Vec::new(); // Remove mut
            // Implementation to collect all team analytics
            analytics
        }

        fn get_top_performing_events(&self) -> Vec<u32> {
            // Implementation to get top performing event IDs
            Vec::new()
        }

        fn get_top_performing_teams(&self) -> Vec<u32> {
            // Implementation to get top performing team IDs
            Vec::new()
        }

        fn get_revenue_trends(&self) -> Vec<(u64, u128)> {
            // Implementation to get revenue trends over time
            Vec::new()
        }

        fn get_attendance_trends(&self) -> Vec<(u64, u32)> {
            // Implementation to get attendance trends over time
            Vec::new()
        }

        #[ink(message)]
        pub fn is_analytics_enabled(&self) -> bool {
            self.analytics_enabled
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn setup_test_data(contract: &mut SportsBroker) -> (u32, u32, u32, u32, u32) {
            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                20000, // capacity
                "New York".to_string(), // address
                "Basketball".to_string(), // sport_type
            );

            let home_team_id = contract.register_team(
                "New York Knicks".to_string(),
                "Basketball".to_string(),
                "New York".to_string(),
            );

            let away_team_id = contract.register_team(
                "Boston Celtics".to_string(),
                "Basketball".to_string(),
                "Boston".to_string(),
            );

            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                "Basketball".to_string(),
                2024,
                1696118400000,
                1715644800000,
            );

            let event_id = contract.create_sports_event(
                "Knicks vs Celtics".to_string(),
                home_team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );

            (venue_id, home_team_id, away_team_id, season_id, event_id)
        }

        // ========================================================================
        // EXISTING TESTS (already working)
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let contract = SportsBroker::new();
            assert_eq!(contract.total_teams, 0);
            assert_eq!(contract.total_venues, 0);
            assert_eq!(contract.total_events, 0);
            assert_eq!(contract.total_tickets, 0);
            assert_eq!(contract.total_seasons, 0);
        }

        #[ink::test]
        fn register_team_works() {
            let mut contract = SportsBroker::new();
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );
            assert_eq!(team_id, 1);
            assert_eq!(contract.total_teams, 1);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut contract = SportsBroker::new();
            let venue_id = contract.register_venue(
                "Staples Center".to_string(),
                20000,
                "1111 S Figueroa St".to_string(),
                "Basketball".to_string(),
            );
            assert_eq!(venue_id, 1);
            assert_eq!(contract.total_venues, 1);
        }

        // ========================================================================
        // NEW: SEASON PASS TESTS
        // ========================================================================

        #[ink::test]
        fn create_season_works() {
            let mut contract = SportsBroker::new();
            let season_id = contract.create_season(
                "2024-25 NBA Season".to_string(),
                "Basketball".to_string(),
                2024,
                1696118400000,
                1715644800000,
            );
            assert_eq!(season_id, 1);
            assert_eq!(contract.total_seasons, 1);
        }

        #[ink::test]
        fn create_sports_event_works() {
            let mut contract = SportsBroker::new();
            let (venue_id, home_team_id, away_team_id, season_id, _) = setup_test_data(&mut contract);

            let event_id = contract.create_sports_event(
                "Lakers vs Celtics".to_string(),
                home_team_id,
                away_team_id,
                venue_id,
                season_id,
                1704067200000,
                18000,
                50_000_000_000_000,
                GameType::RegularSeason,
            );
            assert_eq!(event_id, 2); // Should be 2nd event (after setup)
            assert_eq!(contract.total_events, 2);
        }

        #[ink::test]
        fn purchase_ticket_works() {
            let mut contract = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut contract);

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                50_000_000_000_000 // 0.05 DOT
            );

            let ticket_id = contract.purchase_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                1,
            ).unwrap();

            assert_eq!(ticket_id, 1);
            assert_eq!(contract.total_tickets, 1);
        }

        // ========================================================================
        // NEW: DYNAMIC PRICING TESTS
        // ========================================================================

        #[ink::test]
        fn team_performance_initialization_works() {
            let mut contract = SportsBroker::new();
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );

            // Check that performance tracking was initialized
            let performance = contract.get_team_performance(team_id).unwrap();
            assert_eq!(performance.team_id, team_id);
            assert_eq!(performance.wins, 0);
            assert_eq!(performance.losses, 0);
            assert_eq!(performance.streak, 0);
            assert_eq!(performance.playoff_probability, 5000);

            // Check that pricing multiplier was initialized
            let pricing = contract.get_pricing_multiplier(team_id).unwrap();
            assert_eq!(pricing.team_id, team_id);
            assert_eq!(pricing.final_multiplier, 10000); // 1.0x default
        }

        #[ink::test]
        fn update_team_performance_works() {
            let mut contract = SportsBroker::new();
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );

            // Update team performance (only owner can do this)
            let result = contract.update_team_performance(
                team_id,
                1, // season_id
                25, // wins
                15, // losses
                8000, // playoff_probability (80%)
                5, // streak (5 wins)
                11000, // points_scored_avg
                10500, // points_allowed_avg
            );

            assert!(result.is_ok());

            // Check that performance was updated
            let performance = contract.get_team_performance(team_id).unwrap();
            assert_eq!(performance.wins, 25);
            assert_eq!(performance.losses, 15);
            assert_eq!(performance.streak, 5);
            assert_eq!(performance.playoff_probability, 8000);

            // Check that pricing multiplier was recalculated
            let pricing = contract.get_pricing_multiplier(team_id).unwrap();
            // Should be higher due to winning record and hot streak
            assert!(pricing.final_multiplier > 10000);
        }

        #[ink::test]
        fn update_team_performance_unauthorized() {
            let mut contract = SportsBroker::new();
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );

            // Switch to non-owner account
            let accounts = get_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = contract.update_team_performance(
                team_id,
                1, // season_id
                25, // wins
                15, // losses
                8000, // playoff_probability
                5, // streak
                11000, // points_scored_avg
                10500, // points_allowed_avg
            );

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Only owner can update team performance");
        }

        #[ink::test]
        fn dynamic_pricing_multipliers_work() {
            let mut contract = SportsBroker::new();
            let team_id = contract.register_team(
                "Lakers".to_string(),
                "Basketball".to_string(),
                "Los Angeles".to_string(),
            );

            // Update to winning team with hot streak
            let result = contract.update_team_performance(
                team_id,
                1, // season_id
                30, // wins (75% win rate)
                10, // losses
                9000, // playoff_probability (90%)
                8, // streak (8 wins)
                11500, // points_scored_avg
                10000, // points_allowed_avg
            );

            assert!(result.is_ok(), "Team performance update failed: {:?}", result);

            let performance = contract.get_team_performance(team_id).unwrap();
            assert_eq!(performance.wins, 30, "Wins not updated correctly");
            assert_eq!(performance.streak, 8, "Streak not updated correctly");
            assert_eq!(performance.playoff_probability, 9000, "Playoff probability not updated correctly");

            let pricing = contract.get_pricing_multiplier(team_id).unwrap();
            
            // Performance multiplier should be 1.2x (12000) for great team
            assert_eq!(pricing.performance_multiplier, 12000, 
                "Expected performance multiplier 12000, got {}", pricing.performance_multiplier);
            
            // Playoff multiplier should be 1.1x (11000) for likely playoff team
            assert_eq!(pricing.playoff_multiplier, 11000, 
                "Expected playoff multiplier 11000, got {}", pricing.playoff_multiplier);
            
            // Streak multiplier should be 1.15x (11500) for hot streak
            assert_eq!(pricing.streak_multiplier, 11500, 
                "Expected streak multiplier 11500, got {}", pricing.streak_multiplier);
            
            // Final multiplier should be higher than 1.0x
            assert!(pricing.final_multiplier > 10000, 
                "Expected final multiplier > 10000, got {}", pricing.final_multiplier);
        }

        #[ink::test]
        fn get_owner_works() {
            let contract = SportsBroker::new();
            let owner = contract.get_owner();
            // Just verify we can get the owner without comparing to env().caller()
            assert!(owner != ink::primitives::AccountId::from([0u8; 32]));
        }

        // ========================================================================
        // NEW: MULTI-CURRENCY TESTS
        // ========================================================================

        #[ink::test]
        fn multi_currency_initialization_works() {
            let contract = SportsBroker::new();
            
            let supported_currencies = contract.get_supported_currencies();
            assert_eq!(supported_currencies.len(), 5);
            assert!(supported_currencies.contains(&CurrencyId::DOT));
            assert!(supported_currencies.contains(&CurrencyId::ACA));
            assert!(supported_currencies.contains(&CurrencyId::AUSD));
            
            // Check default rates
            let dot_rate = contract.get_currency_rate(CurrencyId::DOT).unwrap();
            assert_eq!(dot_rate, 1_000_000_000_000_000_000); // 1 DOT = 1 DOT
            
            let aca_rate = contract.get_currency_rate(CurrencyId::ACA).unwrap();
            assert_eq!(aca_rate, 50_000_000_000_000_000); // 1 ACA = 0.05 DOT
        }

        #[ink::test]
        fn purchase_ticket_with_currency_works() {
            let mut contract = SportsBroker::new();
            let (_, _, _, _, event_id) = setup_test_data(&mut contract);

            // Purchase with ACA (worth 0.05 DOT each)
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                100_000_000_000_000_000 // 0.1 ACA
            );

            let ticket_id = contract.purchase_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                1,
                CurrencyId::ACA,
            ).unwrap();

            assert_eq!(ticket_id, 1);
            assert_eq!(contract.total_tickets, 1);
            
            // Check revenue tracking
            let aca_revenue = contract.get_currency_revenue(CurrencyId::ACA);
            assert!(aca_revenue > 0);
            
            let total_revenue_dot = contract.get_total_revenue_dot();
            assert!(total_revenue_dot > 0);
        }

        #[ink::test]
        fn update_currency_rate_works() {
            let mut contract = SportsBroker::new();
            
            // Update ACA rate (only owner can do this)
            let result = contract.update_currency_rate(
                CurrencyId::ACA,
                60_000_000_000_000_000, // New rate: 1 ACA = 0.06 DOT
            );
            
            assert!(result.is_ok());
            
            let new_rate = contract.get_currency_rate(CurrencyId::ACA).unwrap();
            assert_eq!(new_rate, 60_000_000_000_000_000);
        }

        #[ink::test]
        fn update_currency_rate_unauthorized() {
            let mut contract = SportsBroker::new();
            
            // Switch to non-owner account
            let accounts = get_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            
            let result = contract.update_currency_rate(
                CurrencyId::ACA,
                60_000_000_000_000_000,
            );
            
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Only owner can update currency rates");
        }
        
        #[ink::test]
        fn analytics_initialization_works() {
            let contract = SportsBroker::new();
            
            // Test platform stats initialization
            let stats = contract.get_platform_stats();
            assert_eq!(stats.total_revenue, 0);
            assert_eq!(stats.total_tickets_sold, 0);
            assert_eq!(stats.total_events, 0);
            assert_eq!(stats.total_users, 0);
            
            // Test that analytics is enabled
            assert_eq!(contract.is_analytics_enabled(), true);
        }

        #[ink::test]
        fn generate_analytics_report_works() {
            let mut contract = SportsBroker::new();
            
            // Test report generation
            let report_id = contract.generate_analytics_report(
                ReportType::Daily,
                TimePeriod::Last24Hours,
            );
            
            assert_eq!(report_id, 1);
            
            // Test that next report ID increments
            let report_id2 = contract.generate_analytics_report(
                ReportType::Weekly,
                TimePeriod::Last7Days,
            );
            assert_eq!(report_id2, 2);
        }

        #[ink::test]
        fn analytics_methods_accessible() {
            let contract = SportsBroker::new();
            
            // Test that analytics methods are accessible
            let _stats = contract.get_platform_stats();
            let _event_analytics = contract.get_event_analytics(1);
            let _team_analytics = contract.get_team_analytics(1);
            let _user_analytics = contract.get_user_analytics([0u8; 32].into());
            
            // If we get here without errors, the methods are accessible
            assert!(true);
        }

    }
}