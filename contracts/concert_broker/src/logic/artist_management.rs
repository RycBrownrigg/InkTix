use crate::types::{Artist, MusicGenre, SocialMediaHandles, StreamingPlatform};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;
use ink::prelude::string::String;

/// Artist management business logic
#[derive(Debug, Default)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ArtistManager {
    pub artists: Mapping<u32, Artist>,
    pub next_artist_id: u32,
    pub artists_by_genre: Mapping<u32, Vec<u32>>,
    pub verified_artists: Mapping<bool, Vec<u32>>,
}

impl ArtistManager {
    pub fn new() -> Self {
        Self {
            artists: Mapping::new(),
            next_artist_id: 1,
            artists_by_genre: Mapping::new(),
            verified_artists: Mapping::new(),
        }
    }

    pub fn register_artist(
        &mut self,
        name: String,
        stage_name: Option<String>,
        genre: MusicGenre,
        sub_genres: Vec<MusicGenre>,
        social_media: SocialMediaHandles,
        streaming_platforms: Vec<StreamingPlatform>,
        fan_token_address: Option<[u8; 32]>,
    ) -> u32 {
        let artist_id = self.next_artist_id;
        self.next_artist_id += 1;

        let artist = Artist {
            id: artist_id,
            name,
            stage_name,
            genre,
            sub_genres,
            social_media,
            streaming_platforms,
            fan_token_address,
            verified: false,
        };

        self.artists.insert(artist_id, &artist);
        artist_id
    }

    pub fn get_artist(&self, artist_id: u32) -> Option<Artist> {
        self.artists.get(artist_id)
    }

    pub fn verify_artist(&mut self, artist_id: u32) -> bool {
        if let Some(mut artist) = self.artists.get(artist_id) {
            artist.verified = true;
            self.artists.insert(artist_id, &artist);
            true
        } else {
            false
        }
    }

    pub fn search_artists_by_genre(&self, genre: &MusicGenre) -> Vec<u32> {
        self.artists_by_genre.get(&(*genre as u32)).unwrap_or_default()
    }

    pub fn total_artists(&self) -> u32 {
        self.next_artist_id - 1
    }
}