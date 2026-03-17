//! Season pass package creation, purchasing, and event redemption logic.
//!
//! Manages the full lifecycle of season passes: creating package configurations,
//! processing purchases with staking requirements, and redeeming passes for individual events.
//!
//! # Functions
//! - `create_season_pass_package` -- defines a purchasable season pass offering
//! - `purchase_season_pass` -- issues a season pass to a buyer
//! - `use_season_pass_for_event` -- redeems a pass for entry to a specific event

use crate::storage::contract_storage::InkTixStorage;
use crate::types::sports::season_pass::*;
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::prelude::string::ToString;

/// Season pass lifecycle management
pub struct SeasonPassManagement;

#[allow(clippy::arithmetic_side_effects)]
impl SeasonPassManagement {
    /// Create a new season pass package with pricing and benefits
    pub fn create_season_pass_package(
        storage: &mut InkTixStorage, name: String, team_id: u32, season_id: u32,
        price: u128, currency: crate::types::core::currency::CurrencyId,
        staking_requirement: u128, benefits: Vec<SeasonPassBenefits>,
    ) -> Result<u32, String> {
        let package_id = storage.get_next_season_pass_package_id();
        let package = SeasonPassPackage {
            id: package_id, team_id, season_id, package_name: name,
            pass_type: SeasonPassType::FullSeason, total_games: 41, base_price: price, currency,
            max_quantity: 1000, sold_quantity: 0, benefits: benefits[0].clone(),
            staking_required: staking_requirement > 0, min_staking_amount: staking_requirement,
            staking_reward_rate: 500, active: true,
            sale_start_date: ink::env::block_timestamp::<DefaultEnvironment>(),
            sale_end_date: ink::env::block_timestamp::<DefaultEnvironment>() + 365 * 24 * 60 * 60 * 1000,
        };
        storage.season_pass_packages.insert(package_id, &package);
        storage.total_season_pass_packages += 1;
        Ok(package_id)
    }

    /// Purchase a season pass from an active package
    pub fn purchase_season_pass(storage: &mut InkTixStorage, user: AccountId, package_id: u32) -> Result<u32, String> {
        let package = storage.season_pass_packages.get(package_id).ok_or("Package not found")?;
        if !package.active { return Err("Package is not active".to_string()); }
        let pass_id = storage.get_next_season_pass_id();
        let current_time = ink::env::block_timestamp::<DefaultEnvironment>();
        let season_pass = SeasonPass {
            id: pass_id, owner: user, team_id: package.team_id, season_id: package.season_id,
            pass_type: package.pass_type.clone(), status: SeasonPassStatus::Active,
            purchase_date: current_time, activation_date: current_time,
            expiry_date: current_time + 365 * 24 * 60 * 60 * 1000,
            total_games: package.total_games, games_attended: 0, games_remaining: package.total_games,
            purchase_price: package.base_price, purchase_currency: package.currency,
            benefits: package.benefits.clone(), staking_amount: 0, staking_rewards_earned: 0,
            last_staking_update: current_time, transferable: true,
            transfer_cooldown_until: current_time + 30 * 24 * 60 * 60 * 1000,
        };
        storage.season_passes.insert(pass_id, &season_pass);
        let mut user_passes = storage.user_season_passes.get(user).unwrap_or(Vec::new());
        user_passes.push(pass_id);
        storage.user_season_passes.insert(user, &user_passes);
        let mut updated_package = package;
        updated_package.sold_quantity += 1;
        storage.season_pass_packages.insert(package_id, &updated_package);
        Ok(pass_id)
    }

    /// Redeem a season pass for entry to a specific event, issuing a free ticket
    pub fn use_season_pass_for_event(storage: &mut InkTixStorage, user: AccountId, season_pass_id: u32, event_id: u32) -> Result<u64, String> {
        let mut season_pass = storage.season_passes.get(season_pass_id).ok_or("Season pass not found")?;
        if season_pass.owner != user { return Err("Not the owner of this season pass".to_string()); }
        if season_pass.status != SeasonPassStatus::Active { return Err("Season pass is not active".to_string()); }
        if season_pass.games_remaining == 0 { return Err("No games remaining on this season pass".to_string()); }
        season_pass.games_attended += 1;
        season_pass.games_remaining -= 1;
        storage.season_passes.insert(season_pass_id, &season_pass);
        let ticket_id = storage.get_next_ticket_id();
        let ticket = crate::types::core::ticket::Ticket {
            id: ticket_id, event_id, owner: user, purchase_price: 0,
            purchase_currency: season_pass.purchase_currency,
            purchase_date: ink::env::block_timestamp::<DefaultEnvironment>(),
            seat_number: 1, section: "Season Pass".to_string(), row: "N/A".to_string(),
            seat_type: crate::types::core::seat::SeatType::GeneralAdmission,
            access_level: crate::types::core::seat::AccessLevel::Standard,
            transferable: true, loyalty_points_earned: 0, season_pass_discount_applied: true,
            is_season_pass_ticket: true, dynamic_price_paid: 0, performance_multiplier_applied: 0,
            dot_equivalent_paid: 0,
        };
        storage.tickets.insert(ticket_id, &ticket);
        let mut user_tickets = storage.user_tickets.get(user).unwrap_or(Vec::new());
        user_tickets.push(ticket_id);
        storage.user_tickets.insert(user, &user_tickets);
        Ok(ticket_id)
    }
}
