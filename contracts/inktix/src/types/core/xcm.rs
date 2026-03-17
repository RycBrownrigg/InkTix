//! XCM (Cross-Consensus Messaging) type definitions.
//!
//! Models XCM messages, statuses, fee structures, connectivity tracking,
//! and analytics for cross-chain ticket operations between parachains.

use ink::prelude::vec::Vec;
use ink::prelude::string::String;

/// XCM message types for cross-chain ticket operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum XcmMessageType {
    TicketPurchaseRequest,
    TicketPurchaseResponse,
    PaymentConfirmation,
    TicketTransfer,
    ConnectivityUpdate,
    Error,
}

/// XCM message status tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum XcmMessageStatus {
    Sent,
    Processing,
    Completed,
    Failed,
    Timeout,
    Cancelled,
}

/// XCM message for cross-chain ticket operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessage {
    pub message_id: u64,
    pub message_type: XcmMessageType,
    pub source_chain: String,
    pub destination_chain: String,
    pub source_parachain_id: u32,
    pub destination_parachain_id: u32,
    pub xcm_version: u8,
    pub payload: Vec<u8>,
    pub status: XcmMessageStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub fee_paid: u128,
    pub fee_currency: String,
    pub error_message: Option<String>,
}

/// XCM ticket purchase request payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmTicketPurchaseRequest {
    pub user: String,
    pub source_event_id: u32,
    pub destination_event_id: u32,
    pub quantity: u32,
    pub preferred_sections: Vec<String>,
    pub payment_currency: String,
    pub payment_amount: u128,
    pub user_signature: Vec<u8>,
    pub nonce: u64,
}

/// XCM ticket purchase response payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmTicketPurchaseResponse {
    pub request_message_id: u64,
    pub success: bool,
    pub ticket_ids: Option<Vec<u32>>,
    pub error_message: Option<String>,
    pub transaction_hash: Option<String>,
    pub timestamp: u64,
}

/// XCM payment confirmation payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmPaymentConfirmation {
    pub payment_hash: String,
    pub amount: u128,
    pub currency: String,
    pub source_chain: String,
    pub destination_chain: String,
    pub timestamp: u64,
    pub block_number: u64,
}

/// XCM connectivity status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmConnectivityStatus {
    pub chain_id: String,
    pub is_connected: bool,
    pub last_heartbeat: u64,
    pub latency_ms: Option<u64>,
    pub supported_xcm_version: u8,
    pub max_message_size: u32,
    pub fee_structure: XcmFeeStructure,
}

/// XCM fee structure for different operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmFeeStructure {
    pub base_fee: u128,
    pub fee_per_byte: u128,
    pub ticket_purchase_fee: u128,
    pub payment_confirmation_fee: u128,
    pub ticket_transfer_fee: u128,
    pub fee_currency: String,
}

/// XCM message filters for querying
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessageFilters {
    pub message_type: Option<XcmMessageType>,
    pub source_chain: Option<String>,
    pub destination_chain: Option<String>,
    pub status: Option<XcmMessageStatus>,
    pub date_range: Option<XcmDateRange>,
    pub user: Option<String>,
}

/// Date range for filtering
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmDateRange {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

/// XCM analytics and statistics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmAnalytics {
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub total_successful_operations: u64,
    pub total_failed_operations: u64,
    pub total_fees_paid: u128,
    pub average_processing_time: u64,
    pub connected_chains_count: u32,
    pub messages_by_type: Vec<XcmMessageTypeCount>,
    pub messages_by_status: Vec<XcmStatusCount>,
}

/// Count of messages by type
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessageTypeCount {
    pub message_type: XcmMessageType,
    pub count: u64,
}

/// Count of messages by status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmStatusCount {
    pub status: XcmMessageStatus,
    pub count: u64,
}
