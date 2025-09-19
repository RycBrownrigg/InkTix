use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

/// Enhanced Venue structure with comprehensive venue-specific features
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Venue {
    pub id: u32,
    pub name: String,
    pub city: String,
    pub capacity: u32,
    pub venue_type: VenueType,
    pub amenities: Vec<VenueAmenity>,
    pub parking_info: ParkingInfo,
    pub concession_info: ConcessionInfo,
    pub merchandise_info: MerchandiseInfo,
    pub loyalty_program: VenueLoyaltyProgram,
    pub pricing_tiers: Vec<VenuePricingTier>,
    pub capacity_management: CapacityManagement,
    pub active: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Types of venues supported by the system
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum VenueType {
    Stadium,          // Large outdoor sports venues
    Arena,            // Indoor sports venues
    Ballpark,         // Baseball-specific venues
    SoccerField,      // Soccer/football fields
    TennisCourt,      // Tennis venues
    BasketballCourt,  // Basketball courts
    IceRink,          // Hockey venues
    SwimmingPool,     // Aquatic venues
    TrackAndField,    // Athletics venues
    ConcertHall,      // Music performance venues
    ConventionCenter, // Multi-purpose event spaces
    RaceTrack,        // Motorsports venues
    GolfCourse,       // Golf tournament venues
    Other(String),    // Custom venue types
}

/// Venue amenities available
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum VenueAmenity {
    WheelchairAccessible,
    PremiumSeating,
    VIPLounges,
    FoodAndBeverage,
    MerchandiseStores,
    ParkingGarage,
    PublicTransportation,
    FamilyRestrooms,
    NursingRooms,
    FirstAidStations,
    ATMs,
    WiFi,
    ChargingStations,
    SmokingAreas,
    PetFriendly,
    Other(String),
}

/// Parking information and management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ParkingInfo {
    pub total_spaces: u32,
    pub reserved_spaces: u32,
    pub general_parking_price: u128,
    pub premium_parking_price: u128,
    pub valet_available: bool,
    pub valet_price: u128,
    pub parking_passes_available: bool,
    pub parking_pass_price: u128,
    pub parking_pass_duration: u64, // Duration in seconds
    pub overflow_lots: Vec<OverflowLot>,
    pub pricing_tiers: Vec<ParkingPricingTier>,
    pub access_control: ParkingAccessControl,
    pub payment_methods: Vec<PaymentMethod>,
    pub special_events_pricing: bool,
}

/// Parking access control levels
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum ParkingAccessControl {
    Open,
    Restricted,
    VIPOnly,
    StaffOnly,
    Closed,
}

/// Parking pricing tier
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ParkingPricingTier {
    pub tier_name: String,
    pub price_multiplier: u32, // Multiplier as basis points (10000 = 1.0x)
    pub conditions: Vec<String>,
}

/// Payment method types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    DigitalWallet,
    Cryptocurrency,
    VenueCredits,
    Other(String),
}

/// Overflow parking lot information
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct OverflowLot {
    pub name: String,
    pub capacity: u32,
    pub price: u128,
    pub distance_from_venue: u32, // Distance in meters
    pub shuttle_service: bool,
}

/// Concession information and management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcessionInfo {
    pub food_available: bool,
    pub beverage_available: bool,
    pub alcohol_available: bool,
    pub concession_credits_supported: bool,
    pub credit_denomination: u128, // Smallest credit unit
    pub credit_packages: Vec<ConcessionCreditPackage>,
    pub dietary_options: Vec<DietaryOption>,
    pub average_meal_price: u128,
    pub total_stalls: u32,
    pub food_categories: Vec<FoodCategory>,
    pub pricing_tiers: Vec<ConcessionPricingTier>,
    pub payment_methods: Vec<PaymentMethod>,
    pub special_dietary_options: bool,
}

/// Food category types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum FoodCategory {
    Appetizers,
    MainCourses,
    Desserts,
    Beverages,
    Snacks,
    HealthyOptions,
    International,
    LocalSpecialties,
    Other(String),
}

/// Concession pricing tier
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcessionPricingTier {
    pub tier_name: String,
    pub price_multiplier: u32, // Multiplier as basis points (10000 = 1.0x)
    pub applicable_categories: Vec<FoodCategory>,
}

/// Concession credit package options
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcessionCreditPackage {
    pub id: u32,
    pub name: String,
    pub credit_amount: u128,
    pub price: u128,
    pub discount_percentage: u8, // Discount vs buying credits individually
    pub valid_duration: u64,     // Duration in seconds
    pub active: bool,
}

