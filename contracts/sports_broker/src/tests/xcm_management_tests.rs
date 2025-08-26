use crate::tests::setup_with_test_env;
use crate::types::xcm::*;

#[test]
fn test_send_xcm_message_works() {
    setup_with_test_env(|contract| {
        // First, set up chain connectivity
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "moonbeam".to_string(),
                true,
                Some(50),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        // Send XCM message
        let payload = vec![1, 2, 3, 4, 5];
        let result = contract.send_xcm_message(
            "moonbeam".to_string(),
            2004,
            XcmMessageType::TicketPurchaseRequest,
            payload,
            "DOT".to_string(),
        );

        assert!(result.is_ok());
        let message_id = result.unwrap();
        assert!(message_id > 0);

        // Verify message was stored
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.destination_chain, "moonbeam");
        assert_eq!(message.message_type, XcmMessageType::TicketPurchaseRequest);
        assert_eq!(message.status, XcmMessageStatus::Sent);
    });
}

#[test]
fn test_send_xcm_message_to_unconnected_chain_fails() {
    setup_with_test_env(|contract| {
        let payload = vec![1, 2, 3];
        let result = contract.send_xcm_message(
            "unconnected_chain".to_string(),
            9999,
            XcmMessageType::TicketPurchaseRequest,
            payload,
            "DOT".to_string(),
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Destination chain is not connected");
    });
}

#[test]
fn test_process_incoming_xcm_message_works() {
    setup_with_test_env(|contract| {
        // Set up source chain connectivity
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "astar".to_string(),
                true,
                Some(100),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        // Process incoming message
        let payload = vec![10, 20, 30];
        let result = contract.process_incoming_xcm_message(
            "astar".to_string(),
            2006,
            XcmMessageType::TicketPurchaseResponse,
            payload,
            3,
        );

        assert!(result.is_ok());
        let message_id = result.unwrap();
        assert!(message_id > 0);

        // Verify message was stored
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.source_chain, "astar");
        assert_eq!(message.message_type, XcmMessageType::TicketPurchaseResponse);
        assert_eq!(message.status, XcmMessageStatus::Completed);
    });
}

#[test]
fn test_process_incoming_xcm_message_from_unconnected_chain_fails() {
    setup_with_test_env(|contract| {
        let payload = vec![1, 2, 3];
        let result = contract.process_incoming_xcm_message(
            "unconnected_chain".to_string(),
            9999,
            XcmMessageType::TicketPurchaseRequest,
            payload,
            3,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Source chain is not connected");
    });
}

#[test]
fn test_process_incoming_xcm_message_unsupported_version_fails() {
    setup_with_test_env(|contract| {
        // Set up source chain connectivity
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "polkadot".to_string(),
                true,
                Some(25),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        let payload = vec![1, 2, 3];
        let result = contract.process_incoming_xcm_message(
            "polkadot".to_string(),
            0,
            XcmMessageType::TicketPurchaseRequest,
            payload,
            5, // Unsupported version
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unsupported XCM version");
    });
}

#[test]
fn test_update_xcm_message_status_works() {
    setup_with_test_env(|contract| {
        // First, set up chain connectivity and send a message
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "kusama".to_string(),
                true,
                Some(75),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        let payload = vec![1, 2, 3];
        let message_id = contract
            .send_xcm_message(
                "kusama".to_string(),
                2000,
                XcmMessageType::PaymentConfirmation,
                payload,
                "KSM".to_string(),
            )
            .expect("Failed to send XCM message");

        // Update message status
        let result =
            contract.update_xcm_message_status(message_id, XcmMessageStatus::Processing, None);

        assert!(result.is_ok());

        // Verify status was updated
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.status, XcmMessageStatus::Processing);
    });
}

#[test]
fn test_update_xcm_message_status_with_error_works() {
    setup_with_test_env(|contract| {
        // First, set up chain connectivity and send a message
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "bifrost".to_string(),
                true,
                Some(60),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        let payload = vec![1, 2, 3];
        let message_id = contract
            .send_xcm_message(
                "bifrost".to_string(),
                2001,
                XcmMessageType::TicketTransfer,
                payload,
                "BIF".to_string(),
            )
            .expect("Failed to send XCM message");

        // Update message status with error
        let error_message = "Insufficient funds".to_string();
        let result = contract.update_xcm_message_status(
            message_id,
            XcmMessageStatus::Failed,
            Some(error_message.clone()),
        );

        assert!(result.is_ok());

        // Verify status and error message were updated
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.status, XcmMessageStatus::Failed);
        assert_eq!(message.error_message, Some(error_message));
    });
}

#[test]
fn test_update_xcm_message_status_nonexistent_message_fails() {
    setup_with_test_env(|contract| {
        let result = contract.update_xcm_message_status(99999, XcmMessageStatus::Completed, None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "XCM message not found");
    });
}

#[test]
fn test_send_xcm_ticket_purchase_request_works() {
    setup_with_test_env(|contract| {
        // Set up destination chain connectivity
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "karura".to_string(),
                true,
                Some(80),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        // Send ticket purchase request
        let preferred_sections = vec!["A1".to_string(), "A2".to_string()];
        let user_signature = vec![1, 2, 3, 4, 5];
        let result = contract.send_xcm_ticket_purchase_request(
            "karura".to_string(),
            2000,
            123,
            2,
            preferred_sections,
            "KAR".to_string(),
            1000000,
            user_signature,
        );

        assert!(result.is_ok());
        let message_id = result.unwrap();
        assert!(message_id > 0);

        // Verify message was stored
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.message_type, XcmMessageType::TicketPurchaseRequest);
        assert_eq!(message.destination_chain, "karura");
    });
}

