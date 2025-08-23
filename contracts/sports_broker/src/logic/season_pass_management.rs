
use crate::types::*;
use crate::storage::*;
use ink::primitives::AccountId;
use ink::env::DefaultEnvironment;

/// Season pass management functionality
pub struct SeasonPassManagement;

impl SeasonPassManagement {
    /// Create a new season pass package
    pub fn create_season_pass_package(
        storage: &mut SportsBrokerStorage,
        team_id: u32,
        season_id: u32,
        package_name: String,
        pass_type: SeasonPassType,
        total_games: u32,
        base_price: u128,
        currency: CurrencyId,
        max_quantity: u32,
        benefits: SeasonPassBenefits,
        staking_required: bool,
        min_staking_amount: u128,
        staking_reward_rate: u32,
        sale_start_date: u64,
        sale_end_date: u64,
    ) -> Result<u32, String> {
        // Validate team exists
        let _team = storage.teams.get(team_id)
            .ok_or("Team not found")?;

        // Validate season exists
        let _season = storage.seasons.get(season_id)
            .ok_or("Season not found")?;

        // Validate dates
        let current_time = ink::env::block_timestamp::<DefaultEnvironment>();
        if sale_start_date < current_time {
            return Err("Sale start date must be in the future".to_string());
        }
        if sale_end_date <= sale_start_date {
            return Err("Sale end date must be after start date".to_string());
        }

        // Create package
        let package_id = storage.get_next_season_pass_package_id();
        let package = SeasonPassPackage {
            id: package_id,
            team_id,
            season_id,
            package_name,
            pass_type,
            total_games,
            base_price,
            currency,
            max_quantity,
            sold_quantity: 0,
            benefits,
            staking_required,
            min_staking_amount,
            staking_reward_rate,
            active: true,
            sale_start_date,
            sale_end_date,
        };

        storage.season_pass_packages.insert(package_id, &package);

        // Initialize analytics for this package
        let analytics = SeasonPassAnalytics {
            total_passes_sold: 0,
            total_revenue: 0,
            average_attendance_rate: 0,
            total_staking_amount: 0,
            total_staking_rewards: 0,
            most_popular_package: package_id,
            renewal_rate: 0,
            customer_satisfaction_score: 0,
            last_updated: current_time,
        };
        storage.season_pass_analytics.insert(package_id, &analytics);

        Ok(package_id)
    }

    /// Purchase a season pass
    pub fn purchase_season_pass(
        storage: &mut SportsBrokerStorage,
        package_id: u32,
        buyer: AccountId,
        staking_amount: u128,
        current_time: u64,
    ) -> Result<u32, String> {

        // Get package
        let package = storage.season_pass_packages.get(package_id)
            .ok_or("Season pass package not found")?;

        // Validate package is active
        if !package.active {
            return Err("Package is not active".to_string());
        }

        // Validate sale dates
        if current_time < package.sale_start_date {
            return Err("Package is not yet on sale".to_string());
        }
        if current_time > package.sale_end_date {
            return Err("Package sale has ended".to_string());
        }

        // Validate quantity
        if package.sold_quantity >= package.max_quantity {
            return Err("Package is sold out".to_string());
        }

        // Validate staking requirements
        if package.staking_required && staking_amount < package.min_staking_amount {
            return Err("Insufficient staking amount".to_string());
        }

        // Create season pass
        let pass_id = storage.get_next_season_pass_id();
        let season_pass = SeasonPass {
            id: pass_id,
            owner: buyer,
            team_id: package.team_id,
            season_id: package.season_id,
            pass_type: package.pass_type.clone(),
            status: SeasonPassStatus::PendingActivation,
            purchase_date: current_time,
            activation_date: 0, // Will be set when activated
            expiry_date: 0,     // Will be set when activated
            total_games: package.total_games,
            games_attended: 0,
            games_remaining: package.total_games,
            purchase_price: package.base_price,
            purchase_currency: package.currency,
            benefits: package.benefits.clone(),
            staking_amount,
            staking_rewards_earned: 0,
            last_staking_update: current_time,
            transferable: true,
            transfer_cooldown_until: current_time + 30 * 24 * 60 * 60 * 1000, // 30 days
        };

        // Store season pass
        storage.season_passes.insert(pass_id, &season_pass);

        // Update package sold quantity
        let mut updated_package = package.clone();
        updated_package.sold_quantity += 1;
        storage.season_pass_packages.insert(package_id, &updated_package);

        // Update user's season passes
        let mut user_passes = storage.user_season_passes.get(buyer)
            .unwrap_or(Vec::new());
        user_passes.push(pass_id);
        storage.user_season_passes.insert(buyer, &user_passes);

        // Update team's season passes
        let mut team_passes = storage.team_season_passes.get(package.team_id)
            .unwrap_or(Vec::new());
        team_passes.push(pass_id);
        storage.team_season_passes.insert(package.team_id, &team_passes);

        // Update analytics
        if let Some(mut analytics) = storage.season_pass_analytics.get(package_id) {
            analytics.total_passes_sold += 1;
            analytics.total_revenue += package.base_price;
            analytics.total_staking_amount += staking_amount;
            analytics.last_updated = current_time;
            storage.season_pass_analytics.insert(package_id, &analytics);
        }

        // Update platform stats
        storage.platform_stats.total_season_passes += 1;
        storage.platform_stats.total_revenue += package.base_price;

        Ok(pass_id)
    }

