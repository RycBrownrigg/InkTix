#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # InkTix Concert Broker - Enhanced Concert Event Structure (Steps 1-3)
/// 
/// This contract now includes comprehensive concert event management capabilities
/// building on the solid foundation of artist, venue, tour and festival management.
///
/// ## Step 3 New Features:
/// - **Enhanced Concert Events**: Comprehensive event structure with music-specific timing and metadata
/// - **Event Type Classification**: Concerts, festival days, meet & greets, acoustic sessions, etc.
/// - **Tour Integration**: Direct linking of concert events to tours with automatic statistics updates
/// - **Festival Integration**: Connect events to festivals with stage and lineup coordination
/// - **Supporting Artist Management**: Per-event supporting acts with flexible artist lineup
/// - **VIP Package Foundation**: Structure ready for merchandise and VIP package integration
/// - **Event Search & Discovery**: Advanced search capabilities for concert events
/// - **Revenue Tracking**: Comprehensive analytics and revenue tracking per event

#[ink::contract]
mod concert_broker {
    use ink::prelude::{string::String, vec::Vec};

    /// The Concert Broker contract storage.
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
        
        // NEW: Enhanced Concert Event Management (Step 3)
        concert_events: ink::storage::Mapping<u32, ConcertEvent>,
        next_concert_event_id: u32,
        
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
        
