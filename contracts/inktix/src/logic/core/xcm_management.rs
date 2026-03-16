use crate::storage::contract_storage::InkTixStorage;
use crate::types::core::xcm::*;
use ink::prelude::vec::Vec;
use ink::env::DefaultEnvironment;
use ink::prelude::string::String;
use ink::prelude::string::ToString;

#[allow(clippy::arithmetic_side_effects)]

impl InkTixStorage {
    /// Send an XCM message to another chain
    pub fn send_xcm_message(
        &mut self,
        destination_chain: String,
        destination_parachain_id: u32,
        message_type: XcmMessageType,
        payload: Vec<u8>,
        fee_currency: String,
    ) -> Result<u64, String> {
        if !self.is_chain_connected(&destination_chain) {
            return Err("Destination chain is not connected".to_string());
        }
        let fee = self.calculate_xcm_message_fee(&destination_chain, message_type.clone(), payload.len())?;
        let message_id = self.get_next_xcm_message_id();

        let xcm_message = XcmMessage {
            message_id, message_type: message_type.clone(),
            source_chain: self.get_current_chain_id(), destination_chain: destination_chain.clone(),
            source_parachain_id: self.get_current_parachain_id(), destination_parachain_id,
            xcm_version: self.get_supported_xcm_version(), payload,
            status: XcmMessageStatus::Sent,
            created_at: ink::env::block_timestamp::<DefaultEnvironment>(),
            updated_at: ink::env::block_timestamp::<DefaultEnvironment>(),
            fee_paid: fee, fee_currency, error_message: None,
        };

        self.xcm_messages.insert(message_id, &xcm_message);
        self.total_xcm_messages_sent += 1;
        self.update_chain_message_tracking(&destination_chain, message_id);
        Ok(message_id)
    }

    /// Get XCM message by ID
    pub fn get_xcm_message(&self, message_id: u64) -> Option<XcmMessage> {
        self.xcm_messages.get(message_id)
    }

    /// Get XCM analytics
    pub fn get_xcm_analytics(&self) -> XcmAnalytics {
        XcmAnalytics {
            total_messages_sent: self.total_xcm_messages_sent,
            total_messages_received: self.total_xcm_messages_received,
            total_successful_operations: 0, total_failed_operations: 0,
            total_fees_paid: 0, average_processing_time: 0,
            connected_chains_count: self.total_connected_chains,
            messages_by_type: Vec::new(), messages_by_status: Vec::new(),
        }
    }

    /// Update chain connectivity status
    pub fn update_xcm_chain_connectivity(
        &mut self, chain_id: String, is_connected: bool, latency_ms: Option<u64>,
        supported_xcm_version: u8, max_message_size: u32, fee_structure: XcmFeeStructure,
    ) -> Result<(), String> {
        let current_time = ink::env::block_timestamp::<DefaultEnvironment>();
        let connectivity_status = XcmConnectivityStatus {
            chain_id: chain_id.clone(), is_connected, last_heartbeat: current_time,
            latency_ms, supported_xcm_version, max_message_size, fee_structure,
        };
        let was_connected = self.xcm_chain_connectivity.get(&chain_id).map(|s| s.is_connected).unwrap_or(false);
        self.xcm_chain_connectivity.insert(&chain_id, &connectivity_status);
        if is_connected && !was_connected { self.total_connected_chains += 1; }
        else if !is_connected && was_connected && self.total_connected_chains > 0 { self.total_connected_chains -= 1; }
        Ok(())
    }

    fn is_chain_connected(&self, chain_id: &str) -> bool {
        self.xcm_chain_connectivity.get(chain_id).map(|s| s.is_connected).unwrap_or(false)
    }

    fn calculate_xcm_message_fee(&self, destination_chain: &str, message_type: XcmMessageType, payload_size: usize) -> Result<u128, String> {
        if let Some(connectivity) = self.xcm_chain_connectivity.get(destination_chain) {
            let fs = &connectivity.fee_structure;
            let operation_fee = match message_type {
                XcmMessageType::TicketPurchaseRequest => fs.ticket_purchase_fee,
                XcmMessageType::PaymentConfirmation => fs.payment_confirmation_fee,
                XcmMessageType::TicketTransfer => fs.ticket_transfer_fee,
                _ => 0,
            };
            Ok(fs.base_fee + fs.fee_per_byte * payload_size as u128 + operation_fee)
        } else { Err("Chain connectivity information not found".to_string()) }
    }

    fn get_next_xcm_message_id(&mut self) -> u64 { self.next_xcm_message_id += 1; self.next_xcm_message_id }
    fn get_current_chain_id(&self) -> String { "inktix_chain".to_string() }
    fn get_current_parachain_id(&self) -> u32 { 2000 }
    fn get_supported_xcm_version(&self) -> u8 { 3 }

    fn update_chain_message_tracking(&mut self, destination_chain: &str, message_id: u64) {
        let mut chain_messages = self.chain_xcm_messages.get(destination_chain).unwrap_or_default();
        chain_messages.push(message_id);
        self.chain_xcm_messages.insert(destination_chain, &chain_messages);
    }
}
