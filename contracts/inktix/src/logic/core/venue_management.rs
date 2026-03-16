use crate::storage::*;
use crate::types::*;
use crate::types::core::venue;
use ink::prelude::string::String;
use ink::prelude::string::ToString;
use ink::prelude::vec;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]

/// Venue management logic
pub struct VenueManagement;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::cast_possible_truncation)]
impl VenueManagement {
    /// Register a new venue
    pub fn register_venue(
        storage: &mut InkTixStorage,
        name: String,
        capacity: u32,
        location: String,
        venue_type: VenueType,
    ) -> Result<u32, String> {
        let venue_id = storage.get_next_id("venue");
        let current_time = Self::get_current_timestamp();

        let new_venue = Venue {
            id: venue_id, name, city: location, capacity, venue_type,
            amenities: vec![],
            parking_info: venue::ParkingInfo {
                total_spaces: 1000, reserved_spaces: 50,
                general_parking_price: 2000000000000000000, premium_parking_price: 5000000000000000000,
                valet_available: true, valet_price: 10000000000000000000,
                parking_passes_available: true, parking_pass_price: 100000000000000000000,
                parking_pass_duration: 365 * 24 * 60 * 60, overflow_lots: vec![], pricing_tiers: vec![],
                access_control: venue::ParkingAccessControl::Open, payment_methods: vec![],
                special_events_pricing: false,
            },
            concession_info: venue::ConcessionInfo {
                food_available: true, beverage_available: true, alcohol_available: true,
                concession_credits_supported: true, credit_denomination: 1000000000000000000,
                credit_packages: vec![], dietary_options: vec![],
                average_meal_price: 15000000000000000000, total_stalls: 10,
                food_categories: vec![venue::FoodCategory::MainCourses], pricing_tiers: vec![],
                payment_methods: vec![venue::PaymentMethod::Cash], special_dietary_options: true,
            },
            merchandise_info: venue::MerchandiseInfo {
                merchandise_available: true, online_store: true, exclusive_items: true,
                bundle_discounts: true, merchandise_bundles: vec![], loyalty_discounts: vec![],
                average_item_price: 50000000000000000000, total_stores: 5,
                merchandise_categories: vec![], pricing_tiers: vec![], payment_methods: vec![],
                online_ordering: true, delivery_available: true,
            },
            loyalty_program: venue::VenueLoyaltyProgram {
                active: true, points_per_dollar: 1, tier_thresholds: vec![],
                venue_specific_benefits: vec![], partner_benefits: vec![],
                program_name: "Venue Loyalty".to_string(), tier_levels: vec![],
                benefits: vec![], redemption_options: vec![],
                expiration_policy: "Points expire after 1 year".to_string(),
            },
            pricing_tiers: vec![],
            capacity_management: venue::CapacityManagement {
                current_capacity: 0, reserved_capacity: 0, available_capacity: capacity,
                capacity_alerts: vec![], overflow_strategies: vec![],
                dynamic_pricing_enabled: false, capacity_based_pricing: false,
            },
            active: true, created_at: current_time, updated_at: current_time,
        };

        storage.venues.insert(venue_id, &new_venue);
        Ok(venue_id)
    }

    /// Get all venues
    pub fn get_all_venues(storage: &InkTixStorage) -> Vec<Venue> {
        let mut venues = Vec::new();
        for venue_id in 1..=storage.total_venues {
            if let Some(v) = storage.venues.get(venue_id) { venues.push(v); }
        }
        venues
    }

    /// Update venue capacity
    pub fn update_venue_capacity(
        storage: &mut InkTixStorage,
        venue_id: u32,
        new_capacity: u32,
    ) -> Result<(), String> {
        let mut v = storage.venues.get(venue_id).ok_or("Venue not found")?;
        v.capacity = new_capacity;
        v.capacity_management.available_capacity = new_capacity
            - v.capacity_management.current_capacity
            - v.capacity_management.reserved_capacity;
        v.updated_at = Self::get_current_timestamp();
        storage.venues.insert(venue_id, &v);
        Ok(())
    }

    /// Purchase parking pass
    pub fn purchase_parking_pass(
        storage: &mut InkTixStorage,
        buyer: AccountId,
        venue_id: u32,
        pass_type: venue::ParkingPassType,
        valid_from: u64,
        valid_until: u64,
        lot_name: String,
        currency: String,
    ) -> Result<u32, String> {
        let _venue = storage.venues.get(venue_id).ok_or("Venue not found")?;
        let pass_id = storage.get_next_id("venue");
        Ok(pass_id)
    }

    /// Purchase concession credits
    pub fn purchase_concession_credits(
        storage: &mut InkTixStorage,
        owner: AccountId,
        venue_id: u32,
        credit_amount: u128,
        credit_type: venue::ConcessionCreditType,
        valid_from: u64,
        valid_until: u64,
        purchase_price: u128,
        currency: String,
    ) -> Result<u32, String> {
        let v = storage.venues.get(venue_id).ok_or("Venue not found")?;
        if !v.concession_info.concession_credits_supported {
            return Err("Venue does not support concession credits".to_string());
        }
        let credit_id = storage.get_next_id("concession_credits");
        let credits = venue::ConcessionCredits {
            id: credit_id, owner, venue_id, credit_amount, remaining_amount: credit_amount,
            valid_from, valid_until, credit_type, is_active: true, purchase_price, currency,
            usage_history: vec![],
        };
        storage.concession_credits.insert(credit_id, &credits);
        Ok(credit_id)
    }

    /// Purchase merchandise bundle
    pub fn purchase_merchandise_bundle(
        storage: &mut InkTixStorage,
        owner: AccountId,
        venue_id: u32,
        bundle_id: u32,
        loyalty_discount: Option<u8>,
        pickup_location: String,
        pickup_deadline: u64,
    ) -> Result<u32, String> {
        let bundle = storage.merchandise_bundles.get(bundle_id).ok_or("Merchandise bundle not found")?;
        if !bundle.active { return Err("Merchandise bundle is not active".to_string()); }
        let purchase_id = storage.get_next_id("bundle_purchase");
        let current_time = Self::get_current_timestamp();
        let final_price = if let Some(discount) = loyalty_discount {
            bundle.bundle_price.saturating_sub((bundle.bundle_price * discount as u128) / 100)
        } else { bundle.bundle_price };
        let bundle_purchase = venue::MerchandiseBundlePurchase {
            id: purchase_id, owner, venue_id, bundle_id, purchase_date: current_time,
            total_price: final_price, currency: "DOT".to_string(),
            items: bundle.items.iter().map(|item| venue::BundleItemPurchase {
                item_name: item.item_name.clone(), item_type: item.item_type.clone(),
                quantity: item.quantity, individual_price: item.individual_price,
                final_price: item.individual_price, discount_applied: 0,
            }).collect(),
            loyalty_discount_applied: loyalty_discount, total_savings: bundle.savings_amount,
            pickup_location, pickup_deadline, is_picked_up: false,
        };
        storage.bundle_purchases.insert(purchase_id, &bundle_purchase);
        Ok(purchase_id)
    }

    fn get_current_timestamp() -> u64 { 1234567890 }
}