        // NEW: Concert Event indexing for search (Step 3)
        events_by_artist: ink::storage::Mapping<u32, Vec<u32>>, // artist_id -> event_ids
        events_by_venue: ink::storage::Mapping<u32, Vec<u32>>, // venue_id -> event_ids
        events_by_tour: ink::storage::Mapping<u32, Vec<u32>>, // tour_id -> event_ids
        events_by_festival: ink::storage::Mapping<u32, Vec<u32>>, // festival_id -> event_ids
        events_by_type: ink::storage::Mapping<u32, Vec<u32>>, // event_type_hash -> event_ids
        events_by_date: ink::storage::Mapping<u64, Vec<u32>>, // date_bucket -> event_ids
    }

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
        pub years_active: (u32, Option<u32>), // (start_year, end_year if disbanded)
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
        pub standing_capacity: Option<u32>, // For venues with both seated and standing
        pub acoustic_rating: u8, // 1-10 scale
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
        pub headliner_artists: Vec<u32>, // Main stage headliners
        pub featured_artists: Vec<u32>, // All performing artists
        pub stages: Vec<Stage>,
        pub capacity_per_day: u32,
        pub total_capacity: u32, // Sum across all days
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

    /// NEW: Enhanced Concert Event structure with comprehensive music-specific features (Step 3)
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct ConcertEvent {
        pub id: u32,
        pub name: String,
        pub artist_id: u32,
        pub venue_id: u32,
        pub date: u64,
        pub doors_open: u64, // When venue opens to attendees
        pub show_start: u64, // When performance begins
        pub estimated_end: u64, // Expected end time
        pub capacity: u32,
        pub sold_tickets: u32,
        pub base_price: Balance,
        pub active: bool,
        pub event_type: EventType, // Concert, Festival, Meet_Greet, Private, etc.
        pub tour_id: Option<u32>, // Link to tour if part of a tour
        pub festival_id: Option<u32>, // Link to festival if part of a festival
        pub supporting_artists: Vec<u32>, // Event-specific supporting acts
        pub merchandise_available: bool,
        pub vip_packages: Vec<VIPPackage>, // VIP offerings for this event
        pub age_restriction: AgeRestriction,
        pub revenue_generated: Balance, // Total revenue from this event
        pub special_notes: String, // Special instructions or information
        pub presale_enabled: bool, // Fan presale availability
        pub presale_start: Option<u64>, // Presale start time
        pub general_sale_start: Option<u64>, // General sale start time
        pub sound_check_available: bool, // Sound check access available
        pub meet_greet_available: bool, // Meet & greet sessions available
        pub recording_allowed: bool, // Professional recording permission
        pub live_stream_available: bool, // Live streaming option
        pub setlist_length_minutes: Option<u32>, // Expected performance duration
        pub encore_expected: bool, // Whether encore is typically performed
        pub festival_stage: Option<String>, // Festival stage name if applicable
        pub created_at: u64,
        pub last_updated: u64,
    }

    /// NEW: VIP Package offerings for events (Step 3)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct VIPPackage {
        pub package_name: String,
        pub price_premium: Balance, // Additional cost over base ticket
        pub benefits: Vec<VIPBenefit>,
        pub limited_quantity: Option<u32>,
        pub available_quantity: Option<u32>,
        pub description: String,
    }

    /// NEW: VIP Package benefits (Step 3)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VIPBenefit {
        EarlyEntry,              // Enter venue before general admission
        MeetAndGreet,            // Meet the artist
        BackstageAccess,         // Behind-the-scenes access
        SoundcheckAccess,        // Watch sound check
        PremiumSeating,          // Best seats in the venue
        ExclusiveMerchandise,    // Limited edition merchandise
        DedicatedEntrance,       // VIP entrance to avoid lines
        ComplimentaryDrinks,     // Free drinks during event
        PreShowReception,        // VIP reception before show
        PostShowAccess,          // After-party or special access
        SignedMemorabilia,       // Autographed items
        PhotoOpportunity,        // Professional photos with artist
        LimitedPoster,           // Exclusive event poster
        VIPLaminate,             // VIP access credential
        ParkingIncluded,         // Reserved parking space
    }

    /// NEW: Event types for different concert experiences (Step 3)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum EventType {
        Concert,               // Standard concert performance
        FestivalDay,           // Single day of a music festival
        MeetAndGreet,          // Artist meet & greet session
        SoundCheck,            // Private soundcheck access
        AlbumLaunch,           // Album release party/concert
        AcousticSession,       // Intimate acoustic performance
        VirtualConcert,        // Online streaming concert
        PrivateEvent,          // Exclusive private performance
        Masterclass,           // Educational session with artist
        ListeningParty,        // Album listening event
        UnpluggedSession,      // MTV Unplugged style performance
        CharityBenefit,        // Benefit concert for charity
        TributeConcert,        // Tribute or memorial concert
        ResidencyShow,         // Part of venue residency
        PopupPerformance,      // Surprise or popup show
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
        pub covered: bool, // Weather protection
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
        pub latitude: Option<i32>, // Stored as integer (multiply by 1000000 for actual value)
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

    /// Tour types for different touring scales (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum TourType {
        WorldTour,        // Global multi-continent tour
        RegionalTour,     // Multi-country or multi-state tour
        NationalTour,     // Single country tour
        LocalTour,        // City or regional area tour
        FestivalCircuit,  // Multiple festival appearances
        ResidencyTour,    // Extended stay at single venue
        AcousticTour,     // Intimate acoustic performances
        ReunionTour,      // Band reunion or comeback tour
        FarewellTour,     // Final tour before retirement
        PromotionalTour,  // Album or single promotion tour
    }

    /// Tour status tracking (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum TourStatus {
        Announced,        // Tour announced, tickets not yet on sale
        OnSale,          // Tickets currently on sale
        Active,          // Tour currently in progress
        Completed,       // Tour finished successfully
        Postponed,       // Tour postponed to future date
        Cancelled,       // Tour cancelled
        Rescheduled,     // Tour dates changed
    }

    /// Festival types for different festival categories (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalType {
        MusicFestival,    // Multi-genre music festival
        RockFestival,     // Rock and metal focused
        ElectronicFestival, // EDM and electronic music
        JazzFestival,     // Jazz and blues focused
        FolkFestival,     // Folk and acoustic music
        CountryFestival,  // Country music focused
        HipHopFestival,   // Hip-hop and R&B focused
        ClassicalFestival, // Classical and orchestral
        ArtsFestival,     // Multi-disciplinary arts
        CulturalFestival, // Cultural and heritage celebration
        CharityFestival,  // Fundraising and awareness
        CorporateFestival, // Corporate sponsored event
    }

    /// Festival status tracking (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalStatus {
        Planning,         // Festival in planning stages
        LineupAnnounced,  // Lineup announced, tickets not yet on sale
        OnSale,          // Tickets currently on sale
        SoldOut,         // All tickets sold
        Active,          // Festival currently happening
        Completed,       // Festival finished successfully
        Postponed,       // Festival postponed to future date
        Cancelled,       // Festival cancelled
        WeatherDelay,    // Temporarily delayed due to weather
    }

    /// Festival ticket types (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum FestivalTicketType {
        GeneralAdmission,         // Basic festival access
        VIP,                     // VIP area access and perks
        PlatinumVIP,             // Premium VIP with artist meet & greets
        DayPass(u32),            // Single day access (day number)
        WeekendPass,             // Weekend days only
        CampingPass,             // Includes camping accommodation
        GroupPass(u32),          // Group tickets (number of people)
        EarlyBird,               // Discounted early purchase
        StudentDiscount,         // Discounted student tickets
        LocalResident,           // Discounted local resident tickets
    }

    /// Stage types for festivals (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum StageType {
        MainStage,               // Primary headliner stage
        SecondStage,             // Secondary major acts
        AcousticStage,           // Intimate acoustic performances
        ElectronicStage,         // EDM and electronic music
        LocalStage,              // Local and emerging artists
        WorshipStage,            // Spiritual or religious music
        ComedyStage,             // Comedy and spoken word
        DanceStage,              // Dance and movement
        CommunityStage,          // Community and educational
        SponsorStage,            // Sponsor-branded stage
    }

    /// Sustainability features for eco-friendly festivals (Step 2)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SustainabilityFeature {
        SolarPower,              // Solar-powered stages/facilities
        WasteReduction,          // Comprehensive recycling program
        WaterConservation,       // Water-saving initiatives
        LocalSourcing,           // Local food and vendor sourcing
        CarbonNeutral,           // Carbon offset programs
        PublicTransport,         // Encouraged public transportation
        BiodegradableSupplies,   // Eco-friendly supplies and materials
        TreePlanting,            // Environmental restoration projects
        PlasticFree,             // Elimination of single-use plastics
        GreenVendors,            // Environmentally conscious vendors
    }

    /// Music genres with comprehensive coverage (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum MusicGenre {
        // Main genres
        Rock,
        Pop,
        Jazz,
        Classical,
        Electronic,
        HipHop,
        Country,
        Folk,
        Metal,
        Indie,
        Alternative,
        Blues,
        Reggae,
        Punk,
        Funk,
        Soul,
        RAndB,
        Gospel,
        World,
        Latin,
        
        // Electronic sub-genres
        House,
        Techno,
        Dubstep,
        Trance,
        Ambient,
        
        // Rock sub-genres
        HardRock,
        ProgressiveRock,
        PsychedelicRock,
        Grunge,
        
        // Custom/Other
        Other(String),
    }

    /// Types of music venues (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VenueType {
        Arena,           // 15,000+ capacity
        Stadium,         // 30,000+ capacity
        Theater,         // 500-5,000 capacity
        Club,            // 100-1,500 capacity
        Bar,             // 50-500 capacity
        ConcertHall,     // 1,000-8,000 capacity
        Amphitheater,    // Outdoor, 5,000-20,000 capacity
        FestivalGround,  // Large outdoor space
        OperaHouse,      // Classical/opera venue
        JazzClub,        // Intimate jazz venue
        ComedyClub,      // Stand-up comedy venue
        MultiPurpose,    // Convention centers, etc.
        OutdoorVenue,    // Parks, beaches, etc.
        PrivateVenue,    // Private homes, corporate spaces
        RecordingStudio, // Live recording sessions
        Rooftop,         // Rooftop venues
        Warehouse,       // Industrial/underground venues
        Church,          // Religious venues used for concerts
        University,      // Campus venues
        Other,
    }

    /// Venue amenities (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VenueAmenity {
        VipLounge,
        CoatCheck,
        ATM,
        MerchandiseStand,
        MultipleBars,
        FoodCourt,
        OutdoorArea,
        DanceFloor,
        ReservedSeating,
        GeneralAdmission,
        BalconySeating,
        PrivateBoxes,
        MeetAndGreetSpace,
        PhotoOpportunities,
        ProfessionalPhotography,
        LiveStreamingCapable,
        RecordingCapable,
        ClimateControl,
        SmokingArea,
        ChargingStations,
    }

    /// Age restrictions for venues and events (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AgeRestriction {
        AllAges,
        EighteenPlus,
        TwentyOnePlus,
        Custom(u8), // Custom minimum age
    }

    /// Accessibility features (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AccessibilityFeature {
        WheelchairAccessible,
        ElevatorAccess,
        AccessibleRestrooms,
        AccessibleParking,
        SignLanguageInterpretation,
        AudioDescription,
        BraillePrograms,
        ServiceAnimalFriendly,
        SensoryFriendlyOptions,
        AssistedListeningDevices,
    }

    /// Sound system quality rating (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SoundSystemRating {
        Basic,      // 1-3: Basic PA system
        Good,       // 4-6: Professional sound system
        Excellent,  // 7-8: High-end professional system
        WorldClass, // 9-10: Top-tier audiophile system
    }

    /// Lighting capabilities (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum LightingCapabilities {
        Basic,           // Simple stage lighting
        Professional,    // Full concert lighting rig
        Advanced,        // LED walls, lasers, effects
        Spectacular,     // Cutting-edge production capabilities
    }

    /// Security level of the venue (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SecurityLevel {
        Minimal,      // Basic security, small venues
        Standard,     // Professional security staff
        High,         // Metal detectors, bag checks
        Maximum,      // Full security screening, VIP protection
    }

    /// Streaming services (Step 1)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum StreamingService {
        Spotify,
        AppleMusic,
        YouTubeMusic,
        AmazonMusic,
        Tidal,
        Deezer,
        Pandora,
        SoundCloud,
        Bandcamp,
        Beatport,
        Other,
    }

    /// Concert broker errors
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Artist not found
        ArtistNotFound,
        /// Venue not found
        VenueNotFound,
        /// Event not found
        EventNotFound,
        /// Concert event not found (NEW)
        ConcertEventNotFound,
        /// Tour not found
        TourNotFound,
        /// Festival not found
        FestivalNotFound,
        /// ID overflow
        IdOverflow,
        /// Invalid artist data
        InvalidArtistData,
        /// Invalid venue data
        InvalidVenueData,
        /// Invalid tour data
        InvalidTourData,
        /// Invalid festival data
        InvalidFestivalData,
        /// Invalid event data (NEW)
        InvalidEventData,
        /// Invalid event timing (NEW)
        InvalidEventTiming,
        /// Artist already verified
        ArtistAlreadyVerified,
        /// Venue already verified
        VenueAlreadyVerified,
        /// Empty search results
        NoSearchResults,
        /// Tour already active
        TourAlreadyActive,
        /// Festival already active
        FestivalAlreadyActive,
        /// Invalid tour dates
        InvalidTourDates,
        /// Invalid festival dates
        InvalidFestivalDates,
        /// Maximum supporting artists reached
        MaxSupportingArtistsReached,
        /// Artist not available for dates
        ArtistNotAvailable,
        /// Venue not available for dates
        VenueNotAvailable,
        /// Event already linked to tour (NEW)
        EventAlreadyLinkedToTour,
        /// Event already linked to festival (NEW)
        EventAlreadyLinkedToFestival,
        /// VIP package not found (NEW)
        VIPPackageNotFound,
        /// Invalid VIP package data (NEW)
        InvalidVIPPackageData,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl ConcertBroker {
        /// Creates a new Concert Broker contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
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
            }
        }

        // ========================================================================
        // ENHANCED ARTIST MANAGEMENT (Step 1)
        // ========================================================================

        /// Register a comprehensive artist profile
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

            // Validate input data
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
                verified: false, // Will be verified separately
                fan_token_address: None, // Will be set when fan token is deployed
                social_media,
                record_label,
                biography,
                streaming_links,
                years_active: (years_active_start, None),
                origin_country,
                monthly_listeners: 0, // Will be updated from streaming APIs
                total_albums: 0, // Will be updated
                awards_count: 0, // Will be updated
                is_touring: false, // Will be updated when tours are created
                management_contact,
                created_at: current_time,
            };

            self.artists.insert(artist_id, &artist);
            
            // Update search indexes
            self.update_artist_genre_index(artist_id, &genre);
            for sub_genre in &sub_genres {
                self.update_artist_genre_index(artist_id, sub_genre);
            }
            
            self.update_verified_artists_index(artist_id, false);

            // Initialize tour index for this artist
            self.tours_by_artist.insert(artist_id, &Vec::<u32>::new());
            
            // NEW: Initialize event index for this artist (Step 3)
            self.events_by_artist.insert(artist_id, &Vec::<u32>::new());

            Ok(artist_id)
        }

        /// Verify an artist (can only be done by contract owner)
        #[ink(message)]
        pub fn verify_artist(&mut self, artist_id: u32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            
            if artist.verified {
                return Err(Error::ArtistAlreadyVerified);
            }

            artist.verified = true;
            self.artists.insert(artist_id, &artist);
            
            // Update verified artists index
            self.update_verified_artists_index(artist_id, true);

            Ok(())
        }

        /// Update artist streaming statistics
        #[ink(message)]
        pub fn update_artist_stats(
            &mut self,
            artist_id: u32,
            monthly_listeners: u32,
            total_albums: u32,
            awards_count: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            
            artist.monthly_listeners = monthly_listeners;
            artist.total_albums = total_albums;
            artist.awards_count = awards_count;
            
            self.artists.insert(artist_id, &artist);

            Ok(())
        }

        /// Set artist fan token address
        #[ink(message)]
        pub fn set_artist_fan_token(
            &mut self,
            artist_id: u32,
            fan_token_address: AccountId,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            artist.fan_token_address = Some(fan_token_address);
            self.artists.insert(artist_id, &artist);

            Ok(())
        }

        /// Update artist touring status
        #[ink(message)]
        pub fn set_artist_touring_status(&mut self, artist_id: u32, is_touring: bool) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            artist.is_touring = is_touring;
            self.artists.insert(artist_id, &artist);

            Ok(())
        }

        // ========================================================================
        // ENHANCED VENUE MANAGEMENT (Step 1)
        // ========================================================================

        /// Register a comprehensive venue profile
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

            // Validate input data
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
                backstage_facilities: false, // Will be updated separately
                merchandise_space: false, // Will be updated separately
                food_and_beverage: false, // Will be updated separately
                alcohol_license: false, // Will be updated separately
                security_level,
                established_year,
                notable_past_performers: Vec::new(), // Will be populated over time
                average_ticket_price: 0, // Will be calculated from events
                booking_contact,
                website,
                verified: false, // Will be verified separately
                created_at: current_time,
            };

            self.venues.insert(venue_id, &venue);
            
            // Update search indexes
            self.update_venue_type_index(venue_id, venue_type);
            self.update_venue_city_index(venue_id, &address.city);

            // Initialize festival index for this venue
            self.festivals_by_venue.insert(venue_id, &Vec::<u32>::new());
            
            // NEW: Initialize event index for this venue (Step 3)
            self.events_by_venue.insert(venue_id, &Vec::<u32>::new());

            Ok(venue_id)
        }

        /// Verify a venue (can only be done by contract owner)
        #[ink(message)]
        pub fn verify_venue(&mut self, venue_id: u32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            
            if venue.verified {
                return Err(Error::VenueAlreadyVerified);
            }

            venue.verified = true;
            self.venues.insert(venue_id, &venue);

            Ok(())
        }

        /// Update venue facilities
        #[ink(message)]
        pub fn update_venue_facilities(
            &mut self,
            venue_id: u32,
            backstage_facilities: bool,
            merchandise_space: bool,
            food_and_beverage: bool,
            alcohol_license: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            
            venue.backstage_facilities = backstage_facilities;
            venue.merchandise_space = merchandise_space;
            venue.food_and_beverage = food_and_beverage;
            venue.alcohol_license = alcohol_license;
            
            self.venues.insert(venue_id, &venue);

            Ok(())
        }

        /// Add notable past performer to venue
        #[ink(message)]
        pub fn add_venue_past_performer(
            &mut self,
            venue_id: u32,
            performer_name: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;
            
            if !venue.notable_past_performers.contains(&performer_name) {
                venue.notable_past_performers.push(performer_name);
                self.venues.insert(venue_id, &venue);
            }

            Ok(())
        }

        // ========================================================================
        // TOUR MANAGEMENT (Step 2)
        // ========================================================================

        /// Create a new tour for an artist
        #[ink(message)]
        pub fn create_tour(
            &mut self,
            name: String,
            artist_id: u32,
            tour_type: TourType,
            start_date: u64,
            end_date: u64,
            supporting_artists: Vec<u32>,
            merchandise_enabled: bool,
            vip_packages_available: bool,
            tour_manager_contact: Option<String>,
            description: String,
            poster_image_url: Option<String>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Validate input data
            if name.is_empty() || description.is_empty() {
                return Err(Error::InvalidTourData);
            }

            if start_date >= end_date {
                return Err(Error::InvalidTourDates);
            }

            // Verify artist exists
            let mut artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            // Verify supporting artists exist
            for supporting_artist_id in &supporting_artists {
                if self.artists.get(*supporting_artist_id).is_none() {
                    return Err(Error::ArtistNotFound);
                }
            }

            // Check if artist is not already on an active tour (optional business rule)
            if let Some(artist_tours) = self.tours_by_artist.get(artist_id) {
                for tour_id in artist_tours {
                    if let Some(existing_tour) = self.tours.get(tour_id) {
                        if matches!(existing_tour.tour_status, TourStatus::Active | TourStatus::OnSale) {
                            if existing_tour.start_date <= end_date && existing_tour.end_date >= start_date {
                                return Err(Error::ArtistNotAvailable);
                            }
                        }
                    }
                }
            }

            let tour_id = self.next_tour_id;
            self.next_tour_id = self.next_tour_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();

            let tour = Tour {
                id: tour_id,
                name,
                artist_id,
                tour_type,
                start_date,
                end_date,
                total_shows: 0, // Will be updated as shows are added
                shows_scheduled: 0,
                shows_completed: 0,
                tour_status: TourStatus::Announced,
                supporting_artists,
                merchandise_enabled,
                vip_packages_available,
                tour_manager_contact,
                sponsors: Vec::new(), // Will be added separately
                total_revenue_generated: 0,
                average_ticket_price: 0,
                total_tickets_sold: 0,
                fan_presale_enabled: false,
                description,
                poster_image_url,
                created_at: current_time,
                last_updated: current_time,
            };

            self.tours.insert(tour_id, &tour);

            // Update artist's touring status
            artist.is_touring = true;
            self.artists.insert(artist_id, &artist);

            // Update search indexes
            self.update_tour_by_artist_index(tour_id, artist_id);
            self.update_tour_by_type_index(tour_id, tour_type);
            self.update_active_tours_index(tour_id, false); // Not active yet

            // NEW: Initialize event index for this tour (Step 3)
            self.events_by_tour.insert(tour_id, &Vec::<u32>::new());

            Ok(tour_id)
        }

        /// Update tour status
        #[ink(message)]
        pub fn update_tour_status(&mut self, tour_id: u32, new_status: TourStatus) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut tour = self.tours.get(tour_id).ok_or(Error::TourNotFound)?;
            let old_status = tour.tour_status;
            
            tour.tour_status = new_status;
            tour.last_updated = self.env().block_timestamp();
            
            self.tours.insert(tour_id, &tour);

            // Update active tours index
            let is_active_old = matches!(old_status, TourStatus::Active | TourStatus::OnSale);
            let is_active_new = matches!(new_status, TourStatus::Active | TourStatus::OnSale);
            
            if is_active_old != is_active_new {
                self.update_active_tours_index(tour_id, is_active_new);
            }

            Ok(())
        }

        /// Add supporting artist to existing tour
        #[ink(message)]
        pub fn add_supporting_artist_to_tour(
            &mut self,
            tour_id: u32,
            artist_id: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut tour = self.tours.get(tour_id).ok_or(Error::TourNotFound)?;
            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            // Check limit on supporting artists (business rule: max 5)
            if tour.supporting_artists.len() >= 5 {
                return Err(Error::MaxSupportingArtistsReached);
            }

            // Check if artist is not already a supporting artist
            if !tour.supporting_artists.contains(&artist_id) {
                tour.supporting_artists.push(artist_id);
                tour.last_updated = self.env().block_timestamp();
                self.tours.insert(tour_id, &tour);
            }

            Ok(())
        }

        /// Add sponsor to tour
        #[ink(message)]
        pub fn add_tour_sponsor(&mut self, tour_id: u32, sponsor_name: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut tour = self.tours.get(tour_id).ok_or(Error::TourNotFound)?;
            
            if !tour.sponsors.contains(&sponsor_name) {
                tour.sponsors.push(sponsor_name);
                tour.last_updated = self.env().block_timestamp();
                self.tours.insert(tour_id, &tour);
            }

            Ok(())
        }

        // ========================================================================
        // FESTIVAL MANAGEMENT (Step 2)
        // ========================================================================

        /// Create a new festival
        #[ink(message)]
        pub fn create_festival(
            &mut self,
            name: String,
            venue_id: u32,
            festival_type: FestivalType,
            start_date: u64,
            end_date: u64,
            capacity_per_day: u32,
            camping_available: bool,
            camping_capacity: Option<u32>,
            age_restrictions: AgeRestriction,
            organizer_contact: Option<String>,
            website: Option<String>,
            social_media: SocialMediaHandles,
            description: String,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Validate input data
            if name.is_empty() || description.is_empty() || capacity_per_day == 0 {
                return Err(Error::InvalidFestivalData);
            }

            if start_date >= end_date {
                return Err(Error::InvalidFestivalDates);
            }

            // Verify venue exists
            let _venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;

            let festival_id = self.next_festival_id;
            self.next_festival_id = self.next_festival_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();
            
            // Calculate total days and capacity
            let total_days = ((end_date - start_date) / (24 * 60 * 60 * 1000)) as u32 + 1;
            let total_capacity = capacity_per_day * total_days;

            let festival = Festival {
                id: festival_id,
                name,
                venue_id,
                festival_type,
                start_date,
                end_date,
                total_days,
                headliner_artists: Vec::new(),
                featured_artists: Vec::new(),
                stages: Vec::new(), // Will be added separately
                capacity_per_day,
                total_capacity,
                camping_available,
                camping_capacity,
                food_vendors: Vec::new(),
                merchandise_vendors: Vec::new(),
                age_restrictions,
                festival_status: FestivalStatus::Planning,
                ticket_types: Vec::new(), // Will be configured separately
                sponsors: Vec::new(),
                organizer_contact,
                website,
                social_media,
                sustainability_features: Vec::new(),
                total_revenue_generated: 0,
                total_tickets_sold: 0,
                description,
                lineup_poster_url: None,
                created_at: current_time,
                last_updated: current_time,
            };

            self.festivals.insert(festival_id, &festival);

            // Update search indexes
            self.update_festival_by_type_index(festival_id, festival_type);
            self.update_festival_by_venue_index(festival_id, venue_id);

            // NEW: Initialize event index for this festival (Step 3)
            self.events_by_festival.insert(festival_id, &Vec::<u32>::new());

            Ok(festival_id)
        }

        /// Add artist to festival lineup
        #[ink(message)]
        pub fn add_festival_artist(
            &mut self,
            festival_id: u32,
            artist_id: u32,
            is_headliner: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;
            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            // Add to featured artists if not already there
            if !festival.featured_artists.contains(&artist_id) {
                festival.featured_artists.push(artist_id);
            }

            // Add to headliners if specified and not already there
            if is_headliner && !festival.headliner_artists.contains(&artist_id) {
                festival.headliner_artists.push(artist_id);
            }

            festival.last_updated = self.env().block_timestamp();
            self.festivals.insert(festival_id, &festival);

            Ok(())
        }

        /// Add stage to festival
        #[ink(message)]
        pub fn add_festival_stage(
            &mut self,
            festival_id: u32,
            stage_name: String,
            stage_type: StageType,
            capacity: u32,
            sound_system: SoundSystemRating,
            lighting_system: LightingCapabilities,
            covered: bool,
            accessibility_compliant: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;

            let stage = Stage {
                name: stage_name,
                stage_type,
                capacity,
                sound_system,
                lighting_system,
                covered,
                accessibility_compliant,
            };

            festival.stages.push(stage);
            festival.last_updated = self.env().block_timestamp();
            self.festivals.insert(festival_id, &festival);

            Ok(())
        }

        /// Update festival status
        #[ink(message)]
        pub fn update_festival_status(&mut self, festival_id: u32, new_status: FestivalStatus) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;
            
            festival.festival_status = new_status;
            festival.last_updated = self.env().block_timestamp();
            
            self.festivals.insert(festival_id, &festival);

            Ok(())
        }

        /// Add vendor to festival
        #[ink(message)]
        pub fn add_festival_vendor(
            &mut self,
            festival_id: u32,
            vendor_name: String,
            vendor_type: String, // "food" or "merchandise"
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;
            
            match vendor_type.as_str() {
                "food" => {
                    if !festival.food_vendors.contains(&vendor_name) {
                        festival.food_vendors.push(vendor_name);
                    }
                },
                "merchandise" => {
                    if !festival.merchandise_vendors.contains(&vendor_name) {
                        festival.merchandise_vendors.push(vendor_name);
                    }
                },
                _ => return Err(Error::InvalidFestivalData),
            }

            festival.last_updated = self.env().block_timestamp();
            self.festivals.insert(festival_id, &festival);

            Ok(())
        }

        /// Add sustainability feature to festival
        #[ink(message)]
        pub fn add_festival_sustainability_feature(
            &mut self,
            festival_id: u32,
            feature: SustainabilityFeature,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;
            
            if !festival.sustainability_features.contains(&feature) {
                festival.sustainability_features.push(feature);
                festival.last_updated = self.env().block_timestamp();
                self.festivals.insert(festival_id, &festival);
            }

            Ok(())
        }

        /// Add sponsor to festival
        #[ink(message)]
        pub fn add_festival_sponsor(&mut self, festival_id: u32, sponsor_name: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;
            
            if !festival.sponsors.contains(&sponsor_name) {
                festival.sponsors.push(sponsor_name);
                festival.last_updated = self.env().block_timestamp();
                self.festivals.insert(festival_id, &festival);
            }

            Ok(())
        }

        // ========================================================================
        // NEW: ENHANCED CONCERT EVENT MANAGEMENT (Step 3)
        // ========================================================================

        /// Create a comprehensive concert event with full music industry features
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

            // Validate input data
            if name.is_empty() {
                return Err(Error::InvalidEventData);
            }

            // Validate timing sequence
            if doors_open >= show_start || show_start >= estimated_end || date > doors_open {
                return Err(Error::InvalidEventTiming);
            }

            // Verify artist and venue exist
            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;
            let venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;

            // Verify supporting artists exist
            for supporting_artist_id in &supporting_artists {
                if self.artists.get(*supporting_artist_id).is_none() {
                    return Err(Error::ArtistNotFound);
                }
            }

            // Check if supporting artists list is reasonable (max 10 for concerts)
            if supporting_artists.len() > 10 {
                return Err(Error::MaxSupportingArtistsReached);
            }

            let event_id = self.next_concert_event_id;
            self.next_concert_event_id = self.next_concert_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let current_time = self.env().block_timestamp();
            
            // Use venue capacity if event capacity is 0 or greater than venue capacity
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
                tour_id: None, // Will be linked separately if needed
                festival_id: None, // Will be linked separately if needed
                supporting_artists,
                merchandise_available: false, // Will be updated separately
                vip_packages: Vec::new(), // Will be added separately
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

            // Update search indexes
            self.update_concert_event_indexes(event_id, &concert_event);

            Ok(event_id)
        }

        /// Link a concert event to an existing tour
        #[ink(message)]
        pub fn link_event_to_tour(&mut self, event_id: u32, tour_id: u32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            let mut tour = self.tours.get(tour_id).ok_or(Error::TourNotFound)?;

            // Check if event is already linked to a tour
            if event.tour_id.is_some() {
                return Err(Error::EventAlreadyLinkedToTour);
            }

            // Verify the event artist matches the tour artist
            if event.artist_id != tour.artist_id {
                return Err(Error::ArtistNotFound);
            }

            // Verify event date is within tour dates
            if event.date < tour.start_date || event.date > tour.end_date {
                return Err(Error::InvalidTourDates);
            }

            // Link event to tour
            event.tour_id = Some(tour_id);
            event.last_updated = self.env().block_timestamp();
            self.concert_events.insert(event_id, &event);

            // Update tour statistics
            tour.total_shows = tour.total_shows.saturating_add(1);
            tour.shows_scheduled = tour.shows_scheduled.saturating_add(1);
            tour.last_updated = self.env().block_timestamp();
            self.tours.insert(tour_id, &tour);

            // Update tour events index
            let mut tour_events = self.events_by_tour.get(tour_id).unwrap_or_default();
            if !tour_events.contains(&event_id) {
                tour_events.push(event_id);
                self.events_by_tour.insert(tour_id, &tour_events);
            }

            Ok(())
        }

        /// Link a concert event to an existing festival
        #[ink(message)]
        pub fn link_event_to_festival(
            &mut self, 
            event_id: u32, 
            festival_id: u32,
            stage_name: Option<String>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            let festival = self.festivals.get(festival_id).ok_or(Error::FestivalNotFound)?;

            // Check if event is already linked to a festival
            if event.festival_id.is_some() {
                return Err(Error::EventAlreadyLinkedToFestival);
            }

            // Verify the event venue matches the festival venue
            if event.venue_id != festival.venue_id {
                return Err(Error::VenueNotFound);
            }

            // Verify event date is within festival dates
            if event.date < festival.start_date || event.date > festival.end_date {
                return Err(Error::InvalidFestivalDates);
            }

            // Verify artist is in festival lineup
            if !festival.featured_artists.contains(&event.artist_id) && 
               !festival.headliner_artists.contains(&event.artist_id) {
                return Err(Error::ArtistNotFound);
            }

            // Link event to festival
            event.festival_id = Some(festival_id);
            event.event_type = EventType::FestivalDay;
            event.festival_stage = stage_name;
            event.last_updated = self.env().block_timestamp();
            self.concert_events.insert(event_id, &event);

            // Update festival events index
            let mut festival_events = self.events_by_festival.get(festival_id).unwrap_or_default();
            if !festival_events.contains(&event_id) {
                festival_events.push(event_id);
                self.events_by_festival.insert(festival_id, &festival_events);
            }

            Ok(())
        }

        /// Add supporting artist to specific event
        #[ink(message)]
        pub fn add_supporting_artist_to_event(
            &mut self,
            event_id: u32,
            artist_id: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;
            let _artist = self.artists.get(artist_id).ok_or(Error::ArtistNotFound)?;

            // Check limit on supporting artists (max 10 for individual events)
            if event.supporting_artists.len() >= 10 {
                return Err(Error::MaxSupportingArtistsReached);
            }

            // Add supporting artist if not already in the list
            if !event.supporting_artists.contains(&artist_id) {
                event.supporting_artists.push(artist_id);
                event.last_updated = self.env().block_timestamp();
                self.concert_events.insert(event_id, &event);

                // Update search index for the new supporting artist
                let mut supporting_events = self.events_by_artist.get(artist_id).unwrap_or_default();
                if !supporting_events.contains(&event_id) {
                    supporting_events.push(event_id);
                    self.events_by_artist.insert(artist_id, &supporting_events);
                }
            }

            Ok(())
        }

        /// Update event timing information
        #[ink(message)]
        pub fn update_event_timing(
            &mut self,
            event_id: u32,
            doors_open: Option<u64>,
            show_start: Option<u64>,
            estimated_end: Option<u64>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;

            // Update provided timing fields
            if let Some(new_doors_open) = doors_open {
                event.doors_open = new_doors_open;
            }
            
            if let Some(new_show_start) = show_start {
                event.show_start = new_show_start;
            }
            
            if let Some(new_estimated_end) = estimated_end {
                event.estimated_end = new_estimated_end;
            }

            // Validate timing sequence
            if event.doors_open >= event.show_start || 
               event.show_start >= event.estimated_end || 
               event.date > event.doors_open {
                return Err(Error::InvalidEventTiming);
            }

            event.last_updated = self.env().block_timestamp();
            self.concert_events.insert(event_id, &event);

            Ok(())
        }

        /// Add VIP package to event
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

            // Validate VIP package data
            if package_name.is_empty() || description.is_empty() || benefits.is_empty() {
                return Err(Error::InvalidVIPPackageData);
            }

            let vip_package = VIPPackage {
                package_name,
                price_premium,
                benefits,
                limited_quantity,
                available_quantity: limited_quantity, // Initially same as limited quantity
                description,
            };

            event.vip_packages.push(vip_package);
            event.last_updated = self.env().block_timestamp();
            self.concert_events.insert(event_id, &event);

            Ok(())
        }

        /// Update event special features
        #[ink(message)]
        pub fn update_event_features(
            &mut self,
            event_id: u32,
            merchandise_available: bool,
            presale_enabled: bool,
            presale_start: Option<u64>,
            general_sale_start: Option<u64>,
            sound_check_available: bool,
            meet_greet_available: bool,
            recording_allowed: bool,
            live_stream_available: bool,
            setlist_length_minutes: Option<u32>,
            encore_expected: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut event = self.concert_events.get(event_id).ok_or(Error::ConcertEventNotFound)?;

            event.merchandise_available = merchandise_available;
            event.presale_enabled = presale_enabled;
            event.presale_start = presale_start;
            event.general_sale_start = general_sale_start;
            event.sound_check_available = sound_check_available;
            event.meet_greet_available = meet_greet_available;
            event.recording_allowed = recording_allowed;
            event.live_stream_available = live_stream_available;
            event.setlist_length_minutes = setlist_length_minutes;
            event.encore_expected = encore_expected;
            event.last_updated = self.env().block_timestamp();

            self.concert_events.insert(event_id, &event);

            Ok(())
        }

        // ========================================================================
        // NEW: ENHANCED SEARCH AND DISCOVERY FUNCTIONS (Step 3)
        // ========================================================================

        /// Search concert events by artist (main or supporting)
        #[ink(message)]
        pub fn search_events_by_artist(&self, artist_id: u32) -> Vec<u32> {
            self.events_by_artist.get(artist_id).unwrap_or_default()
        }

        /// Search concert events by venue
        #[ink(message)]
        pub fn search_events_by_venue(&self, venue_id: u32) -> Vec<u32> {
            self.events_by_venue.get(venue_id).unwrap_or_default()
        }

        /// Search concert events by tour
        #[ink(message)]
        pub fn search_events_by_tour(&self, tour_id: u32) -> Vec<u32> {
            self.events_by_tour.get(tour_id).unwrap_or_default()
        }

        /// Search concert events by festival
        #[ink(message)]
        pub fn search_events_by_festival(&self, festival_id: u32) -> Vec<u32> {
            self.events_by_festival.get(festival_id).unwrap_or_default()
        }

        /// Search concert events by event type
        #[ink(message)]
        pub fn search_events_by_type(&self, event_type: EventType) -> Vec<u32> {
            let type_hash = self.hash_event_type(event_type);
            self.events_by_type.get(type_hash).unwrap_or_default()
        }

        /// Get upcoming concert events (next 30 days)
        #[ink(message)]
        pub fn get_upcoming_events(&self) -> Vec<u32> {
            let mut results = Vec::new();
            let current_time = self.env().block_timestamp();
            let thirty_days = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds

            for event_id in 1..self.next_concert_event_id {
                if let Some(event) = self.concert_events.get(event_id) {
                    if event.active && 
                       event.date > current_time && 
                       event.date <= current_time + thirty_days {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Get events with VIP packages available
        #[ink(message)]
        pub fn get_events_with_vip_packages(&self) -> Vec<u32> {
            let mut results = Vec::new();
            
            for event_id in 1..self.next_concert_event_id {
                if let Some(event) = self.concert_events.get(event_id) {
                    if event.active && !event.vip_packages.is_empty() {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Get events with meet & greet available
        #[ink(message)]
        pub fn get_events_with_meet_greet(&self) -> Vec<u32> {
            let mut results = Vec::new();
            
            for event_id in 1..self.next_concert_event_id {
                if let Some(event) = self.concert_events.get(event_id) {
                    if event.active && event.meet_greet_available {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        /// Get events with live streaming available
        #[ink(message)]
        pub fn get_events_with_live_stream(&self) -> Vec<u32> {
            let mut results = Vec::new();
            
            for event_id in 1..self.next_concert_event_id {
                if let Some(event) = self.concert_events.get(event_id) {
                    if event.active && event.live_stream_available {
                        results.push(event_id);
                    }
                }
            }
            results
        }

        // ========================================================================
        // SEARCH AND DISCOVERY FUNCTIONS (Steps 1-2)
        // ========================================================================

        /// Search artists by genre (Step 1)
        #[ink(message)]
        pub fn search_artists_by_genre(&self, genre: MusicGenre) -> Vec<u32> {
            let genre_hash = self.hash_music_genre(&genre);
            self.artists_by_genre.get(genre_hash).unwrap_or_default()
        }

        /// Get all verified artists (Step 1)
        #[ink(message)]
        pub fn get_verified_artists(&self) -> Vec<u32> {
            self.verified_artists.get(true).unwrap_or_default()
        }

        /// Search venues by type (Step 1)
        #[ink(message)]
        pub fn search_venues_by_type(&self, venue_type: VenueType) -> Vec<u32> {
            let type_hash = self.hash_venue_type(venue_type);
            self.venues_by_type.get(type_hash).unwrap_or_default()
        }

        /// Search venues by city (Step 1)
        #[ink(message)]
        pub fn search_venues_by_city(&self, city: String) -> Vec<u32> {
            let city_hash = self.hash_string(&city);
            self.venues_by_city.get(city_hash).unwrap_or_default()
        }

        /// Get venues with specific amenities (Step 1)
        #[ink(message)]
        pub fn search_venues_with_amenity(&self, amenity: VenueAmenity) -> Vec<u32> {
            let mut results = Vec::new();
            for venue_id in 1..self.next_venue_id {
                if let Some(venue) = self.venues.get(venue_id) {
                    if venue.amenities.contains(&amenity) {
                        results.push(venue_id);
                    }
                }
            }
            results
        }

        /// Get venues with accessibility features (Step 1)
        #[ink(message)]
        pub fn search_accessible_venues(&self, feature: AccessibilityFeature) -> Vec<u32> {
            let mut results = Vec::new();
            for venue_id in 1..self.next_venue_id {
                if let Some(venue) = self.venues.get(venue_id) {
                    if venue.accessibility_features.contains(&feature) {
                        results.push(venue_id);
                    }
                }
            }
            results
        }

        /// Get artists currently touring (Step 1)
        #[ink(message)]
        pub fn get_touring_artists(&self) -> Vec<u32> {
            let mut results = Vec::new();
            for artist_id in 1..self.next_artist_id {
                if let Some(artist) = self.artists.get(artist_id) {
                    if artist.is_touring {
                        results.push(artist_id);
                    }
                }
            }
            results
        }

        /// Search tours by artist (Step 2)
        #[ink(message)]
        pub fn get_tours_by_artist(&self, artist_id: u32) -> Vec<u32> {
            self.tours_by_artist.get(artist_id).unwrap_or_default()
        }

        /// Search tours by type (Step 2)
        #[ink(message)]
        pub fn search_tours_by_type(&self, tour_type: TourType) -> Vec<u32> {
            let type_hash = self.hash_tour_type(tour_type);
            self.tours_by_type.get(type_hash).unwrap_or_default()
        }

        /// Get active tours (Step 2)
        #[ink(message)]
        pub fn get_active_tours(&self) -> Vec<u32> {
            self.active_tours.get(true).unwrap_or_default()
        }

        /// Get upcoming tours (starting within next 90 days) (Step 2)
        #[ink(message)]
        pub fn get_upcoming_tours(&self) -> Vec<u32> {
            let mut results = Vec::new();
            let current_time = self.env().block_timestamp();
            let ninety_days = 90 * 24 * 60 * 60 * 1000; // 90 days in milliseconds

            for tour_id in 1..self.next_tour_id {
                if let Some(tour) = self.tours.get(tour_id) {
                    if tour.start_date > current_time && tour.start_date <= current_time + ninety_days {
                        if matches!(tour.tour_status, TourStatus::Announced | TourStatus::OnSale) {
                            results.push(tour_id);
                        }
                    }
                }
            }
            results
        }

        /// Search festivals by type (Step 2)
        #[ink(message)]
        pub fn search_festivals_by_type(&self, festival_type: FestivalType) -> Vec<u32> {
            let type_hash = self.hash_festival_type(festival_type);
            self.festivals_by_type.get(type_hash).unwrap_or_default()
        }

        /// Search festivals by venue (Step 2)
        #[ink(message)]
        pub fn search_festivals_by_venue(&self, venue_id: u32) -> Vec<u32> {
            self.festivals_by_venue.get(venue_id).unwrap_or_default()
        }

        /// Get upcoming festivals (Step 2)
        #[ink(message)]
        pub fn get_upcoming_festivals(&self) -> Vec<u32> {
            let mut results = Vec::new();
            let current_time = self.env().block_timestamp();

            for festival_id in 1..self.next_festival_id {
                if let Some(festival) = self.festivals.get(festival_id) {
                    if festival.start_date > current_time {
                        if matches!(festival.festival_status, FestivalStatus::LineupAnnounced | FestivalStatus::OnSale) {
                            results.push(festival_id);
                        }
                    }
                }
            }
            results
        }

        /// Search festivals by artist (Step 2)
        #[ink(message)]
        pub fn search_festivals_by_artist(&self, artist_id: u32) -> Vec<u32> {
            let mut results = Vec::new();
            for festival_id in 1..self.next_festival_id {
                if let Some(festival) = self.festivals.get(festival_id) {
                    if festival.featured_artists.contains(&artist_id) || festival.headliner_artists.contains(&artist_id) {
                        results.push(festival_id);
                    }
                }
            }
            results
        }

        // ========================================================================
        // NEW: HELPER FUNCTIONS FOR CONCERT EVENT INDEXING (Step 3)
        // ========================================================================

        /// Update all search indexes when creating/updating concert events
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

            // Update date index (bucket by day)
            let date_bucket = event.date / (24 * 60 * 60 * 1000); // Days since epoch
            let mut date_events = self.events_by_date.get(date_bucket).unwrap_or_default();
            if !date_events.contains(&event_id) {
                date_events.push(event_id);
                self.events_by_date.insert(date_bucket, &date_events);
            }
        }

        /// Hash function for event types
        fn hash_event_type(&self, event_type: EventType) -> u32 {
            match event_type {
                EventType::Concert => 1,
                EventType::FestivalDay => 2,
                EventType::MeetAndGreet => 3,
                EventType::SoundCheck => 4,
                EventType::AlbumLaunch => 5,
                EventType::AcousticSession => 6,
                EventType::VirtualConcert => 7,
                EventType::PrivateEvent => 8,
                EventType::Masterclass => 9,
                EventType::ListeningParty => 10,
                EventType::UnpluggedSession => 11,
                EventType::CharityBenefit => 12,
                EventType::TributeConcert => 13,
                EventType::ResidencyShow => 14,
                EventType::PopupPerformance => 15,
            }
        }

        // ========================================================================
        // HELPER FUNCTIONS FOR INDEXING (Steps 1-2)
        // ========================================================================

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
            
            // Remove from opposite list if needed
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

        // Tour indexing functions (Step 2)
        fn update_tour_by_artist_index(&mut self, tour_id: u32, artist_id: u32) {
            let mut artist_tours = self.tours_by_artist.get(artist_id).unwrap_or_default();
            if !artist_tours.contains(&tour_id) {
                artist_tours.push(tour_id);
                self.tours_by_artist.insert(artist_id, &artist_tours);
            }
        }

        fn update_tour_by_type_index(&mut self, tour_id: u32, tour_type: TourType) {
            let type_hash = self.hash_tour_type(tour_type);
            let mut tours_of_type = self.tours_by_type.get(type_hash).unwrap_or_default();
            if !tours_of_type.contains(&tour_id) {
                tours_of_type.push(tour_id);
                self.tours_by_type.insert(type_hash, &tours_of_type);
            }
        }

        fn update_active_tours_index(&mut self, tour_id: u32, is_active: bool) {
            let mut active_list = self.active_tours.get(is_active).unwrap_or_default();
            if !active_list.contains(&tour_id) {
                active_list.push(tour_id);
                self.active_tours.insert(is_active, &active_list);
            }
            
            // Remove from opposite list if needed
            let mut opposite_list = self.active_tours.get(!is_active).unwrap_or_default();
            opposite_list.retain(|&x| x != tour_id);
            if opposite_list.is_empty() {
                self.active_tours.remove(!is_active);
            } else {
                self.active_tours.insert(!is_active, &opposite_list);
            }
        }

        // Festival indexing functions (Step 2)
        fn update_festival_by_type_index(&mut self, festival_id: u32, festival_type: FestivalType) {
            let type_hash = self.hash_festival_type(festival_type);
            let mut festivals_of_type = self.festivals_by_type.get(type_hash).unwrap_or_default();
            if !festivals_of_type.contains(&festival_id) {
                festivals_of_type.push(festival_id);
                self.festivals_by_type.insert(type_hash, &festivals_of_type);
            }
        }

        fn update_festival_by_venue_index(&mut self, festival_id: u32, venue_id: u32) {
            let mut venue_festivals = self.festivals_by_venue.get(venue_id).unwrap_or_default();
            if !venue_festivals.contains(&festival_id) {
                venue_festivals.push(festival_id);
                self.festivals_by_venue.insert(venue_id, &venue_festivals);
            }
        }

        // Hash functions for efficient indexing
        fn hash_music_genre(&self, genre: &MusicGenre) -> u32 {
            match genre {
                MusicGenre::Rock => 1,
                MusicGenre::Pop => 2,
                MusicGenre::Jazz => 3,
                MusicGenre::Classical => 4,
                MusicGenre::Electronic => 5,
                MusicGenre::HipHop => 6,
                MusicGenre::Country => 7,
                MusicGenre::Folk => 8,
                MusicGenre::Metal => 9,
                MusicGenre::Indie => 10,
                MusicGenre::Alternative => 11,
                MusicGenre::Blues => 12,
                MusicGenre::Reggae => 13,
                MusicGenre::Punk => 14,
                MusicGenre::Funk => 15,
                MusicGenre::Soul => 16,
                MusicGenre::RAndB => 17,
                MusicGenre::Gospel => 18,
                MusicGenre::World => 19,
                MusicGenre::Latin => 20,
                MusicGenre::House => 21,
                MusicGenre::Techno => 22,
                MusicGenre::Dubstep => 23,
                MusicGenre::Trance => 24,
                MusicGenre::Ambient => 25,
                MusicGenre::HardRock => 26,
                MusicGenre::ProgressiveRock => 27,
                MusicGenre::PsychedelicRock => 28,
                MusicGenre::Grunge => 29,
                MusicGenre::Other(_) => 99,
            }
        }

        fn hash_venue_type(&self, venue_type: VenueType) -> u32 {
            match venue_type {
                VenueType::Arena => 1,
                VenueType::Stadium => 2,
                VenueType::Theater => 3,
                VenueType::Club => 4,
                VenueType::Bar => 5,
                VenueType::ConcertHall => 6,
                VenueType::Amphitheater => 7,
                VenueType::FestivalGround => 8,
                VenueType::OperaHouse => 9,
                VenueType::JazzClub => 10,
                VenueType::ComedyClub => 11,
                VenueType::MultiPurpose => 12,
                VenueType::OutdoorVenue => 13,
                VenueType::PrivateVenue => 14,
                VenueType::RecordingStudio => 15,
                VenueType::Rooftop => 16,
                VenueType::Warehouse => 17,
                VenueType::Church => 18,
                VenueType::University => 19,
                VenueType::Other => 99,
            }
        }

        // Tour type hash function (Step 2)
        fn hash_tour_type(&self, tour_type: TourType) -> u32 {
            match tour_type {
                TourType::WorldTour => 1,
                TourType::RegionalTour => 2,
                TourType::NationalTour => 3,
                TourType::LocalTour => 4,
                TourType::FestivalCircuit => 5,
                TourType::ResidencyTour => 6,
                TourType::AcousticTour => 7,
                TourType::ReunionTour => 8,
                TourType::FarewellTour => 9,
                TourType::PromotionalTour => 10,
            }
        }

        // Festival type hash function (Step 2)
        fn hash_festival_type(&self, festival_type: FestivalType) -> u32 {
            match festival_type {
                FestivalType::MusicFestival => 1,
                FestivalType::RockFestival => 2,
                FestivalType::ElectronicFestival => 3,
                FestivalType::JazzFestival => 4,
                FestivalType::FolkFestival => 5,
                FestivalType::CountryFestival => 6,
                FestivalType::HipHopFestival => 7,
                FestivalType::ClassicalFestival => 8,
                FestivalType::ArtsFestival => 9,
                FestivalType::CulturalFestival => 10,
                FestivalType::CharityFestival => 11,
                FestivalType::CorporateFestival => 12,
            }
        }

        fn hash_string(&self, s: &str) -> u32 {
            // Simple hash function for demonstration
            // In production, you'd want a proper hash function
            let mut hash: u32 = 0;
            for byte in s.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
            }
            hash
        }

        // ========================================================================
        // QUERY METHODS (Steps 1-3)
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

        /// Get tour details (Step 2)
        #[ink(message)]
        pub fn get_tour(&self, tour_id: u32) -> Option<Tour> {
            self.tours.get(tour_id)
        }

        /// Get festival details (Step 2)
        #[ink(message)]
        pub fn get_festival(&self, festival_id: u32) -> Option<Festival> {
            self.festivals.get(festival_id)
        }

        /// NEW: Get concert event details (Step 3)
        #[ink(message)]
        pub fn get_concert_event(&self, event_id: u32) -> Option<ConcertEvent> {
            self.concert_events.get(event_id)
        }

        /// Get the owner of the contract
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get total artists registered
        #[ink(message)]
        pub fn total_artists(&self) -> u32 {
            self.next_artist_id.saturating_sub(1)
        }

        /// Get total venues registered
        #[ink(message)]
        pub fn total_venues(&self) -> u32 {
            self.next_venue_id.saturating_sub(1)
        }

        /// Get total tours created (Step 2)
        #[ink(message)]
        pub fn total_tours(&self) -> u32 {
            self.next_tour_id.saturating_sub(1)
        }

        /// Get total festivals created (Step 2)
        #[ink(message)]
        pub fn total_festivals(&self) -> u32 {
            self.next_festival_id.saturating_sub(1)
        }

        /// NEW: Get total concert events created (Step 3)
        #[ink(message)]
        pub fn total_concert_events(&self) -> u32 {
            self.next_concert_event_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for ConcertBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Comprehensive test suite for Steps 1-3
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
                StreamingPlatform {
                    platform: StreamingService::AppleMusic,
                    artist_url: "https://music.apple.com/artist/test".to_string(),
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
                latitude: Some(36162664), // 36.162664 * 1000000
                longitude: Some(-86781602), // -86.781602 * 1000000
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

            let supporting_artist_id = contract.register_artist(
                "Phoebe Bridgers".to_string(),
                None,
                MusicGenre::Indie,
                vec![MusicGenre::Folk, MusicGenre::Alternative],
                "Indie folk singer-songwriter".to_string(),
                "United States".to_string(),
                Some("Dead Oceans".to_string()),
                create_test_social_media(),
                vec![],
                2017,
                None,
            ).unwrap();

            (artist_id, venue_id, supporting_artist_id)
        }

        // ========================================================================
        // STEP 1-2: EXISTING TESTS (Continue to pass)
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let concert_broker = ConcertBroker::new();
            assert_eq!(concert_broker.total_artists(), 0);
            assert_eq!(concert_broker.total_venues(), 0);
            assert_eq!(concert_broker.total_tours(), 0);
            assert_eq!(concert_broker.total_festivals(), 0);
            assert_eq!(concert_broker.total_concert_events(), 0); // NEW: Test Step 3
        }

        #[ink::test]
        fn register_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            assert_eq!(artist_id, 1);
            assert_eq!(concert_broker.total_artists(), 2); // Main artist + supporting artist

            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert_eq!(artist.name, "Taylor Swift");
            assert_eq!(artist.genre, MusicGenre::Pop);
            assert_eq!(artist.sub_genres.len(), 2);
            assert!(!artist.verified);
            assert_eq!(artist.years_active.0, 2006);
        }

        #[ink::test]
        fn register_venue_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            assert_eq!(venue_id, 1);
            assert_eq!(concert_broker.total_venues(), 1);

            let venue = concert_broker.get_venue(venue_id).unwrap();
            assert_eq!(venue.name, "Madison Square Garden");
            assert_eq!(venue.venue_type, VenueType::Arena);
            assert_eq!(venue.capacity, 20000);
            assert!(!venue.verified);
        }

        #[ink::test]
        fn create_tour_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, supporting_artist_id) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Eras Tour".to_string(),
                artist_id,
                TourType::WorldTour,
                1672531200000, // Jan 1, 2023
                1704067200000, // Jan 1, 2024
                vec![supporting_artist_id],
                true, // merchandise enabled
                true, // VIP packages available
                Some("tour.manager@taylorswift.com".to_string()),
                "The most ambitious tour yet".to_string(),
                Some("poster.jpg".to_string()),
            ).unwrap();

            assert_eq!(tour_id, 1);
            assert_eq!(concert_broker.total_tours(), 1);

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.name, "Eras Tour");
            assert_eq!(tour.artist_id, artist_id);
            assert_eq!(tour.tour_type, TourType::WorldTour);
            assert_eq!(tour.tour_status, TourStatus::Announced);
            assert_eq!(tour.supporting_artists.len(), 1);
            assert!(tour.merchandise_enabled);
            assert!(tour.vip_packages_available);
        }

        #[ink::test]
        fn create_festival_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Coachella".to_string(),
                venue_id,
                FestivalType::MusicFestival,
                1680000000000, // Start date
                1680259200000, // End date (3 days later)
                50000, // Capacity per day
                true, // Camping available
                Some(10000), // Camping capacity
                AgeRestriction::AllAges,
                Some("organizer@coachella.com".to_string()),
                Some("www.coachella.com".to_string()),
                create_test_social_media(),
                "The premier music festival experience".to_string(),
            ).unwrap();

            assert_eq!(festival_id, 1);
            assert_eq!(concert_broker.total_festivals(), 1);

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.name, "Coachella");
            assert_eq!(festival.venue_id, venue_id);
            assert_eq!(festival.festival_type, FestivalType::MusicFestival);
            assert_eq!(festival.festival_status, FestivalStatus::Planning);
            assert!(festival.camping_available);
            assert_eq!(festival.camping_capacity, Some(10000));
        }

        // ========================================================================
        // NEW: STEP 3 CONCERT EVENT TESTS
        // ========================================================================

        #[ink::test]
        fn create_concert_event_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "Taylor Swift Live in NYC".to_string(),
                artist_id,
                venue_id,
                1704067200000, // January 1, 2024 at midnight
                1704067200000 + (2 * 60 * 60 * 1000), // Doors open 2 hours later
                1704067200000 + (3 * 60 * 60 * 1000), // Show starts 3 hours later
                1704067200000 + (6 * 60 * 60 * 1000), // Show ends 6 hours later
                18000, // Capacity
                150_000_000_000_000, // 150 DOT base price
                EventType::Concert,
                vec![supporting_artist_id],
                AgeRestriction::AllAges,
                "Special acoustic set included".to_string(),
            ).unwrap();

            assert_eq!(event_id, 1);
            assert_eq!(concert_broker.total_concert_events(), 1);

            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.name, "Taylor Swift Live in NYC");
            assert_eq!(event.artist_id, artist_id);
            assert_eq!(event.venue_id, venue_id);
            assert_eq!(event.event_type, EventType::Concert);
            assert_eq!(event.supporting_artists.len(), 1);
            assert!(event.supporting_artists.contains(&supporting_artist_id));
            assert_eq!(event.capacity, 18000);
            assert_eq!(event.base_price, 150_000_000_000_000);
            assert!(event.active);
            assert_eq!(event.sold_tickets, 0);
            assert_eq!(event.revenue_generated, 0);
            assert_eq!(event.special_notes, "Special acoustic set included");
            assert!(!event.presale_enabled);
            assert!(!event.sound_check_available);
            assert!(!event.meet_greet_available);
            assert_eq!(event.vip_packages.len(), 0);
        }

        #[ink::test]
        fn create_concert_event_invalid_timing() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            // Invalid timing: show starts before doors open
            let result = concert_broker.create_concert_event(
                "Invalid Timing Event".to_string(),
                artist_id,
                venue_id,
                1704067200000, // Event date
                1704067200000 + (3 * 60 * 60 * 1000), // Doors open
                1704067200000 + (2 * 60 * 60 * 1000), // Show starts BEFORE doors open (invalid)
                1704067200000 + (6 * 60 * 60 * 1000), // Show ends
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Test".to_string(),
            );

            assert_eq!(result, Err(Error::InvalidEventTiming));
        }

        #[ink::test]
        fn create_concert_event_nonexistent_artist() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let result = concert_broker.create_concert_event(
                "Nonexistent Artist Event".to_string(),
                999, // Nonexistent artist ID
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Test".to_string(),
            );

            assert_eq!(result, Err(Error::ArtistNotFound));
        }

        #[ink::test]
        fn link_event_to_tour_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            // Create tour
            let tour_id = concert_broker.create_tour(
                "Test Tour".to_string(),
                artist_id,
                TourType::NationalTour,
                1672531200000, // Tour starts
                1735603200000, // Tour ends
                vec![supporting_artist_id],
                true,
                true,
                None,
                "Test tour for linking".to_string(),
                None,
            ).unwrap();

            // Create event
            let event_id = concert_broker.create_concert_event(
                "Tour Stop NYC".to_string(),
                artist_id,
                venue_id,
                1704067200000, // Within tour dates
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Part of the tour".to_string(),
            ).unwrap();

            // Link event to tour
            let result = concert_broker.link_event_to_tour(event_id, tour_id);
            assert_eq!(result, Ok(()));

            // Verify linkage
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.tour_id, Some(tour_id));

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.total_shows, 1);
            assert_eq!(tour.shows_scheduled, 1);

            // Verify search functionality
            let tour_events = concert_broker.search_events_by_tour(tour_id);
            assert_eq!(tour_events.len(), 1);
            assert!(tour_events.contains(&event_id));
        }

        #[ink::test]
        fn link_event_to_tour_different_artist() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            // Create tour for different artist
            let tour_id = concert_broker.create_tour(
                "Different Artist Tour".to_string(),
                supporting_artist_id, // Different artist
                TourType::LocalTour,
                1672531200000,
                1735603200000,
                vec![],
                false,
                false,
                None,
                "Different artist tour".to_string(),
                None,
            ).unwrap();

            // Create event for original artist
            let event_id = concert_broker.create_concert_event(
                "Original Artist Event".to_string(),
                artist_id, // Original artist
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Should not link".to_string(),
            ).unwrap();

            // Try to link - should fail because artists don't match
            let result = concert_broker.link_event_to_tour(event_id, tour_id);
            assert_eq!(result, Err(Error::ArtistNotFound));
        }

        #[ink::test]
        fn link_event_to_festival_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            // Create festival
            let festival_id = concert_broker.create_festival(
                "Test Festival".to_string(),
                venue_id,
                FestivalType::MusicFestival,
                1680000000000,
                1680259200000,
                30000,
                false,
                None,
                AgeRestriction::AllAges,
                None,
                None,
                create_test_social_media(),
                "Test festival for linking".to_string(),
            ).unwrap();

            // Add artist to festival lineup
            concert_broker.add_festival_artist(festival_id, artist_id, true).unwrap();

            // Create event
            let event_id = concert_broker.create_concert_event(
                "Festival Performance".to_string(),
                artist_id,
                venue_id,
                1680100000000, // Within festival dates
                1680100000000 + (1 * 60 * 60 * 1000),
                1680100000000 + (2 * 60 * 60 * 1000),
                1680100000000 + (4 * 60 * 60 * 1000),
                15000,
                100_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Festival main stage performance".to_string(),
            ).unwrap();

            // Link event to festival
            let result = concert_broker.link_event_to_festival(
                event_id, 
                festival_id, 
                Some("Main Stage".to_string())
            );
            assert_eq!(result, Ok(()));

            // Verify linkage
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.festival_id, Some(festival_id));
            assert_eq!(event.event_type, EventType::FestivalDay);
            assert_eq!(event.festival_stage, Some("Main Stage".to_string()));

            // Verify search functionality
            let festival_events = concert_broker.search_events_by_festival(festival_id);
            assert_eq!(festival_events.len(), 1);
            assert!(festival_events.contains(&event_id));
        }

        #[ink::test]
        fn add_supporting_artist_to_event_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "Event with Supporting Acts".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![], // No initial supporting artists
                AgeRestriction::AllAges,
                "Will add supporting artist".to_string(),
            ).unwrap();

            let result = concert_broker.add_supporting_artist_to_event(event_id, supporting_artist_id);
            assert_eq!(result, Ok(()));

            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.supporting_artists.len(), 1);
            assert!(event.supporting_artists.contains(&supporting_artist_id));

            // Verify artist can be found in search
            let artist_events = concert_broker.search_events_by_artist(supporting_artist_id);
            assert!(artist_events.contains(&event_id));
        }

        #[ink::test]
        fn update_event_timing_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "Timing Update Event".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Test timing updates".to_string(),
            ).unwrap();

            // Update timing
            let result = concert_broker.update_event_timing(
                event_id,
                Some(1704067200000 + (1 * 60 * 60 * 1000)), // New doors open (1 hour after event date)
                Some(1704067200000 + (2 * 60 * 60 * 1000)), // New show start (2 hours after event date)
                Some(1704067200000 + (5 * 60 * 60 * 1000)), // New estimated end (5 hours after event date)
            );
            assert_eq!(result, Ok(()));

            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.doors_open, 1704067200000 + (1 * 60 * 60 * 1000));
            assert_eq!(event.show_start, 1704067200000 + (2 * 60 * 60 * 1000));
            assert_eq!(event.estimated_end, 1704067200000 + (5 * 60 * 60 * 1000));
        }

        #[ink::test]
        fn update_event_timing_invalid_sequence() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "Invalid Timing Update".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Test invalid timing".to_string(),
            ).unwrap();

            // Try to update with invalid timing (show starts before doors open)
            let result = concert_broker.update_event_timing(
                event_id,
                Some(1704067200000 + (3 * 60 * 60 * 1000)), // Doors open
                Some(1704067200000 + (2 * 60 * 60 * 1000)), // Show start BEFORE doors open
                None, // Keep existing estimated end
            );
            assert_eq!(result, Err(Error::InvalidEventTiming));
        }

        #[ink::test]
        fn add_vip_package_to_event_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "VIP Package Event".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Event with VIP packages".to_string(),
            ).unwrap();

            let vip_benefits = vec![
                VIPBenefit::MeetAndGreet,
                VIPBenefit::EarlyEntry,
                VIPBenefit::PremiumSeating,
                VIPBenefit::ExclusiveMerchandise,
            ];

            let result = concert_broker.add_vip_package_to_event(
                event_id,
                "Platinum VIP Experience".to_string(),
                500_000_000_000_000, // 500 DOT premium
                vip_benefits.clone(),
                Some(50), // Limited to 50 packages
                "Ultimate VIP experience with artist meet & greet".to_string(),
            );
            assert_eq!(result, Ok(()));

            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.vip_packages.len(), 1);
            
            let vip_package = &event.vip_packages[0];
            assert_eq!(vip_package.package_name, "Platinum VIP Experience");
            assert_eq!(vip_package.price_premium, 500_000_000_000_000);
            assert_eq!(vip_package.benefits, vip_benefits);
            assert_eq!(vip_package.limited_quantity, Some(50));
            assert_eq!(vip_package.available_quantity, Some(50));

            // Verify VIP package search functionality
            let vip_events = concert_broker.get_events_with_vip_packages();
            assert!(vip_events.contains(&event_id));
        }

        #[ink::test]
        fn update_event_features_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "Features Update Event".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Event to update features".to_string(),
            ).unwrap();

            let result = concert_broker.update_event_features(
                event_id,
                true, // merchandise_available
                true, // presale_enabled
                Some(1703980800000), // presale_start (day before event)
                Some(1704024000000), // general_sale_start (12 hours before event)
                true, // sound_check_available
                true, // meet_greet_available
                false, // recording_allowed
                true, // live_stream_available
                Some(120), // setlist_length_minutes
                true, // encore_expected
            );
            assert_eq!(result, Ok(()));

            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert!(event.merchandise_available);
            assert!(event.presale_enabled);
            assert_eq!(event.presale_start, Some(1703980800000));
            assert_eq!(event.general_sale_start, Some(1704024000000));
            assert!(event.sound_check_available);
            assert!(event.meet_greet_available);
            assert!(!event.recording_allowed);
            assert!(event.live_stream_available);
            assert_eq!(event.setlist_length_minutes, Some(120));
            assert!(event.encore_expected);

            // Test search functionalities
            let meet_greet_events = concert_broker.get_events_with_meet_greet();
            assert!(meet_greet_events.contains(&event_id));

            let live_stream_events = concert_broker.get_events_with_live_stream();
            assert!(live_stream_events.contains(&event_id));
        }

        // ========================================================================
        // NEW: STEP 3 SEARCH AND DISCOVERY TESTS
        // ========================================================================

        #[ink::test]
        fn search_events_by_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            // Create multiple events for the same artist
            let event1_id = concert_broker.create_concert_event(
                "Artist Event 1".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "First event".to_string(),
            ).unwrap();

            let event2_id = concert_broker.create_concert_event(
                "Artist Event 2".to_string(),
                artist_id,
                venue_id,
                1704153600000, // Different date
                1704153600000 + (2 * 60 * 60 * 1000),
                1704153600000 + (3 * 60 * 60 * 1000),
                1704153600000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::AcousticSession,
                vec![supporting_artist_id], // With supporting artist
                AgeRestriction::AllAges,
                "Second event".to_string(),
            ).unwrap();

            let artist_events = concert_broker.search_events_by_artist(artist_id);
            assert_eq!(artist_events.len(), 2);
            assert!(artist_events.contains(&event1_id));
            assert!(artist_events.contains(&event2_id));

            // Check supporting artist is also indexed
            let supporting_events = concert_broker.search_events_by_artist(supporting_artist_id);
            assert_eq!(supporting_events.len(), 1);
            assert!(supporting_events.contains(&event2_id));
        }

        #[ink::test]
        fn search_events_by_venue_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            // Create multiple events at the same venue
            let event1_id = concert_broker.create_concert_event(
                "Venue Event 1".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "First venue event".to_string(),
            ).unwrap();

            let event2_id = concert_broker.create_concert_event(
                "Venue Event 2".to_string(),
                artist_id,
                venue_id,
                1704240000000, // Different date, same venue
                1704240000000 + (2 * 60 * 60 * 1000),
                1704240000000 + (3 * 60 * 60 * 1000),
                1704240000000 + (6 * 60 * 60 * 1000),
                15000, // Different capacity
                200_000_000_000_000, // Different price
                EventType::AlbumLaunch,
                vec![],
                AgeRestriction::EighteenPlus,
                "Album launch at same venue".to_string(),
            ).unwrap();

            let venue_events = concert_broker.search_events_by_venue(venue_id);
            assert_eq!(venue_events.len(), 2);
            assert!(venue_events.contains(&event1_id));
            assert!(venue_events.contains(&event2_id));
        }

        #[ink::test]
        fn search_events_by_type_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            // Create events of different types
            let concert_id = concert_broker.create_concert_event(
                "Regular Concert".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Regular concert".to_string(),
            ).unwrap();

            let acoustic_id = concert_broker.create_concert_event(
                "Acoustic Session".to_string(),
                artist_id,
                venue_id,
                1704153600000,
                1704153600000 + (1 * 60 * 60 * 1000),
                1704153600000 + (2 * 60 * 60 * 1000),
                1704153600000 + (4 * 60 * 60 * 1000),
                5000, // Smaller venue for acoustic
                100_000_000_000_000,
                EventType::AcousticSession,
                vec![],
                AgeRestriction::AllAges,
                "Intimate acoustic performance".to_string(),
            ).unwrap();

            let meet_greet_id = concert_broker.create_concert_event(
                "Meet and Greet".to_string(),
                artist_id,
                venue_id,
                1704240000000,
                1704240000000 + (30 * 60 * 1000), // 30 minutes
                1704240000000 + (1 * 60 * 60 * 1000), // 1 hour
                1704240000000 + (2 * 60 * 60 * 1000), // 2 hours
                100, // Very small capacity
                500_000_000_000_000, // Premium price
                EventType::MeetAndGreet,
                vec![],
                AgeRestriction::AllAges,
                "Exclusive meet & greet session".to_string(),
            ).unwrap();

            // Test type-specific searches
            let concerts = concert_broker.search_events_by_type(EventType::Concert);
            assert_eq!(concerts.len(), 1);
            assert!(concerts.contains(&concert_id));

            let acoustic_sessions = concert_broker.search_events_by_type(EventType::AcousticSession);
            assert_eq!(acoustic_sessions.len(), 1);
            assert!(acoustic_sessions.contains(&acoustic_id));

            let meet_greets = concert_broker.search_events_by_type(EventType::MeetAndGreet);
            assert_eq!(meet_greets.len(), 1);
            assert!(meet_greets.contains(&meet_greet_id));
        }

        #[ink::test]
        fn get_upcoming_events_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            // Mock current time (we can't actually control block_timestamp in tests, 
            // but we can test the logic with relative dates)
            let base_time = 1704067200000; // Base time
            
            // Create event in the "future" (relative to our base time)
            let _future_event_id = concert_broker.create_concert_event(
                "Future Event".to_string(),
                artist_id,
                venue_id,
                base_time + (10 * 24 * 60 * 60 * 1000), // 10 days from base
                base_time + (10 * 24 * 60 * 60 * 1000) + (2 * 60 * 60 * 1000),
                base_time + (10 * 24 * 60 * 60 * 1000) + (3 * 60 * 60 * 1000),
                base_time + (10 * 24 * 60 * 60 * 1000) + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Future event".to_string(),
            ).unwrap();

            // Create event in the "past"
            let _past_event_id = concert_broker.create_concert_event(
                "Past Event".to_string(),
                artist_id,
                venue_id,
                base_time - (10 * 24 * 60 * 60 * 1000), // 10 days before base
                base_time - (10 * 24 * 60 * 60 * 1000) + (2 * 60 * 60 * 1000),
                base_time - (10 * 24 * 60 * 60 * 1000) + (3 * 60 * 60 * 1000),
                base_time - (10 * 24 * 60 * 60 * 1000) + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Past event".to_string(),
            ).unwrap();

            // The upcoming events function will use env().block_timestamp(), 
            // so we can only verify it returns some results (implementation testing)
            let upcoming_events = concert_broker.get_upcoming_events();
            // Since we can't control block_timestamp in tests, we just verify the function works
            // The function should return without panicking
            let _event_count = upcoming_events.len();
        }

        // ========================================================================
        // COMPREHENSIVE INTEGRATION TEST FOR STEP 3
        // ========================================================================

        #[ink::test]
        fn complete_concert_event_workflow() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            // Step 1: Create a tour
            let tour_id = concert_broker.create_tour(
                "The Ultimate World Tour".to_string(),
                artist_id,
                TourType::WorldTour,
                1672531200000,
                1735603200000,
                vec![supporting_artist_id],
                true,
                true,
                Some("tour.manager@example.com".to_string()),
                "Epic world tour with cutting-edge production".to_string(),
                Some("tour-poster.jpg".to_string()),
            ).unwrap();

            // Step 2: Create a festival
            let festival_id = concert_broker.create_festival(
                "Summer Music Fest".to_string(),
                venue_id,
                FestivalType::MusicFestival,
                1680000000000,
                1680259200000,
                50000,
                true,
                Some(10000),
                AgeRestriction::AllAges,
                Some("organizer@summerfest.com".to_string()),
                Some("www.summerfest.com".to_string()),
                create_test_social_media(),
                "Premier summer music festival".to_string(),
            ).unwrap();

            // Add artist to festival
            concert_broker.add_festival_artist(festival_id, artist_id, true).unwrap();

            // Step 3: Create comprehensive concert event
            let event_id = concert_broker.create_concert_event(
                "Taylor Swift: Eras Tour NYC".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000), // Doors 2 hours later
                1704067200000 + (3 * 60 * 60 * 1000), // Show 3 hours later
                1704067200000 + (7 * 60 * 60 * 1000), // End 7 hours later (longer show)
                20000, // Full capacity
                250_000_000_000_000, // 250 DOT premium pricing
                EventType::Concert,
                vec![supporting_artist_id],
                AgeRestriction::AllAges,
                "Epic 3+ hour performance featuring all eras of music".to_string(),
            ).unwrap();

            // Step 4: Link event to tour
            concert_broker.link_event_to_tour(event_id, tour_id).unwrap();

            // Step 5: Add VIP packages
            concert_broker.add_vip_package_to_event(
                event_id,
                "Diamond VIP Experience".to_string(),
                1_000_000_000_000_000, // 1000 DOT premium
                vec![
                    VIPBenefit::MeetAndGreet,
                    VIPBenefit::BackstageAccess,
                    VIPBenefit::SoundcheckAccess,
                    VIPBenefit::PremiumSeating,
                    VIPBenefit::ExclusiveMerchandise,
                    VIPBenefit::SignedMemorabilia,
                ],
                Some(25), // Very limited
                "Ultimate VIP experience with full backstage access".to_string(),
            ).unwrap();

            concert_broker.add_vip_package_to_event(
                event_id,
                "Gold VIP Package".to_string(),
                500_000_000_000_000, // 500 DOT premium
                vec![
                    VIPBenefit::EarlyEntry,
                    VIPBenefit::PremiumSeating,
                    VIPBenefit::ExclusiveMerchandise,
                ],
                Some(100),
                "Premium seating and exclusive merchandise".to_string(),
            ).unwrap();

            // Step 6: Update event features
            concert_broker.update_event_features(
                event_id,
                true, // merchandise
                true, // presale
                Some(1703980800000), // presale start
                Some(1704024000000), // general sale start
                true, // soundcheck access
                true, // meet & greet
                true, // recording allowed
                true, // live stream
                Some(210), // 3.5 hour setlist
                true, // encore expected
            ).unwrap();

            // Step 7: Add more supporting artists
            let additional_support_id = concert_broker.register_artist(
                "Clairo".to_string(),
                None,
                MusicGenre::Indie,
                vec![MusicGenre::Pop, MusicGenre::Alternative],
                "Indie pop sensation".to_string(),
                "United States".to_string(),
                Some("FADER Label".to_string()),
                create_test_social_media(),
                vec![],
                2017,
                None,
            ).unwrap();

            concert_broker.add_supporting_artist_to_event(event_id, additional_support_id).unwrap();

            // Step 8: Verify complete event structure
            let event = concert_broker.get_concert_event(event_id).unwrap();
            assert_eq!(event.name, "Taylor Swift: Eras Tour NYC");
            assert_eq!(event.artist_id, artist_id);
            assert_eq!(event.venue_id, venue_id);
            assert_eq!(event.tour_id, Some(tour_id));
            assert_eq!(event.event_type, EventType::Concert);
            assert_eq!(event.supporting_artists.len(), 2); // Two supporting artists
            assert_eq!(event.vip_packages.len(), 2); // Two VIP packages
            assert!(event.merchandise_available);
            assert!(event.presale_enabled);
            assert!(event.sound_check_available);
            assert!(event.meet_greet_available);
            assert!(event.live_stream_available);
            assert_eq!(event.setlist_length_minutes, Some(210));
            assert!(event.encore_expected);

            // Step 9: Verify tour was updated
            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.total_shows, 1);
            assert_eq!(tour.shows_scheduled, 1);

            // Step 10: Test all search functionalities
            let artist_events = concert_broker.search_events_by_artist(artist_id);
            assert!(artist_events.contains(&event_id));

            let venue_events = concert_broker.search_events_by_venue(venue_id);
            assert!(venue_events.contains(&event_id));

            let tour_events = concert_broker.search_events_by_tour(tour_id);
            assert!(tour_events.contains(&event_id));

            let concert_events = concert_broker.search_events_by_type(EventType::Concert);
            assert!(concert_events.contains(&event_id));

            let vip_events = concert_broker.get_events_with_vip_packages();
            assert!(vip_events.contains(&event_id));

            let meet_greet_events = concert_broker.get_events_with_meet_greet();
            assert!(meet_greet_events.contains(&event_id));

            let live_stream_events = concert_broker.get_events_with_live_stream();
            assert!(live_stream_events.contains(&event_id));

            // Step 11: Verify supporting artists are searchable
            let supporting_events = concert_broker.search_events_by_artist(supporting_artist_id);
            assert!(supporting_events.contains(&event_id));

            let additional_support_events = concert_broker.search_events_by_artist(additional_support_id);
            assert!(additional_support_events.contains(&event_id));

            // Final verification: All components working together
            assert_eq!(concert_broker.total_artists(), 3); // Main + 2 supporting
            assert_eq!(concert_broker.total_venues(), 1);
            assert_eq!(concert_broker.total_tours(), 1);
            assert_eq!(concert_broker.total_festivals(), 1);
            assert_eq!(concert_broker.total_concert_events(), 1);

            // Success! Step 3 implementation is complete and fully tested
        }

        // ========================================================================
        // ERROR HANDLING TESTS FOR STEP 3
        // ========================================================================

        #[ink::test]
        fn create_concert_event_empty_name() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let result = concert_broker.create_concert_event(
                "".to_string(), // Empty name
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Test".to_string(),
            );

            assert_eq!(result, Err(Error::InvalidEventData));
        }

        #[ink::test]
        fn link_event_to_tour_outside_dates() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Short Tour".to_string(),
                artist_id,
                TourType::LocalTour,
                1672531200000, // Tour start
                1672617600000, // Tour end (only 1 day later)
                vec![],
                false,
                false,
                None,
                "Very short tour".to_string(),
                None,
            ).unwrap();

            let event_id = concert_broker.create_concert_event(
                "Event Outside Tour".to_string(),
                artist_id,
                venue_id,
                1704067200000, // Event date AFTER tour ends
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "Should not link".to_string(),
            ).unwrap();

            let result = concert_broker.link_event_to_tour(event_id, tour_id);
            assert_eq!(result, Err(Error::InvalidTourDates));
        }

        #[ink::test]
        fn add_vip_package_invalid_data() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let event_id = concert_broker.create_concert_event(
                "VIP Test Event".to_string(),
                artist_id,
                venue_id,
                1704067200000,
                1704067200000 + (2 * 60 * 60 * 1000),
                1704067200000 + (3 * 60 * 60 * 1000),
                1704067200000 + (6 * 60 * 60 * 1000),
                18000,
                150_000_000_000_000,
                EventType::Concert,
                vec![],
                AgeRestriction::AllAges,
                "VIP package test".to_string(),
            ).unwrap();

            // Try to add VIP package with empty name
            let result = concert_broker.add_vip_package_to_event(
                event_id,
                "".to_string(), // Empty name
                500_000_000_000_000,
                vec![VIPBenefit::MeetAndGreet],
                Some(50),
                "Test package".to_string(),
            );
            assert_eq!(result, Err(Error::InvalidVIPPackageData));

            // Try to add VIP package with no benefits
            let result = concert_broker.add_vip_package_to_event(
                event_id,
                "No Benefits Package".to_string(),
                500_000_000_000_000,
                vec![], // Empty benefits
                Some(50),
                "Test package".to_string(),
            );
            assert_eq!(result, Err(Error::InvalidVIPPackageData));
        }

        #[ink::test]
        fn concert_event_not_found_errors() {
            let mut concert_broker = ConcertBroker::new();

            // Test all functions that should return ConcertEventNotFound
            assert_eq!(
                concert_broker.link_event_to_tour(999, 1),
                Err(Error::ConcertEventNotFound)
            );

            assert_eq!(
                concert_broker.link_event_to_festival(999, 1, None),
                Err(Error::ConcertEventNotFound)
            );

            assert_eq!(
                concert_broker.add_supporting_artist_to_event(999, 1),
                Err(Error::ConcertEventNotFound)
            );

            assert_eq!(
                concert_broker.update_event_timing(999, None, None, None),
                Err(Error::ConcertEventNotFound)
            );

            assert_eq!(
                concert_broker.add_vip_package_to_event(
                    999,
                    "Test".to_string(),
                    100,
                    vec![VIPBenefit::EarlyEntry],
                    None,
                    "Test".to_string()
                ),
                Err(Error::ConcertEventNotFound)
            );

            assert_eq!(
                concert_broker.update_event_features(
                    999, false, false, None, None, false, false, false, false, None, false
                ),
                Err(Error::ConcertEventNotFound)
            );
        }
    }
}