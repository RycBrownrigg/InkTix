//! Team type definition for sports events.
//!
//! Contains the `Team` struct representing a registered sports team
//! with identity, location, sport classification, and verification status.

use ink::prelude::string::String;
use crate::types::core::event::SportType;

/// Enhanced Team structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub city: String,
    pub sport_type: SportType,
    pub verified: bool,
}
