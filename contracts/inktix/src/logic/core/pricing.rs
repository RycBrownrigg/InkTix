use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;
use ink::prelude::string::ToString;

/// Dynamic pricing engine
///
/// Calculates ticket prices based on multiple factors:
/// - **Demand**: sell-through rate (sold / capacity)
/// - **Time**: urgency as event date approaches
/// - **Seat type**: premium seats cost more
/// - **Team performance**: winning teams command higher prices (sports)
/// - **Rivalry**: head-to-head matchups drive demand (sports)
/// - **Season pass discount**: percentage off for pass holders
///
/// All multipliers use basis points (10000 = 1.0x) to avoid floating point.
pub struct DynamicPricing;

impl DynamicPricing {
    /// Calculate the dynamic price for a ticket
    /// Returns (final_price, applied_multiplier) where multiplier is in basis points
    pub fn calculate_price(
        storage: &InkTixStorage,
        event_id: u32,
        seat: &Seat,
        is_season_pass: bool,
    ) -> Result<(u128, u32), String> {
        let event = storage.events.get(event_id)
            .ok_or("Event not found".to_string())?;

        if !event.dynamic_pricing_enabled {
            // Dynamic pricing disabled — return base price with seat multiplier only
            let seat_mult = Self::seat_type_multiplier(&seat.seat_type);
            let price = event.base_price * seat_mult as u128 / 10000;
            return Ok((price, seat_mult));
        }

        let base = event.base_price;

        // 1. Demand multiplier: based on sell-through percentage
        let demand_mult = Self::demand_multiplier(event.sold_tickets, event.capacity);

        // 2. Time urgency multiplier: increases as event approaches
        let time_mult = Self::time_multiplier(event.date);

        // 3. Seat type multiplier: premium seats cost more
        let seat_mult = if seat.price_multiplier > 0 {
            seat.price_multiplier
        } else {
            Self::seat_type_multiplier(&seat.seat_type)
        };

        // 4. Rivalry multiplier (stored on event, from event creation)
        let rivalry_mult = event.rivalry_multiplier;

        // 5. Team performance multiplier (sports only)
        let perf_mult = match &event.category {
            EventCategory::Sports { home_team_id, .. } => {
                Self::performance_multiplier(storage, *home_team_id)
            }
            _ => 10000, // 1.0x for non-sports
        };

        // 6. Season pass discount
        let discount_mult = if is_season_pass && event.season_pass_discount > 0 {
            10000 - (event.season_pass_discount as u32 * 100) // e.g., 15% = 8500
        } else {
            10000
        };

        // Combine all multipliers: base * (each / 10000)
        // To avoid overflow, apply sequentially
        let mut price = base;
        price = price * demand_mult as u128 / 10000;
        price = price * time_mult as u128 / 10000;
        price = price * seat_mult as u128 / 10000;
        price = price * rivalry_mult as u128 / 10000;
        price = price * perf_mult as u128 / 10000;
        price = price * discount_mult as u128 / 10000;

        // Floor: never go below 50% of base price
        let floor = base / 2;
        if price < floor {
            price = floor;
        }

        // Cap: never exceed 3x base price (anti-scalping)
        let cap = base * 3;
        if price > cap {
            price = cap;
        }

        // Calculate combined multiplier for record keeping
        let final_mult = if base > 0 {
            (price * 10000 / base) as u32
        } else {
            10000
        };

        Ok((price, final_mult))
    }

    /// Demand multiplier based on sell-through rate
    /// 0% sold = 8000 (0.8x discount to drive sales)
    /// 50% sold = 10000 (1.0x baseline)
    /// 75% sold = 12000 (1.2x mild surge)
    /// 90% sold = 15000 (1.5x high demand)
    /// 95%+ sold = 18000 (1.8x scarcity premium)
    fn demand_multiplier(sold: u32, capacity: u32) -> u32 {
        if capacity == 0 { return 10000; }
        let pct = (sold as u64 * 100 / capacity as u64) as u32;
        match pct {
            0..=19 => 8000,    // Under 20%: discount
            20..=39 => 9000,   // 20-39%: slight discount
            40..=59 => 10000,  // 40-59%: baseline
            60..=74 => 11000,  // 60-74%: mild surge
            75..=89 => 12500,  // 75-89%: surge
            90..=94 => 15000,  // 90-94%: high demand
            _ => 18000,        // 95%+: scarcity
        }
    }

