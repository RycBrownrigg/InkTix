use ink::prelude::string::String;
use ink::primitives::AccountId;
use crate::types::{CurrencyId, SeatType, AccessLevel};

/// Enhanced Ticket structure for sports with multi-currency support
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SportsTicket {
    pub id: u64,
    pub event_id: u32,
    pub owner: AccountId,
    pub purchase_price: u128,
    pub purchase_currency: CurrencyId,
    pub purchase_date: u64,
    pub seat_number: u32,
    pub transferable: bool,
    pub section: String,
    pub row: String,
    pub seat_type: SeatType,
    pub access_level: AccessLevel,
    pub loyalty_points_earned: u32,
    pub season_pass_discount_applied: bool,
    pub is_season_pass_ticket: bool,
    pub dynamic_price_paid: u128,
    pub performance_multiplier_applied: u32,
    pub dot_equivalent_paid: u128,
}