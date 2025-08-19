use ink::prelude::{string::String, vec::Vec};

/// Merchandise item in the catalog
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandiseItem {
    pub id: u32,
    pub artist_id: u32,
    pub item_name: String,
    pub item_type: MerchandiseType,
    pub price: u128,
    pub sizes_available: Vec<String>,
    pub limited_edition: bool,
    pub stock_quantity: u32,
    pub description: String,
    pub image_url: Option<String>,
    pub active: bool,
    pub created_at: u64,
}

/// Merchandise bundle in a ticket purchase
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct MerchandiseBundle {
    pub merchandise_id: u32,
    pub quantity: u32,
    pub size_selected: Option<String>,
    pub bundle_price: u128,
}

/// Merchandise types
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