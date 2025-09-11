# InkTix Smart Contract Integration

This document describes how to use the smart contract integration features in the InkTix frontend.

## Overview

The InkTix platform now includes full smart contract integration capabilities, allowing users to:

- Deploy Ink! smart contracts to the Polkadot blockchain
- Interact with deployed contracts through a user-friendly interface
- Manage sports ticketing operations through decentralized automation

## Prerequisites

Before using the smart contract features, ensure you have:

1. **Polkadot.js Extension**: Install and configure the Polkadot.js browser extension
2. **Network Connection**: Connect to a Polkadot network (Westend testnet recommended for testing)
3. **Account Balance**: Ensure your account has sufficient balance for contract deployment
4. **Compiled Contract**: Have a compiled `.wasm` contract file ready

## Smart Contract Files

The `sports_broker` contract is located in `contracts/sports_broker/` and provides:

### Core Functions

- **Team Management**: Register and manage sports teams
- **Venue Management**: Register and manage event venues
- **Event Creation**: Create ticketing events with team and venue assignments
- **Ticket Purchasing**: Buy tickets for events with seat selection
- **Analytics**: Get totals for teams, venues, events, and tickets

### Contract Structure

```rust
pub struct SportsBroker {
    owner: AccountId,
    teams: Mapping<u32, Team>,
    venues: Mapping<u32, Venue>,
    events: Mapping<u32, Event>,
    tickets: Mapping<u32, Ticket>,
    next_team_id: u32,
    next_venue_id: u32,
    next_event_id: u32,
    next_ticket_id: u32,
}
```

## Building the Contract

To build the smart contract:

```bash
cd contracts/sports_broker
cargo contract build
```

This generates:

- `sports_broker.wasm` - The compiled contract bytecode
- `sports_broker.contract` - Contract bundle with code and metadata
- `sports_broker.json` - Contract metadata for frontend integration

## Frontend Integration

### SmartContractManager Component

The main interface for smart contract interaction is the `SmartContractManager` component, located at:
`frontend/src/components/SmartContractManager.tsx`

#### Features

- **Contract Deployment**: Upload `.wasm` files and deploy with specified endowment
- **Contract Interaction**: Call contract methods with arguments
- **Status Monitoring**: View contract deployment status and address
- **Real-time Results**: Display deployment and call results

#### Usage

1. Navigate to `/smart-contracts` page
2. Connect your wallet and network
3. Upload your compiled `.wasm` contract file
4. Set the endowment amount in DOT
5. Deploy the contract
6. Use the interaction panel to call contract methods

### Blockchain Service

The smart contract integration is built on top of the `BlockchainService` class:
`frontend/src/services/blockchain.ts`

#### Key Methods

- `deployContract(contractWasm, endowment)`: Deploy a new contract
- `callContract(method, args)`: Call a contract method
- `getContractAddress()`: Get the deployed contract address
- `isContractDeployed()`: Check if a contract is deployed

### Blockchain Context

Global state management for smart contracts is handled by:
`frontend/src/contexts/BlockchainContext.tsx`

#### State Variables

- `contractAddress`: The deployed contract address
- `isContractDeployed`: Whether a contract is currently deployed
- `isDeployingContract`: Deployment status indicator

#### Actions

- `deployContract()`: Deploy a new contract
- `callContract()`: Call a contract method

## Contract Methods

### Read Methods (Query)

- `get_total_teams()`: Returns the total number of registered teams
- `get_total_venues()`: Returns the total number of registered venues
- `get_total_events()`: Returns the total number of created events
- `get_total_tickets()`: Returns the total number of purchased tickets
- `get_owner()`: Returns the contract owner address

### Write Methods (Transaction)

- `register_team(name, sport, city)`: Register a new team
- `register_venue(name, capacity, location)`: Register a new venue
- `create_event(name, home_team_id, away_team_id, venue_id, event_date, base_price, total_tickets)`: Create a new event
- `purchase_ticket(event_id, section, row, seat)`: Purchase a ticket for an event

## Testing the Integration

### Local Development

1. Start the frontend: `npm run dev`
2. Navigate to `http://localhost:3000/smart-contracts`
3. Connect your wallet and network
4. Test contract deployment with the compiled `sports_broker.wasm`

### Testnet Deployment

1. Connect to Westend testnet (recommended for testing)
2. Ensure your account has testnet DOT
3. Deploy the contract with a small endowment (e.g., 0.1 DOT)
4. Test contract interactions

## Error Handling

The integration includes comprehensive error handling for:

- Network connection failures
- Wallet connection issues
- Contract deployment failures
- Contract call errors
- Insufficient balance scenarios

## Security Considerations

- **Endowment**: Set appropriate endowment amounts for contract deployment
- **Network Selection**: Use testnets for development and testing
- **Account Management**: Ensure proper account permissions and security
- **Contract Verification**: Verify contract code before deployment

## Future Enhancements

Planned improvements include:

- Contract upgrade functionality
- Multi-contract management
- Advanced analytics and monitoring
- Gas optimization features
- Cross-chain contract deployment

## Troubleshooting

### Common Issues

1. **"Contract not deployed"**: Ensure you've deployed a contract first
2. **"Insufficient balance"**: Check your account balance and endowment amount
3. **"Network connection failed"**: Verify your network endpoint and connection
4. **"Wallet not connected"**: Ensure Polkadot.js extension is properly configured

### Debug Information

- Check browser console for detailed error messages
- Verify network connection status in the UI
- Confirm wallet connection and account selection
- Review contract deployment logs

## Support

For issues or questions regarding the smart contract integration:

1. Check the browser console for error messages
2. Verify all prerequisites are met
3. Ensure you're using the latest version of the codebase
4. Test with the provided `sports_broker` contract first

---

_This integration demonstrates the power of combining Ink! smart contracts with modern web technologies for decentralized sports ticketing._

