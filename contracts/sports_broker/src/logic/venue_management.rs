use crate::storage::*;
use crate::types::*;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

/// Venue management logic with comprehensive venue-specific features
pub struct VenueManagement;

impl VenueManagement {
    /// Register a new venue with comprehensive features
    pub fn register_venue(
        storage: &mut SportsBrokerStorage,
        name: String,
        capacity: u32,
        address: String,
        _sport_type: String,
        venue_type: venue::VenueType,
        amenities: Vec<venue::VenueAmenity>,
        parking_info: venue::ParkingInfo,
        concession_info: venue::ConcessionInfo,
        merchandise_info: venue::MerchandiseInfo,
        loyalty_program: venue::VenueLoyaltyProgram,
        pricing_tiers: Vec<venue::VenuePricingTier>,
    ) -> u32 {
        let venue_id = storage.get_next_id("venue");
        let current_time = Self::get_current_timestamp();

        let venue = venue::Venue {
            id: venue_id,
            name,
            city: address,
            capacity,
            venue_type,
            amenities,
            parking_info,
            concession_info,
            merchandise_info,
            loyalty_program,
            pricing_tiers,
            capacity_management: venue::CapacityManagement {
                current_capacity: 0,
                reserved_capacity: 0,
                available_capacity: capacity,
                capacity_alerts: vec![],
                overflow_strategies: vec![],
                dynamic_pricing_enabled: false,
                capacity_based_pricing: false,
            },
            active: true,
            created_at: current_time,
            updated_at: current_time,
        };

        storage.venues.insert(venue_id, &venue);
        storage.total_venues += 1;
        venue_id
    }

    /// Get venue information
    pub fn get_venue(storage: &SportsBrokerStorage, venue_id: u32) -> Option<venue::Venue> {
        storage.venues.get(venue_id)
    }

    /// Update venue information
    pub fn update_venue(
        storage: &mut SportsBrokerStorage,
        venue_id: u32,
        name: Option<String>,
        capacity: Option<u32>,
        amenities: Option<Vec<venue::VenueAmenity>>,
        parking_info: Option<venue::ParkingInfo>,
        concession_info: Option<venue::ConcessionInfo>,
        merchandise_info: Option<venue::MerchandiseInfo>,
        loyalty_program: Option<venue::VenueLoyaltyProgram>,
        pricing_tiers: Option<Vec<venue::VenuePricingTier>>,
    ) -> Result<(), String> {
        let mut venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        if let Some(name) = name {
            venue.name = name;
        }
        if let Some(capacity) = capacity {
            venue.capacity = capacity;
            venue.capacity_management.available_capacity = capacity
                - venue.capacity_management.current_capacity
                - venue.capacity_management.reserved_capacity;
        }
        if let Some(amenities) = amenities {
            venue.amenities = amenities;
        }
        if let Some(parking_info) = parking_info {
            venue.parking_info = parking_info;
        }
        if let Some(concession_info) = concession_info {
            venue.concession_info = concession_info;
        }
        if let Some(merchandise_info) = merchandise_info {
            venue.merchandise_info = merchandise_info;
        }
        if let Some(loyalty_program) = loyalty_program {
            venue.loyalty_program = loyalty_program;
        }
        if let Some(pricing_tiers) = pricing_tiers {
            venue.pricing_tiers = pricing_tiers;
        }

        venue.updated_at = Self::get_current_timestamp();
        storage.venues.insert(venue_id, &venue);
        Ok(())
    }

