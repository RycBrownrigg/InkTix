use crate::types::{SeatType, VIPPackage, InkTixError, InkTixResult};

/// Pricing utility functions
pub struct PricingUtils;

impl PricingUtils {
    /// Calculate seat-based pricing
    pub fn calculate_seat_price(base_price: u128, seat_type: &SeatType) -> u128 {
        let multiplier = match seat_type {
            SeatType::GeneralAdmission => 100,
            SeatType::Reserved => 120,
            SeatType::PremiumReserved => 150,
            SeatType::VIPSeating => 200,
            SeatType::FrontRow => 300,
            SeatType::Balcony => 110,
            SeatType::FloorSeating => 180,
            SeatType::BoxSeats => 400,
            SeatType::StandingRoom => 80,
            SeatType::AccessibleSeating => 120,
        };

        (base_price * multiplier) / 100
    }

    /// Calculate VIP package pricing
    pub fn calculate_vip_pricing(base_price: u128, vip_package: &VIPPackage) -> u128 {
        base_price.saturating_add(vip_package.price_premium)
    }

    /// Calculate merchandise bundle pricing
    pub fn calculate_merchandise_bundle_price(
        merchandise_items: &[(u32, u32, u128)] // (id, quantity, unit_price)
    ) -> u128 {
        merchandise_items.iter()
            .map(|(_, quantity, unit_price)| unit_price.saturating_mul(*quantity as u128))
            .sum()
    }

    /// Calculate fan token discount
    pub fn calculate_fan_token_discount(price: u128, discount_percentage: u8) -> InkTixResult<u128> {
        if discount_percentage > 100 {
            return Err(InkTixError::InvalidData);
        }

        let discount_amount = (price * discount_percentage as u128) / 100;
        Ok(price.saturating_sub(discount_amount))
    }

    /// Calculate loyalty points based on purchase
    pub fn calculate_loyalty_points(seat_type: &SeatType, price: u128) -> u32 {
        let base_points = match seat_type {
            SeatType::GeneralAdmission => 10,
            SeatType::Reserved => 15,
            SeatType::PremiumReserved => 25,
            SeatType::VIPSeating => 50,
            SeatType::FrontRow => 100,
            SeatType::Balcony => 12,
            SeatType::FloorSeating => 40,
            SeatType::BoxSeats => 150,
            SeatType::StandingRoom => 8,
            SeatType::AccessibleSeating => 15,
        };

        let price_bonus = (price / 10_000_000_000) as u32;
        base_points + price_bonus
    }
}