/// Dietary options available at concessions
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum DietaryOption {
    Vegetarian,
    Vegan,
    GlutenFree,
    Halal,
    Kosher,
    NutFree,
    DairyFree,
    LowSodium,
    Other(String),
}

/// Merchandise information and management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandiseInfo {
    pub merchandise_available: bool,
    pub online_store: bool,
    pub exclusive_items: bool,
    pub bundle_discounts: bool,
    pub merchandise_bundles: Vec<MerchandiseBundle>,
    pub loyalty_discounts: Vec<LoyaltyDiscount>,
    pub average_item_price: u128,
    pub total_stores: u32,
    pub merchandise_categories: Vec<MerchandiseCategory>,
    pub pricing_tiers: Vec<MerchandisePricingTier>,
    pub payment_methods: Vec<PaymentMethod>,
    pub online_ordering: bool,
    pub delivery_available: bool,
}

/// Merchandise category types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum MerchandiseCategory {
    Clothing,
    Accessories,
    Collectibles,
    Souvenirs,
    FoodAndBeverage,
    DigitalContent,
    Other(String),
}

/// Merchandise pricing tier
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandisePricingTier {
    pub tier_name: String,
    pub price_multiplier: u32, // Multiplier as basis points (10000 = 1.0x)
    pub applicable_categories: Vec<MerchandiseCategory>,
}

/// Merchandise bundle options
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandiseBundle {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub items: Vec<BundleItem>,
    pub bundle_price: u128,
    pub individual_price: u128, // Price if bought separately
    pub savings_amount: u128,
    pub savings_percentage: u8,
    pub limited_quantity: Option<u32>,
    pub active: bool,
}

/// Individual item in a merchandise bundle
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct BundleItem {
    pub item_name: String,
    pub item_type: MerchandiseType,
    pub individual_price: u128,
    pub quantity: u32,
}

/// Types of merchandise available
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum MerchandiseType {
    Clothing,
    Accessories,
    Collectibles,
    Souvenirs,
    FoodAndBeverage,
    DigitalContent,
    Other(String),
}

/// Loyalty discount information
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct LoyaltyDiscount {
    pub loyalty_tier: String,
    pub discount_percentage: u8,
    pub minimum_purchase: u128,
    pub applicable_categories: Vec<MerchandiseType>,
}

/// Venue loyalty program configuration
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct VenueLoyaltyProgram {
    pub active: bool,
    pub points_per_dollar: u32, // Points earned per dollar spent
    pub tier_thresholds: Vec<TierThreshold>,
    pub venue_specific_benefits: Vec<VenueBenefit>,
    pub partner_benefits: Vec<PartnerBenefit>,
    pub program_name: String,
    pub tier_levels: Vec<TierLevel>,
    pub benefits: Vec<LoyaltyBenefit>,
    pub redemption_options: Vec<RedemptionOption>,
    pub expiration_policy: String,
}

/// Loyalty tier level
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TierLevel {
    pub level_name: String,
    pub points_required: u32,
    pub benefits: Vec<String>,
}

/// Loyalty benefit
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct LoyaltyBenefit {
    pub benefit_name: String,
    pub benefit_type: String,
    pub value: u128,
    pub conditions: Vec<String>,
}

/// Redemption option
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct RedemptionOption {
    pub option_name: String,
    pub points_cost: u32,
    pub description: String,
    pub availability: bool,
}

/// Loyalty tier threshold
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct TierThreshold {
    pub tier_name: String,
    pub points_required: u32,
    pub benefits: Vec<String>,
}

/// Venue-specific benefits for loyalty members
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum VenueBenefit {
    FreeParking,
    ConcessionDiscounts(u8),  // Discount percentage
    MerchandiseDiscounts(u8), // Discount percentage
    PrioritySeating,
    VIPAccess,
    MeetAndGreetAccess,
    ExclusiveEvents,
    EarlyAccess(u64), // Early access timestamp
    Other(String),
}

/// Partner benefits for loyalty members
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct PartnerBenefit {
    pub partner_name: String,
    pub benefit_description: String,
    pub discount_percentage: Option<u8>,
    pub free_service: Option<String>,
    pub minimum_tier: String,
}

