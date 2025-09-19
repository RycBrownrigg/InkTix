use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;

/// Anti-scalping functionality
pub struct AntiScalping;

impl AntiScalping {
    /// Configure anti-scalping for event
    pub fn configure_anti_scalping(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        config: AntiScalpingConfig,
    ) -> Result<(), String> {
        let _event = storage.events.get(event_id).ok_or("Event not found")?;
        
        storage.anti_scalping_configs.insert(event_id, &config);
        Ok(())
    }
}