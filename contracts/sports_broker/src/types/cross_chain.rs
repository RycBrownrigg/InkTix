use ink::prelude::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum BlockchainNetwork {
    Polkadot,
    Kusama,
    Moonbeam,
    Astar,
    Bifrost,
    Karura,
    Other(String),
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum PaymentMethod {
    Native,
    StableCoin,
    WrappedToken,
    CrossChainToken,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FeeType {
    Network,
    Processing,
    CrossChain,
    CurrencyConversion,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainFee {
    pub fee_type: FeeType,
    pub amount: u128,
    pub currency: String,
    pub is_optional: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
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

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum RequirementType {
    Age,
    Identity,
    KYC,
    Geographic,
    Technical,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ChainRequirement {
    pub requirement_type: RequirementType,
    pub value: String,
    pub is_mandatory: bool,
}

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
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainTicketRequest {
    pub user: ink::primitives::AccountId,
    pub request_status: CrossChainRequestStatus,
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CrossChainTransaction {
    pub transaction_status: CrossChainTransactionStatus,
    pub updated_at: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct DateRange {
    pub start_date: u64,
    pub end_date: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PriceRange {
    pub min_price: u128,
    pub max_price: u128,
    pub currency: String,
}

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
