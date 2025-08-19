use crate::types::{MerchandiseItem, MerchandiseType, InkTixError, InkTixResult};
use ink::storage::Mapping;
use ink::prelude::vec::Vec;

/// Merchandise management business logic
pub struct MerchandiseManager {
    pub merchandise_catalog: Mapping<u32, MerchandiseItem>,
    pub next_merchandise_id: u32,
    pub artist_merchandise: Mapping<u32, Vec<u32>>,
}

impl MerchandiseManager {
    pub fn new() -> Self {
        Self {
            merchandise_catalog: Mapping::new(),
            next_merchandise_id: 1,
            artist_merchandise: Mapping::new(),
        }
    }

    /// Add merchandise item to artist catalog
    pub fn add_merchandise_item(
        &mut self,
        artist_id: u32,
        item_name: String,
        item_type: MerchandiseType,
        price: u128,
        sizes_available: Vec<String>,
        limited_edition: bool,
        stock_quantity: u32,
        description: String,
        image_url: Option<String>,
    ) -> InkTixResult<u32> {
        if item_name.is_empty() || description.is_empty() {
            return Err(InkTixError::InvalidData);
        }

        let merchandise_id = self.next_merchandise_id;
        self.next_merchandise_id = self.next_merchandise_id
            .checked_add(1)
            .ok_or(InkTixError::IdOverflow)?;

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
            created_at: ink::env::block_timestamp(),
        };

        self.merchandise_catalog.insert(merchandise_id, &merchandise);

        // Update artist merchandise index
        let mut artist_merch = self.artist_merchandise.get(artist_id).unwrap_or_default();
        artist_merch.push(merchandise_id);
        self.artist_merchandise.insert(artist_id, &artist_merch);

        Ok(merchandise_id)
    }

    /// Update merchandise stock
    pub fn update_merchandise_stock(
        &mut self,
        merchandise_id: u32,
        new_stock: u32,
    ) -> InkTixResult<()> {
        let mut merchandise = self.merchandise_catalog.get(merchandise_id)
            .ok_or(InkTixError::NotFound)?;

        merchandise.stock_quantity = new_stock;
        self.merchandise_catalog.insert(merchandise_id, &merchandise);

        Ok(())
    }

    /// Get merchandise item details
    pub fn get_merchandise_item(&self, merchandise_id: u32) -> Option<MerchandiseItem> {
        self.merchandise_catalog.get(merchandise_id)
    }

    /// Get artist's merchandise catalog
    pub fn get_artist_merchandise(&self, artist_id: u32) -> Vec<u32> {
        self.artist_merchandise.get(artist_id).unwrap_or_default()
    }

    /// Get total merchandise count
    pub fn total_merchandise_items(&self) -> u32 {
        self.next_merchandise_id.saturating_sub(1)
    }
}

