#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # InkTix Concert Broker - Enhanced Artist and Venue Management (Step 1)
/// 
/// This step transforms the basic concert broker into a comprehensive music industry platform
/// with detailed artist profiles, venue management, and music-specific categorization.

#[ink::contract]
mod concert_broker {
    use ink::prelude::{string::String, vec::Vec};

    /// The Concert Broker contract storage.
    #[ink(storage)]
    pub struct ConcertBroker {
        /// The contract owner
        owner: AccountId,
        
        // Enhanced Artist management
        artists: ink::storage::Mapping<u32, Artist>,
        next_artist_id: u32,
        
        // Enhanced Venue management  
        venues: ink::storage::Mapping<u32, Venue>,
        next_venue_id: u32,
        
        // Artist indexing for search
        artists_by_genre: ink::storage::Mapping<u32, Vec<u32>>, // genre_hash -> artist_ids
        verified_artists: ink::storage::Mapping<bool, Vec<u32>>, // verified status -> artist_ids
        
        // Venue indexing for search
        venues_by_type: ink::storage::Mapping<u32, Vec<u32>>, // venue_type_hash -> venue_ids
        venues_by_city: ink::storage::Mapping<u32, Vec<u32>>, // city_hash -> venue_ids
        
        // Simple event storage (will be enhanced in later steps)
        events: ink::storage::Mapping<u32, Option<String>>,
        next_event_id: u32,
    }