    /// Time urgency multiplier
    /// >30 days out = 9000 (0.9x early bird)
    /// 7-30 days = 10000 (1.0x baseline)
    /// 1-7 days = 11500 (1.15x last week)
    /// <24 hours = 13000 (1.3x day-of)
    fn time_multiplier(event_date: u64) -> u32 {
        let now = ink::env::block_timestamp::<ink::env::DefaultEnvironment>();
        if event_date <= now { return 10000; } // Past event, baseline
        let diff_ms = event_date - now;
        let days = diff_ms / (24 * 60 * 60 * 1000);
        match days {
            0 => 13000,        // Day of event
            1..=6 => 11500,    // Within a week
            7..=29 => 10000,   // 1-4 weeks out
            _ => 9000,         // Early bird discount
        }
    }

    /// Seat type multiplier
    fn seat_type_multiplier(seat_type: &SeatType) -> u32 {
        match seat_type {
            SeatType::GeneralAdmission => 10000,  // 1.0x
            SeatType::Reserved => 11000,           // 1.1x
            SeatType::PremiumReserved => 13000,    // 1.3x
            SeatType::Club => 15000,               // 1.5x
            SeatType::Suite => 20000,              // 2.0x
            SeatType::FieldLevel => 18000,         // 1.8x
            SeatType::Courtside => 25000,          // 2.5x
            SeatType::StudentSection => 7000,      // 0.7x discount
        }
    }

    /// Team performance multiplier based on stored team performance data
    fn performance_multiplier(storage: &InkTixStorage, team_id: u32) -> u32 {
        match storage.team_performance.get(team_id) {
            Some(perf) => {
                let mut mult: u32 = 10000;
                // Winning teams: up to +20%
                if perf.win_percentage > 700 { mult += 2000; }
                else if perf.win_percentage > 500 { mult += 1000; }
                // Hot streak: up to +10%
                if perf.streak > 5 { mult += 1000; }
                else if perf.streak > 3 { mult += 500; }
                // Playoff contender: up to +15%
                if perf.playoff_probability > 80 { mult += 1500; }
                else if perf.playoff_probability > 50 { mult += 750; }
                mult
            }
            None => 10000, // No data, baseline
        }
    }

    /// Update team performance data (owner only, called externally)
    pub fn update_team_performance(
        storage: &mut InkTixStorage,
        team_id: u32,
        wins: u32,
        losses: u32,
        streak: i32,
        playoff_probability: u32,
    ) -> Result<(), String> {
        let total = wins + losses;
        let win_percentage = if total > 0 { wins * 1000 / total } else { 0 };

        let perf = TeamPerformance {
            team_id,
            season_id: 0,
            wins,
            losses,
            win_percentage,
            streak,
            playoff_probability,
            last_updated: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            performance_rank: 0,
            home_record_wins: 0,
            home_record_losses: 0,
            points_scored_avg: 0,
            points_allowed_avg: 0,
        };

        storage.team_performance.insert(team_id, &perf);
        Ok(())
    }

    /// Get a price quote without purchasing
    pub fn get_price_quote(
        storage: &InkTixStorage,
        event_id: u32,
        seat: &Seat,
        is_season_pass: bool,
    ) -> Result<PriceQuote, String> {
        let event = storage.events.get(event_id)
            .ok_or("Event not found".to_string())?;

        let (final_price, final_multiplier) = Self::calculate_price(
            storage, event_id, seat, is_season_pass
        )?;

        let demand_pct = if event.capacity > 0 {
            (event.sold_tickets as u64 * 100 / event.capacity as u64) as u32
        } else { 0 };

        Ok(PriceQuote {
            base_price: event.base_price,
            final_price,
            multiplier: final_multiplier,
            demand_percentage: demand_pct,
            demand_multiplier: Self::demand_multiplier(event.sold_tickets, event.capacity),
            time_multiplier: Self::time_multiplier(event.date),
            seat_multiplier: if seat.price_multiplier > 0 { seat.price_multiplier } else { Self::seat_type_multiplier(&seat.seat_type) },
            rivalry_multiplier: event.rivalry_multiplier,
            season_pass_discount: if is_season_pass { event.season_pass_discount } else { 0 },
        })
    }
}
