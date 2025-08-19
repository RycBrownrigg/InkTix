use ink::prelude::{string::String, vec::Vec};

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
    pub years_active: (u32, Option<u32>),
    pub origin_country: String,
    pub monthly_listeners: u32,
    pub total_albums: u32,
    pub awards_count: u32,
    pub is_touring: bool,
    pub management_contact: Option<String>,
    pub created_at: u64,
}

/// Music genres with comprehensive coverage
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

/// Streaming services
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum StreamingService {
    Spotify, AppleMusic, YouTubeMusic, AmazonMusic, Tidal, Deezer,
    Pandora, SoundCloud, Bandcamp, Beatport, Other,
}