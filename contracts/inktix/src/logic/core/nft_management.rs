use crate::storage::contract_storage::InkTixStorage;
use crate::types::*;
use ink::prelude::string::String;
use ink::prelude::string::ToString;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

pub struct NftManagement;

impl NftManagement {
    /// Mint an NFT for an existing ticket
    pub fn mint_ticket_nft(
        storage: &mut InkTixStorage,
        caller: AccountId,
        ticket_id: u64,
    ) -> Result<u64, String> {
        // Check ticket exists and caller owns it
        let ticket = storage.tickets.get(ticket_id)
            .ok_or("Ticket not found".to_string())?;

        if ticket.owner != caller {
            return Err("Only ticket owner can mint NFT".to_string());
        }

        // Check if NFT already minted for this ticket
        if storage.ticket_to_nft.get(ticket_id).is_some() {
            return Err("NFT already minted for this ticket".to_string());
        }

        // Get event info for metadata
        let event = storage.events.get(ticket.event_id)
            .ok_or("Event not found".to_string())?;

        // Get venue name
        let venue_name = storage.venues.get(event.venue_id)
            .map(|v| v.name.clone())
            .unwrap_or_else(|| "Unknown Venue".to_string());

        let token_id = storage.get_next_nft_token_id();

        // Generate verification hash from ticket data
        let mut hash_input: Vec<u8> = Vec::new();
        hash_input.extend_from_slice(&ticket_id.to_le_bytes());
        hash_input.extend_from_slice(&ticket.event_id.to_le_bytes());
        hash_input.extend_from_slice(&ticket.seat_number.to_le_bytes());
        hash_input.extend_from_slice(caller.as_ref());
        hash_input.extend_from_slice(&token_id.to_le_bytes());

        let mut verification_hash = [0u8; 32];
        ink::env::hash_bytes::<ink::env::hash::Blake2x256>(&hash_input, &mut verification_hash);

        let nft = TicketNft {
            token_id,
            ticket_id,
            owner: caller,
            event_id: ticket.event_id,
            event_name: event.name.clone(),
            venue_name,
            event_date: event.date,
            section: ticket.section.clone(),
            row: ticket.row.clone(),
            seat_number: ticket.seat_number,
            seat_type: "Reserved".to_string(),
            access_level: "Standard".to_string(),
            minted_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
            metadata_uri: "".to_string(),
            verification_hash,
            is_used: false,
        };

        storage.nft_tickets.insert(token_id, &nft);
        storage.ticket_to_nft.insert(ticket_id, &token_id);

        let mut user_tokens = storage.user_nft_tokens.get(caller).unwrap_or_default();
        user_tokens.push(token_id);
        storage.user_nft_tokens.insert(caller, &user_tokens);

        Ok(token_id)
    }

    /// Verify a ticket NFT by its token_id
    pub fn verify_ticket_nft(
        storage: &InkTixStorage,
        token_id: u64,
    ) -> Result<TicketVerification, String> {
        let nft = storage.nft_tickets.get(token_id)
            .ok_or("NFT not found".to_string())?;

        Ok(TicketVerification {
            is_valid: true,
            is_used: nft.is_used,
            owner: nft.owner,
            event_id: nft.event_id,
            event_name: nft.event_name,
            section: nft.section,
            row: nft.row,
            seat_number: nft.seat_number,
        })
    }

    /// Mark ticket as used (for event entry)
    pub fn use_ticket_nft(
        storage: &mut InkTixStorage,
        _caller: AccountId,
        token_id: u64,
    ) -> Result<u64, String> {
        let mut nft = storage.nft_tickets.get(token_id)
            .ok_or("NFT not found".to_string())?;

        if nft.is_used {
            return Err("Ticket already used".to_string());
        }

        // Mark as used
        nft.is_used = true;
        storage.nft_tickets.insert(token_id, &nft);

        // Mint attendance token
        let attendance_id = storage.get_next_attendance_token_id();
        let attendance = AttendanceToken {
            token_id: attendance_id,
            ticket_id: nft.ticket_id,
            event_id: nft.event_id,
            owner: nft.owner,
            attended_at: ink::env::block_timestamp::<ink::env::DefaultEnvironment>(),
        };

        storage.attendance_tokens.insert(attendance_id, &attendance);
        let mut user_attendance = storage.user_attendance_tokens.get(nft.owner).unwrap_or_default();
        user_attendance.push(attendance_id);
        storage.user_attendance_tokens.insert(nft.owner, &user_attendance);

        Ok(attendance_id)
    }

    /// Get all NFT tokens for a user
    pub fn get_user_nft_tickets(
        storage: &InkTixStorage,
        user: AccountId,
    ) -> Vec<TicketNft> {
        let token_ids = storage.user_nft_tokens.get(user).unwrap_or_default();
        token_ids.iter()
            .filter_map(|id| storage.nft_tickets.get(*id))
            .collect()
    }

    /// Get NFT by ticket_id
    pub fn get_nft_by_ticket(
        storage: &InkTixStorage,
        ticket_id: u64,
    ) -> Option<TicketNft> {
        storage.ticket_to_nft.get(ticket_id)
            .and_then(|token_id| storage.nft_tickets.get(token_id))
    }

    /// Transfer NFT to new owner (follows ticket transfer)
    pub fn transfer_nft(
        storage: &mut InkTixStorage,
        caller: AccountId,
        token_id: u64,
        to: AccountId,
    ) -> Result<(), String> {
        let mut nft = storage.nft_tickets.get(token_id)
            .ok_or("NFT not found".to_string())?;

        if nft.owner != caller {
            return Err("Not NFT owner".to_string());
        }

        // Remove from old owner
        let mut old_tokens = storage.user_nft_tokens.get(caller).unwrap_or_default();
        old_tokens.retain(|&id| id != token_id);
        storage.user_nft_tokens.insert(caller, &old_tokens);

        // Add to new owner
        nft.owner = to;
        storage.nft_tickets.insert(token_id, &nft);
        let mut new_tokens = storage.user_nft_tokens.get(to).unwrap_or_default();
        new_tokens.push(token_id);
        storage.user_nft_tokens.insert(to, &new_tokens);

        Ok(())
    }
}
