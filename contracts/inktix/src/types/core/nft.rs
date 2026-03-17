//! NFT type definitions for ticket tokenization.
//!
//! Defines `TicketNft` for on-chain ticket representation, `AttendanceToken`
//! for proof-of-attendance, and `TicketVerification` for QR-based entry validation.

use ink::prelude::string::String;
use ink::primitives::AccountId;

/// NFT metadata for a ticket
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TicketNft {
    pub token_id: u64,
    pub ticket_id: u64,
    pub owner: AccountId,
    pub event_id: u32,
    pub event_name: String,
    pub venue_name: String,
    pub event_date: u64,
    pub section: String,
    pub row: String,
    pub seat_number: u32,
    pub seat_type: String,
    pub access_level: String,
    pub minted_at: u64,
    pub metadata_uri: String,
    /// Hash of ticket data for QR verification
    pub verification_hash: [u8; 32],
    pub is_used: bool,
}

/// Proof of attendance token - awarded after event
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct AttendanceToken {
    pub token_id: u64,
    pub ticket_id: u64,
    pub event_id: u32,
    pub owner: AccountId,
    pub attended_at: u64,
}

/// Result of ticket verification
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct TicketVerification {
    pub is_valid: bool,
    pub is_used: bool,
    pub owner: AccountId,
    pub event_id: u32,
    pub event_name: String,
    pub section: String,
    pub row: String,
    pub seat_number: u32,
}
