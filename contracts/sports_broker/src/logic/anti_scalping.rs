use ink::prelude::*;
use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Anti-scalping logic
pub struct AntiScalping;

impl AntiScalping {
    /// Configure anti-scalping settings for an event
    pub fn configure_anti_scalping(
        storage: &mut SportsBrokerStorage,
        event_id: u32,
        transfer_lock_period: u64,
        max_resale_price_multiplier: u8,
        resale_allowed: bool,
        max_tickets_per_user: u32,
        _suspicious_activity_threshold: u8,
    ) -> Result<(), String> {
        let config = AntiScalpingConfig {
            event_id,
            transfer_restricted: false,
            max_tickets_per_user,
            resale_allowed,
            max_resale_price_multiplier,
            resale_fee_percentage: 5, // Default 5% fee
            transfer_lock_period,
            blacklisted_addresses: Vec::new(),
            whitelisted_addresses: Vec::new(),
            dynamic_pricing_enabled: false,
            anti_bot_measures: false,
        };
        
        storage.anti_scalping_configs.insert(event_id, &config);
        
        Ok(())
    }
    
    /// Check if a user can purchase a ticket based on anti-scalping rules
    pub fn can_purchase_ticket(
        storage: &SportsBrokerStorage,
        event_id: u32,
        user_id: AccountId,
    ) -> Result<bool, String> {
        let config = storage.anti_scalping_configs.get(event_id)
            .ok_or("Anti-scalping not configured for this event")?;
        
        // Check if user is blacklisted
        if let Some(profile) = storage.user_behavior_profiles.get(user_id) {
            if profile.blacklist_status == BlacklistStatus::Banned {
                return Ok(false);
            }
        }
        
        // Check ticket limit per user
        let user_tickets = Self::count_user_tickets_for_event(storage, event_id, user_id);
        if user_tickets >= config.max_tickets_per_user {
            return Ok(false);
        }
        
        // Check suspicious activity score
        if let Some(profile) = storage.user_behavior_profiles.get(user_id) {
            if profile.suspicious_activity_score > 70 { // Default threshold
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check if a ticket can be transferred based on anti-scalping rules
    pub fn can_transfer_ticket(
        storage: &SportsBrokerStorage,
        event_id: u32,
        ticket_id: u64,
        from_user: AccountId,
        to_user: AccountId,
    ) -> Result<bool, String> {
        let config = storage.anti_scalping_configs.get(event_id)
            .ok_or("Anti-scalping not configured for this event")?;
        
        if config.transfer_restricted {
            return Ok(false);
        }
        
        // Check if recipient is blacklisted
        if let Some(profile) = storage.user_behavior_profiles.get(to_user) {
            if profile.blacklist_status == BlacklistStatus::Banned {
                return Ok(false);
            }
        }
        
        // Check transfer lock period
        if let Some(ticket) = storage.tickets.get(ticket_id) {
            let current_time = 0; // Will be set by caller
            if current_time < ticket.purchase_date + config.transfer_lock_period {
                return Ok(false);
            }
        }
        
        // Check if sender has suspicious activity
        if let Some(profile) = storage.user_behavior_profiles.get(from_user) {
            if profile.suspicious_activity_score > 70 { // Default threshold
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check if a user can set resale price based on anti-scalping rules
    pub fn can_set_resale_price(
        storage: &SportsBrokerStorage,
        event_id: u32,
        ticket_id: u64,
        user_id: AccountId,
        resale_price: u128,
    ) -> Result<bool, String> {
        let config = storage.anti_scalping_configs.get(event_id)
            .ok_or("Anti-scalping not configured for this event")?;
        
        if !config.resale_allowed {
            return Ok(false);
        }
        
        // Check if user is blacklisted
        if let Some(profile) = storage.user_behavior_profiles.get(user_id) {
            if profile.blacklist_status == BlacklistStatus::Banned {
                return Ok(false);
            }
        }
        
        // Check resale price multiplier
        if let Some(ticket) = storage.tickets.get(ticket_id) {
            let max_allowed_price = ticket.purchase_price * config.max_resale_price_multiplier as u128 / 100;
            if resale_price > max_allowed_price {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Flag a user for suspicious activity
    pub fn flag_suspicious_user(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        _reason: String,
        severity: u8,
    ) -> Result<(), String> {
        let mut profile = storage.user_behavior_profiles.get(user_id)
            .unwrap_or(UserBehaviorProfile {
                user_id,
                total_tickets_purchased: 0,
                total_tickets_resold: 0,
                average_hold_time: 0,
                suspicious_activity_score: 0,
                last_purchase_time: 0,
                last_resale_time: 0,
                blacklist_status: BlacklistStatus::Clean,
                warning_count: 0,
            });
        
        // Increase suspicious activity score
        profile.suspicious_activity_score = profile.suspicious_activity_score.saturating_add(severity);
        
        // Update blacklist status based on score
        if profile.suspicious_activity_score >= 80 {
            profile.blacklist_status = BlacklistStatus::Banned;
        } else if profile.suspicious_activity_score >= 60 {
            profile.blacklist_status = BlacklistStatus::Suspended;
        } else if profile.suspicious_activity_score >= 40 {
            profile.blacklist_status = BlacklistStatus::Warning;
        }
        
        // Increment warning count
        profile.warning_count += 1;
        
        storage.user_behavior_profiles.insert(user_id, &profile);
        
        Ok(())
    }
    
    /// Blacklist a user
    pub fn blacklist_user(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        _reason: String,
        _duration: u64,
    ) -> Result<(), String> {
        let mut profile = storage.user_behavior_profiles.get(user_id)
            .unwrap_or(UserBehaviorProfile {
                user_id,
                total_tickets_purchased: 0,
                total_tickets_resold: 0,
                average_hold_time: 0,
                suspicious_activity_score: 100, // Maximum score for blacklisted users
                last_purchase_time: 0,
                last_resale_time: 0,
                blacklist_status: BlacklistStatus::Banned,
                warning_count: 0,
            });
        
        profile.blacklist_status = BlacklistStatus::Banned;
        profile.suspicious_activity_score = 100;
        
        storage.user_behavior_profiles.insert(user_id, &profile);
        
        Ok(())
    }
    
    /// Whitelist a previously blacklisted user
    pub fn whitelist_user(
        storage: &mut SportsBrokerStorage,
        user_id: AccountId,
        _reason: String,
    ) -> Result<(), String> {
        if let Some(mut profile) = storage.user_behavior_profiles.get(user_id) {
            profile.blacklist_status = BlacklistStatus::Clean;
            profile.suspicious_activity_score = 0;
            profile.warning_count = 0;
            
            storage.user_behavior_profiles.insert(user_id, &profile);
        }
        
        Ok(())
    }
    
    /// Get anti-scalping configuration for an event
    pub fn get_anti_scalping_config(
        storage: &SportsBrokerStorage,
        event_id: u32,
    ) -> Option<AntiScalpingConfig> {
        storage.anti_scalping_configs.get(event_id)
    }
    
    /// Get user behavior profile
    pub fn get_user_behavior_profile(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Option<UserBehaviorProfile> {
        storage.user_behavior_profiles.get(user_id)
    }
    
    /// Get user blacklist status
    pub fn get_blacklist_status(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> Option<BlacklistStatus> {
        storage.user_behavior_profiles.get(user_id)
            .map(|profile| profile.blacklist_status)
    }
    
    /// Get user whitelist status (opposite of blacklist)
    pub fn get_whitelist_status(
        storage: &SportsBrokerStorage,
        user_id: AccountId,
    ) -> bool {
        if let Some(profile) = storage.user_behavior_profiles.get(user_id) {
            profile.blacklist_status == BlacklistStatus::Clean
        } else {
            true // New users are considered whitelisted by default
        }
    }
    
    // Helper methods
    fn count_user_tickets_for_event(
        storage: &SportsBrokerStorage,
        event_id: u32,
        user_id: AccountId,
    ) -> u32 {
        let mut count = 0;
        for ticket_id in 1..=storage.total_tickets {
            if let Some(ticket) = storage.tickets.get(ticket_id) {
                if ticket.event_id == event_id && ticket.owner == user_id {
                    count += 1;
                }
            }
        }
        count
    }
    
    fn check_transfer_cooldown(
        _storage: &SportsBrokerStorage,
        _ticket_id: u64,
        _lock_period: u64,
    ) -> bool {
        // Placeholder implementation
        true
    }
    
    fn validate_resale_price(
        _storage: &SportsBrokerStorage,
        _ticket_id: u64,
        _resale_price: u128,
        _max_multiplier: u8,
    ) -> bool {
        // Placeholder implementation
        true
    }
    
    fn update_user_behavior_metrics(
        _storage: &mut SportsBrokerStorage,
        _user_id: AccountId,
        _ticket_id: u64,
        _action: String,
    ) {
        // Placeholder implementation
    }
}

/// Blacklist record
#[derive(Debug, Clone, PartialEq)]
pub struct BlacklistRecord {
    pub user_id: AccountId,
    pub reason: String,
    pub blacklist_status: BlacklistStatus,
    pub blacklisted_at: u64,
    pub expires_at: Option<u64>,
}

/// Whitelist record
#[derive(Debug, Clone, PartialEq)]
pub struct WhitelistRecord {
    pub user_id: AccountId,
    pub reason: String,
    pub whitelisted_at: u64,
}
