use ink::prelude::vec::Vec;

/// XCM message types for cross-chain ticket operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum XcmMessageType {
    /// Request to purchase tickets on another chain
    TicketPurchaseRequest,
    /// Response to a ticket purchase request
    TicketPurchaseResponse,
    /// Payment confirmation across chains
    PaymentConfirmation,
    /// Ticket transfer between chains
    TicketTransfer,
    /// Chain connectivity status update
    ConnectivityUpdate,
    /// Error message for failed operations
    Error,
}

/// XCM message status tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum XcmMessageStatus {
    /// Message has been sent but not yet processed
    Sent,
    /// Message is being processed by the destination chain
    Processing,
    /// Message has been successfully processed
    Completed,
    /// Message processing failed
    Failed,
    /// Message has timed out
    Timeout,
    /// Message has been cancelled
    Cancelled,
}

/// XCM message for cross-chain ticket operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessage {
    /// Unique identifier for the XCM message
    pub message_id: u64,
    /// Type of XCM message
    pub message_type: XcmMessageType,
    /// Source chain identifier
    pub source_chain: String,
    /// Destination chain identifier
    pub destination_chain: String,
    /// Parachain ID of the source
    pub source_parachain_id: u32,
    /// Parachain ID of the destination
    pub destination_parachain_id: u32,
    /// XCM version being used
    pub xcm_version: u8,
    /// Message payload (encoded data)
    pub payload: Vec<u8>,
    /// Current status of the message
    pub status: XcmMessageStatus,
    /// Timestamp when message was created
    pub created_at: u64,
    /// Timestamp when message was last updated
    pub updated_at: u64,
    /// Fee paid for the XCM message
    pub fee_paid: u128,
    /// Fee currency
    pub fee_currency: String,
    /// Error message if status is Failed
    pub error_message: Option<String>,
}

/// XCM ticket purchase request payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmTicketPurchaseRequest {
    /// User requesting the tickets
    pub user: String,
    /// Event ID on the source chain
    pub source_event_id: u32,
    /// Event ID on the destination chain
    pub destination_event_id: u32,
    /// Number of tickets requested
    pub quantity: u32,
    /// Preferred seating sections
    pub preferred_sections: Vec<String>,
    /// Payment currency
    pub payment_currency: String,
    /// Payment amount
    pub payment_amount: u128,
    /// User's signature for verification
    pub user_signature: Vec<u8>,
    /// Nonce for replay protection
    pub nonce: u64,
}

/// XCM ticket purchase response payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmTicketPurchaseResponse {
    /// Original request message ID
    pub request_message_id: u64,
    /// Whether the purchase was successful
    pub success: bool,
    /// Ticket IDs if successful
    pub ticket_ids: Option<Vec<u32>>,
    /// Error message if unsuccessful
    pub error_message: Option<String>,
    /// Transaction hash on destination chain
    pub transaction_hash: Option<String>,
    /// Timestamp of response
    pub timestamp: u64,
}

/// XCM payment confirmation payload
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmPaymentConfirmation {
    /// Payment transaction hash
    pub payment_hash: String,
    /// Amount confirmed
    pub amount: u128,
    /// Currency of payment
    pub currency: String,
    /// Source chain of payment
    pub source_chain: String,
    /// Destination chain
    pub destination_chain: String,
    /// Timestamp of confirmation
    pub timestamp: u64,
    /// Block number of confirmation
    pub block_number: u64,
}

/// XCM connectivity status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmConnectivityStatus {
    /// Chain identifier
    pub chain_id: String,
    /// Whether the chain is currently connected
    pub is_connected: bool,
    /// Last successful heartbeat timestamp
    pub last_heartbeat: u64,
    /// Current latency in milliseconds
    pub latency_ms: Option<u64>,
    /// Supported XCM version
    pub supported_xcm_version: u8,
    /// Maximum message size supported
    pub max_message_size: u32,
    /// Fee structure for XCM messages
    pub fee_structure: XcmFeeStructure,
}

/// XCM fee structure for different operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmFeeStructure {
    /// Base fee for any XCM message
    pub base_fee: u128,
    /// Fee per byte of message payload
    pub fee_per_byte: u128,
    /// Fee for ticket purchase operations
    pub ticket_purchase_fee: u128,
    /// Fee for payment confirmations
    pub payment_confirmation_fee: u128,
    /// Fee for ticket transfers
    pub ticket_transfer_fee: u128,
    /// Currency for fees
    pub fee_currency: String,
}

/// XCM message filters for querying
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessageFilters {
    /// Filter by message type
    pub message_type: Option<XcmMessageType>,
    /// Filter by source chain
    pub source_chain: Option<String>,
    /// Filter by destination chain
    pub destination_chain: Option<String>,
    /// Filter by status
    pub status: Option<XcmMessageStatus>,
    /// Filter by date range
    pub date_range: Option<XcmDateRange>,
    /// Filter by user
    pub user: Option<String>,
}

/// Date range for filtering
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmDateRange {
    /// Start timestamp
    pub start_timestamp: u64,
    /// End timestamp
    pub end_timestamp: u64,
}

/// XCM analytics and statistics
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmAnalytics {
    /// Total XCM messages sent
    pub total_messages_sent: u64,
    /// Total XCM messages received
    pub total_messages_received: u64,
    /// Total successful operations
    pub total_successful_operations: u64,
    /// Total failed operations
    pub total_failed_operations: u64,
    /// Total fees paid for XCM operations
    pub total_fees_paid: u128,
    /// Average message processing time
    pub average_processing_time: u64,
    /// Connected chains count
    pub connected_chains_count: u32,
    /// Messages by type distribution
    pub messages_by_type: Vec<XcmMessageTypeCount>,
    /// Messages by status distribution
    pub messages_by_status: Vec<XcmStatusCount>,
}

/// Count of messages by type
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmMessageTypeCount {
    /// Message type
    pub message_type: XcmMessageType,
    /// Count of messages
    pub count: u64,
}

/// Count of messages by status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct XcmStatusCount {
    /// Message status
    pub status: XcmMessageStatus,
    /// Count of messages
    pub count: u64,
}
