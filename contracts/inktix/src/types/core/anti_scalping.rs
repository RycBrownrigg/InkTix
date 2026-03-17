//! Anti-scalping type definitions.
//!
//! Provides configuration structs for per-event purchase limits, transfer restrictions,
//! resale controls, user behavior monitoring, and blacklist management.

use ink::primitives::AccountId;
use ink::prelude::vec::Vec;

/// Anti-scalping configuration for events
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct AntiScalpingConfig {
    pub event_id: u32,
    pub transfer_restricted: bool,
    pub max_tickets_per_user: u32,
    pub resale_allowed: bool,
    pub max_resale_price_multiplier: u8,
    pub resale_fee_percentage: u8,
    pub transfer_lock_period: u64,
    pub blacklisted_addresses: Vec<AccountId>,
    pub whitelisted_addresses: Vec<AccountId>,
    pub dynamic_pricing_enabled: bool,
    pub anti_bot_measures: bool,
}

/// Anti-scalping presets for common configurations
impl AntiScalpingConfig {
    /// Concert preset: 4 tickets per user, restricted transfers
    pub fn concert_preset(event_id: u32) -> Self {
        Self {
            event_id,
            transfer_restricted: true,
            max_tickets_per_user: 4,
            resale_allowed: false,
            max_resale_price_multiplier: 100,
            resale_fee_percentage: 0,
            transfer_lock_period: 86400,
            blacklisted_addresses: Vec::new(),
            whitelisted_addresses: Vec::new(),
            dynamic_pricing_enabled: false,
            anti_bot_measures: true,
        }
    }
}

/// Ticket transfer restrictions and history
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TicketTransferHistory {
    pub ticket_id: u64,
    pub original_owner: AccountId,
    pub current_owner: AccountId,
    pub transfer_count: u32,
    pub first_transfer_time: u64,
    pub last_transfer_time: u64,
    pub transfer_reasons: Vec<TransferReason>,
    pub price_history: Vec<(u64, u128)>,
}

/// Reasons for ticket transfers
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TransferReason {
    Gift,
    Resale,
    Refund,
    AdminTransfer,
    EmergencyTransfer,
}

/// User behavior monitoring for anti-scalping
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct UserBehaviorProfile {
    pub user_id: AccountId,
    pub total_tickets_purchased: u32,
    pub total_tickets_resold: u32,
    pub average_hold_time: u64,
    pub suspicious_activity_score: u8,
    pub last_purchase_time: u64,
    pub last_resale_time: u64,
    pub blacklist_status: BlacklistStatus,
    pub warning_count: u8,
}

/// Blacklist status levels
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum BlacklistStatus {
    Clean,
    Warning,
    Suspended,
    Banned,
}

/// Resale marketplace entry
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ResaleListing {
    pub listing_id: u64,
    pub ticket_id: u64,
    pub seller: AccountId,
    pub asking_price: u128,
    pub original_price: u128,
    pub listing_time: u64,
    pub expiry_time: u64,
    pub is_active: bool,
    pub approved: bool,
}
