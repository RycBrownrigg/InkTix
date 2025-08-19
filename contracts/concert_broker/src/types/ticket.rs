use ink::prelude::{string::String, vec::Vec};

/// Enhanced Concert Ticket structure with music industry features
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct ConcertTicket {
    pub id: u64,
    pub event_id: u32,
    pub owner: AccountId,
    pub purchase_price: u128,
    pub purchase_currency: CurrencyId,
    pub purchase_date: u64,
    pub seat_section: String,
    pub seat_row: String,
    pub seat_number: u32,
    pub seat_type: SeatType,
    pub access_level: AccessLevel,
    pub transferable: bool,
    pub vip_package_id: Option<u32>,
    pub merchandise_bundle: Vec<MerchandiseBundle>,
    pub fan_token_discount_applied: bool,
    pub loyalty_points_earned: u32,
    pub special_access: Vec<SpecialAccess>,
    pub qr_code: String,
    pub resale_allowed: bool,
    pub resale_price_limit: Option<u128>,
    pub artist_revenue_share: u128,
    pub dynamic_price_paid: u128,
    pub dot_equivalent_paid: u128,
    pub verified_fan_purchase: bool,
    pub created_at: u64,
    pub last_updated: u64,
}

/// Seat types for concerts
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeatType {
    GeneralAdmission,
    Reserved,
    PremiumReserved,
    VIPSeating,
    FrontRow,
    Balcony,
    FloorSeating,
    BoxSeats,
    StandingRoom,
    AccessibleSeating,
}

/// Access levels for ticket holders
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum AccessLevel {
    Standard,           // Basic concert access
    Premium,            // Premium seating and amenities
    VIP,               // VIP amenities and access
    AllAccess,         // Full venue access including backstage
    ArtistAccess,      // Artist-level access and privileges
}

/// Special access types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SpecialAccess {
    SoundcheckAccess,   // Watch sound check before show
    MeetAndGreet,       // Meet the artist
    BackstageAccess,    // Backstage area access
    PhotoOpportunity,   // Professional photos with artist
    EarlyVenueEntry,    // Enter venue before general admission
    ExclusiveMerchandise, // Access to exclusive merchandise
    VIPLounge,          // VIP lounge access
    ComplimentaryDrinks, // Free drinks
    PriorityParking,    // Reserved parking
    PostShowAccess,     // After-party or special access
}