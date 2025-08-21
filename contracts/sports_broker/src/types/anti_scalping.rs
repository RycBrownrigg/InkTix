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
    pub max_resale_price_multiplier: u8, // 100 = 1.0x, 150 = 1.5x
    pub resale_fee_percentage: u8, // 0-100
    pub transfer_lock_period: u64, // seconds after purchase
    pub blacklisted_addresses: Vec<AccountId>,
    pub whitelisted_addresses: Vec<AccountId>,
    pub dynamic_pricing_enabled: bool,
    pub anti_bot_measures: bool,
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
    pub price_history: Vec<(u64, u128)>, // timestamp, price
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
    pub average_hold_time: u64, // seconds
    pub suspicious_activity_score: u8, // 0-100
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