use ink::prelude::{string::String, vec::Vec};

/// VIP Package offerings for events
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct VIPPackage {
    pub package_name: String,
    pub price_premium: u128,
    pub benefits: Vec<VIPBenefit>,
    pub limited_quantity: Option<u32>,
    pub available_quantity: Option<u32>,
    pub description: String,
}

/// VIP Package benefits
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum VIPBenefit {
    EarlyEntry,
    MeetAndGreet,
    BackstageAccess,
    SoundcheckAccess,
    PremiumSeating,
    ExclusiveMerchandise,
    DedicatedEntrance,
    ComplimentaryDrinks,
    PreShowReception,
    PostShowAccess,
    SignedMemorabilia,
    PhotoOpportunity,
    LimitedPoster,
    VIPLaminate,
    ParkingIncluded,
}