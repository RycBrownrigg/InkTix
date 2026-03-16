use crate::storage::contract_storage::InkTixStorage;
use crate::types::concert::artist::Artist;
use ink::primitives::AccountId;
use ink::prelude::string::String;
use ink::prelude::string::ToString;

/// Artist management for concert events
pub struct ArtistManagement;

#[allow(clippy::arithmetic_side_effects)]
impl ArtistManagement {
    /// Register a new artist
    pub fn register_artist(
        storage: &mut InkTixStorage,
        caller: AccountId,
        name: String,
    ) -> Result<u32, String> {
        if name.is_empty() {
            return Err("Artist name cannot be empty".to_string());
        }

        let artist_id = storage.get_next_artist_id();
        let artist = Artist {
            id: artist_id,
            name,
            verified: false,
            account: Some(caller),
        };

        storage.artists.insert(artist_id, &artist);
        storage.total_artists += 1;
        Ok(artist_id)
    }

    /// Verify an artist (owner only)
    pub fn verify_artist(
        storage: &mut InkTixStorage,
        artist_id: u32,
    ) -> Result<(), String> {
        let mut artist = storage.artists.get(artist_id).ok_or("Artist not found")?;
        artist.verified = true;
        storage.artists.insert(artist_id, &artist);
        Ok(())
    }

    /// Get artist by ID
    pub fn get_artist(
        storage: &InkTixStorage,
        artist_id: u32,
    ) -> Option<Artist> {
        storage.artists.get(artist_id)
    }
}