/// Venue pricing tier information
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct VenuePricingTier {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub base_price_multiplier: u32, // Multiplier as basis points (10000 = 1.0x)
    pub amenities_included: Vec<VenueAmenity>,
    pub parking_included: bool,
    pub concession_credits_included: u128,
    pub merchandise_discount: u8,
    pub active: bool,
}

/// Capacity management for venues
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CapacityManagement {
    pub current_capacity: u32,
    pub reserved_capacity: u32,
    pub available_capacity: u32,
    pub capacity_alerts: Vec<CapacityAlert>,
    pub overflow_strategies: Vec<OverflowStrategy>,
    pub dynamic_pricing_enabled: bool,
    pub capacity_based_pricing: bool,
}

/// Capacity alert thresholds
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CapacityAlert {
    pub threshold_percentage: u8, // Alert when capacity reaches this percentage
    pub alert_type: AlertType,
    pub action_required: String,
}

/// Types of capacity alerts
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum AlertType {
    Warning,
    Critical,
    Emergency,
    Information,
}

/// Overflow strategies for capacity management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum OverflowStrategy {
    StandingRoomOnly,
    AdditionalSeating,
    OverflowVenues,
    VirtualAttendance,
    WaitlistSystem,
    Other(String),
}

/// Parking pass for venue access
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ParkingPass {
    pub id: u32,
    pub owner: AccountId,
    pub venue_id: u32,
    pub pass_type: ParkingPassType,
    pub valid_from: u64,
    pub valid_until: u64,
    pub parking_lot: String,
    pub space_number: Option<String>,
    pub is_active: bool,
    pub purchase_price: u128,
    pub currency: String,
    pub transferable: bool,
    pub transfer_cooldown_until: u64,
}

/// Types of parking passes
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum ParkingPassType {
    SingleEvent,     // Valid for one event
    MultiEvent(u32), // Valid for multiple events
    Season,          // Valid for entire season
    Premium,         // Premium parking with reserved space
    Valet,           // Valet parking service
    Overflow,        // Overflow lot parking
}

/// Concession credits for venue purchases
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcessionCredits {
    pub id: u32,
    pub owner: AccountId,
    pub venue_id: u32,
    pub credit_amount: u128,
    pub remaining_amount: u128,
    pub valid_from: u64,
    pub valid_until: u64,
    pub credit_type: ConcessionCreditType,
    pub is_active: bool,
    pub purchase_price: u128,
    pub currency: String,
    pub usage_history: Vec<CreditUsage>,
}

/// Types of concession credits
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum ConcessionCreditType {
    Food,     // Food purchases only
    Beverage, // Beverage purchases only
    General,  // Any concession purchase
    Premium,  // Premium food and beverage
    Dietary,  // Specific dietary options
    Alcohol,  // Alcoholic beverages only
}

/// Credit usage tracking
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CreditUsage {
    pub timestamp: u64,
    pub amount_used: u128,
    pub item_purchased: String,
    pub location: String,
    pub remaining_balance: u128,
}

/// Merchandise bundle purchase
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandiseBundlePurchase {
    pub id: u32,
    pub owner: AccountId,
    pub venue_id: u32,
    pub bundle_id: u32,
    pub purchase_date: u64,
    pub total_price: u128,
    pub currency: String,
    pub items: Vec<BundleItemPurchase>,
    pub loyalty_discount_applied: Option<u8>,
    pub total_savings: u128,
    pub pickup_location: String,
    pub pickup_deadline: u64,
    pub is_picked_up: bool,
}

/// Individual item purchase in a bundle
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct BundleItemPurchase {
    pub item_name: String,
    pub item_type: MerchandiseType,
    pub quantity: u32,
    pub individual_price: u128,
    pub final_price: u128,
    pub discount_applied: u8,
}

/// Venue capacity reservation
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CapacityReservation {
    pub id: u32,
    pub venue_id: u32,
    pub event_id: u32,
    pub reserved_by: AccountId,
    pub reservation_type: ReservationType,
    pub capacity_reserved: u32,
    pub reservation_date: u64,
    pub valid_until: u64,
    pub is_active: bool,
    pub reservation_fee: u128,
    pub currency: String,
}

/// Types of capacity reservations
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[allow(clippy::cast_possible_truncation)]
pub enum ReservationType {
    Corporate,     // Corporate event reservation
    Group,         // Group reservation
    VIP,           // VIP section reservation
    Media,         // Media and press reservation
    Staff,         // Staff and crew reservation
    Overflow,      // Overflow capacity reservation
    Other(String), // Custom reservation type
}
