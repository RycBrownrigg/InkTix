use ink::prelude::{string::String, vec::Vec};

/// Enhanced Concert Event structure with comprehensive music-specific features
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcertEvent {
    pub id: u32,
    pub name: String,
    pub artist_id: u32,
    pub venue_id: u32,
    pub date: u64,
    pub doors_open: u64,
    pub show_start: u64,
    pub estimated_end: u64,
    pub capacity: u32,
    pub sold_tickets: u32,
    pub base_price: u128,
    pub active: bool,
    pub event_type: EventType,
    pub tour_id: Option<u32>,
    pub festival_id: Option<u32>,
    pub supporting_artists: Vec<u32>,
    pub merchandise_available: bool,
    pub vip_packages: Vec<VIPPackage>,
    pub age_restriction: AgeRestriction,
    pub revenue_generated: u128,
    pub special_notes: String,
    pub presale_enabled: bool,
    pub presale_start: Option<u64>,
    pub general_sale_start: Option<u64>,
    pub sound_check_available: bool,
    pub meet_greet_available: bool,
    pub recording_allowed: bool,
    pub live_stream_available: bool,
    pub setlist_length_minutes: Option<u32>,
    pub encore_expected: bool,
    pub festival_stage: Option<String>,
    pub created_at: u64,
    pub last_updated: u64,
}

/// Event types for different concert experiences
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum EventType {
    Concert,
    FestivalDay,
    MeetAndGreet,
    SoundCheck,
    AlbumLaunch,
    AcousticSession,
    VirtualConcert,
    PrivateEvent,
    Masterclass,
    ListeningParty,
    UnpluggedSession,
    CharityBenefit,
    TributeConcert,
    ResidencyShow,
    PopupPerformance,
}

/// Tour structure for multi-date tour management
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Tour {
    pub id: u32,
    pub name: String,
    pub artist_id: u32,
    pub tour_type: TourType,
    pub start_date: u64,
    pub end_date: u64,
    pub total_shows: u32,
    pub shows_scheduled: u32,
    pub shows_completed: u32,
    pub tour_status: TourStatus,
    pub supporting_artists: Vec<u32>,
    pub merchandise_enabled: bool,
    pub vip_packages_available: bool,
    pub tour_manager_contact: Option<String>,
    pub sponsors: Vec<String>,
    pub total_revenue_generated: u128,
    pub average_ticket_price: u128,
    pub total_tickets_sold: u32,
    pub fan_presale_enabled: bool,
    pub description: String,
    pub poster_image_url: Option<String>,
    pub created_at: u64,
    pub last_updated: u64,
}

/// Tour types for different touring scales
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TourType {
    WorldTour, RegionalTour, NationalTour, LocalTour, FestivalCircuit,
    ResidencyTour, AcousticTour, ReunionTour, FarewellTour, PromotionalTour,
}

/// Tour status tracking
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum TourStatus {
    Announced, OnSale, Active, Completed, Postponed, Cancelled, Rescheduled,
}

/// Festival structure for multi-artist event management
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Festival {
    pub id: u32,
    pub name: String,
    pub venue_id: u32,
    pub festival_type: FestivalType,
    pub start_date: u64,
    pub end_date: u64,
    pub total_days: u32,
    pub headliner_artists: Vec<u32>,
    pub featured_artists: Vec<u32>,
    pub stages: Vec<Stage>,
    pub capacity_per_day: u32,
    pub total_capacity: u32,
    pub camping_available: bool,
    pub camping_capacity: Option<u32>,
    pub food_vendors: Vec<String>,
    pub merchandise_vendors: Vec<String>,
    pub age_restrictions: AgeRestriction,
    pub festival_status: FestivalStatus,
    pub ticket_types: Vec<FestivalTicketType>,
    pub sponsors: Vec<String>,
    pub organizer_contact: Option<String>,
    pub website: Option<String>,
    pub social_media: SocialMediaHandles,
    pub sustainability_features: Vec<SustainabilityFeature>,
    pub total_revenue_generated: u128,
    pub total_tickets_sold: u32,
    pub description: String,
    pub lineup_poster_url: Option<String>,
    pub created_at: u64,
    pub last_updated: u64,
}

/// Festival types for different festival categories
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FestivalType {
    MusicFestival, RockFestival, ElectronicFestival, JazzFestival, FolkFestival,
    CountryFestival, HipHopFestival, ClassicalFestival, ArtsFestival, CulturalFestival,
    CharityFestival, CorporateFestival,
}

/// Festival status tracking
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FestivalStatus {
    Planning, LineupAnnounced, OnSale, SoldOut, Active, Completed,
    Postponed, Cancelled, WeatherDelay,
}

/// Festival ticket types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum FestivalTicketType {
    GeneralAdmission, VIP, PlatinumVIP, DayPass(u32), WeekendPass,
    CampingPass, GroupPass(u32), EarlyBird, StudentDiscount, LocalResident,
}

/// Stage configuration for festivals
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Stage {
    pub name: String,
    pub stage_type: StageType,
    pub capacity: u32,
    pub sound_system: SoundSystemRating,
    pub lighting_system: LightingCapabilities,
    pub covered: bool,
    pub accessibility_compliant: bool,
}

/// Stage types for festivals
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum StageType {
    MainStage, SecondStage, AcousticStage, ElectronicStage, LocalStage,
    WorshipStage, ComedyStage, DanceStage, CommunityStage, SponsorStage,
}

/// Sustainability features for eco-friendly festivals
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SustainabilityFeature {
    SolarPower, WasteReduction, WaterConservation, LocalSourcing, CarbonNeutral,
    PublicTransport, BiodegradableSupplies, TreePlanting, PlasticFree, GreenVendors,
}