    /// Create a parking pass
    pub fn create_parking_pass(
        storage: &mut SportsBrokerStorage,
        owner: AccountId,
        venue_id: u32,
        pass_type: venue::ParkingPassType,
        valid_from: u64,
        valid_until: u64,
        parking_lot: String,
        space_number: Option<String>,
        purchase_price: u128,
        currency: String,
    ) -> Result<u32, String> {
        // Verify venue exists
        let _venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        let pass_id = storage.get_next_id("parking_pass");
        let _current_time = Self::get_current_timestamp();

        let parking_pass = venue::ParkingPass {
            id: pass_id,
            owner,
            venue_id,
            pass_type,
            valid_from,
            valid_until,
            parking_lot,
            space_number,
            is_active: true,
            purchase_price,
            currency,
            transferable: false,
            transfer_cooldown_until: 0,
        };

        storage.parking_passes.insert(pass_id, &parking_pass);
        storage.total_parking_passes += 1;

        // Update user's parking passes
        let mut user_passes = storage.user_parking_passes.get(&owner).unwrap_or_default();
        user_passes.push(pass_id);
        storage.user_parking_passes.insert(&owner, &user_passes);

        // Update venue's parking passes
        let mut venue_passes = storage
            .venue_parking_passes
            .get(&venue_id)
            .unwrap_or_default();
        venue_passes.push(pass_id);
        storage
            .venue_parking_passes
            .insert(&venue_id, &venue_passes);

        Ok(pass_id)
    }

    /// Purchase concession credits
    pub fn purchase_concession_credits(
        storage: &mut SportsBrokerStorage,
        owner: AccountId,
        venue_id: u32,
        credit_amount: u128,
        credit_type: venue::ConcessionCreditType,
        valid_from: u64,
        valid_until: u64,
        purchase_price: u128,
        currency: String,
    ) -> Result<u32, String> {
        // Verify venue exists and supports concession credits
        let venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        if !venue.concession_info.concession_credits_supported {
            return Err("Venue does not support concession credits".to_string());
        }

        let credit_id = storage.get_next_id("concession_credits");

        let concession_credits = venue::ConcessionCredits {
            id: credit_id,
            owner,
            venue_id,
            credit_amount,
            remaining_amount: credit_amount,
            valid_from,
            valid_until,
            credit_type,
            is_active: true,
            purchase_price,
            currency,
            usage_history: vec![],
        };

        storage
            .concession_credits
            .insert(credit_id, &concession_credits);
        storage.total_concession_credits += 1;

        // Update user's concession credits
        let mut user_credits = storage
            .user_concession_credits
            .get(&owner)
            .unwrap_or_default();
        user_credits.push(credit_id);
        storage
            .user_concession_credits
            .insert(&owner, &user_credits);

        // Update venue's concession credits
        let mut venue_credits = storage
            .venue_concession_credits
            .get(&venue_id)
            .unwrap_or_default();
        venue_credits.push(credit_id);
        storage
            .venue_concession_credits
            .insert(&venue_id, &venue_credits);

        Ok(credit_id)
    }

    /// Use concession credits
    pub fn use_concession_credits(
        storage: &mut SportsBrokerStorage,
        credit_id: u32,
        amount: u128,
        item_purchased: String,
        location: String,
    ) -> Result<(), String> {
        let mut credits = storage
            .concession_credits
            .get(credit_id)
            .ok_or("Concession credits not found")?;

        if !credits.is_active {
            return Err("Concession credits are not active".to_string());
        }

        if credits.remaining_amount < amount {
            return Err("Insufficient concession credits".to_string());
        }

        let current_time = Self::get_current_timestamp();
        if current_time < credits.valid_from || current_time > credits.valid_until {
            return Err("Concession credits are not valid at this time".to_string());
        }

        credits.remaining_amount -= amount;

        // Record usage
        let usage = venue::CreditUsage {
            timestamp: current_time,
            amount_used: amount,
            item_purchased,
            location,
            remaining_balance: credits.remaining_amount,
        };
        credits.usage_history.push(usage);

        storage.concession_credits.insert(credit_id, &credits);
        Ok(())
    }

