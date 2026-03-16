use ink::prelude::string::String;

#[allow(clippy::cast_possible_truncation)]

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
    #[allow(clippy::cast_possible_truncation)]
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

/// Event category for the unified contract
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum EventCategory {
    Sports {
        home_team_id: u32,
        away_team_id: u32,
        season_id: u32,
        game_type: GameType,
        sport_type: SportType,
    },
    Concert {
        artist_id: u32,
    },
    Generic,
}

/// Unified Event structure (replaces SportsEvent and ConcertEvent)
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub venue_id: u32,
    pub date: u64,
    pub capacity: u32,
    pub sold_tickets: u32,
    pub base_price: u128,
    pub active: bool,
    pub category: EventCategory,
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
