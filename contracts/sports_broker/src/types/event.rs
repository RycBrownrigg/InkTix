use ink::prelude::string::String;

/// Sport types
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SportType {
    Basketball,
    Football,
    Soccer,
    Baseball,
    Hockey,
    Tennis,
    Other(String),
}

/// Game/Event types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum GameType {
    RegularSeason,
    Playoff,
    Championship,
    AllStar,
    Preseason,
    Tournament,
    Exhibition,
}

/// Enhanced Event structure for sports
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SportsEvent {
    pub id: u32,
    pub name: String,
    pub venue_id: u32,
    pub date: u64,
    pub capacity: u32,
    pub sold_tickets: u32,
    pub base_price: u128,
    pub active: bool,
    pub sport_type: SportType,
    pub home_team_id: u32,
    pub away_team_id: u32,
    pub season_id: u32,
    pub game_type: GameType,
    pub season_pass_discount: u8,
    pub dynamic_pricing_enabled: bool,
    pub rivalry_multiplier: u32,
    pub revenue_generated: u128,
}

/// Event status
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum EventStatus {
    Scheduled,
    OnSale,
    SoldOut,
    InProgress,
    Completed,
    Cancelled,
}