    /// Create a merchandise bundle
    pub fn create_merchandise_bundle(
        storage: &mut SportsBrokerStorage,
        venue_id: u32,
        name: String,
        description: String,
        items: Vec<venue::BundleItem>,
        bundle_price: u128,
        individual_price: u128,
        limited_quantity: Option<u32>,
    ) -> Result<u32, String> {
        // Verify venue exists
        let _venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        let bundle_id = storage.get_next_id("merchandise_bundle");

        let savings_amount = individual_price.saturating_sub(bundle_price);
        let savings_percentage = if individual_price > 0 {
            ((savings_amount * 100) / individual_price) as u8
        } else {
            0
        };

        let merchandise_bundle = venue::MerchandiseBundle {
            id: bundle_id,
            name,
            description,
            items,
            bundle_price,
            individual_price,
            savings_amount,
            savings_percentage,
            limited_quantity,
            active: true,
        };

        storage
            .merchandise_bundles
            .insert(bundle_id, &merchandise_bundle);
        storage.total_merchandise_bundles += 1;

        // Update venue's merchandise bundles
        let mut venue_bundles = storage
            .venue_merchandise_bundles
            .get(&venue_id)
            .unwrap_or_default();
        venue_bundles.push(bundle_id);
        storage
            .venue_merchandise_bundles
            .insert(&venue_id, &venue_bundles);

        Ok(bundle_id)
    }

    /// Purchase a merchandise bundle
    pub fn purchase_merchandise_bundle(
        storage: &mut SportsBrokerStorage,
        owner: AccountId,
        venue_id: u32,
        bundle_id: u32,
        loyalty_discount: Option<u8>,
        pickup_location: String,
        pickup_deadline: u64,
    ) -> Result<u32, String> {
        // Verify bundle exists and is active
        let bundle = storage
            .merchandise_bundles
            .get(bundle_id)
            .ok_or("Merchandise bundle not found")?;

        if !bundle.active {
            return Err("Merchandise bundle is not active".to_string());
        }

        // Check quantity limits
        if let Some(_limited_quantity) = bundle.limited_quantity {
            // TODO: Implement quantity tracking
        }

        let purchase_id = storage.get_next_id("bundle_purchase");
        let current_time = Self::get_current_timestamp();

        // Calculate final price with loyalty discount
        let final_price = if let Some(discount) = loyalty_discount {
            bundle
                .bundle_price
                .saturating_sub((bundle.bundle_price * discount as u128) / 100)
        } else {
            bundle.bundle_price
        };

        let total_savings = bundle.savings_amount;

        let bundle_purchase = venue::MerchandiseBundlePurchase {
            id: purchase_id,
            owner,
            venue_id,
            bundle_id,
            purchase_date: current_time,
            total_price: final_price,
            currency: "DOT".to_string(), // TODO: Make configurable
            items: bundle
                .items
                .iter()
                .map(|item| {
                    venue::BundleItemPurchase {
                        item_name: item.item_name.clone(),
                        item_type: item.item_type.clone(),
                        quantity: item.quantity,
                        individual_price: item.individual_price,
                        final_price: item.individual_price,
                        discount_applied: 0, // TODO: Calculate individual discounts
                    }
                })
                .collect(),
            loyalty_discount_applied: loyalty_discount,
            total_savings,
            pickup_location,
            pickup_deadline,
            is_picked_up: false,
        };

        storage
            .bundle_purchases
            .insert(purchase_id, &bundle_purchase);
        storage.total_bundle_purchases += 1;

        // Update user's merchandise bundles
        let mut user_bundles = storage
            .user_merchandise_bundles
            .get(&owner)
            .unwrap_or_default();
        user_bundles.push(purchase_id);
        storage
            .user_merchandise_bundles
            .insert(&owner, &user_bundles);

        Ok(purchase_id)
    }