    /// Activate a season pass
    pub fn activate_season_pass(
        storage: &mut SportsBrokerStorage,
        pass_id: u32,
        owner: AccountId,
        current_time: u64,
    ) -> Result<(), String> {

        // Get season pass
        let mut season_pass = storage.season_passes.get(pass_id)
            .ok_or("Season pass not found")?;

        // Validate ownership
        if season_pass.owner != owner {
            return Err("Not the owner of this season pass".to_string());
        }

        // Validate status
        if season_pass.status != SeasonPassStatus::PendingActivation {
            return Err("Season pass is not pending activation".to_string());
        }

        // Activate the pass
        season_pass.status = SeasonPassStatus::Active;
        season_pass.activation_date = current_time;
        season_pass.expiry_date = current_time + 365 * 24 * 60 * 60 * 1000; // 1 year

        storage.season_passes.insert(pass_id, &season_pass);

        Ok(())
    }

    /// Use season pass for an event
    pub fn use_season_pass_for_event(
        storage: &mut SportsBrokerStorage,
        pass_id: u32,
        event_id: u32,
        owner: AccountId,
        current_time: u64,
    ) -> Result<(), String> {

        // Get season pass
        let mut season_pass = storage.season_passes.get(pass_id)
            .ok_or("Season pass not found")?;

        // Validate ownership
        if season_pass.owner != owner {
            return Err("Not the owner of this season pass".to_string());
        }

        // Validate status
        if season_pass.status != SeasonPassStatus::Active {
            return Err("Season pass is not active".to_string());
        }

        // Validate event is for the same team and season
        let event = storage.events.get(event_id)
            .ok_or("Event not found")?;
        if (event.home_team_id != season_pass.team_id && event.away_team_id != season_pass.team_id) || event.season_id != season_pass.season_id {
            return Err("Event is not covered by this season pass".to_string());
        }

        // Validate games remaining
        if season_pass.games_remaining == 0 {
            return Err("No games remaining on this season pass".to_string());
        }

        // Use the pass
        season_pass.games_attended += 1;
        season_pass.games_remaining -= 1;
        storage.season_passes.insert(pass_id, &season_pass);

        // Record usage
        let usage_id = storage.get_next_season_pass_package_id(); // Reusing the counter
        let usage = SeasonPassUsage {
            pass_id,
            event_id,
            usage_date: current_time,
            entry_time: current_time,
            exit_time: None,
            benefits_used: Vec::new(), // Will be populated based on actual usage
            loyalty_points_earned: Self::calculate_loyalty_points(season_pass.purchase_price),
            staking_rewards_earned: 0, // Will be calculated based on staking
        };
        storage.season_pass_usage.insert(usage_id, &usage);

        // Update loyalty profile if exists
        if let Some(mut profile) = storage.loyalty_profiles.get(owner) {
            let points_earned = Self::calculate_loyalty_points(season_pass.purchase_price);
            profile.points_earned_this_month += points_earned;
            profile.points_earned_this_year += points_earned;
            profile.total_points += points_earned;
            
            // Apply loyalty multiplier if applicable
            if season_pass.benefits.loyalty_multiplier > 10000 {
                let bonus_points = points_earned * (season_pass.benefits.loyalty_multiplier - 10000) / 10000;
                profile.points_earned_this_month += bonus_points;
                profile.points_earned_this_year += bonus_points;
                profile.total_points += bonus_points;
            }
            
            storage.loyalty_profiles.insert(owner, &profile);
        }

        // Calculate and update staking rewards
        if season_pass.staking_amount > 0 && season_pass.benefits.staking_rewards {
            let rewards = Self::calculate_staking_rewards(
                season_pass.staking_amount,
                season_pass.last_staking_update,
                current_time,
                season_pass.benefits.loyalty_multiplier,
            );
            
            let mut updated_pass = season_pass;
            updated_pass.staking_rewards_earned += rewards;
            updated_pass.last_staking_update = current_time;
            storage.season_passes.insert(pass_id, &updated_pass);
        }

        Ok(())
    }

