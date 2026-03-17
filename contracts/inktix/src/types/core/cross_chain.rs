//! Cross-chain type definitions for multi-network ticket distribution.
//!
//! Defines blockchain network identifiers, cross-chain event metadata,
//! transaction and request statuses, fee structures, filtering, and analytics
//! for bridging ticket operations across Polkadot ecosystem chains.

use ink::prelude::vec::Vec;
use ink::prelude::string::String;

/// Supported blockchain networks for cross-chain operations
#[allow(clippy::cast_possible_truncation)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum BlockchainNetwork {
    Polkadot,
    Kusama,
    Moonbeam,
    Astar,
    Bifrost,
    Karura,
    Other(String),
}

/// Lifecycle status of a cross-chain event
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CrossChainEventStatus {
    Active,
    Pending,
    Completed,
    Cancelled,
    Expired,
}

/// Rich metadata attached to a cross-chain event listing
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainEventMetadata {
    pub description: String,
    pub image_url: String,
    pub external_links: Vec<String>,
    pub tags: Vec<String>,
    pub chain_specific_data: Vec<String>,
}

/// Payment methods available for cross-chain transactions
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum CrossChainPaymentMethod {
    Native,
    StableCoin,
    WrappedToken,
    CrossChainToken,
    Other(String),
}

/// Categories of fees charged during cross-chain operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum FeeType {
    Network,
    Processing,
    CrossChain,
    CurrencyConversion,
    Other(String),
}

/// A single fee line item for a cross-chain operation
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainFee {
    pub fee_type: FeeType,
    pub amount: u128,
    pub currency: String,
    pub is_optional: bool,
}

/// Native and wrapped currencies supported across connected chains
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum SupportedCurrency {
    DOT,
    KSM,
    GLMR,
    ASTR,
    BIF,
    KAR,
    USDC,
    USDT,
    DAI,
    Other(String),
}

/// Types of requirements for cross-chain event participation
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum RequirementType {
    Age,
    Identity,
    KYC,
    Geographic,
    Technical,
    Other(String),
}

/// A prerequisite that must be satisfied for cross-chain participation
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ChainRequirement {
    pub requirement_type: RequirementType,
    pub value: String,
    pub is_mandatory: bool,
}

/// Full representation of an event bridged to another chain
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainEvent {
    pub event_id: u32,
    pub source_chain: BlockchainNetwork,
    pub event_name: String,
    pub event_description: String,
    pub base_ticket_price: u128,
    pub currency: String,
    pub event_date: u64,
    pub venue_name: String,
    pub venue_location: String,
    pub sport_type: String,
    pub team_names: Vec<String>,
    pub total_tickets: u32,
    pub available_tickets: u32,
    pub status: CrossChainEventStatus,
    pub metadata: CrossChainEventMetadata,
    pub fees: Vec<CrossChainFee>,
    pub supported_currencies: Vec<SupportedCurrency>,
    pub requirements: Vec<ChainRequirement>,
    pub created_at: u64,
    pub updated_at: u64,
    pub max_tickets: u32,
    pub tickets_sold: u32,
    pub target_chain: BlockchainNetwork,
    pub bridge_fee: u128,
    pub cross_chain_fee: u128,
    pub bridge_tx_hash: Option<String>,
    pub completion_timestamp: Option<u64>,
}

/// Status of a cross-chain ticket purchase request
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CrossChainRequestStatus {
    Pending,
    Approved,
    Rejected,
    Processing,
    Completed,
    Failed,
}

/// A user's request to purchase a ticket on another chain
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainTicketRequest {
    pub user: ink::primitives::AccountId,
    pub request_status: CrossChainRequestStatus,
}

/// Lifecycle status of a cross-chain transaction
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CrossChainTransactionStatus {
    Initiated,
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Tracks the state of a cross-chain bridging transaction
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainTransaction {
    pub transaction_status: CrossChainTransactionStatus,
    pub updated_at: u64,
}

/// A start/end date pair for filtering operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct DateRange {
    pub start_date: u64,
    pub end_date: u64,
}

/// A min/max price range with currency for filtering
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PriceRange {
    pub min_price: u128,
    pub max_price: u128,
    pub currency: String,
}

/// Filter criteria for searching cross-chain events
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainEventFilters {
    pub sport_types: Vec<String>,
    pub date_range: Option<DateRange>,
    pub price_range: Option<PriceRange>,
    pub source_chains: Vec<BlockchainNetwork>,
    pub venue_locations: Vec<String>,
    pub team_names: Vec<String>,
}

/// Aggregate statistics for cross-chain operations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainAnalytics {
    pub total_cross_chain_requests: u32,
    pub total_connected_chains: u32,
    pub events_by_chain: Vec<String>,
    pub requests_by_status: Vec<String>,
    pub transactions_by_status: Vec<String>,
    pub total_fees_collected: u128,
}

/// Real-time connectivity status for a connected blockchain
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ChainConnectivityStatus {
    pub chain: BlockchainNetwork,
    pub is_connected: bool,
    pub last_heartbeat: u64,
    pub latency_ms: Option<u64>,
    pub supported_features: Vec<String>,
    pub maintenance_mode: bool,
}