#[test]
fn test_process_xcm_ticket_purchase_response_success_works() {
    setup_with_test_env(|contract| {
        // First, set up chain connectivity and send a request
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "moonbeam".to_string(),
                true,
                Some(50),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        let payload = vec![1, 2, 3];
        let message_id = contract
            .send_xcm_message(
                "moonbeam".to_string(),
                2004,
                XcmMessageType::TicketPurchaseRequest,
                payload,
                "GLMR".to_string(),
            )
            .expect("Failed to send XCM message");

        // Process successful response
        let ticket_ids = vec![1001, 1002];
        let transaction_hash = "0x1234567890abcdef".to_string();
        let result = contract.process_xcm_ticket_purchase_response(
            message_id,
            true,
            Some(ticket_ids.clone()),
            None,
            Some(transaction_hash.clone()),
        );

        assert!(result.is_ok());

        // Verify message status was updated
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.status, XcmMessageStatus::Completed);
    });
}

#[test]
fn test_process_xcm_ticket_purchase_response_failure_works() {
    setup_with_test_env(|contract| {
        // First, set up chain connectivity and send a request
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "astar".to_string(),
                true,
                Some(100),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        let payload = vec![1, 2, 3];
        let message_id = contract
            .send_xcm_message(
                "astar".to_string(),
                2006,
                XcmMessageType::TicketPurchaseRequest,
                payload,
                "ASTR".to_string(),
            )
            .expect("Failed to send XCM message");

        // Process failed response
        let error_message = "Event sold out".to_string();
        let result = contract.process_xcm_ticket_purchase_response(
            message_id,
            false,
            None,
            Some(error_message.clone()),
            None,
        );

        assert!(result.is_ok());

        // Verify message status was updated
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.status, XcmMessageStatus::Failed);
        assert_eq!(message.error_message, Some(error_message));
    });
}

#[test]
fn test_process_xcm_ticket_purchase_response_nonexistent_message_fails() {
    setup_with_test_env(|contract| {
        let result = contract.process_xcm_ticket_purchase_response(
            99999,
            true,
            Some(vec![1, 2, 3]),
            None,
            Some("0x123".to_string()),
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Original request message not found");
    });
}

#[test]
fn test_get_xcm_analytics_works() {
    setup_with_test_env(|contract| {
        // Set up multiple chain connections
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        // Connect to multiple chains
        contract
            .update_xcm_chain_connectivity(
                "moonbeam".to_string(),
                true,
                Some(50),
                3,
                1024,
                fee_structure.clone(),
            )
            .expect("Failed to update chain connectivity");

        contract
            .update_xcm_chain_connectivity(
                "astar".to_string(),
                true,
                Some(100),
                3,
                1024,
                fee_structure.clone(),
            )
            .expect("Failed to update chain connectivity");

        // Send some messages
        let payload = vec![1, 2, 3];
        contract
            .send_xcm_message(
                "moonbeam".to_string(),
                2004,
                XcmMessageType::TicketPurchaseRequest,
                payload.clone(),
                "GLMR".to_string(),
            )
            .expect("Failed to send XCM message");

        contract
            .send_xcm_message(
                "astar".to_string(),
                2006,
                XcmMessageType::PaymentConfirmation,
                payload,
                "ASTR".to_string(),
            )
            .expect("Failed to send XCM message");

        // Get analytics
        let analytics = contract.get_xcm_analytics();

        assert_eq!(analytics.total_messages_sent, 2);
        assert_eq!(analytics.total_messages_received, 0);
        assert_eq!(analytics.connected_chains_count, 2);
    });
}

#[test]
fn test_update_xcm_chain_connectivity_works() {
    setup_with_test_env(|contract| {
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        // Update connectivity
        let result = contract.update_xcm_chain_connectivity(
            "test_chain".to_string(),
            true,
            Some(25),
            3,
            1024,
            fee_structure,
        );

        assert!(result.is_ok());

        // Verify connectivity was updated
        let analytics = contract.get_xcm_analytics();
        assert_eq!(analytics.connected_chains_count, 1);

        // Disconnect chain
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        let result = contract.update_xcm_chain_connectivity(
            "test_chain".to_string(),
            false,
            None,
            3,
            1024,
            fee_structure,
        );

        assert!(result.is_ok());

        // Verify chain was disconnected
        let analytics = contract.get_xcm_analytics();
        assert_eq!(analytics.connected_chains_count, 0);
    });
}

#[test]
fn test_xcm_message_fee_calculation_works() {
    setup_with_test_env(|contract| {
        let fee_structure = XcmFeeStructure {
            base_fee: 1000,
            fee_per_byte: 10,
            ticket_purchase_fee: 5000,
            payment_confirmation_fee: 2000,
            ticket_transfer_fee: 3000,
            fee_currency: "DOT".to_string(),
        };

        contract
            .update_xcm_chain_connectivity(
                "fee_test_chain".to_string(),
                true,
                Some(50),
                3,
                1024,
                fee_structure,
            )
            .expect("Failed to update chain connectivity");

        // Send message with specific payload size
        let payload = vec![0u8; 100]; // 100 bytes
        let result = contract.send_xcm_message(
            "fee_test_chain".to_string(),
            9999,
            XcmMessageType::TicketPurchaseRequest,
            payload,
            "DOT".to_string(),
        );

        assert!(result.is_ok());
        let message_id = result.unwrap();

        // Verify fee was calculated correctly
        let message = contract.get_xcm_message(message_id);
        assert!(message.is_some());
        let message = message.unwrap();

        // Expected fee: base_fee (1000) + per_byte_fee (10 * 100) + ticket_purchase_fee (5000) = 7000
        assert_eq!(message.fee_paid, 7000);
        assert_eq!(message.fee_currency, "DOT");
    });
}