    /// Enhanced Artist structure with music industry-specific fields
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
        pub latitude: Option<i32>, // Stored as integer (multiply by 1000000 for actual value)
        pub longitude: Option<i32>,
    }

    /// Social media handles for artists
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

    /// Streaming platform links
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct StreamingPlatform {
        pub platform: StreamingService,
        pub artist_url: String,
        pub verified: bool,
    }

    /// Music genres with comprehensive coverage
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

    /// Types of music venues
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

    /// Venue amenities
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

    /// Age restrictions for venues and events
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum AgeRestriction {
        AllAges,
        EighteenPlus,
        TwentyOnePlus,
        Custom(u8), // Custom minimum age
    }

    /// Accessibility features
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

    /// Sound system quality rating
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SoundSystemRating {
        Basic,      // 1-3: Basic PA system
        Good,       // 4-6: Professional sound system
        Excellent,  // 7-8: High-end professional system
        WorldClass, // 9-10: Top-tier audiophile system
    }

    /// Lighting capabilities
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum LightingCapabilities {
        Basic,           // Simple stage lighting
        Professional,    // Full concert lighting rig
        Advanced,        // LED walls, lasers, effects
        Spectacular,     // Cutting-edge production capabilities
    }

    /// Security level of the venue
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum SecurityLevel {
        Minimal,      // Basic security, small venues
        Standard,     // Professional security staff
        High,         // Metal detectors, bag checks
        Maximum,      // Full security screening, VIP protection
    }

    /// Streaming services
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
        /// ID overflow
        IdOverflow,
        /// Invalid artist data
        InvalidArtistData,
        /// Invalid venue data
        InvalidVenueData,
        /// Artist already verified
        ArtistAlreadyVerified,
        /// Venue already verified
        VenueAlreadyVerified,
        /// Empty search results
        NoSearchResults,
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
                artists_by_genre: ink::storage::Mapping::new(),
                verified_artists: ink::storage::Mapping::new(),
                venues_by_type: ink::storage::Mapping::new(),
                venues_by_city: ink::storage::Mapping::new(),
                events: ink::storage::Mapping::new(),
                next_event_id: 1,
            }
        }

        // ========================================================================
        // ENHANCED ARTIST MANAGEMENT
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
        // ENHANCED VENUE MANAGEMENT
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
        // SEARCH AND DISCOVERY FUNCTIONS
        // ========================================================================

        /// Search artists by genre
        #[ink(message)]
        pub fn search_artists_by_genre(&self, genre: MusicGenre) -> Vec<u32> {
            let genre_hash = self.hash_music_genre(&genre);
            self.artists_by_genre.get(genre_hash).unwrap_or_default()
        }

        /// Get all verified artists
        #[ink(message)]
        pub fn get_verified_artists(&self) -> Vec<u32> {
            self.verified_artists.get(true).unwrap_or_default()
        }

        /// Search venues by type
        #[ink(message)]
        pub fn search_venues_by_type(&self, venue_type: VenueType) -> Vec<u32> {
            let type_hash = self.hash_venue_type(venue_type);
            self.venues_by_type.get(type_hash).unwrap_or_default()
        }

        /// Search venues by city
        #[ink(message)]
        pub fn search_venues_by_city(&self, city: String) -> Vec<u32> {
            let city_hash = self.hash_string(&city);
            self.venues_by_city.get(city_hash).unwrap_or_default()
        }

        /// Get venues with specific amenities
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

        /// Get venues with accessibility features
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

        /// Get artists currently touring
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

        // ========================================================================
        // HELPER FUNCTIONS FOR INDEXING
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
        // BASIC EVENT MANAGEMENT (Placeholder for later steps)
        // ========================================================================

        /// Create a basic event (will be enhanced in later steps)
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
        // QUERY FUNCTIONS
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

    /// Comprehensive test suite for Step 1
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

        #[ink::test]
        fn new_works() {
            let concert_broker = ConcertBroker::new();
            assert_eq!(concert_broker.total_artists(), 0);
            assert_eq!(concert_broker.total_venues(), 0);
            assert_eq!(concert_broker.total_events(), 0);
        }

        #[ink::test]
        fn register_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let result = concert_broker.register_artist(
                "Taylor Swift".to_string(),
                Some("T.S.".to_string()),
                MusicGenre::Pop,
                vec![MusicGenre::Country, MusicGenre::Folk],
                "Award-winning singer-songwriter known for storytelling lyrics".to_string(),
                "United States".to_string(),
                Some("Big Machine Records".to_string()),
                create_test_social_media(),
                create_test_streaming_links(),
                2006,
                Some("management@taylorswift.com".to_string()),
            );
            
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            assert_eq!(concert_broker.total_artists(), 1);

            let artist = concert_broker.get_artist(1).unwrap();
            assert_eq!(artist.name, "Taylor Swift");
            assert_eq!(artist.genre, MusicGenre::Pop);
            assert_eq!(artist.sub_genres.len(), 2);
            assert!(!artist.verified);
            assert_eq!(artist.years_active.0, 2006);
        }

        #[ink::test]
        fn register_artist_invalid_data() {
            let mut concert_broker = ConcertBroker::new();
            
            let result = concert_broker.register_artist(
                "".to_string(), // Empty name
                None,
                MusicGenre::Pop,
                vec![],
                "".to_string(), // Empty biography
                "".to_string(), // Empty country
                None,
                create_test_social_media(),
                vec![],
                2020,
                None,
            );
            
            assert_eq!(result, Err(Error::InvalidArtistData));
        }

        #[ink::test]
        fn verify_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let artist_id = concert_broker.register_artist(
                "Ed Sheeran".to_string(),
                None,
                MusicGenre::Pop,
                vec![MusicGenre::Folk],
                "British singer-songwriter".to_string(),
                "United Kingdom".to_string(),
                None,
                create_test_social_media(),
                create_test_streaming_links(),
                2011,
                None,
            ).unwrap();

            // Artist should not be verified initially
            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert!(!artist.verified);

            // Verify the artist
            let result = concert_broker.verify_artist(artist_id);
            assert_eq!(result, Ok(()));

            // Artist should now be verified
            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert!(artist.verified);

            // Should appear in verified artists list
            let verified_artists = concert_broker.get_verified_artists();
            assert!(verified_artists.contains(&artist_id));
        }

        #[ink::test]
        fn register_venue_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let result = concert_broker.register_venue(
                "Madison Square Garden".to_string(),
                create_test_venue_address(),
                VenueType::Arena,
                20000,
                Some(5000),
                9, // Excellent acoustics
                vec![VenueAmenity::VipLounge, VenueAmenity::MultipleBars, VenueAmenity::MerchandiseStand],
                AgeRestriction::AllAges,
                true,
                Some(1000),
                true,
                vec![AccessibilityFeature::WheelchairAccessible, AccessibilityFeature::ElevatorAccess],
                SoundSystemRating::WorldClass,
                LightingCapabilities::Spectacular,
                SecurityLevel::Maximum,
                Some(1968),
                Some("booking@msg.com".to_string()),
                Some("www.msg.com".to_string()),
            );
            
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
            assert_eq!(concert_broker.total_venues(), 1);

            let venue = concert_broker.get_venue(1).unwrap();
            assert_eq!(venue.name, "Madison Square Garden");
            assert_eq!(venue.venue_type, VenueType::Arena);
            assert_eq!(venue.capacity, 20000);
            assert_eq!(venue.acoustic_rating, 9);
            assert!(venue.parking_available);
            assert!(!venue.verified);
        }

        #[ink::test]
        fn register_venue_invalid_data() {
            let mut concert_broker = ConcertBroker::new();
            
            let mut invalid_address = create_test_venue_address();
            invalid_address.city = "".to_string(); // Empty city

            let result = concert_broker.register_venue(
                "".to_string(), // Empty name
                invalid_address,
                VenueType::Club,
                0, // Zero capacity
                None,
                15, // Invalid acoustic rating (>10)
                vec![],
                AgeRestriction::AllAges,
                false,
                None,
                false,
                vec![],
                SoundSystemRating::Basic,
                LightingCapabilities::Basic,
                SecurityLevel::Minimal,
                None,
                None,
                None,
            );
            
            assert_eq!(result, Err(Error::InvalidVenueData));
        }

        #[ink::test]
        fn search_artists_by_genre_works() {
            let mut concert_broker = ConcertBroker::new();
            
            // Register artists with different genres
            concert_broker.register_artist(
                "Metallica".to_string(),
                None,
                MusicGenre::Metal,
                vec![MusicGenre::Rock, MusicGenre::HardRock],
                "Heavy metal band".to_string(),
                "United States".to_string(),
                None,
                create_test_social_media(),
                vec![],
                1981,
                None,
            ).unwrap();

            concert_broker.register_artist(
                "Adele".to_string(),
                None,
                MusicGenre::Pop,
                vec![MusicGenre::Soul],
                "British singer".to_string(),
                "United Kingdom".to_string(),
                None,
                create_test_social_media(),
                vec![],
                2006,
                None,
            ).unwrap();

            concert_broker.register_artist(
                "Iron Maiden".to_string(),
                None,
                MusicGenre::Metal,
                vec![MusicGenre::Rock],
                "Heavy metal legends".to_string(),
                "United Kingdom".to_string(),
                None,
                create_test_social_media(),
                vec![],
                1975,
                None,
            ).unwrap();

            let metal_artists = concert_broker.search_artists_by_genre(MusicGenre::Metal);
            assert_eq!(metal_artists.len(), 2);
            assert!(metal_artists.contains(&1)); // Metallica
            assert!(metal_artists.contains(&3)); // Iron Maiden

            let pop_artists = concert_broker.search_artists_by_genre(MusicGenre::Pop);
            assert_eq!(pop_artists.len(), 1);
            assert!(pop_artists.contains(&2)); // Adele

            // Test sub-genre search
            let rock_artists = concert_broker.search_artists_by_genre(MusicGenre::Rock);
            assert_eq!(rock_artists.len(), 2);
            assert!(rock_artists.contains(&1)); // Metallica (sub-genre)
            assert!(rock_artists.contains(&3)); // Iron Maiden (sub-genre)
        }

        #[ink::test]
        fn search_venues_by_type_works() {
            let mut concert_broker = ConcertBroker::new();
            
            // Register different venue types
            concert_broker.register_venue(
                "The Fillmore".to_string(),
                create_test_venue_address(),
                VenueType::Club,
                1800,
                Some(500),
                8,
                vec![VenueAmenity::DanceFloor, VenueAmenity::MultipleBars],
                AgeRestriction::TwentyOnePlus,
                false,
                None,
                true,
                vec![AccessibilityFeature::WheelchairAccessible],
                SoundSystemRating::Excellent,
                LightingCapabilities::Professional,
                SecurityLevel::Standard,
                Some(1968),
                None,
                None,
            ).unwrap();

            concert_broker.register_venue(
                "Hollywood Bowl".to_string(),
                create_test_venue_address(),
                VenueType::Amphitheater,
                17500,
                None,
                9,
                vec![VenueAmenity::OutdoorArea, VenueAmenity::ReservedSeating],
                AgeRestriction::AllAges,
                true,
                Some(800),
                true,
                vec![AccessibilityFeature::WheelchairAccessible, AccessibilityFeature::AccessibleParking],
                SoundSystemRating::WorldClass,
                LightingCapabilities::Spectacular,
                SecurityLevel::High,
                Some(1922),
                None,
                None,
            ).unwrap();

            let clubs = concert_broker.search_venues_by_type(VenueType::Club);
            assert_eq!(clubs.len(), 1);
            assert!(clubs.contains(&1)); // The Fillmore

            let amphitheaters = concert_broker.search_venues_by_type(VenueType::Amphitheater);
            assert_eq!(amphitheaters.len(), 1);
            assert!(amphitheaters.contains(&2)); // Hollywood Bowl
        }

        #[ink::test]
        fn search_venues_by_city_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let mut nashville_address = create_test_venue_address();
            nashville_address.city = "Nashville".to_string();

            let mut los_angeles_address = create_test_venue_address();
            los_angeles_address.city = "Los Angeles".to_string();

            // Register venues in different cities
            concert_broker.register_venue(
                "Grand Ole Opry".to_string(),
                nashville_address,
                VenueType::ConcertHall,
                4400,
                None,
                10,
                vec![VenueAmenity::ReservedSeating, VenueAmenity::VipLounge],
                AgeRestriction::AllAges,
                true,
                Some(200),
                false,
                vec![AccessibilityFeature::WheelchairAccessible],
                SoundSystemRating::WorldClass,
                LightingCapabilities::Advanced,
                SecurityLevel::High,
                Some(1925),
                None,
                None,
            ).unwrap();

            concert_broker.register_venue(
                "The Troubadour".to_string(),
                los_angeles_address,
                VenueType::Club,
                500,
                Some(200),
                7,
                vec![VenueAmenity::DanceFloor, VenueAmenity::MerchandiseStand],
                AgeRestriction::TwentyOnePlus,
                false,
                None,
                true,
                vec![],
                SoundSystemRating::Good,
                LightingCapabilities::Professional,
                SecurityLevel::Standard,
                Some(1957),
                None,
                None,
            ).unwrap();

            let nashville_venues = concert_broker.search_venues_by_city("Nashville".to_string());
            assert_eq!(nashville_venues.len(), 1);
            assert!(nashville_venues.contains(&1)); // Grand Ole Opry

            let la_venues = concert_broker.search_venues_by_city("Los Angeles".to_string());
            assert_eq!(la_venues.len(), 1);
            assert!(la_venues.contains(&2)); // The Troubadour
        }

        #[ink::test]
        fn search_venues_with_amenity_works() {
            let mut concert_broker = ConcertBroker::new();
            
            // Register venue with VIP lounge
            concert_broker.register_venue(
                "VIP Venue".to_string(),
                create_test_venue_address(),
                VenueType::Arena,
                15000,
                None,
                8,
                vec![VenueAmenity::VipLounge, VenueAmenity::MultipleBars],
                AgeRestriction::AllAges,
                true,
                Some(500),
                true,
                vec![],
                SoundSystemRating::Excellent,
                LightingCapabilities::Advanced,
                SecurityLevel::High,
                None,
                None,
                None,
            ).unwrap();

            // Register venue without VIP lounge
            concert_broker.register_venue(
                "Basic Venue".to_string(),
                create_test_venue_address(),
                VenueType::Club,
                800,
                None,
                6,
                vec![VenueAmenity::DanceFloor],
                AgeRestriction::EighteenPlus,
                false,
                None,
                true,
                vec![],
                SoundSystemRating::Good,
                LightingCapabilities::Basic,
                SecurityLevel::Standard,
                None,
                None,
                None,
            ).unwrap();

            let vip_venues = concert_broker.search_venues_with_amenity(VenueAmenity::VipLounge);
            assert_eq!(vip_venues.len(), 1);
            assert!(vip_venues.contains(&1)); // VIP Venue

            let dance_floor_venues = concert_broker.search_venues_with_amenity(VenueAmenity::DanceFloor);
            assert_eq!(dance_floor_venues.len(), 1);
            assert!(dance_floor_venues.contains(&2)); // Basic Venue
        }

        #[ink::test]
        fn update_artist_stats_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let artist_id = concert_broker.register_artist(
                "The Weeknd".to_string(),
                None,
                MusicGenre::RAndB,
                vec![MusicGenre::Pop],
                "Canadian singer".to_string(),
                "Canada".to_string(),
                None,
                create_test_social_media(),
                vec![],
                2010,
                None,
            ).unwrap();

            // Update stats
            let result = concert_broker.update_artist_stats(artist_id, 50_000_000, 5, 3);
            assert_eq!(result, Ok(()));

            let artist = concert_broker.get_artist(artist_id).unwrap();
            assert_eq!(artist.monthly_listeners, 50_000_000);
            assert_eq!(artist.total_albums, 5);
            assert_eq!(artist.awards_count, 3);
        }

        #[ink::test]
        fn create_event_with_artist_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let artist_id = concert_broker.register_artist(
                "Coldplay".to_string(),
                None,
                MusicGenre::Alternative,
                vec![MusicGenre::Rock, MusicGenre::Pop],
                "British rock band".to_string(),
                "United Kingdom".to_string(),
                None,
                create_test_social_media(),
                vec![],
                1996,
                None,
            ).unwrap();

            let result = concert_broker.create_event("Coldplay World Tour".to_string(), artist_id);
            assert_eq!(result, Ok(1));
            assert_eq!(concert_broker.total_events(), 1);

            let event_name = concert_broker.get_event(1).unwrap();
            assert_eq!(event_name, "Coldplay World Tour");
        }

        #[ink::test]
        fn create_event_artist_not_found() {
            let mut concert_broker = ConcertBroker::new();
            
            let result = concert_broker.create_event("Non-existent Artist Concert".to_string(), 999);
            assert_eq!(result, Err(Error::ArtistNotFound));
        }

        #[ink::test]
        fn get_touring_artists_works() {
            let mut concert_broker = ConcertBroker::new();
            
            let artist1_id = concert_broker.register_artist(
                "U2".to_string(),
                None,
                MusicGenre::Rock,
                vec![MusicGenre::Alternative],
                "Irish rock band".to_string(),
                "Ireland".to_string(),
                None,
                create_test_social_media(),
                vec![],
                1976,
                None,
            ).unwrap();

            let artist2_id = concert_broker.register_artist(
                "Radiohead".to_string(),
                None,
                MusicGenre::Alternative,
                vec![MusicGenre::Rock],
                "English rock band".to_string(),
                "United Kingdom".to_string(),
                None,
                create_test_social_media(),
                vec![],
                1985,
                None,
            ).unwrap();

            // Set one artist as touring
            concert_broker.set_artist_touring_status(artist1_id, true).unwrap();

            let touring_artists = concert_broker.get_touring_artists();
            assert_eq!(touring_artists.len(), 1);
            assert!(touring_artists.contains(&artist1_id));
            assert!(!touring_artists.contains(&artist2_id));
        }
    }
}