    /// Reserve venue capacity
    pub fn reserve_venue_capacity(
        storage: &mut SportsBrokerStorage,
        venue_id: u32,
        event_id: u32,
        reserved_by: AccountId,
        reservation_type: venue::ReservationType,
        capacity_reserved: u32,
        reservation_fee: u128,
        currency: String,
        valid_until: u64,
    ) -> Result<u32, String> {
        let mut venue = storage.venues.get(venue_id).ok_or("Venue not found")?;

        if venue.capacity_management.available_capacity < capacity_reserved {
            return Err("Insufficient venue capacity available".to_string());
        }

        let reservation_id = storage.get_next_id("capacity_reservation");
        let current_time = Self::get_current_timestamp();

        let capacity_reservation = venue::CapacityReservation {
            id: reservation_id,
            venue_id,
            event_id,
            reserved_by,
            reservation_type,
            capacity_reserved,
            reservation_date: current_time,
            valid_until,
            is_active: true,
            reservation_fee,
            currency,
        };

        storage
            .capacity_reservations
            .insert(reservation_id, &capacity_reservation);
        storage.total_capacity_reservations += 1;

        // Update venue capacity
        venue.capacity_management.reserved_capacity += capacity_reserved;
        venue.capacity_management.available_capacity -= capacity_reserved;
        venue.updated_at = current_time;

        storage.venues.insert(venue_id, &venue);

        Ok(reservation_id)
    }

    /// Get user's parking passes
    pub fn get_user_parking_passes(
        storage: &SportsBrokerStorage,
        user: AccountId,
    ) -> Vec<venue::ParkingPass> {
        let pass_ids = storage.user_parking_passes.get(&user).unwrap_or_default();
        pass_ids
            .iter()
            .filter_map(|&id| storage.parking_passes.get(id))
            .collect()
    }

    /// Get user's concession credits
    pub fn get_user_concession_credits(
        storage: &SportsBrokerStorage,
        user: AccountId,
    ) -> Vec<venue::ConcessionCredits> {
        let credit_ids = storage
            .user_concession_credits
            .get(&user)
            .unwrap_or_default();
        credit_ids
            .iter()
            .filter_map(|&id| storage.concession_credits.get(id))
            .collect()
    }

    /// Get user's merchandise bundles
    pub fn get_user_merchandise_bundles(
        storage: &SportsBrokerStorage,
        user: AccountId,
    ) -> Vec<venue::MerchandiseBundlePurchase> {
        let bundle_ids = storage
            .user_merchandise_bundles
            .get(&user)
            .unwrap_or_default();
        bundle_ids
            .iter()
            .filter_map(|&id| storage.bundle_purchases.get(id))
            .collect()
    }

    /// Get venue's parking passes
    pub fn get_venue_parking_passes(
        storage: &SportsBrokerStorage,
        venue_id: u32,
    ) -> Vec<venue::ParkingPass> {
        let pass_ids = storage
            .venue_parking_passes
            .get(&venue_id)
            .unwrap_or_default();
        pass_ids
            .iter()
            .filter_map(|&id| storage.parking_passes.get(id))
            .collect()
    }

    /// Get venue's merchandise bundles
    pub fn get_venue_merchandise_bundles(
        storage: &SportsBrokerStorage,
        venue_id: u32,
    ) -> Vec<venue::MerchandiseBundle> {
        let bundle_ids = storage
            .venue_merchandise_bundles
            .get(&venue_id)
            .unwrap_or_default();
        bundle_ids
            .iter()
            .filter_map(|&id| storage.merchandise_bundles.get(id))
            .collect()
    }

    /// Check if parking pass is valid
    pub fn is_parking_pass_valid(
        storage: &SportsBrokerStorage,
        pass_id: u32,
        current_time: u64,
    ) -> bool {
        if let Some(pass) = storage.parking_passes.get(pass_id) {
            pass.is_active && current_time >= pass.valid_from && current_time <= pass.valid_until
        } else {
            false
        }
    }

    /// Check if concession credits are valid
    pub fn are_concession_credits_valid(
        storage: &SportsBrokerStorage,
        credit_id: u32,
        current_time: u64,
    ) -> bool {
        if let Some(credits) = storage.concession_credits.get(credit_id) {
            credits.is_active
                && current_time >= credits.valid_from
                && current_time <= credits.valid_until
        } else {
            false
        }
    }

    /// Get current timestamp (placeholder - should use proper timestamp)
    fn get_current_timestamp() -> u64 {
        // TODO: Replace with proper timestamp from blockchain
        1234567890
    }
}
