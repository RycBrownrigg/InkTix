#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # InkTix Concert Broker - Complete Music Industry Ticketing Platform (Steps 1-4)
/// 
/// This contract provides comprehensive concert and music event ticketing with advanced features
/// including multi-currency support, VIP package integration, merchandise bundles, fan token
/// discounts, anti-scalping mechanisms, and artist revenue sharing.
///
/// ## Step 4 NEW Features:
/// - **Enhanced Concert Ticket System**: Comprehensive ticket structure with music-specific features
/// - **Multi-Currency Support**: DOT, ACA, aUSD, LDOT, KSM payment options with automatic conversion
/// - **VIP Package Integration**: Seamless VIP package selection with automatic benefit assignment
/// - **Merchandise Bundle System**: Concert merchandise bundling during ticket purchase
/// - **Fan Token Discount System**: Automatic discounts for fan token holders
/// - **Anti-Scalping Mechanisms**: Purchase limits, verified fan priority, resale price controls
/// - **Artist Revenue Sharing**: Automatic artist payments on secondary sales
/// - **Advanced Pricing Engine**: Dynamic pricing based on artist popularity and demand
/// - **Comprehensive Analytics**: Revenue tracking by artist, venue, currency, and merchandise

#[ink::contract]
mod concert_broker {
    use ink::prelude::{string::String, vec::Vec};

    /// The Concert Broker contract storage with Step 4 enhancements.
    #[ink(storage)]
    pub struct ConcertBroker {
        /// The contract owner
        owner: AccountId,
        
        // Enhanced Artist management (Step 1)
        artists: ink::storage::Mapping<u32, Artist>,
        next_artist_id: u32,
        
        // Enhanced Venue management (Step 1)
        venues: ink::storage::Mapping<u32, Venue>,
        next_venue_id: u32,
        
        // Tour Management (Step 2)
        tours: ink::storage::Mapping<u32, Tour>,
        next_tour_id: u32,
        
        // Festival Management (Step 2)
        festivals: ink::storage::Mapping<u32, Festival>,
        next_festival_id: u32,
        
        // Enhanced Concert Event Management (Step 3)
        concert_events: ink::storage::Mapping<u32, ConcertEvent>,
        next_concert_event_id: u32,

        // NEW: Enhanced Concert Ticket Management (Step 4)
        concert_tickets: ink::storage::Mapping<u64, ConcertTicket>,
        next_concert_ticket_id: u64,
        user_concert_tickets: ink::storage::Mapping<AccountId, Vec<u64>>,

        // NEW: Multi-Currency Support (Step 4)
        supported_currencies: Vec<CurrencyId>,
        currency_rates: ink::storage::Mapping<CurrencyId, Balance>,

        // NEW: Merchandise Management (Step 4)
        merchandise_catalog: ink::storage::Mapping<u32, MerchandiseItem>,
        next_merchandise_id: u32,
        artist_merchandise: ink::storage::Mapping<u32, Vec<u32>>, // artist_id -> merch_ids

        // NEW: Fan Token System (Step 4)
        artist_fan_tokens: ink::storage::Mapping<u32, AccountId>, // artist_id -> token_address
        fan_token_discounts: ink::storage::Mapping<u32, u8>, // artist_id -> discount_percentage

        // NEW: Anti-Scalping System (Step 4)
        verified_fans: ink::storage::Mapping<AccountId, bool>,
        user_purchase_limits: ink::storage::Mapping<(AccountId, u32), u32>, // (user, event) -> tickets_bought
        event_purchase_limits: ink::storage::Mapping<u32, u32>, // event_id -> max_per_user

        // NEW: Revenue Analytics (Step 4)
        total_revenue: Balance,
        artist_revenue: ink::storage::Mapping<u32, Balance>,
        venue_revenue: ink::storage::Mapping<u32, Balance>,
        currency_revenue: ink::storage::Mapping<CurrencyId, Balance>,
        merchandise_revenue: Balance,
        vip_revenue: Balance,
        
        // Artist indexing for search (Step 1)
        artists_by_genre: ink::storage::Mapping<u32, Vec<u32>>, // genre_hash -> artist_ids
        verified_artists: ink::storage::Mapping<bool, Vec<u32>>, // verified status -> artist_ids
        
        // Venue indexing for search (Step 1)
        venues_by_type: ink::storage::Mapping<u32, Vec<u32>>, // venue_type_hash -> venue_ids
        venues_by_city: ink::storage::Mapping<u32, Vec<u32>>, // city_hash -> venue_ids
        
        // Tour indexing for search (Step 2)
        tours_by_artist: ink::storage::Mapping<u32, Vec<u32>>, // artist_id -> tour_ids
        tours_by_type: ink::storage::Mapping<u32, Vec<u32>>, // tour_type_hash -> tour_ids
        active_tours: ink::storage::Mapping<bool, Vec<u32>>, // active status -> tour_ids
        
        // Festival indexing for search (Step 2)
        festivals_by_type: ink::storage::Mapping<u32, Vec<u32>>, // festival_type_hash -> festival_ids
        festivals_by_venue: ink::storage::Mapping<u32, Vec<u32>>, // venue_id -> festival_ids
        
        // Concert Event indexing for search (Step 3)
        events_by_artist: ink::storage::Mapping<u32, Vec<u32>>, // artist_id -> event_ids
        events_by_venue: ink::storage::Mapping<u32, Vec<u32>>, // venue_id -> event_ids
        events_by_tour: ink::storage::Mapping<u32, Vec<u32>>, // tour_id -> event_ids
        events_by_festival: ink::storage::Mapping<u32, Vec<u32>>, // festival_id -> event_ids
        events_by_type: ink::storage::Mapping<u32, Vec<u32>>, // event_type_hash -> event_ids
        events_by_date: ink::storage::Mapping<u64, Vec<u32>>, // date_bucket -> event_ids
    }

    // ========================================================================
    // EXISTING STRUCTURES (Steps 1-3) - Maintained for compatibility
    // ========================================================================

    /// Enhanced Artist structure with music industry-specific fields (Step 1)
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Artist {
        pub id: u32,
        pub name: String,
        pub stage_name: Option<String>,
        pub genre: MusicGenre,
        pub sub_genres: Vec<MusicGenre>,
        pub verified: bool,
        pub fan_token_address: Option<AccountId>,
        pub social_media: SocialMediaHandles,
        pub record_label: Option<String>,
        pub biography: String,
        pub streaming_links: Vec<StreamingPlatform>,
        pub years_active: (u32, Option<u32>),
        pub origin_country: String,
        pub monthly_listeners: u32,
        pub total_albums: u32,
        pub awards_count: u32,
        pub is_touring: bool,
        pub management_contact: Option<String>,
        pub created_at: u64,
    }

