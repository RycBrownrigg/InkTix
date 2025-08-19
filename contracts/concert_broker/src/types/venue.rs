use ink::prelude::{string::String, vec::Vec};

/// Comprehensive Venue structure for music venues
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Venue {
    pub id: u32,
    pub name: String,
    pub address: VenueAddress,
    pub venue_type: VenueType,
    pub capacity: u32,
    pub standing_capacity: Option<u32>,
    pub acoustic_rating: u8,
    pub amenities: Vec<VenueAmenity>,
    pub age_restrictions: AgeRestriction,
    pub parking_available: bool,
    pub parking_capacity: Option<u32>,
    pub public_transport_access: bool,
    pub accessibility_features: Vec<AccessibilityFeature>,
    pub sound_system_quality: SoundSystemRating,
    pub lighting_capabilities: LightingCapabilities,
    pub backstage_facilities: bool,
    pub merchandise_space: bool,
    pub food_and_beverage: bool,
    pub alcohol_license: bool,
    pub security_level: SecurityLevel,
    pub established_year: Option<u32>,
    pub notable_past_performers: Vec<String>,
    pub average_ticket_price: u128,
    pub booking_contact: Option<String>,
    pub website: Option<String>,
    pub verified: bool,
    pub created_at: u64,
}

/// Address structure for venues
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct VenueAddress {
    pub street: String,
    pub city: String,
    pub state_province: String,
    pub country: String,
    pub postal_code: String,
    pub latitude: Option<i32>,
    pub longitude: Option<i32>,
}

/// Types of music venues
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum VenueType {
    Arena, Stadium, Theater, Club, Bar, ConcertHall, Amphitheater, FestivalGround,
    OperaHouse, JazzClub, ComedyClub, MultiPurpose, OutdoorVenue, PrivateVenue,
    RecordingStudio, Rooftop, Warehouse, Church, University, Other,
}

/// Venue amenities
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum VenueAmenity {
    VipLounge, CoatCheck, ATM, MerchandiseStand, MultipleBars, FoodCourt,
    OutdoorArea, DanceFloor, ReservedSeating, GeneralAdmission, BalconySeating,
    PrivateBoxes, MeetAndGreetSpace, PhotoOpportunities, ProfessionalPhotography,
    LiveStreamingCapable, RecordingCapable, ClimateControl, SmokingArea, ChargingStations,
}

/// Age restrictions for venues and events
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum AgeRestriction {
    AllAges,
    EighteenPlus,
    TwentyOnePlus,
    Custom(u8),
}

/// Accessibility features
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum AccessibilityFeature {
    WheelchairAccessible, ElevatorAccess, AccessibleRestrooms, AccessibleParking,
    SignLanguageInterpretation, AudioDescription, BraillePrograms, ServiceAnimalFriendly,
    SensoryFriendlyOptions, AssistedListeningDevices,
}

/// Sound system quality rating
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SoundSystemRating {
    Basic, Good, Excellent, WorldClass,
}

/// Lighting capabilities
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum LightingCapabilities {
    Basic, Professional, Advanced, Spectacular,
}

/// Security level of the venue
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SecurityLevel {
    Minimal, Standard, High, Maximum,
}