    /// Transfer season pass to another user
    pub fn transfer_season_pass(
        storage: &mut SportsBrokerStorage,
        pass_id: u32,
        from: AccountId,
        to: AccountId,
        current_time: u64,
    ) -> Result<(), String> {

        // Get season pass
        let mut season_pass = storage.season_passes.get(pass_id)
            .ok_or("Season pass not found")?;

        // Validate ownership
        if season_pass.owner != from {
            return Err("Not the owner of this season pass".to_string());
        }

        // Validate transferability
        if !season_pass.transferable {
            return Err("Season pass is not transferable".to_string());
        }

        // Validate cooldown period
        if current_time < season_pass.transfer_cooldown_until {
            return Err("Transfer cooldown period not yet expired".to_string());
        }

        // Update ownership
        season_pass.owner = to;
        season_pass.transfer_cooldown_until = current_time + 30 * 24 * 60 * 60 * 1000; // 30 days
        storage.season_passes.insert(pass_id, &season_pass);

        // Update user season pass lists
        let mut from_passes = storage.user_season_passes.get(from)
            .unwrap_or(Vec::new());
        from_passes.retain(|&id| id != pass_id);
        storage.user_season_passes.insert(from, &from_passes);

        let mut to_passes = storage.user_season_passes.get(to)
            .unwrap_or(Vec::new());
        to_passes.push(pass_id);
        storage.user_season_passes.insert(to, &to_passes);

        Ok(())
    }

    /// Get season pass by ID
    pub fn get_season_pass(
        storage: &SportsBrokerStorage,
        pass_id: u32,
    ) -> Option<SeasonPass> {
        storage.season_passes.get(pass_id)
    }

    /// Get all season passes for a user
    pub fn get_user_season_passes(
        storage: &SportsBrokerStorage,
        user: AccountId,
    ) -> Vec<SeasonPass> {
        if let Some(pass_ids) = storage.user_season_passes.get(user) {
            pass_ids.iter()
                .filter_map(|&id| storage.season_passes.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all season passes for a team
    pub fn get_team_season_passes(
        storage: &SportsBrokerStorage,
        team_id: u32,
    ) -> Vec<SeasonPass> {
        if let Some(pass_ids) = storage.team_season_passes.get(team_id) {
            pass_ids.iter()
                .filter_map(|&id| storage.season_passes.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get season pass package by ID
    pub fn get_season_pass_package(
        storage: &SportsBrokerStorage,
        package_id: u32,
    ) -> Option<SeasonPassPackage> {
        storage.season_pass_packages.get(package_id)
    }

    /// Get available season pass packages for a team
    pub fn get_available_packages_for_team(
        _storage: &SportsBrokerStorage,
        _team_id: u32,
    ) -> Vec<SeasonPassPackage> {
        // Since Mapping doesn't have iter(), we'll use a different approach
        // For now, return empty vector - this can be enhanced later
        Vec::new()
    }

    /// Calculate loyalty points for season pass usage
    fn calculate_loyalty_points(purchase_price: u128) -> u32 {
        // 2 points per 0.001 DOT (higher than regular tickets for season passes)
        (purchase_price / 500_000_000_000_000_000) as u32
    }

    /// Calculate staking rewards
    fn calculate_staking_rewards(
        staked_amount: u128,
        last_update: u64,
        current_time: u64,
        loyalty_multiplier: u32,
    ) -> u128 {
        let time_diff = current_time - last_update;
        let days = time_diff / (24 * 60 * 60 * 1000);
        
        // Base annual rate: 5% (500 basis points)
        let base_rate = 500;
        let daily_rate = base_rate * loyalty_multiplier / (365 * 10000);
        
        let rewards = staked_amount * daily_rate as u128 * days as u128 / 10000;
        rewards
    }

    /// Get season pass analytics
    pub fn get_season_pass_analytics(
        storage: &SportsBrokerStorage,
        package_id: u32,
    ) -> Option<SeasonPassAnalytics> {
        storage.season_pass_analytics.get(package_id)
    }

    /// Update season pass analytics
    pub fn update_season_pass_analytics(
        storage: &mut SportsBrokerStorage,
        package_id: u32,
        analytics: SeasonPassAnalytics,
    ) -> Result<(), String> {
        storage.season_pass_analytics.insert(package_id, &analytics);
        Ok(())
    }
}