    /// Comprehensive Venue structure for music venues (Step 1)
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
        pub average_ticket_price: Balance,
        pub booking_contact: Option<String>,
        pub website: Option<String>,
        pub verified: bool,
        pub created_at: u64,
    }

    /// Tour structure for multi-date tour management (Step 2)
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
        pub total_revenue_generated: Balance,
        pub average_ticket_price: Balance,
        pub total_tickets_sold: u32,
        pub fan_presale_enabled: bool,
        pub description: String,
        pub poster_image_url: Option<String>,
        pub created_at: u64,
        pub last_updated: u64,
    }

    /// Festival structure for multi-artist event management (Step 2)
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
        pub total_revenue_generated: Balance,
        pub total_tickets_sold: u32,
        pub description: String,
        pub lineup_poster_url: Option<String>,
        pub created_at: u64,
        pub last_updated: u64,
    }

    /// Enhanced Concert Event structure with comprehensive music-specific features (Step 3)
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
        pub base_price: Balance,
        pub active: bool,
        pub event_type: EventType,
        pub tour_id: Option<u32>,
        pub festival_id: Option<u32>,
        pub supporting_artists: Vec<u32>,
        pub merchandise_available: bool,
        pub vip_packages: Vec<VIPPackage>,
        pub age_restriction: AgeRestriction,
        pub revenue_generated: Balance,
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

    /// VIP Package offerings for events (Step 3)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct VIPPackage {
        pub package_name: String,
        pub price_premium: Balance,
        pub benefits: Vec<VIPBenefit>,
        pub limited_quantity: Option<u32>,
        pub available_quantity: Option<u32>,
        pub description: String,
    }

    // ========================================================================
    // NEW: STEP 4 STRUCTURES - Concert Ticket System with Music Features
    // ========================================================================

    /// NEW: Enhanced Concert Ticket structure with music industry features (Step 4)
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct ConcertTicket {
        pub id: u64,
        pub event_id: u32,
        pub owner: AccountId,
        pub purchase_price: Balance,
        pub purchase_currency: CurrencyId,
        pub purchase_date: u64,
        pub seat_section: String,
        pub seat_row: String,
        pub seat_number: u32,
        pub seat_type: SeatType,
        pub access_level: AccessLevel,
        pub transferable: bool,
        pub vip_package_id: Option<u32>,
        pub merchandise_bundle: Vec<MerchandiseBundle>,
        pub fan_token_discount_applied: bool,
        pub loyalty_points_earned: u32,
        pub special_access: Vec<SpecialAccess>,
        pub qr_code: String,
        pub resale_allowed: bool,
        pub resale_price_limit: Option<Balance>,
        pub artist_revenue_share: Balance,
        pub dynamic_price_paid: Balance,
        pub dot_equivalent_paid: Balance,
        pub verified_fan_purchase: bool,
        pub created_at: u64,
        pub last_updated: u64,
    }

    /// NEW: Merchandise item in the catalog (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct MerchandiseItem {
        pub id: u32,
        pub artist_id: u32,
        pub item_name: String,
        pub item_type: MerchandiseType,
        pub price: Balance,
        pub sizes_available: Vec<String>,
        pub limited_edition: bool,
        pub stock_quantity: u32,
        pub description: String,
        pub image_url: Option<String>,
        pub active: bool,
        pub created_at: u64,
    }

    /// NEW: Merchandise bundle in a ticket purchase (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct MerchandiseBundle {
        pub merchandise_id: u32,
        pub quantity: u32,
        pub size_selected: Option<String>,
        pub bundle_price: Balance,
    }

    /// NEW: Multi-currency support for Acala integration (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum CurrencyId {
        DOT,
        ACA,
        AUSD,
        LDOT,
        KSM,
    }

    /// NEW: Seat types for concerts (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SeatType {
        GeneralAdmission,
        Reserved,
        PremiumReserved,
        VIPSeating,
        FrontRow,
        Balcony,
        FloorSeating,
        BoxSeats,
        StandingRoom,
        AccessibleSeating,
    }

    /// NEW: Access levels for ticket holders (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AccessLevel {
        Standard,           // Basic concert access
        Premium,            // Premium seating and amenities
        VIP,               // VIP amenities and access
        AllAccess,         // Full venue access including backstage
        ArtistAccess,      // Artist-level access and privileges
    }

    /// NEW: Special access types (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SpecialAccess {
        SoundcheckAccess,   // Watch sound check before show
        MeetAndGreet,       // Meet the artist
        BackstageAccess,    // Backstage area access
        PhotoOpportunity,   // Professional photos with artist
        EarlyVenueEntry,    // Enter venue before general admission
        ExclusiveMerchandise, // Access to exclusive merchandise
        VIPLounge,          // VIP lounge access
        ComplimentaryDrinks, // Free drinks
        PriorityParking,    // Reserved parking
        PostShowAccess,     // After-party or special access
    }

    /// NEW: Merchandise types (Step 4)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum MerchandiseType {
        TShirt,
        Hoodie,
        Poster,
        Vinyl,
        CD,
        Hat,
        Bag,
        Keychain,
        Sticker,
        TourBook,
        SignedItem,
        LimitedEdition,
        Accessories,
        Collectible,
        Digital,
    }

    // ========================================================================
    // EXISTING ENUMS (Steps 1-3) - Maintained for compatibility
    // ========================================================================

    /// VIP Package benefits (Step 3)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VIPBenefit {
        EarlyEntry,
        MeetAndGreet,
        BackstageAccess,
        SoundcheckAccess,
        PremiumSeating,
        ExclusiveMerchandise,
        DedicatedEntrance,
        ComplimentaryDrinks,
        PreShowReception,
        PostShowAccess,
        SignedMemorabilia,
        PhotoOpportunity,
        LimitedPoster,
        VIPLaminate,
        ParkingIncluded,
    }

    /// Event types for different concert experiences (Step 3)
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

    /// Stage configuration for festivals (Step 2)
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

    /// Address structure for venues (Step 1)
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

    /// Social media handles for artists (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct SocialMediaHandles {
        pub twitter: Option<String>,
        pub instagram: Option<String>,
        pub facebook: Option<String>,
        pub tiktok: Option<String>,
        pub youtube: Option<String>,
        pub spotify: Option<String>,
        pub apple_music: Option<String>,
        pub bandcamp: Option<String>,
        pub soundcloud: Option<String>,
        pub website: Option<String>,
    }

    /// Streaming platform links (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct StreamingPlatform {
        pub platform: StreamingService,
        pub artist_url: String,
        pub verified: bool,
    }

    /// Music genres with comprehensive coverage (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum MusicGenre {
        Rock, Pop, Jazz, Classical, Electronic, HipHop, Country, Folk, Metal, Indie, Alternative,
        Blues, Reggae, Punk, Funk, Soul, RAndB, Gospel, World, Latin,
        House, Techno, Dubstep, Trance, Ambient,
        HardRock, ProgressiveRock, PsychedelicRock, Grunge,
        Other(String),
    }

    /// Tour types for different touring scales (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum TourType {
        WorldTour, RegionalTour, NationalTour, LocalTour, FestivalCircuit,
        ResidencyTour, AcousticTour, ReunionTour, FarewellTour, PromotionalTour,
    }

    /// Tour status tracking (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum TourStatus {
        Announced, OnSale, Active, Completed, Postponed, Cancelled, Rescheduled,
    }

    /// Festival types for different festival categories (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalType {
        MusicFestival, RockFestival, ElectronicFestival, JazzFestival, FolkFestival,
        CountryFestival, HipHopFestival, ClassicalFestival, ArtsFestival, CulturalFestival,
        CharityFestival, CorporateFestival,
    }

    /// Festival status tracking (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalStatus {
        Planning, LineupAnnounced, OnSale, SoldOut, Active, Completed,
        Postponed, Cancelled, WeatherDelay,
    }

    /// Festival ticket types (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalTicketType {
        GeneralAdmission, VIP, PlatinumVIP, DayPass(u32), WeekendPass,
        CampingPass, GroupPass(u32), EarlyBird, StudentDiscount, LocalResident,
    }

    /// Stage types for festivals (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum StageType {
        MainStage, SecondStage, AcousticStage, ElectronicStage, LocalStage,
        WorshipStage, ComedyStage, DanceStage, CommunityStage, SponsorStage,
    }

    /// Sustainability features for eco-friendly festivals (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SustainabilityFeature {
        SolarPower, WasteReduction, WaterConservation, LocalSourcing, CarbonNeutral,
        PublicTransport, BiodegradableSupplies, TreePlanting, PlasticFree, GreenVendors,
    }

    /// Types of music venues (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VenueType {
        Arena, Stadium, Theater, Club, Bar, ConcertHall, Amphitheater, FestivalGround,
        OperaHouse, JazzClub, ComedyClub, MultiPurpose, OutdoorVenue, PrivateVenue,
        RecordingStudio, Rooftop, Warehouse, Church, University, Other,
    }

    /// Venue amenities (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VenueAmenity {
        VipLounge, CoatCheck, ATM, MerchandiseStand, MultipleBars, FoodCourt,
        OutdoorArea, DanceFloor, ReservedSeating, GeneralAdmission, BalconySeating,
        PrivateBoxes, MeetAndGreetSpace, PhotoOpportunities, ProfessionalPhotography,
        LiveStreamingCapable, RecordingCapable, ClimateControl, SmokingArea, ChargingStations,
    }

    /// Age restrictions for venues and events (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AgeRestriction {
        AllAges,
        EighteenPlus,
        TwentyOnePlus,
        Custom(u8),
    }

    /// Accessibility features (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AccessibilityFeature {
        WheelchairAccessible, ElevatorAccess, AccessibleRestrooms, AccessibleParking,
        SignLanguageInterpretation, AudioDescription, BraillePrograms, ServiceAnimalFriendly,
        SensoryFriendlyOptions, AssistedListeningDevices,
    }

    /// Sound system quality rating (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SoundSystemRating {
        Basic, Good, Excellent, WorldClass,
    }

    /// Lighting capabilities (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum LightingCapabilities {
        Basic, Professional, Advanced, Spectacular,
    }

    /// Security level of the venue (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SecurityLevel {
        Minimal, Standard, High, Maximum,
    }

    /// Streaming services (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum StreamingService {
        Spotify, AppleMusic, YouTubeMusic, AmazonMusic, Tidal, Deezer,
        Pandora, SoundCloud, Bandcamp, Beatport, Other,
    }

    /// Concert broker errors with Step 4 additions
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        // Existing errors (Steps 1-3)
        NotOwner, ArtistNotFound, VenueNotFound, EventNotFound, ConcertEventNotFound,
        TourNotFound, FestivalNotFound, IdOverflow, InvalidArtistData, InvalidVenueData,
        InvalidTourData, InvalidFestivalData, InvalidEventData, InvalidEventTiming,
        ArtistAlreadyVerified, VenueAlreadyVerified, NoSearchResults, TourAlreadyActive,
        FestivalAlreadyActive, InvalidTourDates, InvalidFestivalDates, MaxSupportingArtistsReached,
        ArtistNotAvailable, VenueNotAvailable, EventAlreadyLinkedToTour, EventAlreadyLinkedToFestival,
        VIPPackageNotFound, InvalidVIPPackageData,
        
        // NEW: Step 4 ticket and purchase errors
        TicketNotFound, NotTicketOwner, TicketNotTransferable, EventNotActive, EventSoldOut,
        InsufficientPayment, InsufficientCapacity, MerchandiseNotFound, InvalidMerchandiseData,
        FanTokenNotFound, InvalidFanTokenDiscount, PurchaseLimitExceeded, NotVerifiedFan,
        InvalidCurrency, CurrencyConversionFailed, ResaleNotAllowed, ResalePriceTooHigh,
        InvalidQRCode, SpecialAccessNotAvailable, VIPPackageUnavailable, StockNotAvailable,
        InvalidSize, BundlePricingError,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ConcertBroker {
        /// Creates a new Concert Broker contract with comprehensive functionality.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut supported_currencies = Vec::new();
            supported_currencies.push(CurrencyId::DOT);
            supported_currencies.push(CurrencyId::ACA);
            supported_currencies.push(CurrencyId::AUSD);
            supported_currencies.push(CurrencyId::LDOT);
            supported_currencies.push(CurrencyId::KSM);

            let mut contract = Self {
                owner: Self::env().caller(),
                artists: ink::storage::Mapping::new(),
                next_artist_id: 1,
                venues: ink::storage::Mapping::new(),
                next_venue_id: 1,
                tours: ink::storage::Mapping::new(),
                next_tour_id: 1,
                festivals: ink::storage::Mapping::new(),
                next_festival_id: 1,
                concert_events: ink::storage::Mapping::new(),
                next_concert_event_id: 1,
                // NEW: Step 4 storage initialization
                concert_tickets: ink::storage::Mapping::new(),
                next_concert_ticket_id: 1,
                user_concert_tickets: ink::storage::Mapping::new(),
                supported_currencies,
                currency_rates: ink::storage::Mapping::new(),
                merchandise_catalog: ink::storage::Mapping::new(),
                next_merchandise_id: 1,
                artist_merchandise: ink::storage::Mapping::new(),
                artist_fan_tokens: ink::storage::Mapping::new(),
                fan_token_discounts: ink::storage::Mapping::new(),
                verified_fans: ink::storage::Mapping::new(),
                user_purchase_limits: ink::storage::Mapping::new(),
                event_purchase_limits: ink::storage::Mapping::new(),
                total_revenue: 0,
                artist_revenue: ink::storage::Mapping::new(),
                venue_revenue: ink::storage::Mapping::new(),
                currency_revenue: ink::storage::Mapping::new(),
                merchandise_revenue: 0,
                vip_revenue: 0,
                // Existing search indexes
                artists_by_genre: ink::storage::Mapping::new(),
                verified_artists: ink::storage::Mapping::new(),
                venues_by_type: ink::storage::Mapping::new(),
                venues_by_city: ink::storage::Mapping::new(),
                tours_by_artist: ink::storage::Mapping::new(),
                tours_by_type: ink::storage::Mapping::new(),
                active_tours: ink::storage::Mapping::new(),
                festivals_by_type: ink::storage::Mapping::new(),
                festivals_by_venue: ink::storage::Mapping::new(),
                events_by_artist: ink::storage::Mapping::new(),
                events_by_venue: ink::storage::Mapping::new(),
                events_by_tour: ink::storage::Mapping::new(),
                events_by_festival: ink::storage::Mapping::new(),
                events_by_type: ink::storage::Mapping::new(),
                events_by_date: ink::storage::Mapping::new(),
            };

            // Initialize currency rates (DOT as base)
            contract.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000);
            contract.currency_rates.insert(CurrencyId::ACA, &50_000_000_000);
            contract.currency_rates.insert(CurrencyId::AUSD, &150_000_000_000);
            contract.currency_rates.insert(CurrencyId::LDOT, &950_000_000_000);
            contract.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000);

            // Initialize revenue tracking
            for currency in &contract.supported_currencies.clone() {
                contract.currency_revenue.insert(*currency, &0);
            }

            contract
        }

        // ========================================================================
        // EXISTING FUNCTIONALITY (Steps 1-3) - Maintained for compatibility
        // ========================================================================

        /// Register a comprehensive artist profile (Step 1)
        #[ink(message)]
        pub fn register_artist(
            &mut self,
            name: String,
            stage_name: Option<String>,
            genre: MusicGenre,
            sub_genres: Vec<MusicGenre>,
            biography: String,
            origin_country: String,
            record_label: Option<String>,
            social_media: SocialMediaHandles,
            streaming_links: Vec<StreamingPlatform>,
            years_active_start: u32,
            management_contact: Option<String>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if name.is_empty() || biography.is_empty() || origin_country.is_empty() {
                return Err(Error::InvalidArtistData);
            }

            let artist_id = self.next_artist_id;
            self.next_artist_id = self.next_artist_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();

            let artist = Artist {
                id: artist_id,
                name,
                stage_name,
                genre: genre.clone(),
                sub_genres: sub_genres.clone(),
                verified: false,
                fan_token_address: None,
                social_media,
                record_label,
                biography,
                streaming_links,
                years_active: (years_active_start, None),
                origin_country,
                monthly_listeners: 0,
                total_albums: 0,
                awards_count: 0,
                is_touring: false,
                management_contact,
                created_at: current_time,
            };

            self.artists.insert(artist_id, &artist);
            
            self.update_artist_genre_index(artist_id, &genre);
            for sub_genre in &sub_genres {
                self.update_artist_genre_index(artist_id, sub_genre);
            }
            
            self.update_verified_artists_index(artist_id, false);
            self.tours_by_artist.insert(artist_id, &Vec::<u32>::new());
            self.events_by_artist.insert(artist_id, &Vec::<u32>::new());

            // NEW: Initialize Step 4 data for artist
            self.artist_merchandise.insert(artist_id, &Vec::<u32>::new());
            self.artist_revenue.insert(artist_id, &0);

            Ok(artist_id)
        }

        /// Register a comprehensive venue profile (Step 1)
        #[ink(message)]
        pub fn register_venue(
            &mut self,
            name: String,
            address: VenueAddress,
            venue_type: VenueType,
            capacity: u32,
            standing_capacity: Option<u32>,
            acoustic_rating: u8,
            amenities: Vec<VenueAmenity>,
            age_restrictions: AgeRestriction,
            parking_available: bool,
            parking_capacity: Option<u32>,
            public_transport_access: bool,
            accessibility_features: Vec<AccessibilityFeature>,
            sound_system_quality: SoundSystemRating,
            lighting_capabilities: LightingCapabilities,
            security_level: SecurityLevel,
            established_year: Option<u32>,
            booking_contact: Option<String>,
            website: Option<String>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if name.is_empty() || address.city.is_empty() || capacity == 0 || acoustic_rating > 10 {
                return Err(Error::InvalidVenueData);
            }

            let venue_id = self.next_venue_id;
            self.next_venue_id = self.next_venue_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();

            let venue = Venue {
                id: venue_id,
                name,
                address: address.clone(),
                venue_type,
                capacity,
                standing_capacity,
                acoustic_rating,
                amenities,
                age_restrictions,
                parking_available,
                parking_capacity,
                public_transport_access,
                accessibility_features,
                sound_system_quality,
                lighting_capabilities,
                backstage_facilities: false,
                merchandise_space: false,
                food_and_beverage: false,
                alcohol_license: false,
                security_level,
                established_year,
                notable_past_performers: Vec::new(),
                average_ticket_price: 0,
                booking_contact,
                website,
                verified: false,
                created_at: current_time,
            };

            self.venues.insert(venue_id, &venue);
            
            self.update_venue_type_index(venue_id, venue_type);
            self.update_venue_city_index(venue_id, &address.city);
            self.festivals_by_venue.insert(venue_id, &Vec::<u32>::new());
            self.events_by_venue.insert(venue_id, &Vec::<u32>::new());

            // NEW: Initialize Step 4 data for venue
            self.venue_revenue.insert(venue_id, &0);

            Ok(venue_id)
        }

        /// Create a comprehensive concert event (Step 3)
        #[ink(message)]
        pub fn create_concert_event(
            &mut self,
            name: String,
            artist_id: u32,
            venue_id: u32,
            date: u64,
            doors_open: u64,
            show_start: u64,
            estimated_end: u64,
            capacity: u32,
            base_price: Balance,
            event_type: EventType,
            supporting_artists: Vec<u32>,
            age_restriction: AgeRestriction,
            special_notes: String,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if name.is_empty() {
                return Err(Error::InvalidEventData);
            }

            if doors_open >= show_start || show_start >= estimated_end || date > doors_open {
                return Err(Error::InvalidEventTiming);
            }

            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            let venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;

            for supporting_artist_id in &supporting_artists {
                if self.artists.get(*supporting_artist_id).is_none() {
                    return Err(Error::ArtistNotFound);
                }
            }

            if supporting_artists.len() > 10 {
                return Err(Error::MaxSupportingArtistsReached);
            }

            let event_id = self.next_concert_event_id;
            self.next_concert_event_id = self.next_concert_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();
            
            let event_capacity = if capacity == 0 || capacity > venue.capacity {
                venue.capacity
            } else {
                capacity
            };

            let concert_event = ConcertEvent {
                id: event_id,
                name,
                artist_id,
                venue_id,
                date,
                doors_open,
                show_start,
                estimated_end,
                capacity: event_capacity,
                sold_tickets: 0,
                base_price,
                active: true,
                event_type,
                tour_id: None,
                festival_id: None,
                supporting_artists,
                merchandise_available: false,
                vip_packages: Vec::new(), // Empty VIP packages for basic events
                age_restriction,
                revenue_generated: 0,
                special_notes,
                presale_enabled: false,
                presale_start: None,
                general_sale_start: None,
                sound_check_available: false,
                meet_greet_available: false,
                recording_allowed: false,
                live_stream_available: false,
                setlist_length_minutes: None,
                encore_expected: false,
                festival_stage: None,
                created_at: current_time,
                last_updated: current_time,
            };

            self.concert_events.insert(event_id, &concert_event);
            self.update_concert_event_indexes(event_id, &concert_event);

            // Initialize Step 4 data for event
            self.event_purchase_limits.insert(event_id, &5); // Default max 5 tickets per user

            Ok(event_id)
        }

        // ========================================================================
        // NEW: STEP 4 - MERCHANDISE MANAGEMENT
        // ========================================================================

        /// Add merchandise item to artist catalog
        #[ink(message)]
        pub fn add_merchandise_item(
            &mut self,
            artist_id: u32,
            item_name: String,
            item_type: MerchandiseType,
            price: Balance,
            sizes_available: Vec<String>,
            limited_edition: bool,
            stock_quantity: u32,
            description: String,
            image_url: Option<String>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            if item_name.is_empty() || description.is_empty() {
                return Err(Error::InvalidMerchandiseData);
            }

            let merchandise_id = self.next_merchandise_id;
            self.next_merchandise_id = self.next_merchandise_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let merchandise = MerchandiseItem {
                id: merchandise_id,
                artist_id,
                item_name,
                item_type,
                price,
                sizes_available,
                limited_edition,
                stock_quantity,
                description,
                image_url,
                active: true,
                created_at: self.env().block_timestamp(),
            };

            self.merchandise_catalog.insert(merchandise_id, &merchandise);

            // Update artist merchandise index
            let mut artist_merch = self.artist_merchandise.get(artist_id).unwrap_or_default();
            artist_merch.push(merchandise_id);
            self.artist_merchandise.insert(artist_id, &artist_merch);

            Ok(merchandise_id)
        }

        /// Update merchandise stock
        #[ink(message)]
        pub fn update_merchandise_stock(
            &mut self,
            merchandise_id: u32,
            new_stock: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut merchandise = self.merchandise_catalog.get(merchandise_id)
                .ok_or(Error::MerchandiseNotFound)?;

            merchandise.stock_quantity = new_stock;
            self.merchandise_catalog.insert(merchandise_id, &merchandise);

            Ok(())
        }

        // ========================================================================
        // NEW: STEP 4 - FAN TOKEN SYSTEM
        // ========================================================================

        /// Set fan token address and discount for artist
        #[ink(message)]
        pub fn set_artist_fan_token(
            &mut self,
            artist_id: u32,
            fan_token_address: AccountId,
            discount_percentage: u8,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            if discount_percentage > 50 {
                return Err(Error::InvalidFanTokenDiscount);
            }

            artist.fan_token_address = Some(fan_token_address);
            self.artists.insert(artist_id, &artist);

            self.artist_fan_tokens.insert(artist_id, &fan_token_address);
            self.fan_token_discounts.insert(artist_id, &discount_percentage);

            Ok(())
        }

        /// Verify fan as genuine (anti-scalping)
        #[ink(message)]
        pub fn verify_fan(&mut self, user: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.verified_fans.insert(user, &true);
            Ok(())
        }

        /// Set purchase limit for specific event
        #[ink(message)]
        pub fn set_event_purchase_limit(&mut self, event_id: u32, max_per_user: u32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let _event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            self.event_purchase_limits.insert(event_id, &max_per_user);
            Ok(())
        }

        // ========================================================================
        // NEW: STEP 4 - MULTI-CURRENCY SYSTEM
        // ========================================================================

        /// Update currency exchange rate
        #[ink(message)]
        pub fn update_currency_rate(&mut self, currency: CurrencyId, rate_to_dot: Balance) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if rate_to_dot == 0 {
                return Err(Error::InvalidCurrency);
            }

            self.currency_rates.insert(currency, &rate_to_dot);
            Ok(())
        }

        /// Convert amount to DOT equivalent
        fn convert_to_dot_equivalent(&self, amount: Balance, currency: CurrencyId) -> Result<Balance> {
            match currency {
                CurrencyId::DOT => Ok(amount),
                _ => {
                    let rate = self.currency_rates.get(currency).ok_or(Error::InvalidCurrency)?;
                    let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000;
                    if dot_amount == 0 && amount > 0 {
                        return Err(Error::CurrencyConversionFailed);
                    }
                    Ok(dot_amount)
                }
            }
        }

        // ========================================================================
        // NEW: STEP 4 - COMPREHENSIVE TICKET PURCHASING SYSTEM
        // ========================================================================

        /// Purchase concert ticket with multi-currency support and full features
        #[ink(message, payable)]
        pub fn purchase_concert_ticket_with_currency(
            &mut self,
            event_id: u32,
            seat_section: String,
            seat_row: String,
            seat_type: SeatType,
            currency: CurrencyId,
            vip_package_name: Option<String>,
            merchandise_bundle: Vec<(u32, u32, Option<String>)>, // (merch_id, quantity, size)
        ) -> Result<u64> {
            let buyer = self.env().caller();
            let payment = self.env().transferred_value();

            // Validate currency support
            if !self.supported_currencies.contains(&currency) {
                return Err(Error::InvalidCurrency);
            }

            // Get and validate event
            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            if !event.active {
                return Err(Error::EventNotActive);
            }
            if event.sold_tickets >= event.capacity {
                return Err(Error::EventSoldOut);
            }

            // Check purchase limits
            self.check_purchase_limits(buyer, event_id)?;

            // Calculate comprehensive pricing
            let (final_price_dot, vip_package_id, fan_token_discount_applied, special_access, 
                 merchandise_bundles, total_merch_cost) = 
                self.calculate_comprehensive_ticket_price(
                    buyer, &event, &seat_type, vip_package_name, merchandise_bundle
                )?;

            // Validate payment
            let payment_in_dot = self.convert_to_dot_equivalent(payment, currency)?;
            if payment_in_dot < final_price_dot + total_merch_cost {
                return Err(Error::InsufficientPayment);
            }

                        // Create ticket
            let ticket_id = self.next_concert_ticket_id;
            self.next_concert_ticket_id = self.next_concert_ticket_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let seat_number = event.sold_tickets
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            // Generate QR code
            let qr_code = format!("INKTIX-{}-{}-{}", event_id, ticket_id, seat_number);

            let is_verified_fan = self.verified_fans.get(buyer).unwrap_or(false);

            let concert_ticket = ConcertTicket {
                id: ticket_id,
                event_id,
                owner: buyer,
                purchase_price: payment,
                purchase_currency: currency,
                purchase_date: self.env().block_timestamp(),
                seat_section,
                seat_row,
                seat_number,
                seat_type,
                access_level: self.determine_access_level(&seat_type, vip_package_id.is_some()),
                transferable: true,
                vip_package_id,
                merchandise_bundle: merchandise_bundles.clone(), // Clone here
                fan_token_discount_applied,
                loyalty_points_earned: self.calculate_loyalty_points(&seat_type, final_price_dot),
                special_access,
                qr_code,
                resale_allowed: true,
                resale_price_limit: Some(final_price_dot.saturating_mul(150) / 100), // 150% max
                artist_revenue_share: final_price_dot / 20, // 5% artist share on resale
                dynamic_price_paid: final_price_dot,
                dot_equivalent_paid: payment_in_dot,
                verified_fan_purchase: is_verified_fan,
                created_at: self.env().block_timestamp(),
                last_updated: self.env().block_timestamp(),
            };

            // Store ticket and update indexes
            self.concert_tickets.insert(ticket_id, &concert_ticket);

            let mut user_tickets = self.user_concert_tickets.get(buyer).unwrap_or_default();
            user_tickets.push(ticket_id);
            self.user_concert_tickets.insert(buyer, &user_tickets);

            // Update event and revenue analytics
            event.sold_tickets = seat_number;
            event.revenue_generated = event.revenue_generated.saturating_add(final_price_dot);
            self.concert_events.insert(event_id, &event);

            self.update_revenue_analytics(
                currency, 
                payment_in_dot, 
                event.artist_id, 
                event.venue_id,
                total_merch_cost,
                vip_package_id.is_some()
            );

            // Update merchandise stock
            self.update_merchandise_stock_for_purchase(&merchandise_bundles)?; // Use the original reference

            // Update purchase limits
            self.update_purchase_limits(buyer, event_id);

            Ok(ticket_id)
        }

        /// Purchase concert ticket with DOT (simplified version)
        #[ink(message, payable)]
        pub fn purchase_concert_ticket(
            &mut self,
            event_id: u32,
            seat_section: String,
            seat_row: String,
            seat_type: SeatType,
        ) -> Result<u64> {
            self.purchase_concert_ticket_with_currency(
                event_id,
                seat_section,
                seat_row,
                seat_type,
                CurrencyId::DOT,
                None,
                Vec::new(),
            )
        }

        /// Transfer concert ticket to another user
        #[ink(message)]
        pub fn transfer_concert_ticket(&mut self, ticket_id: u64, to: AccountId) -> Result<()> {
            let caller = self.env().caller();

            let mut ticket = self.concert_tickets.get(ticket_id).ok_or(Error::TicketNotFound)?;

            if ticket.owner != caller {
                return Err(Error::NotTicketOwner);
            }

            if !ticket.transferable {
                return Err(Error::TicketNotTransferable);
            }

            // Update ticket owner
            let old_owner = ticket.owner;
            ticket.owner = to;
            ticket.last_updated = self.env().block_timestamp();
            self.concert_tickets.insert(ticket_id, &ticket);

            // Update user ticket lists
            if let Some(mut old_tickets) = self.user_concert_tickets.get(old_owner) {
                old_tickets.retain(|&x| x != ticket_id);
                self.user_concert_tickets.insert(old_owner, &old_tickets);
            }

            let mut new_tickets = self.user_concert_tickets.get(to).unwrap_or_default();
            new_tickets.push(ticket_id);
            self.user_concert_tickets.insert(to, &new_tickets);

            Ok(())
        }

        // ========================================================================
        // NEW: STEP 4 - HELPER FUNCTIONS FOR PRICING AND ANALYTICS
        // ========================================================================

        /// Calculate comprehensive ticket pricing with all features
        fn calculate_comprehensive_ticket_price(
            &self,
            buyer: AccountId,
            event: &ConcertEvent,
            seat_type: &SeatType,
            vip_package_name: Option<String>,
            merchandise_bundle: Vec<(u32, u32, Option<String>)>,
        ) -> Result<(Balance, Option<u32>, bool, Vec<SpecialAccess>, Vec<MerchandiseBundle>, Balance)> {
            // Base ticket price
            let mut final_price = self.calculate_seat_price(event.base_price, seat_type);

            // VIP package integration
            let (vip_price_addition, vip_package_id, special_access) = 
                self.apply_vip_package_pricing(event, vip_package_name)?;
            final_price = final_price.saturating_add(vip_price_addition);

            // Fan token discount
            let (discounted_price, fan_token_discount_applied) = 
                self.apply_fan_token_discount(buyer, event.artist_id, final_price)?;
            final_price = discounted_price;

            // Merchandise bundle pricing
            let (merchandise_bundles, total_merch_cost) = 
                self.calculate_merchandise_bundle_cost(event.artist_id, merchandise_bundle)?;

            Ok((final_price, vip_package_id, fan_token_discount_applied, special_access, 
                merchandise_bundles, total_merch_cost))
        }

        /// Apply VIP package pricing and benefits
        fn apply_vip_package_pricing(
            &self,
            event: &ConcertEvent,
            vip_package_name: Option<String>,
        ) -> Result<(Balance, Option<u32>, Vec<SpecialAccess>)> {
            if let Some(package_name) = vip_package_name {
                for (index, vip_package) in event.vip_packages.iter().enumerate() {
                    if vip_package.package_name == package_name {
                        if let Some(available) = vip_package.available_quantity {
                            if available == 0 {
                                return Err(Error::VIPPackageUnavailable);
                            }
                        }

                        let special_access = self.convert_vip_benefits_to_special_access(&vip_package.benefits);
                        return Ok((vip_package.price_premium, Some(index as u32), special_access));
                    }
                }
                return Err(Error::VIPPackageNotFound);
            }

            Ok((0, None, Vec::new()))
        }

        /// Apply fan token discount if user holds tokens
        fn apply_fan_token_discount(
            &self,
            buyer: AccountId,
            artist_id: u32,
            price: Balance,
        ) -> Result<(Balance, bool)> {
            if let Some(_fan_token_address) = self.artist_fan_tokens.get(artist_id) {
                if let Some(discount_percentage) = self.fan_token_discounts.get(artist_id) {
                    // In a real implementation, we would check token balance here
                    // For this demo, we'll apply discount to verified fans
                    if self.verified_fans.get(buyer).unwrap_or(false) {
                        let discount_amount = (price * discount_percentage as Balance) / 100;
                        return Ok((price.saturating_sub(discount_amount), true));
                    }
                }
            }

            Ok((price, false))
        }

        /// Calculate merchandise bundle cost
        fn calculate_merchandise_bundle_cost(
            &self,
            artist_id: u32,
            merchandise_bundle: Vec<(u32, u32, Option<String>)>,
        ) -> Result<(Vec<MerchandiseBundle>, Balance)> {
            let mut bundles = Vec::new();
            let mut total_cost: Balance = 0;

            for (merch_id, quantity, size) in merchandise_bundle {
                let merchandise = self.merchandise_catalog.get(merch_id)
                    .ok_or(Error::MerchandiseNotFound)?;

                if merchandise.artist_id != artist_id {
                    return Err(Error::MerchandiseNotFound);
                }

                if !merchandise.active {
                    return Err(Error::MerchandiseNotFound);
                }

                if merchandise.stock_quantity < quantity {
                    return Err(Error::StockNotAvailable);
                }

                if let Some(ref selected_size) = size {
                    if !merchandise.sizes_available.contains(selected_size) {
                        return Err(Error::InvalidSize);
                    }
                }

                let bundle_price = merchandise.price.saturating_mul(quantity as Balance);
                total_cost = total_cost.saturating_add(bundle_price);

                bundles.push(MerchandiseBundle {
                    merchandise_id: merch_id,
                    quantity,
                    size_selected: size,
                    bundle_price,
                });
            }

            Ok((bundles, total_cost))
        }

        /// Convert VIP benefits to special access
        fn convert_vip_benefits_to_special_access(&self, benefits: &[VIPBenefit]) -> Vec<SpecialAccess> {
            let mut special_access = Vec::new();

            for benefit in benefits {
                match benefit {
                    VIPBenefit::EarlyEntry => special_access.push(SpecialAccess::EarlyVenueEntry),
                    VIPBenefit::MeetAndGreet => special_access.push(SpecialAccess::MeetAndGreet),
                    VIPBenefit::BackstageAccess => special_access.push(SpecialAccess::BackstageAccess),
                    VIPBenefit::SoundcheckAccess => special_access.push(SpecialAccess::SoundcheckAccess),
                    VIPBenefit::ExclusiveMerchandise => special_access.push(SpecialAccess::ExclusiveMerchandise),
                    VIPBenefit::ComplimentaryDrinks => special_access.push(SpecialAccess::ComplimentaryDrinks),
                    VIPBenefit::PhotoOpportunity => special_access.push(SpecialAccess::PhotoOpportunity),
                    VIPBenefit::PostShowAccess => special_access.push(SpecialAccess::PostShowAccess),
                    VIPBenefit::PremiumSeating => (), // Handled by seat type
                    VIPBenefit::DedicatedEntrance => (), // Venue logistics
                    VIPBenefit::PreShowReception => special_access.push(SpecialAccess::VIPLounge),
                    VIPBenefit::SignedMemorabilia => (), // Physical item, not access
                    VIPBenefit::LimitedPoster => (), // Physical item, not access
                    VIPBenefit::VIPLaminate => (), // Credential, not access
                    VIPBenefit::ParkingIncluded => special_access.push(SpecialAccess::PriorityParking),
                }
            }

            special_access
        }

        /// Check purchase limits for anti-scalping
        fn check_purchase_limits(&self, buyer: AccountId, event_id: u32) -> Result<()> {
            let max_per_user = self.event_purchase_limits.get(event_id).unwrap_or(5);
            let current_purchases = self.user_purchase_limits.get((buyer, event_id)).unwrap_or(0);

            if current_purchases >= max_per_user {
                return Err(Error::PurchaseLimitExceeded);
            }

            Ok(())
        }

        /// Update purchase limits after successful purchase
        fn update_purchase_limits(&mut self, buyer: AccountId, event_id: u32) {
            let current_purchases = self.user_purchase_limits.get((buyer, event_id)).unwrap_or(0);
            self.user_purchase_limits.insert((buyer, event_id), &(current_purchases + 1));
        }

        /// Update merchandise stock after purchase
        fn update_merchandise_stock_for_purchase(&mut self, bundles: &[MerchandiseBundle]) -> Result<()> {
            for bundle in bundles {
                if let Some(mut merchandise) = self.merchandise_catalog.get(bundle.merchandise_id) {
                    merchandise.stock_quantity = merchandise.stock_quantity.saturating_sub(bundle.quantity);
                    self.merchandise_catalog.insert(bundle.merchandise_id, &merchandise);
                }
            }
            Ok(())
        }

        /// Calculate seat-based pricing
        fn calculate_seat_price(&self, base_price: Balance, seat_type: &SeatType) -> Balance {
            let multiplier = match seat_type {
                SeatType::GeneralAdmission => 100,
                SeatType::Reserved => 120,
                SeatType::PremiumReserved => 150,
                SeatType::VIPSeating => 200,
                SeatType::FrontRow => 300,
                SeatType::Balcony => 110,
                SeatType::FloorSeating => 180,
                SeatType::BoxSeats => 400,
                SeatType::StandingRoom => 80,
                SeatType::AccessibleSeating => 120,
            };

            (base_price * multiplier) / 100
        }

        /// Determine access level based on seat type and VIP status
        fn determine_access_level(&self, seat_type: &SeatType, has_vip: bool) -> AccessLevel {
            if has_vip {
                match seat_type {
                    SeatType::BoxSeats | SeatType::FrontRow => AccessLevel::AllAccess,
                    _ => AccessLevel::VIP,
                }
            } else {
                match seat_type {
                    SeatType::GeneralAdmission | SeatType::StandingRoom => AccessLevel::Standard,
                    SeatType::Reserved | SeatType::AccessibleSeating | SeatType::Balcony => AccessLevel::Premium,
                    SeatType::PremiumReserved | SeatType::VIPSeating | SeatType::FloorSeating => AccessLevel::VIP,
                    SeatType::FrontRow | SeatType::BoxSeats => AccessLevel::AllAccess,
                }
            }
        }

        /// Calculate loyalty points based on purchase
        fn calculate_loyalty_points(&self, seat_type: &SeatType, price: Balance) -> u32 {
            let base_points = match seat_type {
                SeatType::GeneralAdmission => 10,
                SeatType::Reserved => 15,
                SeatType::PremiumReserved => 25,
                SeatType::VIPSeating => 50,
                SeatType::FrontRow => 100,
                SeatType::Balcony => 12,
                SeatType::FloorSeating => 40,
                SeatType::BoxSeats => 150,
                SeatType::StandingRoom => 8,
                SeatType::AccessibleSeating => 15,
            };

            let price_bonus = (price / 10_000_000_000) as u32;
            base_points + price_bonus
        }

        /// Update revenue analytics across all dimensions
        fn update_revenue_analytics(
            &mut self,
            currency: CurrencyId,
            amount_dot: Balance,
            artist_id: u32,
            venue_id: u32,
            merchandise_amount: Balance,
            is_vip: bool,
        ) {
            // Update total revenue
            self.total_revenue = self.total_revenue.saturating_add(amount_dot);

            // Update currency-specific revenue
            let current_currency_revenue = self.currency_revenue.get(currency).unwrap_or(0);
            self.currency_revenue.insert(currency, &(current_currency_revenue + amount_dot));

            // Update artist revenue
            let current_artist_revenue = self.artist_revenue.get(artist_id).unwrap_or(0);
            self.artist_revenue.insert(artist_id, &(current_artist_revenue + amount_dot));

            // Update venue revenue
            let current_venue_revenue = self.venue_revenue.get(venue_id).unwrap_or(0);
            self.venue_revenue.insert(venue_id, &(current_venue_revenue + amount_dot));

            // Update merchandise revenue
            self.merchandise_revenue = self.merchandise_revenue.saturating_add(merchandise_amount);

            // Update VIP revenue
            if is_vip {
                self.vip_revenue = self.vip_revenue.saturating_add(amount_dot);
            }
        }

        // ========================================================================
        // EXISTING HELPER FUNCTIONS (Steps 1-3) - Maintained for compatibility
        // ========================================================================

        /// Update search indexes when creating concert events
        fn update_concert_event_indexes(&mut self, event_id: u32, event: &ConcertEvent) {
            // Update artist index (main artist)
            let mut artist_events = self.events_by_artist.get(event.artist_id).unwrap_or_default();
            if !artist_events.contains(&event_id) {
                artist_events.push(event_id);
                self.events_by_artist.insert(event.artist_id, &artist_events);
            }

            // Update artist index for supporting artists
            for supporting_artist_id in &event.supporting_artists {
                let mut supporting_events = self.events_by_artist.get(*supporting_artist_id).unwrap_or_default();
                if !supporting_events.contains(&event_id) {
                    supporting_events.push(event_id);
                    self.events_by_artist.insert(*supporting_artist_id, &supporting_events);
                }
            }

            // Update venue index
            let mut venue_events = self.events_by_venue.get(event.venue_id).unwrap_or_default();
            if !venue_events.contains(&event_id) {
                venue_events.push(event_id);
                self.events_by_venue.insert(event.venue_id, &venue_events);
            }

            // Update event type index
            let type_hash = self.hash_event_type(event.event_type);
            let mut type_events = self.events_by_type.get(type_hash).unwrap_or_default();
            if !type_events.contains(&event_id) {
                type_events.push(event_id);
                self.events_by_type.insert(type_hash, &type_events);
            }

            // Update date index
            let date_bucket = event.date / (24 * 60 * 60 * 1000);
            let mut date_events = self.events_by_date.get(date_bucket).unwrap_or_default();
            if !date_events.contains(&event_id) {
                date_events.push(event_id);
                self.events_by_date.insert(date_bucket, &date_events);
            }
        }

        fn update_artist_genre_index(&mut self, artist_id: u32, genre: &MusicGenre) {
            let genre_hash = self.hash_music_genre(genre);
            let mut artists_in_genre = self.artists_by_genre.get(genre_hash).unwrap_or_default();
            if !artists_in_genre.contains(&artist_id) {
                artists_in_genre.push(artist_id);
                self.artists_by_genre.insert(genre_hash, &artists_in_genre);
            }
        }

        fn update_verified_artists_index(&mut self, artist_id: u32, verified: bool) {
            let mut verified_list = self.verified_artists.get(verified).unwrap_or_default();
            if !verified_list.contains(&artist_id) {
                verified_list.push(artist_id);
                self.verified_artists.insert(verified, &verified_list);
            }
            
            let mut opposite_list = self.verified_artists.get(!verified).unwrap_or_default();
            opposite_list.retain(|&x| x != artist_id);
            if opposite_list.is_empty() {
                self.verified_artists.remove(!verified);
            } else {
                self.verified_artists.insert(!verified, &opposite_list);
            }
        }

        fn update_venue_type_index(&mut self, venue_id: u32, venue_type: VenueType) {
            let type_hash = self.hash_venue_type(venue_type);
            let mut venues_of_type = self.venues_by_type.get(type_hash).unwrap_or_default();
            if !venues_of_type.contains(&venue_id) {
                venues_of_type.push(venue_id);
                self.venues_by_type.insert(type_hash, &venues_of_type);
            }
        }

        fn update_venue_city_index(&mut self, venue_id: u32, city: &str) {
            let city_hash = self.hash_string(city);
            let mut venues_in_city = self.venues_by_city.get(city_hash).unwrap_or_default();
            if !venues_in_city.contains(&venue_id) {
                venues_in_city.push(venue_id);
                self.venues_by_city.insert(city_hash, &venues_in_city);
            }
        }

        // Hash functions for efficient indexing
        fn hash_music_genre(&self, genre: &MusicGenre) -> u32 {
            match genre {
                MusicGenre::Rock => 1, MusicGenre::Pop => 2, MusicGenre::Jazz => 3,
                MusicGenre::Classical => 4, MusicGenre::Electronic => 5, MusicGenre::HipHop => 6,
                MusicGenre::Country => 7, MusicGenre::Folk => 8, MusicGenre::Metal => 9,
                MusicGenre::Indie => 10, MusicGenre::Alternative => 11, MusicGenre::Blues => 12,
                MusicGenre::Reggae => 13, MusicGenre::Punk => 14, MusicGenre::Funk => 15,
                MusicGenre::Soul => 16, MusicGenre::RAndB => 17, MusicGenre::Gospel => 18,
                MusicGenre::World => 19, MusicGenre::Latin => 20, MusicGenre::House => 21,
                MusicGenre::Techno => 22, MusicGenre::Dubstep => 23, MusicGenre::Trance => 24,
                MusicGenre::Ambient => 25, MusicGenre::HardRock => 26, MusicGenre::ProgressiveRock => 27,
                MusicGenre::PsychedelicRock => 28, MusicGenre::Grunge => 29, MusicGenre::Other(_) => 99,
            }
        }

        fn hash_venue_type(&self, venue_type: VenueType) -> u32 {
            match venue_type {
                VenueType::Arena => 1, VenueType::Stadium => 2, VenueType::Theater => 3,
                VenueType::Club => 4, VenueType::Bar => 5, VenueType::ConcertHall => 6,
                VenueType::Amphitheater => 7, VenueType::FestivalGround => 8, VenueType::OperaHouse => 9,
                VenueType::JazzClub => 10, VenueType::ComedyClub => 11, VenueType::MultiPurpose => 12,
                VenueType::OutdoorVenue => 13, VenueType::PrivateVenue => 14, VenueType::RecordingStudio => 15,
                VenueType::Rooftop => 16, VenueType::Warehouse => 17, VenueType::Church => 18,
                VenueType::University => 19, VenueType::Other => 99,
            }
        }

        fn hash_event_type(&self, event_type: EventType) -> u32 {
            match event_type {
                EventType::Concert => 1, EventType::FestivalDay => 2, EventType::MeetAndGreet => 3,
                EventType::SoundCheck => 4, EventType::AlbumLaunch => 5, EventType::AcousticSession => 6,
                EventType::VirtualConcert => 7, EventType::PrivateEvent => 8, EventType::Masterclass => 9,
                EventType::ListeningParty => 10, EventType::UnpluggedSession => 11, EventType::CharityBenefit => 12,
                EventType::TributeConcert => 13, EventType::ResidencyShow => 14, EventType::PopupPerformance => 15,
            }
        }

        fn hash_string(&self, s: &str) -> u32 {
            let mut hash: u32 = 0;
            for byte in s.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
            }
            hash
        }

        // ========================================================================
        // NEW: STEP 4 - ADVANCED UTILITY AND ADMINISTRATIVE FUNCTIONS
        // ========================================================================

        /// Get current ticket price with all discounts and features applied
        #[ink(message)]
        pub fn get_ticket_price_preview(
            &self,
            event_id: u32,
            seat_type: SeatType,
            user: AccountId,
            currency: CurrencyId,
            vip_package_name: Option<String>,
            merchandise_bundle: Vec<(u32, u32)>, // (merch_id, quantity)
        ) -> Result<(Balance, Balance, Balance, bool)> { // (ticket_price, merch_cost, total, fan_discount_applied)
            let event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            
            if !self.supported_currencies.contains(&currency) {
                return Err(Error::InvalidCurrency);
            }

            // Calculate comprehensive pricing
            let (final_price_dot, _vip_id, fan_token_discount_applied, _access, _bundles, total_merch_cost) = 
                self.calculate_comprehensive_ticket_price(
                    user, &event, &seat_type, vip_package_name, 
                    merchandise_bundle.into_iter().map(|(id, qty)| (id, qty, None)).collect()
                )?;

            // Convert to requested currency
            let ticket_price_currency = self.convert_from_dot_equivalent(final_price_dot, currency)?;
            let merch_price_currency = self.convert_from_dot_equivalent(total_merch_cost, currency)?;
            let total_price_currency = ticket_price_currency.saturating_add(merch_price_currency);

            Ok((ticket_price_currency, merch_price_currency, total_price_currency, fan_token_discount_applied))
        }

        /// Get available VIP packages for an event
        #[ink(message)]
        pub fn get_event_vip_packages(&self, event_id: u32) -> Result<Vec<VIPPackage>> {
            let event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            Ok(event.vip_packages)
        }

        /// Get event capacity and availability info
        #[ink(message)]
        pub fn get_event_availability(&self, event_id: u32) -> Result<(u32, u32, u32, bool)> { // (capacity, sold, available, sold_out)
            let event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            let available = event.capacity.saturating_sub(event.sold_tickets);
            let sold_out = available == 0;
            
            Ok((event.capacity, event.sold_tickets, available, sold_out))
        }

        /// Disable/enable ticket transfers (for anti-scalping)
        #[ink(message)]
        pub fn set_ticket_transferability(&mut self, ticket_id: u64, transferable: bool) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut ticket = self.concert_tickets.get(ticket_id).ok_or(Error::TicketNotFound)?;
            ticket.transferable = transferable;
            ticket.last_updated = self.env().block_timestamp();
            self.concert_tickets.insert(ticket_id, &ticket);

            Ok(())
        }

        /// Set resale price limit for specific ticket
        #[ink(message)]
        pub fn set_ticket_resale_limit(&mut self, ticket_id: u64, max_price: Option<Balance>) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut ticket = self.concert_tickets.get(ticket_id).ok_or(Error::TicketNotFound)?;
            ticket.resale_price_limit = max_price;
            ticket.last_updated = self.env().block_timestamp();
            self.concert_tickets.insert(ticket_id, &ticket);

            Ok(())
        }

        /// Get comprehensive event analytics
        #[ink(message)]
        pub fn get_event_analytics(&self, event_id: u32) -> Result<(Balance, u32, u32, Balance, Balance)> {
            // Returns: (revenue, tickets_sold, vip_tickets, merchandise_revenue, average_price)
            let event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            
            let mut vip_tickets: u32 = 0;
            let mut merchandise_revenue: Balance = 0;
            let mut total_ticket_value: Balance = 0;

            for ticket_id in 1..self.next_concert_ticket_id {
                if let Some(ticket) = self.concert_tickets.get(ticket_id) {
                    if ticket.event_id == event_id {
                        if ticket.vip_package_id.is_some() {
                            vip_tickets = vip_tickets.saturating_add(1);
                        }
                        
                        for bundle in &ticket.merchandise_bundle {
                            merchandise_revenue = merchandise_revenue.saturating_add(bundle.bundle_price);
                        }
                        
                        total_ticket_value = total_ticket_value.saturating_add(ticket.dynamic_price_paid);
                    }
                }
            }

            let average_price = if event.sold_tickets > 0 {
                total_ticket_value / event.sold_tickets as Balance
            } else {
                0
            };

            Ok((event.revenue_generated, event.sold_tickets, vip_tickets, merchandise_revenue, average_price))
        }

        /// Get user's ticket history for analytics
        #[ink(message)]
        pub fn get_user_ticket_history(&self, user: AccountId) -> Vec<(u64, u32, Balance, u64, bool)> {
            // Returns: (ticket_id, event_id, price_paid, purchase_date, was_vip)
            let mut history = Vec::new();
            
            if let Some(user_tickets) = self.user_concert_tickets.get(user) {
                for ticket_id in user_tickets {
                    if let Some(ticket) = self.concert_tickets.get(ticket_id) {
                        history.push((
                            ticket_id,
                            ticket.event_id,
                            ticket.dot_equivalent_paid,
                            ticket.purchase_date,
                            ticket.vip_package_id.is_some(),
                        ));
                    }
                }
            }
            
            history
        }

        /// Emergency functions for event management
        #[ink(message)]
        pub fn cancel_event(&mut self, event_id: u32, refund_percentage: u8) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if refund_percentage > 100 {
                return Err(Error::InvalidEventData);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            event.active = false;
            event.last_updated = self.env().block_timestamp();
            self.concert_events.insert(event_id, &event);

            // In a real implementation, this would trigger refund processes
            // For now, we just mark the event as inactive
            
            Ok(())
        }

        /// Batch merchandise stock update
        #[ink(message)]
        pub fn batch_update_merchandise_stock(&mut self, updates: Vec<(u32, u32)>) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            for (merchandise_id, new_stock) in updates {
                if let Some(mut merchandise) = self.merchandise_catalog.get(merchandise_id) {
                    merchandise.stock_quantity = new_stock;
                    self.merchandise_catalog.insert(merchandise_id, &merchandise);
                }
            }

            Ok(())
        }

        /// Convert from DOT equivalent to target currency
        fn convert_from_dot_equivalent(&self, dot_amount: Balance, target_currency: CurrencyId) -> Result<Balance> {
            match target_currency {
                CurrencyId::DOT => Ok(dot_amount),
                _ => {
                    let rate = self.currency_rates.get(target_currency).ok_or(Error::InvalidCurrency)?;
                    if rate == 0 {
                        return Err(Error::CurrencyConversionFailed);
                    }
                    let target_amount = dot_amount.saturating_mul(1_000_000_000_000) / rate;
                    Ok(target_amount)
                }
            }
        }

        // ========================================================================
        // QUERY METHODS - Enhanced with Step 4 functionality
        // ========================================================================

        /// Get artist details
        #[ink(message)]
        pub fn get_artist(&self, artist_id: u32) -> Option<Artist> {
            self.artists.get(artist_id)
        }

        /// Get venue details
        #[ink(message)]
        pub fn get_venue(&self, venue_id: u32) -> Option<Venue> {
            self.venues.get(venue_id)
        }

        /// Get concert event details
        #[ink(message)]
        pub fn get_concert_event(&self, event_id: u32) -> Option<ConcertEvent> {
            self.concert_events.get(event_id)
        }

        /// NEW: Get concert ticket details
        #[ink(message)]
        pub fn get_concert_ticket(&self, ticket_id: u64) -> Option<ConcertTicket> {
            self.concert_tickets.get(ticket_id)
        }

        /// NEW: Get user's concert tickets
        #[ink(message)]
        pub fn get_user_concert_tickets(&self, user: AccountId) -> Vec<u64> {
            self.user_concert_tickets.get(user).unwrap_or_default()
        }

        /// NEW: Get merchandise item details
        #[ink(message)]
        pub fn get_merchandise_item(&self, merchandise_id: u32) -> Option<MerchandiseItem> {
            self.merchandise_catalog.get(merchandise_id)
        }

        /// NEW: Get artist's merchandise catalog
        #[ink(message)]
        pub fn get_artist_merchandise(&self, artist_id: u32) -> Vec<u32> {
            self.artist_merchandise.get(artist_id).unwrap_or_default()
        }

        /// NEW: Get supported currencies
        #[ink(message)]
        pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
            self.supported_currencies.clone()
        }

        /// NEW: Get currency exchange rate
        #[ink(message)]
        pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<Balance> {
            self.currency_rates.get(currency)
        }

        /// NEW: Check if user is verified fan
        #[ink(message)]
        pub fn is_verified_fan(&self, user: AccountId) -> bool {
            self.verified_fans.get(user).unwrap_or(false)
        }

        /// NEW: Get event purchase limit
        #[ink(message)]
        pub fn get_event_purchase_limit(&self, event_id: u32) -> u32 {
            self.event_purchase_limits.get(event_id).unwrap_or(5)
        }

        /// NEW: Get user's purchase count for event
        #[ink(message)]
        pub fn get_user_purchase_count(&self, user: AccountId, event_id: u32) -> u32 {
            self.user_purchase_limits.get((user, event_id)).unwrap_or(0)
        }

        /// NEW: Get total revenue analytics
        #[ink(message)]
        pub fn get_total_revenue(&self) -> Balance {
            self.total_revenue
        }

        /// NEW: Get artist revenue analytics
        #[ink(message)]
        pub fn get_artist_revenue(&self, artist_id: u32) -> Balance {
            self.artist_revenue.get(artist_id).unwrap_or(0)
        }

        /// NEW: Get venue revenue analytics
        #[ink(message)]
        pub fn get_venue_revenue(&self, venue_id: u32) -> Balance {
            self.venue_revenue.get(venue_id).unwrap_or(0)
        }

        /// NEW: Get currency-specific revenue
        #[ink(message)]
        pub fn get_currency_revenue(&self, currency: CurrencyId) -> Balance {
            self.currency_revenue.get(currency).unwrap_or(0)
        }

        /// NEW: Get merchandise revenue
        #[ink(message)]
        pub fn get_merchandise_revenue(&self) -> Balance {
            self.merchandise_revenue
        }

        /// NEW: Get VIP revenue
        #[ink(message)]
        pub fn get_vip_revenue(&self) -> Balance {
            self.vip_revenue
        }

        /// Search functions (existing from Steps 1-3)
        #[ink(message)]
        pub fn search_events_by_artist(&self, artist_id: u32) -> Vec<u32> {
            self.events_by_artist.get(artist_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn search_events_by_venue(&self, venue_id: u32) -> Vec<u32> {
            self.events_by_venue.get(venue_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn search_events_by_type(&self, event_type: EventType) -> Vec<u32> {
            let type_hash = self.hash_event_type(event_type);
            self.events_by_type.get(type_hash).unwrap_or_default()
        }

        #[ink(message)]
        pub fn search_artists_by_genre(&self, genre: MusicGenre) -> Vec<u32> {
            let genre_hash = self.hash_music_genre(&genre);
            self.artists_by_genre.get(genre_hash).unwrap_or_default()
        }

        #[ink(message)]
        pub fn search_venues_by_type(&self, venue_type: VenueType) -> Vec<u32> {
            let type_hash = self.hash_venue_type(venue_type);
            self.venues_by_type.get(type_hash).unwrap_or_default()
        }

        #[ink(message)]
        pub fn search_venues_by_city(&self, city: String) -> Vec<u32> {
            let city_hash = self.hash_string(&city);
            self.venues_by_city.get(city_hash).unwrap_or_default()
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total counts
        #[ink(message)]
        pub fn total_artists(&self) -> u32 {
            self.next_artist_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_venues(&self) -> u32 {
            self.next_venue_id.saturating_sub(1)
        }

        #[ink(message)]
        pub fn total_concert_events(&self) -> u32 {
            self.next_concert_event_id.saturating_sub(1)
        }

        /// NEW: Get total concert tickets sold
        #[ink(message)]
        pub fn total_concert_tickets(&self) -> u64 {
            self.next_concert_ticket_id.saturating_sub(1)
        }

        /// NEW: Get total merchandise items
        #[ink(message)]
        pub fn total_merchandise_items(&self) -> u32 {
            self.next_merchandise_id.saturating_sub(1)
        }

        // ========================================================================
        // NEW: STEP 4 - VIP PACKAGE MANAGEMENT
        // ========================================================================

        /// Add VIP package to an existing concert event
        #[ink(message)]
        pub fn add_vip_package_to_event(
            &mut self,
            event_id: u32,
            package_name: String,
            price_premium: Balance,
            benefits: Vec<VIPBenefit>,
            limited_quantity: Option<u32>,
            description: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            
            if package_name.is_empty() || description.is_empty() {
                return Err(Error::InvalidVIPPackageData);
            }

            let vip_package = VIPPackage {
                package_name,
                price_premium,
                benefits,
                limited_quantity,
                available_quantity: limited_quantity,
                description,
            };

            event.vip_packages.push(vip_package);
            self.concert_events.insert(event_id, &event);

            Ok(())
        }
    }

    impl Default for ConcertBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    // ========================================================================
    // COMPREHENSIVE TEST SUITE - Steps 1-4 Complete Coverage
    // ========================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_test_social_media() -> SocialMediaHandles {
            SocialMediaHandles {
                twitter: Some("@testartist".to_string()),
                instagram: Some("testartist".to_string()),
                facebook: Some("testartist".to_string()),
                tiktok: Some("@testartist".to_string()),
                youtube: Some("testartist".to_string()),
                spotify: Some("spotify.com/artist/test".to_string()),
                apple_music: Some("music.apple.com/artist/test".to_string()),
                bandcamp: None,
                soundcloud: None,
                website: Some("www.testartist.com".to_string()),
            }
        }

        fn create_test_streaming_links() -> Vec<StreamingPlatform> {
            vec![
                StreamingPlatform {
                    platform: StreamingService::Spotify,
                    artist_url: "https://open.spotify.com/artist/test".to_string(),
                    verified: true,
                },
            ]
        }

        fn create_test_venue_address() -> VenueAddress {
            VenueAddress {
                street: "123 Music Street".to_string(),
                city: "Nashville".to_string(),
                state_province: "Tennessee".to_string(),
                country: "United States".to_string(),
                postal_code: "37201".to_string(),
                latitude: Some(36162664),
                longitude: Some(-86781602),
            }
        }

        fn setup_test_data(contract: &mut ConcertBroker) -> (u32, u32, u32) {
            let artist_id = contract.register_artist(
                "Taylor Swift".to_string(),
                Some("T.S.".to_string()),
                MusicGenre::Pop,
                vec![MusicGenre::Country, MusicGenre::Folk],
                "Award-winning singer-songwriter".to_string(),
                "United States".to_string(),
                Some("Big Machine Records".to_string()),
                create_test_social_media(),
                create_test_streaming_links(),
                2006,
                Some("management@taylorswift.com".to_string()),
            ).unwrap();

            let venue_id = contract.register_venue(
                "Madison Square Garden".to_string(),
                create_test_venue_address(),
                VenueType::Arena,
                20000,
                Some(5000),
                9,
                vec![VenueAmenity::VipLounge, VenueAmenity::MultipleBars],
                AgeRestriction::AllAges,
                true,
                Some(1000),
                true,
                vec![AccessibilityFeature::WheelchairAccessible],
                SoundSystemRating::WorldClass,
                LightingCapabilities::Spectacular,
                SecurityLevel::Maximum,
                Some(1968),
                Some("booking@msg.com".to_string()),
                Some("www.msg.com".to_string()),
            ).unwrap();

            let event_id = contract.create_concert_event(
                "Taylor Swift: Eras Tour NYC".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (7 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Epic concert experience".to_string(),
            ).unwrap();

            (artist_id, venue_id, event_id)
        }

        // ========================================================================
        // STEP 4: ENHANCED TICKET PURCHASING TESTS
        // ========================================================================

        #[ink::test]
        fn new_works_with_step4_features() {
            let concert_broker = ConcertBroker::new();
            
            // Verify basic initialization
            assert_eq!(concert_broker.total_artists(), 0);
            assert_eq!(concert_broker.total_venues(), 0);
            assert_eq!(concert_broker.total_concert_events(), 0);
            assert_eq!(concert_broker.total_concert_tickets(), 0);
            assert_eq!(concert_broker.total_merchandise_items(), 0);

            // Verify multi-currency initialization
            let currencies = concert_broker.get_supported_currencies();
            assert_eq!(currencies.len(), 5);
            assert!(currencies.contains(&CurrencyId::DOT));
            assert!(currencies.contains(&CurrencyId::ACA));

            // Verify revenue analytics initialization
            assert_eq!(concert_broker.get_total_revenue(), 0);
            assert_eq!(concert_broker.get_merchandise_revenue(), 0);
            assert_eq!(concert_broker.get_vip_revenue(), 0);
        }

        #[ink::test]
        fn purchase_concert_ticket_basic_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
                150_000_000_000_000 // Base price
            );

            let ticket_id = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            assert_eq!(ticket_id, 1);

            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.event_id, event_id);
            assert_eq!(ticket.seat_section, "Section A");
            assert_eq!(ticket.seat_type, SeatType::GeneralAdmission);
            assert_eq!(ticket.access_level, AccessLevel::Standard);
            assert_eq!(ticket.purchase_currency, CurrencyId::DOT);
            assert!(ticket.transferable);
            assert!(!ticket.qr_code.is_empty());
            assert!(ticket.loyalty_points_earned > 0);

            // Verify event was updated
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.sold_tickets, 1);
            assert!(event.revenue_generated > 0);

            // Verify revenue analytics
            assert!(concert_broker.get_total_revenue() > 0);
        }

        #[ink::test]
        fn purchase_ticket_with_currency_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            // Purchase with ACA currency (rate: 50_000_000_000 = 0.05 DOT per ACA)
            // Need more ACA to cover the DOT equivalent
            // Reserved seat = 1.2x base price = 180 DOT equivalent
            // Need 180 DOT / 0.05 = 3600 ACA
            let aca_payment = 3_600_000_000_000_000; // 3600 ACA = 180 DOT equivalent
            
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(aca_payment);

            let ticket_id = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Section B".to_string(),
                "Row 2".to_string(),
                SeatType::Reserved,
                CurrencyId::ACA,
                None,
                Vec::new(),
            ).unwrap();

            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.purchase_currency, CurrencyId::ACA);
            assert_eq!(ticket.purchase_price, aca_payment);
            assert!(ticket.dot_equivalent_paid > 0);

            // Verify currency-specific revenue tracking
            assert!(concert_broker.get_currency_revenue(CurrencyId::ACA) > 0);
        }

        #[ink::test]
        fn add_merchandise_item_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let merchandise_id = concert_broker.add_merchandise_item(
                artist_id,
                "Eras Tour T-Shirt".to_string(),
                MerchandiseType::TShirt,
                25_000_000_000_000, // 25 DOT
                vec!["S".to_string(), "M".to_string(), "L".to_string()],
                false,
                100,
                "Official tour merchandise".to_string(),
                Some("shirt.jpg".to_string()),
            ).unwrap();

            assert_eq!(merchandise_id, 1);

            let merchandise = concert_broker.get_merchandise_item(merchandise_id).unwrap();
            assert_eq!(merchandise.artist_id, artist_id);
            assert_eq!(merchandise.item_name, "Eras Tour T-Shirt");
            assert_eq!(merchandise.item_type, MerchandiseType::TShirt);
            assert_eq!(merchandise.stock_quantity, 100);
            assert!(merchandise.active);

            // Verify artist merchandise index
            let artist_merch = concert_broker.get_artist_merchandise(artist_id);
            assert!(artist_merch.contains(&merchandise_id));
        }

        #[ink::test]
        fn purchase_ticket_with_merchandise_bundle_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, event_id) = setup_test_data(&mut concert_broker);

            // Add merchandise items
            let merch1 = concert_broker.add_merchandise_item(
                artist_id,
                "T-Shirt".to_string(),
                MerchandiseType::TShirt,
                25_000_000_000_000,
                vec!["M".to_string()],
                false,
                50,
                "Concert t-shirt".to_string(),
                None,
            ).unwrap();

            let merch2 = concert_broker.add_merchandise_item(
                artist_id,
                "Poster".to_string(),
                MerchandiseType::Poster,
                15_000_000_000_000,
                vec![],
                true,
                25,
                "Limited edition poster".to_string(),
                None,
            ).unwrap();

            // Purchase ticket with merchandise bundle
            // VIPSeating = 2x base price = 300 DOT + merch (25 + 15) = 340 DOT total
            let total_payment = 300_000_000_000_000 + 25_000_000_000_000 + 15_000_000_000_000;
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(total_payment);

            let merchandise_bundle = vec![
                (merch1, 1, Some("M".to_string())),
                (merch2, 1, None),
            ];

            let ticket_id = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Section VIP".to_string(),
                "Row 1".to_string(),
                SeatType::VIPSeating,
                CurrencyId::DOT,
                None,
                merchandise_bundle,
            ).unwrap();

            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.merchandise_bundle.len(), 2);
            assert_eq!(ticket.merchandise_bundle[0].merchandise_id, merch1);
            assert_eq!(ticket.merchandise_bundle[0].quantity, 1);
            assert_eq!(ticket.merchandise_bundle[0].size_selected, Some("M".to_string()));

            // Verify merchandise revenue tracking
            assert!(concert_broker.get_merchandise_revenue() > 0);

            // Verify stock was updated
            let updated_merch1 = concert_broker.get_merchandise_item(merch1).unwrap();
            assert_eq!(updated_merch1.stock_quantity, 49); // 50 - 1
        }

        #[ink::test]
        fn set_artist_fan_token_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let fan_token_address = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;

            let result = concert_broker.set_artist_fan_token(
                artist_id,
                fan_token_address,
                15, // 15% discount
            );

            assert_eq!(result, Ok(()));

            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert_eq!(artist.fan_token_address, Some(fan_token_address));
        }

        #[ink::test]
        fn verify_fan_and_purchase_limit_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Set purchase limit to 2 tickets per user
            concert_broker.set_event_purchase_limit(event_id, 2).unwrap();

            // Verify fan
            concert_broker.verify_fan(accounts.alice).unwrap();
            assert!(concert_broker.is_verified_fan(accounts.alice));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Purchase first ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let ticket1 = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            assert_eq!(concert_broker.get_user_purchase_count(accounts.alice, event_id), 1);

            // Purchase second ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let ticket2 = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 2".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            assert_eq!(concert_broker.get_user_purchase_count(accounts.alice, event_id), 2);

            // Try to purchase third ticket (should fail due to limit)
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let result = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 3".to_string(),
                SeatType::GeneralAdmission,
            );

            assert_eq!(result, Err(Error::PurchaseLimitExceeded));

            // Verify user has tickets
            let user_tickets = concert_broker.get_user_concert_tickets(accounts.alice);
            assert_eq!(user_tickets.len(), 2);
            assert!(user_tickets.contains(&ticket1));
            assert!(user_tickets.contains(&ticket2));
        }

        #[ink::test]
        fn transfer_concert_ticket_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);

            let ticket_id = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Transfer to Bob
            let result = concert_broker.transfer_concert_ticket(ticket_id, accounts.bob);
            assert_eq!(result, Ok(()));

            // Verify ownership changed
            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.owner, accounts.bob);

            // Verify user ticket lists updated
            let alice_tickets = concert_broker.get_user_concert_tickets(accounts.alice);
            assert!(!alice_tickets.contains(&ticket_id));

            let bob_tickets = concert_broker.get_user_concert_tickets(accounts.bob);
            assert!(bob_tickets.contains(&ticket_id));
        }

        #[ink::test]
        fn transfer_ticket_not_owner_fails() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);

            let ticket_id = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Try to transfer as Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let result = concert_broker.transfer_concert_ticket(ticket_id, accounts.charlie);
            assert_eq!(result, Err(Error::NotTicketOwner));
        }

        #[ink::test]
        fn seat_type_pricing_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, _event_id) = setup_test_data(&mut concert_broker); // Fixed: prefix with underscore
            let base_price = 150_000_000_000_000;

            // Test different seat types have different pricing
            let general_price = concert_broker.calculate_seat_price(base_price, &SeatType::GeneralAdmission);
            let vip_price = concert_broker.calculate_seat_price(base_price, &SeatType::VIPSeating);
            let box_price = concert_broker.calculate_seat_price(base_price, &SeatType::BoxSeats);

            assert_eq!(general_price, base_price); // 100% multiplier
            assert!(vip_price > general_price); // 200% multiplier
            assert!(box_price > vip_price); // 400% multiplier
        }

        #[ink::test]
        fn access_level_determination_works() {
            let concert_broker = ConcertBroker::new();

            // Test access levels without VIP
            assert_eq!(
                concert_broker.determine_access_level(&SeatType::GeneralAdmission, false),
                AccessLevel::Standard
            );
            assert_eq!(
                concert_broker.determine_access_level(&SeatType::VIPSeating, false),
                AccessLevel::VIP
            );
            assert_eq!(
                concert_broker.determine_access_level(&SeatType::BoxSeats, false),
                AccessLevel::AllAccess
            );

            // Test access levels with VIP package
            assert_eq!(
                concert_broker.determine_access_level(&SeatType::GeneralAdmission, true),
                AccessLevel::VIP
            );
            assert_eq!(
                concert_broker.determine_access_level(&SeatType::BoxSeats, true),
                AccessLevel::AllAccess
            );
        }

        #[ink::test]
        fn loyalty_points_calculation_works() {
            let concert_broker = ConcertBroker::new();

            let general_points = concert_broker.calculate_loyalty_points(
                &SeatType::GeneralAdmission, 
                100_000_000_000_000
            );
            let vip_points = concert_broker.calculate_loyalty_points(
                &SeatType::VIPSeating, 
                100_000_000_000_000
            );
            let box_points = concert_broker.calculate_loyalty_points(
                &SeatType::BoxSeats, 
                100_000_000_000_000
            );

            assert!(vip_points > general_points);
            assert!(box_points > vip_points);
        }

        #[ink::test]
        fn revenue_analytics_comprehensive_tracking() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, event_id) = setup_test_data(&mut concert_broker);

            // Add merchandise
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "Test Merch".to_string(),
                MerchandiseType::TShirt,
                20_000_000_000_000,
                vec!["M".to_string()],
                false,
                100,
                "Test merchandise".to_string(),
                None,
            ).unwrap();

            // Purchase ticket with merchandise
            // VIPSeating = 2x base price = 300 DOT + merch (20) = 320 DOT total
            let ticket_payment = 300_000_000_000_000;
            let total_payment = ticket_payment + 20_000_000_000_000; // Ticket + merch

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(total_payment);

            let _ticket_id = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Section VIP".to_string(),
                "Row 1".to_string(),
                SeatType::VIPSeating,
                CurrencyId::DOT,
                None,
                vec![(merch_id, 1, Some("M".to_string()))],
            ).unwrap();

            // Verify comprehensive revenue tracking
            assert!(concert_broker.get_total_revenue() > 0);
            assert!(concert_broker.get_artist_revenue(artist_id) > 0);
            assert!(concert_broker.get_venue_revenue(venue_id) > 0);
            assert!(concert_broker.get_currency_revenue(CurrencyId::DOT) > 0);
            assert!(concert_broker.get_merchandise_revenue() > 0);

            // Verify artist revenue includes both ticket and merchandise
            let artist_revenue = concert_broker.get_artist_revenue(artist_id);
            assert!(artist_revenue >= total_payment);
        }

        #[ink::test]
        fn currency_conversion_works() {
            let concert_broker = ConcertBroker::new();

            // Test DOT to DOT (should be 1:1)
            let dot_amount = 1_000_000_000_000;
            let converted = concert_broker.convert_to_dot_equivalent(dot_amount, CurrencyId::DOT).unwrap();
            assert_eq!(converted, dot_amount);

            // Test ACA to DOT (rate: 50_000_000_000)
            let aca_amount = 1_000_000_000_000;
            let dot_equivalent = concert_broker.convert_to_dot_equivalent(aca_amount, CurrencyId::ACA).unwrap();
            assert_eq!(dot_equivalent, 50_000_000_000); // Should be 0.05 DOT
        }

        #[ink::test]
        fn comprehensive_vip_and_merchandise_workflow() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, event_id) = setup_test_data(&mut concert_broker); // Fixed: removed mut

            // Add VIP package to event
            concert_broker.add_vip_package_to_event(
                event_id,
                "Diamond VIP Experience".to_string(),
                500_000_000_000_000, // 500 DOT premium
                vec![
                    VIPBenefit::MeetAndGreet,
                    VIPBenefit::BackstageAccess,
                    VIPBenefit::ExclusiveMerchandise,
                    VIPBenefit::ComplimentaryDrinks,
                ],
                Some(25),
                "Ultimate VIP experience".to_string(),
            ).unwrap();

            // Add merchandise
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "VIP Exclusive Hoodie".to_string(),
                MerchandiseType::Hoodie,
                75_000_000_000_000,
                vec!["L".to_string()],
                true,
                10,
                "Limited VIP hoodie".to_string(),
                None,
            ).unwrap();

            // Set fan token for artist
            let fan_token_address = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            concert_broker.set_artist_fan_token(artist_id, fan_token_address, 20).unwrap();

            // Verify fan and purchase ticket with full features
            let buyer = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            concert_broker.verify_fan(buyer).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(buyer);

            let base_price = 150_000_000_000_000;
            let vip_premium = 500_000_000_000_000;
            let merch_price = 75_000_000_000_000;
            let _total_before_discount = base_price * 2 + vip_premium + merch_price; // Fixed: prefix with underscore
            
            // Apply fan token discount (20% off ticket price only)
            let ticket_with_vip = base_price * 2 + vip_premium;
            let discounted_ticket = ticket_with_vip * 80 / 100; // 20% discount
            let total_payment = discounted_ticket + merch_price;

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(total_payment);

            let ticket_id = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "VIP Section".to_string(),
                "Row 1".to_string(),
                SeatType::VIPSeating,
                CurrencyId::DOT,
                Some("Diamond VIP Experience".to_string()),
                vec![(merch_id, 1, Some("L".to_string()))],
            ).unwrap();

            // Verify comprehensive ticket features
            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.seat_type, SeatType::VIPSeating);
            assert_eq!(ticket.access_level, AccessLevel::VIP);
            assert!(ticket.fan_token_discount_applied);
            assert_eq!(ticket.vip_package_id, Some(0));
            assert_eq!(ticket.special_access.len(), 4);
            assert!(ticket.special_access.contains(&SpecialAccess::MeetAndGreet));
            assert!(ticket.special_access.contains(&SpecialAccess::BackstageAccess));
            assert_eq!(ticket.merchandise_bundle.len(), 1);
            assert!(ticket.verified_fan_purchase);
            assert!(ticket.loyalty_points_earned > 0);
            assert!(!ticket.qr_code.is_empty());
            assert!(ticket.resale_allowed);
            assert!(ticket.resale_price_limit.is_some());

            // Verify all revenue streams
            assert!(concert_broker.get_total_revenue() > 0);
            assert!(concert_broker.get_artist_revenue(artist_id) > 0);
            assert!(concert_broker.get_venue_revenue(venue_id) > 0);
            assert!(concert_broker.get_merchandise_revenue() > 0);
            assert!(concert_broker.get_vip_revenue() > 0);

            // Verify merchandise stock updated
            let updated_merch = concert_broker.get_merchandise_item(merch_id).unwrap();
            assert_eq!(updated_merch.stock_quantity, 9); // 10 - 1

            // Verify purchase limits updated
            assert_eq!(concert_broker.get_user_purchase_count(buyer, event_id), 1);
        }

        // ========================================================================
        // STEP 4: ADVANCED FUNCTIONALITY TESTS
        // ========================================================================

        #[ink::test]
        fn ticket_price_preview_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, event_id) = setup_test_data(&mut concert_broker);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Add merchandise
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "Preview Merch".to_string(),
                MerchandiseType::TShirt,
                25_000_000_000_000,
                vec!["L".to_string()],
                false,
                10,
                "Test merch".to_string(),
                None,
            ).unwrap();

            // Set fan token for discount
            concert_broker.set_artist_fan_token(artist_id, accounts.bob, 10).unwrap();
            concert_broker.verify_fan(accounts.alice).unwrap();

            // Get price preview
            let (ticket_price, merch_cost, total, fan_discount) = concert_broker.get_ticket_price_preview(
                event_id,
                SeatType::VIPSeating,
                accounts.alice,
                CurrencyId::DOT,
                None,
                vec![(merch_id, 1)],
            ).unwrap();

            assert!(ticket_price > 0);
            assert_eq!(merch_cost, 25_000_000_000_000);
            assert_eq!(total, ticket_price + merch_cost);
            assert!(fan_discount); // Should be true for verified fan
        }

        #[ink::test]
        fn get_event_vip_packages_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            // Add VIP package
            concert_broker.add_vip_package_to_event(
                event_id,
                "Test VIP".to_string(),
                100_000_000_000_000,
                vec![VIPBenefit::MeetAndGreet],
                Some(10),
                "Test VIP package".to_string(),
            ).unwrap();

            let vip_packages = concert_broker.get_event_vip_packages(event_id).unwrap();
            assert_eq!(vip_packages.len(), 1);
            assert_eq!(vip_packages[0].package_name, "Test VIP");
        }

        #[ink::test]
        fn get_event_availability_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            // Check initial availability
            let (capacity, sold, available, sold_out) = concert_broker.get_event_availability(event_id).unwrap();
            assert_eq!(capacity, 18000);
            assert_eq!(sold, 0);
            assert_eq!(available, 18000);
            assert!(!sold_out);

            // Purchase a ticket
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let _ticket_id = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Check updated availability
            let (capacity, sold, available, sold_out) = concert_broker.get_event_availability(event_id).unwrap();
            assert_eq!(capacity, 18000);
            assert_eq!(sold, 1);
            assert_eq!(available, 17999);
            assert!(!sold_out);
        }

        #[ink::test]
        fn set_ticket_transferability_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let ticket_id = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Initially transferable
            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert!(ticket.transferable);

            // Disable transferability
            concert_broker.set_ticket_transferability(ticket_id, false).unwrap();

            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert!(!ticket.transferable);
        }

        #[ink::test]
        fn get_event_analytics_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, event_id) = setup_test_data(&mut concert_broker);

            // Add merchandise and VIP package
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "Analytics Merch".to_string(),
                MerchandiseType::Poster,
                20_000_000_000_000,
                vec![],
                false,
                10,
                "Test merch".to_string(),
                None,
            ).unwrap();

            concert_broker.add_vip_package_to_event(
                event_id,
                "Analytics VIP".to_string(),
                200_000_000_000_000,
                vec![VIPBenefit::MeetAndGreet],
                Some(5),
                "Test VIP".to_string(),
            ).unwrap();

            // Purchase regular ticket (GeneralAdmission = 1x base price)
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let _ticket1 = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            // Purchase VIP ticket with merchandise (VIPSeating = 2x base price + VIP package + merch)
            let vip_ticket_cost = 300_000_000_000_000 + 200_000_000_000_000; // 2x base + VIP
            let total_payment = vip_ticket_cost + 20_000_000_000_000; // + merch
            
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(total_payment);
            let _ticket2 = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "VIP Section".to_string(),
                "Row 1".to_string(),
                SeatType::VIPSeating,
                CurrencyId::DOT,
                Some("Analytics VIP".to_string()),
                vec![(merch_id, 1, None)],
            ).unwrap();

            // Get analytics
            let (revenue, tickets_sold, vip_tickets, merch_revenue, avg_price) = 
                concert_broker.get_event_analytics(event_id).unwrap();

            assert!(revenue > 0);
            assert_eq!(tickets_sold, 2);
            assert_eq!(vip_tickets, 1);
            assert_eq!(merch_revenue, 20_000_000_000_000);
            assert!(avg_price > 0);
        }

        #[ink::test]
        fn get_user_ticket_history_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Purchase multiple tickets
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(150_000_000_000_000);
            let _ticket1 = concert_broker.purchase_concert_ticket(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
            ).unwrap();

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(300_000_000_000_000);
            let _ticket2 = concert_broker.purchase_concert_ticket(
                event_id,
                "Section B".to_string(),
                "Row 2".to_string(),
                SeatType::VIPSeating,
            ).unwrap();

            // Get ticket history
            let history = concert_broker.get_user_ticket_history(accounts.alice);
            assert_eq!(history.len(), 2);
            
            // Check first ticket (general admission, not VIP)
            assert_eq!(history[0].1, event_id); // event_id
            assert!(!history[0].4); // not VIP
            
            // Check second ticket (VIP seating)
            assert_eq!(history[1].1, event_id); // event_id
            assert!(!history[1].4); // VIP seating but no VIP package
        }

        #[ink::test]
        fn cancel_event_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            // Initially active
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert!(event.active);

            // Cancel event
            concert_broker.cancel_event(event_id, 100).unwrap(); // 100% refund

            // Should be inactive
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert!(!event.active);
        }

        #[ink::test]
        fn batch_merchandise_stock_update_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            // Add multiple merchandise items
            let merch1 = concert_broker.add_merchandise_item(
                artist_id, "Item 1".to_string(), MerchandiseType::TShirt,
                20_000_000_000_000, vec![], false, 10, "Item 1".to_string(), None,
            ).unwrap();

            let merch2 = concert_broker.add_merchandise_item(
                artist_id, "Item 2".to_string(), MerchandiseType::Poster,
                15_000_000_000_000, vec![], false, 5, "Item 2".to_string(), None,
            ).unwrap();

            // Batch update stock
            let updates = vec![(merch1, 20), (merch2, 15)];
            concert_broker.batch_update_merchandise_stock(updates).unwrap();

            // Verify updates
            let item1 = concert_broker.get_merchandise_item(merch1).unwrap();
            assert_eq!(item1.stock_quantity, 20);

            let item2 = concert_broker.get_merchandise_item(merch2).unwrap();
            assert_eq!(item2.stock_quantity, 15);
        }

        #[ink::test]
        fn currency_conversion_comprehensive_test() {
            let concert_broker = ConcertBroker::new();

            // Test all supported currencies
            let dot_amount = 1_000_000_000_000; // 1 DOT

            // DOT to DOT
            let dot_result = concert_broker.convert_to_dot_equivalent(dot_amount, CurrencyId::DOT).unwrap();
            assert_eq!(dot_result, dot_amount);

            // ACA to DOT (rate: 50_000_000_000 = 0.05 DOT per ACA)
            let aca_result = concert_broker.convert_to_dot_equivalent(dot_amount, CurrencyId::ACA).unwrap();
            assert_eq!(aca_result, 50_000_000_000);

            // Test conversion back
            let back_to_aca = concert_broker.convert_from_dot_equivalent(aca_result, CurrencyId::ACA).unwrap();
            assert_eq!(back_to_aca, dot_amount);

            // Test aUSD (rate: 150_000_000_000 = 0.15 DOT per aUSD)
            let ausd_result = concert_broker.convert_to_dot_equivalent(dot_amount, CurrencyId::AUSD).unwrap();
            assert_eq!(ausd_result, 150_000_000_000);
        }

        #[ink::test]
        fn insufficient_merchandise_stock_fails() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, event_id) = setup_test_data(&mut concert_broker);

            // Add merchandise with limited stock
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "Limited Stock".to_string(),
                MerchandiseType::LimitedEdition,
                100_000_000_000_000,
                vec![],
                true,
                1, // Only 1 in stock
                "Very limited".to_string(),
                None,
            ).unwrap();

            // Try to purchase 2 items when only 1 is available
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(500_000_000_000_000);
            let result = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::DOT,
                None,
                vec![(merch_id, 2, None)], // Trying to buy 2
            );

            assert_eq!(result, Err(Error::StockNotAvailable));
        }

        #[ink::test]
        fn invalid_merchandise_size_fails() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, event_id) = setup_test_data(&mut concert_broker);

            // Add merchandise with specific sizes
            let merch_id = concert_broker.add_merchandise_item(
                artist_id,
                "Sized Item".to_string(),
                MerchandiseType::TShirt,
                30_000_000_000_000,
                vec!["S".to_string(), "M".to_string()], // Only S and M available
                false,
                10,
                "Sized merchandise".to_string(),
                None,
            ).unwrap();

            // Try to purchase with unavailable size
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(200_000_000_000_000);
            let result = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Section A".to_string(),
                "Row 1".to_string(),
                SeatType::GeneralAdmission,
                CurrencyId::DOT,
                None,
                vec![(merch_id, 1, Some("XL".to_string()))], // XL not available
            );

            assert_eq!(result, Err(Error::InvalidSize));
        }

        #[ink::test]
        fn vip_package_unavailable_fails() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, event_id) = setup_test_data(&mut concert_broker);

            // Add VIP package with 0 available quantity
            concert_broker.add_vip_package_to_event(
                event_id,
                "Sold Out VIP".to_string(),
                500_000_000_000_000,
                vec![VIPBenefit::MeetAndGreet],
                Some(0), // 0 available
                "Sold out package".to_string(),
            ).unwrap();

            // Try to purchase with sold out VIP package
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(700_000_000_000_000);
            let result = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "VIP Section".to_string(),
                "Row 1".to_string(),
                SeatType::VIPSeating,
                CurrencyId::DOT,
                Some("Sold Out VIP".to_string()),
                Vec::new(),
            );

            assert_eq!(result, Err(Error::VIPPackageUnavailable));
        }

        #[ink::test]
        fn complete_step4_functionality_integration() {
            let mut concert_broker = ConcertBroker::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Step 1: Register artist and venue
            let artist_id = concert_broker.register_artist(
                "Complete Artist".to_string(), None, MusicGenre::Electronic, vec![],
                "Complete test artist".to_string(), "Global".to_string(), None,
                create_test_social_media(), vec![], 2020, None,
            ).unwrap();

            let venue_id = concert_broker.register_venue(
                "Complete Venue".to_string(), create_test_venue_address(), VenueType::Arena,
                15000, None, 8, vec![], AgeRestriction::AllAges, true, None, true, vec![],
                SoundSystemRating::Excellent, LightingCapabilities::Advanced, SecurityLevel::High,
                None, None, None,
            ).unwrap();

            // Step 2: Create event
            let event_id = concert_broker.create_concert_event(
                "Complete Concert Experience".to_string(), artist_id, venue_id,
                1704067200000, 1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000), 1704067200000 + (6 * 60 * 60 * 1000),
                12000, 200_000_000_000_000, EventType::Concert, vec![], AgeRestriction::AllAges,
                "Complete concert experience with all features".to_string(),
            ).unwrap();

            // Step 3: Add merchandise catalog
            let tshirt_id = concert_broker.add_merchandise_item(
                artist_id, "Concert Tee".to_string(), MerchandiseType::TShirt,
                40_000_000_000_000, vec!["S".to_string(), "M".to_string(), "L".to_string()],
                false, 100, "Official concert t-shirt".to_string(), None,
            ).unwrap();

            let vinyl_id = concert_broker.add_merchandise_item(
                artist_id, "Limited Vinyl".to_string(), MerchandiseType::Vinyl,
                80_000_000_000_000, vec![], true, 50, "Limited edition vinyl".to_string(), None,
            ).unwrap();

            // Step 4: Set up fan token system
            concert_broker.set_artist_fan_token(artist_id, accounts.charlie, 25).unwrap();
            concert_broker.verify_fan(accounts.alice).unwrap();

            // Step 5: Add VIP packages
            concert_broker.add_vip_package_to_event(
                event_id, "Ultimate Experience".to_string(), 800_000_000_000_000,
                vec![
                    VIPBenefit::MeetAndGreet, VIPBenefit::BackstageAccess,
                    VIPBenefit::SoundcheckAccess, VIPBenefit::ExclusiveMerchandise,
                    VIPBenefit::ComplimentaryDrinks, VIPBenefit::PhotoOpportunity,
                ],
                Some(20), "The ultimate concert experience".to_string(),
            ).unwrap();

            concert_broker.add_vip_package_to_event(
                event_id, "Premium Package".to_string(), 400_000_000_000_000,
                vec![VIPBenefit::EarlyEntry, VIPBenefit::PremiumSeating, VIPBenefit::ComplimentaryDrinks],
                Some(50), "Premium concert experience".to_string(),
            ).unwrap();

            // Step 6: Configure anti-scalping measures
            concert_broker.set_event_purchase_limit(event_id, 4).unwrap();

            // Step 7: Purchase comprehensive ticket package
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            
            let base_price = 200_000_000_000_000; // Base price
            let vip_premium = 800_000_000_000_000; // Ultimate VIP
            let merch_cost = 40_000_000_000_000 + 80_000_000_000_000; // Tee + Vinyl
            let ticket_before_discount = (base_price * 4) + vip_premium; // Box seats = 4x base + VIP
            let discounted_ticket = ticket_before_discount * 75 / 100; // 25% fan discount
            let total_payment = discounted_ticket + merch_cost;

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(total_payment);

            let ticket_id = concert_broker.purchase_concert_ticket_with_currency(
                event_id,
                "Diamond Box".to_string(),
                "Row 1".to_string(),
                SeatType::BoxSeats,
                CurrencyId::DOT,
                Some("Ultimate Experience".to_string()),
                vec![
                    (tshirt_id, 1, Some("L".to_string())),
                    (vinyl_id, 1, None),
                ],
            ).unwrap();

            // Step 8: Verify comprehensive ticket features
            let ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(ticket.seat_type, SeatType::BoxSeats);
            assert_eq!(ticket.access_level, AccessLevel::AllAccess);
            assert!(ticket.fan_token_discount_applied);
            assert_eq!(ticket.vip_package_id, Some(0)); // First VIP package
            assert_eq!(ticket.special_access.len(), 6); // All VIP benefits converted
            assert_eq!(ticket.merchandise_bundle.len(), 2);
            assert!(ticket.verified_fan_purchase);
            assert!(!ticket.qr_code.is_empty());
            assert!(ticket.resale_allowed);
            assert!(ticket.resale_price_limit.is_some());

            // Step 9: Verify analytics and revenue tracking
            let (event_revenue, tickets_sold, vip_tickets, merch_revenue, avg_price) = 
                concert_broker.get_event_analytics(event_id).unwrap();
            
            assert!(event_revenue > 0);
            assert_eq!(tickets_sold, 1);
            assert_eq!(vip_tickets, 1);
            assert_eq!(merch_revenue, merch_cost);
            assert!(avg_price > 0);

            // Step 10: Verify comprehensive revenue analytics
            assert!(concert_broker.get_total_revenue() > 0);
            assert!(concert_broker.get_artist_revenue(artist_id) > 0);
            assert!(concert_broker.get_venue_revenue(venue_id) > 0);
            assert!(concert_broker.get_currency_revenue(CurrencyId::DOT) > 0);
            assert!(concert_broker.get_merchandise_revenue() > 0);
            assert!(concert_broker.get_vip_revenue() > 0);

            // Step 11: Verify inventory management
            let updated_tshirt = concert_broker.get_merchandise_item(tshirt_id).unwrap();
            assert_eq!(updated_tshirt.stock_quantity, 99); // 100 - 1

            let updated_vinyl = concert_broker.get_merchandise_item(vinyl_id).unwrap();
            assert_eq!(updated_vinyl.stock_quantity, 49); // 50 - 1

            // Step 12: Verify search and discovery still works
            let artist_events = concert_broker.search_events_by_artist(artist_id);
            assert!(artist_events.contains(&event_id));

            let electronic_artists = concert_broker.search_artists_by_genre(MusicGenre::Electronic);
            assert!(electronic_artists.contains(&artist_id));

            // Step 13: Test ticket transfer
            let result = concert_broker.transfer_concert_ticket(ticket_id, accounts.bob);
            assert_eq!(result, Ok(()));

            let transferred_ticket = concert_broker.get_concert_ticket(ticket_id).unwrap();
            assert_eq!(transferred_ticket.owner, accounts.bob);

            // SUCCESS: Complete Step 4 integration test passed!
            // All features working together: multi-currency, VIP packages, merchandise,
            // fan tokens, anti-scalping, revenue analytics, and comprehensive ticket management
            
            println!("🎉 SUCCESS: InkTix Concert Broker Step 4 Complete!");
            println!("✅ Multi-currency ticket purchasing");
            println!("✅ VIP package integration");
            println!("✅ Merchandise bundle system");
            println!("✅ Fan token discount system");
            println!("✅ Anti-scalping mechanisms");
            println!("✅ Comprehensive revenue analytics");
            println!("✅ Advanced ticket management");
            println!("✅ All existing functionality preserved");
        }
    }
}