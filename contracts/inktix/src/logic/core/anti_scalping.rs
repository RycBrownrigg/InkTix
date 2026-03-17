//! Anti-scalping configuration logic.
//!
//! Allows the contract owner to configure per-event anti-scalping rules
//! such as purchase limits, transfer restrictions, and bot-detection flags.
//!
//! # Functions
//! - `configure_anti_scalping` -- stores an anti-scalping config for a specific event

use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;

/// Anti-scalping functionality
pub struct AntiScalping;

impl AntiScalping {
    /// Configure anti-scalping for event
    pub fn configure_anti_scalping(
        storage: &mut InkTixStorage,
        event_id: u32,
        config: AntiScalpingConfig,
    ) -> Result<(), String> {
        let _event = storage.events.get(event_id).ok_or("Event not found")?;
        storage.anti_scalping_configs.insert(event_id, &config);
        Ok(())
    }
}
