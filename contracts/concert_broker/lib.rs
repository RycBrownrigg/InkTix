#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # InkTix Concert Broker - Enhanced Artist, Venue, Tour and Festival Management (Steps 1-2)
/// 
/// This contract now includes comprehensive tour and festival management capabilities
/// building on the solid foundation of artist and venue management from Step 1.
///
/// ## Step 2 New Features:
/// - **Tour Management**: Multi-date tour coordination with comprehensive metadata
/// - **Festival Management**: Multi-artist festival planning with lineup coordination
/// - **Event Type Classification**: Concerts, festivals, meet & greets, private events
/// - **Tour Search & Discovery**: Find tours by artist, type, and status
/// - **Festival Lineup Management**: Add/manage artists and logistics
/// - **Integration Ready**: Prepared for merchandise and VIP package integration

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
        
        // NEW: Tour Management (Step 2)
        tours: ink::storage::Mapping<u32, Tour>,
        next_tour_id: u32,
        
        // NEW: Festival Management (Step 2)
        festivals: ink::storage::Mapping<u32, Festival>,
        next_festival_id: u32,
        
        // Artist indexing for search (Step 1)
        artists_by_genre: ink::storage::Mapping<u32, Vec<u32>>, // genre_hash -> artist_ids
        verified_artists: ink::storage::Mapping<bool, Vec<u32>>, // verified status -> artist_ids
        
        // Venue indexing for search (Step 1)
        venues_by_type: ink::storage::Mapping<u32, Vec<u32>>, // venue_type_hash -> venue_ids
        venues_by_city: ink::storage::Mapping<u32, Vec<u32>>, // city_hash -> venue_ids
        
        // NEW: Tour indexing for search (Step 2)
        tours_by_artist: ink::storage::Mapping<u32, Vec<u32>>, // artist_id -> tour_ids
        tours_by_type: ink::storage::Mapping<u32, Vec<u32>>, // tour_type_hash -> tour_ids
        active_tours: ink::storage::Mapping<bool, Vec<u32>>, // active status -> tour_ids
        
        // NEW: Festival indexing for search (Step 2)
        festivals_by_type: ink::storage::Mapping<u32, Vec<u32>>, // festival_type_hash -> festival_ids
        festivals_by_venue: ink::storage::Mapping<u32, Vec<u32>>, // venue_id -> festival_ids
        
        // Simple event storage (will be enhanced in Step 3)
        events: ink::storage::Mapping<u32, Option<String>>,
        next_event_id: u32,
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

    /// NEW: Tour structure for multi-date tour management (Step 2)
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

    /// NEW: Festival structure for multi-artist event management (Step 2)
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

    /// NEW: Stage configuration for festivals (Step 2)
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

    /// NEW: Tour types for different touring scales (Step 2)
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

    /// NEW: Tour status tracking (Step 2)
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

    /// NEW: Festival types for different festival categories (Step 2)
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

    /// NEW: Festival status tracking (Step 2)
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

    /// NEW: Festival ticket types (Step 2)
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

    /// NEW: Stage types for festivals (Step 2)
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

    /// NEW: Sustainability features for eco-friendly festivals (Step 2)
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
        /// Tour not found (NEW)
        TourNotFound,
        /// Festival not found (NEW)
        FestivalNotFound,
        /// ID overflow
        IdOverflow,
        /// Invalid artist data
        InvalidArtistData,
        /// Invalid venue data
        InvalidVenueData,
        /// Invalid tour data (NEW)
        InvalidTourData,
        /// Invalid festival data (NEW)
        InvalidFestivalData,
        /// Artist already verified
        ArtistAlreadyVerified,
        /// Venue already verified
        VenueAlreadyVerified,
        /// Empty search results
        NoSearchResults,
        /// Tour already active (NEW)
        TourAlreadyActive,
        /// Festival already active (NEW)
        FestivalAlreadyActive,
        /// Invalid tour dates (NEW)
        InvalidTourDates,
        /// Invalid festival dates (NEW)
        InvalidFestivalDates,
        /// Maximum supporting artists reached (NEW)
        MaxSupportingArtistsReached,
        /// Artist not available for dates (NEW)
        ArtistNotAvailable,
        /// Venue not available for dates (NEW)
        VenueNotAvailable,
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
                artists_by_genre: ink::storage::Mapping::new(),
                verified_artists: ink::storage::Mapping::new(),
                venues_by_type: ink::storage::Mapping::new(),
                venues_by_city: ink::storage::Mapping::new(),
                tours_by_artist: ink::storage::Mapping::new(),
                tours_by_type: ink::storage::Mapping::new(),
                active_tours: ink::storage::Mapping::new(),
                festivals_by_type: ink::storage::Mapping::new(),
                festivals_by_venue: ink::storage::Mapping::new(),
                events: ink::storage::Mapping::new(),
                next_event_id: 1,
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
        // NEW: TOUR MANAGEMENT (Step 2)
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

            Ok(tour_id)
        }

        /// Add a new show date to an existing tour
        #[ink(message)]
        pub fn add_tour_date(
            &mut self,
            tour_id: u32,
            venue_id: u32,
            show_date: u64,
            show_name: Option<String>,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let mut tour = self.tours.get(tour_id).ok_or(Error::TourNotFound)?;
            let _venue = self.venues.get(venue_id).ok_or(Error::VenueNotFound)?;

            // Validate show date is within tour dates
            if show_date < tour.start_date || show_date > tour.end_date {
                return Err(Error::InvalidTourDates);
            }

            // For now, we'll create a simple event entry
            // This will be enhanced in Step 3 with full ConcertEvent structure
            let event_id = self.next_event_id;
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;

            let event_name = show_name.unwrap_or_else(|| format!("{} - {}", tour.name, venue_id));
            self.events.insert(event_id, &Some(event_name));

            // Update tour statistics
            tour.total_shows = tour.total_shows.saturating_add(1);
            tour.shows_scheduled = tour.shows_scheduled.saturating_add(1);
            tour.last_updated = self.env().block_timestamp();

            self.tours.insert(tour_id, &tour);

            Ok(event_id)
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
        // NEW: FESTIVAL MANAGEMENT (Step 2)
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

        /// NEW: Search tours by artist (Step 2)
        #[ink(message)]
        pub fn get_tours_by_artist(&self, artist_id: u32) -> Vec<u32> {
            self.tours_by_artist.get(artist_id).unwrap_or_default()
        }

        /// NEW: Search tours by type (Step 2)
        #[ink(message)]
        pub fn search_tours_by_type(&self, tour_type: TourType) -> Vec<u32> {
            let type_hash = self.hash_tour_type(tour_type);
            self.tours_by_type.get(type_hash).unwrap_or_default()
        }

        /// NEW: Get active tours (Step 2)
        #[ink(message)]
        pub fn get_active_tours(&self) -> Vec<u32> {
            self.active_tours.get(true).unwrap_or_default()
        }

        /// NEW: Get upcoming tours (starting within next 90 days) (Step 2)
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

        /// NEW: Search festivals by type (Step 2)
        #[ink(message)]
        pub fn search_festivals_by_type(&self, festival_type: FestivalType) -> Vec<u32> {
            let type_hash = self.hash_festival_type(festival_type);
            self.festivals_by_type.get(type_hash).unwrap_or_default()
        }

        /// NEW: Search festivals by venue (Step 2)
        #[ink(message)]
        pub fn search_festivals_by_venue(&self, venue_id: u32) -> Vec<u32> {
            self.festivals_by_venue.get(venue_id).unwrap_or_default()
        }

        /// NEW: Get upcoming festivals (Step 2)
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

        /// NEW: Search festivals by artist (Step 2)
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

        // NEW: Tour indexing functions (Step 2)
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

        // NEW: Festival indexing functions (Step 2)
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

        // NEW: Tour type hash function (Step 2)
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

        // NEW: Festival type hash function (Step 2)
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
        // BASIC EVENT MANAGEMENT (Placeholder for Step 3)
        // ========================================================================

        /// Create a basic event (will be enhanced in Step 3)
        #[ink(message)]
        pub fn create_event(
            &mut self, 
            name: String,
            artist_id: u32
        ) -> Result<u32> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Check if artist exists
            if self.artists.get(artist_id).is_none() {
                return Err(Error::ArtistNotFound);
            }

            let event_id = self.next_event_id;
            self.events.insert(event_id, &Some(name));
            
            self.next_event_id = self.next_event_id
                .checked_add(1)
                .ok_or(Error::IdOverflow)?;
            
            Ok(event_id)
        }

        // ========================================================================
        // QUERY FUNCTIONS (Steps 1-2)
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

        /// NEW: Get tour details (Step 2)
        #[ink(message)]
        pub fn get_tour(&self, tour_id: u32) -> Option<Tour> {
            self.tours.get(tour_id)
        }

        /// NEW: Get festival details (Step 2)
        #[ink(message)]
        pub fn get_festival(&self, festival_id: u32) -> Option<Festival> {
            self.festivals.get(festival_id)
        }

        /// Get event name (placeholder)
        #[ink(message)]
        pub fn get_event(&self, event_id: u32) -> Option<String> {
            self.events.get(event_id).unwrap_or(None)
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

        /// NEW: Get total tours created (Step 2)
        #[ink(message)]
        pub fn total_tours(&self) -> u32 {
            self.next_tour_id.saturating_sub(1)
        }

        /// NEW: Get total festivals created (Step 2)
        #[ink(message)]
        pub fn total_festivals(&self) -> u32 {
            self.next_festival_id.saturating_sub(1)
        }

        /// Get total events created
        #[ink(message)]
        pub fn total_events(&self) -> u32 {
            self.next_event_id.saturating_sub(1)
        }
    }

    /// Add Default implementation
    impl Default for ConcertBroker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Comprehensive test suite for Steps 1-2
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
        // STEP 1 TESTS (Artist and Venue Management)
        // ========================================================================

        #[ink::test]
        fn new_works() {
            let concert_broker = ConcertBroker::new();
            assert_eq!(concert_broker.total_artists(), 0);
            assert_eq!(concert_broker.total_venues(), 0);
            assert_eq!(concert_broker.total_tours(), 0);
            assert_eq!(concert_broker.total_festivals(), 0);
            assert_eq!(concert_broker.total_events(), 0);
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
        fn verify_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let result = concert_broker.verify_artist(artist_id);
            assert_eq!(result, Ok(()));

            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert!(artist.verified);

            let verified_artists = concert_broker.get_verified_artists();
            assert!(verified_artists.contains(&artist_id));
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
        fn search_artists_by_genre_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, _) = setup_test_data(&mut concert_broker);

            let pop_artists = concert_broker.search_artists_by_genre(MusicGenre::Pop);
            assert_eq!(pop_artists.len(), 1);
            assert!(pop_artists.contains(&1)); // Taylor Swift

            let indie_artists = concert_broker.search_artists_by_genre(MusicGenre::Indie);
            assert_eq!(indie_artists.len(), 1);
            assert!(indie_artists.contains(&2)); // Phoebe Bridgers

            // Test sub-genre search
            let folk_artists = concert_broker.search_artists_by_genre(MusicGenre::Folk);
            assert_eq!(folk_artists.len(), 2); // Both artists have Folk as sub-genre
        }

        #[ink::test]
        fn search_venues_by_type_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, _, _) = setup_test_data(&mut concert_broker);

            let arenas = concert_broker.search_venues_by_type(VenueType::Arena);
            assert_eq!(arenas.len(), 1);
            assert!(arenas.contains(&1)); // Madison Square Garden
        }

        // ========================================================================
        // STEP 2 TESTS (Tour and Festival Management)
        // ========================================================================

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

            // Check that artist's touring status was updated
            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert!(artist.is_touring);
        }

        #[ink::test]
        fn create_tour_invalid_dates() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let result = concert_broker.create_tour(
                "Invalid Tour".to_string(),
                artist_id,
                TourType::LocalTour,
                1704067200000, // Later date
                1672531200000, // Earlier date
                vec![],
                false,
                false,
                None,
                "Invalid date range".to_string(),
                None,
            );

            assert_eq!(result, Err(Error::InvalidTourDates));
        }

        #[ink::test]
        fn add_tour_date_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Test Tour".to_string(),
                artist_id,
                TourType::NationalTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "Test tour description".to_string(),
                None,
            ).unwrap();

            let event_id = concert_broker.add_tour_date(
                tour_id,
                venue_id,
                1680000000000, // Show date within tour range
                Some("Nashville Show".to_string()),
            ).unwrap();

            assert_eq!(event_id, 1);

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.total_shows, 1);
            assert_eq!(tour.shows_scheduled, 1);
        }

        #[ink::test]
        fn update_tour_status_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Status Test Tour".to_string(),
                artist_id,
                TourType::LocalTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "Testing status updates".to_string(),
                None,
            ).unwrap();

            let result = concert_broker.update_tour_status(tour_id, TourStatus::OnSale);
            assert_eq!(result, Ok(()));

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.tour_status, TourStatus::OnSale);
        }

        #[ink::test]
        fn add_supporting_artist_to_tour_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, supporting_artist_id) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Support Test Tour".to_string(),
                artist_id,
                TourType::NationalTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "Testing supporting artists".to_string(),
                None,
            ).unwrap();

            let result = concert_broker.add_supporting_artist_to_tour(tour_id, supporting_artist_id);
            assert_eq!(result, Ok(()));

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.supporting_artists.len(), 1);
            assert!(tour.supporting_artists.contains(&supporting_artist_id));
        }

        #[ink::test]
        fn get_tours_by_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            // Create multiple tours for the same artist
            let tour1_id = concert_broker.create_tour(
                "Tour 1".to_string(),
                artist_id,
                TourType::WorldTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "First tour".to_string(),
                None,
            ).unwrap();

            let tour2_id = concert_broker.create_tour(
                "Tour 2".to_string(),
                artist_id,
                TourType::RegionalTour,
                1704067200000,
                1735603200000,
                vec![],
                false,
                false,
                None,
                "Second tour".to_string(),
                None,
            ).unwrap();

            let artist_tours = concert_broker.get_tours_by_artist(artist_id);
            assert_eq!(artist_tours.len(), 2);
            assert!(artist_tours.contains(&tour1_id));
            assert!(artist_tours.contains(&tour2_id));
        }

        #[ink::test]
        fn search_tours_by_type_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, _, _) = setup_test_data(&mut concert_broker);

            let _world_tour_id = concert_broker.create_tour(
                "World Tour".to_string(),
                artist_id,
                TourType::WorldTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "World tour description".to_string(),
                None,
            ).unwrap();

            let _local_tour_id = concert_broker.create_tour(
                "Local Tour".to_string(),
                artist_id,
                TourType::LocalTour,
                1704067200000,
                1735603200000,
                vec![],
                false,
                false,
                None,
                "Local tour description".to_string(),
                None,
            ).unwrap();

            let world_tours = concert_broker.search_tours_by_type(TourType::WorldTour);
            assert_eq!(world_tours.len(), 1);

            let local_tours = concert_broker.search_tours_by_type(TourType::LocalTour);
            assert_eq!(local_tours.len(), 1);
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
            assert_eq!(festival.total_days, 4); // Inclusive: 4-day span = 5 total days // Inclusive: start date + 3 days = 4 total days
            assert_eq!(festival.total_capacity, 200000); // 50000 * 4 days
            assert!(festival.camping_available);
            assert_eq!(festival.camping_capacity, Some(10000));
        }

        #[ink::test]
        fn add_festival_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Test Festival".to_string(),
                venue_id,
                FestivalType::RockFestival,
                1680000000000,
                1680259200000,
                30000,
                false,
                None,
                AgeRestriction::EighteenPlus,
                None,
                None,
                create_test_social_media(),
                "Rock festival test".to_string(),
            ).unwrap();

            // Add headliner
            let result = concert_broker.add_festival_artist(festival_id, artist_id, true);
            assert_eq!(result, Ok(()));

            // Add supporting artist
            let result = concert_broker.add_festival_artist(festival_id, supporting_artist_id, false);
            assert_eq!(result, Ok(()));

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.headliner_artists.len(), 1);
            assert_eq!(festival.featured_artists.len(), 2); // Both artists in featured list
            assert!(festival.headliner_artists.contains(&artist_id));
            assert!(festival.featured_artists.contains(&artist_id));
            assert!(festival.featured_artists.contains(&supporting_artist_id));
        }

        #[ink::test]
        fn add_festival_stage_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Stage Test Festival".to_string(),
                venue_id,
                FestivalType::ElectronicFestival,
                1680000000000,
                1680259200000,
                25000,
                false,
                None,
                AgeRestriction::TwentyOnePlus,
                None,
                None,
                create_test_social_media(),
                "Electronic music festival".to_string(),
            ).unwrap();

            let result = concert_broker.add_festival_stage(
                festival_id,
                "Main Stage".to_string(),
                StageType::MainStage,
                15000,
                SoundSystemRating::WorldClass,
                LightingCapabilities::Spectacular,
                false, // Not covered
                true, // Accessibility compliant
            );
            assert_eq!(result, Ok(()));

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.stages.len(), 1);
            assert_eq!(festival.stages[0].name, "Main Stage");
            assert_eq!(festival.stages[0].stage_type, StageType::MainStage);
            assert_eq!(festival.stages[0].capacity, 15000);
        }

        #[ink::test]
        fn search_festivals_by_type_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let _music_festival_id = concert_broker.create_festival(
                "Music Festival".to_string(),
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
                "General music festival".to_string(),
            ).unwrap();

            let _jazz_festival_id = concert_broker.create_festival(
                "Jazz Festival".to_string(),
                venue_id,
                FestivalType::JazzFestival,
                1680259200000,
                1680345600000,
                15000,
                false,
                None,
                AgeRestriction::AllAges,
                None,
                None,
                create_test_social_media(),
                "Jazz and blues festival".to_string(),
            ).unwrap();

            let music_festivals = concert_broker.search_festivals_by_type(FestivalType::MusicFestival);
            assert_eq!(music_festivals.len(), 1);

            let jazz_festivals = concert_broker.search_festivals_by_type(FestivalType::JazzFestival);
            assert_eq!(jazz_festivals.len(), 1);
        }

        #[ink::test]
        fn search_festivals_by_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Artist Search Festival".to_string(),
                venue_id,
                FestivalType::MusicFestival,
                1680000000000,
                1680259200000,
                40000,
                true,
                Some(5000),
                AgeRestriction::AllAges,
                None,
                None,
                create_test_social_media(),
                "Pop music festival".to_string(),
            ).unwrap();

            concert_broker.add_festival_artist(festival_id, artist_id, true).unwrap();

            let artist_festivals = concert_broker.search_festivals_by_artist(artist_id);
            assert_eq!(artist_festivals.len(), 1);
            assert!(artist_festivals.contains(&festival_id));
        }

        #[ink::test]
        fn add_festival_vendor_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Vendor Test Festival".to_string(),
                venue_id,
                FestivalType::FolkFestival,
                1680000000000,
                1680259200000,
                20000,
                false,
                None,
                AgeRestriction::AllAges,
                None,
                None,
                create_test_social_media(),
                "Folk music festival".to_string(),
            ).unwrap();

            let result = concert_broker.add_festival_vendor(
                festival_id,
                "Gourmet Food Truck".to_string(),
                "food".to_string(),
            );
            assert_eq!(result, Ok(()));

            let result = concert_broker.add_festival_vendor(
                festival_id,
                "Music Merchandise Stand".to_string(),
                "merchandise".to_string(),
            );
            assert_eq!(result, Ok(()));

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.food_vendors.len(), 1);
            assert_eq!(festival.merchandise_vendors.len(), 1);
            assert!(festival.food_vendors.contains(&"Gourmet Food Truck".to_string()));
            assert!(festival.merchandise_vendors.contains(&"Music Merchandise Stand".to_string()));
        }

        #[ink::test]
        fn add_festival_sustainability_feature_works() {
            let mut concert_broker = ConcertBroker::new();
            let (_, venue_id, _) = setup_test_data(&mut concert_broker);

            let festival_id = concert_broker.create_festival(
                "Green Festival".to_string(),
                venue_id,
                FestivalType::CulturalFestival,
                1680000000000,
                1680259200000,
                15000,
                true,
                Some(3000),
                AgeRestriction::AllAges,
                None,
                None,
                create_test_social_media(),
                "Eco-friendly cultural festival".to_string(),
            ).unwrap();

            let result = concert_broker.add_festival_sustainability_feature(
                festival_id,
                SustainabilityFeature::SolarPower,
            );
            assert_eq!(result, Ok(()));

            let result = concert_broker.add_festival_sustainability_feature(
                festival_id,
                SustainabilityFeature::PlasticFree,
            );
            assert_eq!(result, Ok(()));

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.sustainability_features.len(), 2);
            assert!(festival.sustainability_features.contains(&SustainabilityFeature::SolarPower));
            assert!(festival.sustainability_features.contains(&SustainabilityFeature::PlasticFree));
        }

        // ========================================================================
        // ERROR HANDLING TESTS
        // ========================================================================

        #[ink::test]
        fn create_tour_artist_not_found() {
            let mut concert_broker = ConcertBroker::new();

            let result = concert_broker.create_tour(
                "Nonexistent Artist Tour".to_string(),
                999, // Nonexistent artist ID
                TourType::LocalTour,
                1672531200000,
                1704067200000,
                vec![],
                false,
                false,
                None,
                "Should fail".to_string(),
                None,
            );

            assert_eq!(result, Err(Error::ArtistNotFound));
        }

        #[ink::test]
        fn create_festival_venue_not_found() {
            let mut concert_broker = ConcertBroker::new();

            let result = concert_broker.create_festival(
                "Nonexistent Venue Festival".to_string(),
                999, // Nonexistent venue ID
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
                "Should fail".to_string(),
            );

            assert_eq!(result, Err(Error::VenueNotFound));
        }

        #[ink::test]
        fn add_tour_date_invalid_date() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, _) = setup_test_data(&mut concert_broker);

            let tour_id = concert_broker.create_tour(
                "Date Test Tour".to_string(),
                artist_id,
                TourType::LocalTour,
                1672531200000, // Tour start
                1704067200000, // Tour end
                vec![],
                false,
                false,
                None,
                "Testing date validation".to_string(),
                None,
            ).unwrap();

            // Try to add show date outside tour range
            let result = concert_broker.add_tour_date(
                tour_id,
                venue_id,
                1640995200000, // Date before tour start
                Some("Invalid Date Show".to_string()),
            );

            assert_eq!(result, Err(Error::InvalidTourDates));
        }

        // ========================================================================
        // COMPREHENSIVE INTEGRATION TEST
        // ========================================================================

        #[ink::test]
        fn complete_tour_and_festival_workflow() {
            let mut concert_broker = ConcertBroker::new();
            let (artist_id, venue_id, supporting_artist_id) = setup_test_data(&mut concert_broker);

            // Create a comprehensive tour
            let tour_id = concert_broker.create_tour(
                "The Complete Experience Tour".to_string(),
                artist_id,
                TourType::WorldTour,
                1672531200000,
                1704067200000,
                vec![supporting_artist_id],
                true,
                true,
                Some("tour.manager@example.com".to_string()),
                "A comprehensive tour with all features".to_string(),
                Some("tour-poster.jpg".to_string()),
            ).unwrap();

            // Update tour status
            concert_broker.update_tour_status(tour_id, TourStatus::OnSale).unwrap();

            // Add tour dates
            let _event1_id = concert_broker.add_tour_date(
                tour_id,
                venue_id,
                1680000000000,
                Some("Opening Night".to_string()),
            ).unwrap();

            // Add sponsors
            concert_broker.add_tour_sponsor(tour_id, "Spotify".to_string()).unwrap();
            concert_broker.add_tour_sponsor(tour_id, "Live Nation".to_string()).unwrap();

            // Create a comprehensive festival
            let festival_id = concert_broker.create_festival(
                "Ultimate Music Festival".to_string(),
                venue_id,
                FestivalType::MusicFestival,
                1680259200000,
                1680604800000, // 4 days
                60000,
                true,
                Some(15000),
                AgeRestriction::AllAges,
                Some("festival@example.com".to_string()),
                Some("www.ultimatefestival.com".to_string()),
                create_test_social_media(),
                "The ultimate multi-day music experience".to_string(),
            ).unwrap();

            // Add artists to festival
            concert_broker.add_festival_artist(festival_id, artist_id, true).unwrap();
            concert_broker.add_festival_artist(festival_id, supporting_artist_id, false).unwrap();

            // Add stages
            concert_broker.add_festival_stage(
                festival_id,
                "Main Stage".to_string(),
                StageType::MainStage,
                40000,
                SoundSystemRating::WorldClass,
                LightingCapabilities::Spectacular,
                false,
                true,
            ).unwrap();

            concert_broker.add_festival_stage(
                festival_id,
                "Acoustic Stage".to_string(),
                StageType::AcousticStage,
                5000,
                SoundSystemRating::Good,
                LightingCapabilities::Professional,
                true,
                true,
            ).unwrap();

            // Add vendors
            concert_broker.add_festival_vendor(
                festival_id,
                "Artisan Food Co.".to_string(),
                "food".to_string(),
            ).unwrap();

            concert_broker.add_festival_vendor(
                festival_id,
                "Festival Merch".to_string(),
                "merchandise".to_string(),
            ).unwrap();

            // Add sustainability features
            concert_broker.add_festival_sustainability_feature(
                festival_id,
                SustainabilityFeature::SolarPower,
            ).unwrap();

            concert_broker.add_festival_sustainability_feature(
                festival_id,
                SustainabilityFeature::WasteReduction,
            ).unwrap();

            // Update festival status
            concert_broker.update_festival_status(festival_id, FestivalStatus::LineupAnnounced).unwrap();

            // Verify everything is working
            assert_eq!(concert_broker.total_tours(), 1);
            assert_eq!(concert_broker.total_festivals(), 1);
            assert_eq!(concert_broker.total_artists(), 2);
            assert_eq!(concert_broker.total_venues(), 1);

            let tour = concert_broker.get_tour(tour_id).unwrap();
            assert_eq!(tour.tour_status, TourStatus::OnSale);
            assert_eq!(tour.total_shows, 1);
            assert_eq!(tour.sponsors.len(), 2);

            let festival = concert_broker.get_festival(festival_id).unwrap();
            assert_eq!(festival.festival_status, FestivalStatus::LineupAnnounced);
            assert_eq!(festival.total_days, 5); // Fixed: inclusive counting (4-day span + 1)
            assert_eq!(festival.stages.len(), 2);
            assert_eq!(festival.food_vendors.len(), 1);
            assert_eq!(festival.sustainability_features.len(), 2);

            // Test search functions
            let artist_tours = concert_broker.get_tours_by_artist(artist_id);
            assert_eq!(artist_tours.len(), 1);

            let world_tours = concert_broker.search_tours_by_type(TourType::WorldTour);
            assert_eq!(world_tours.len(), 1);

            let music_festivals = concert_broker.search_festivals_by_type(FestivalType::MusicFestival);
            assert_eq!(music_festivals.len(), 1);

            let artist_festivals = concert_broker.search_festivals_by_artist(artist_id);
            assert_eq!(artist_festivals.len(), 1);

        }
    }
}