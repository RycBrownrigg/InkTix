# InkTix: Cross-Chain Ticket Marketplace Network

<p align="center">
  <img src="./docs/InkTix_logo.png" alt="InkTix Logo" width="350">
</p>

![version](https://img.shields.io/badge/version-v0.1.0-blue)
![status](https://img.shields.io/badge/status-alpha-orange)
![license](https://img.shields.io/badge/license-MIT-green)

> A decentralized, cross-chain ticketing marketplace built with ink! smart contracts on Polkadot.

---

## Overview

**InkTix** is a cross-chain ticket marketplace that connects event brokers across multiple Polkadot parachains. Built with Rust and ink!, the system enables seamless ticket discovery, fraud-resistant NFT ticket trading, and cross-chain ownership using XCM.

---

## Documentation

- [Product Specification](./docs/product_specification.md)
- [System Architecture](./docs/system_architecture.md)
- [Changelog](./CHANGELOG.md)

---

## Project Structure

```bash
InkTix/
├── README.md                 # Main documentation
├── CHANGELOG.md              # Version history
├── docs/                     # Supporting documentation
│   ├── product_specification.md
│   └── system_architecture.md
├── contracts/                # ink! smart contracts
├── tests/                    # Cross-contract integration tests
├── scripts/                  # Build and deploy scripts
├── examples/                 # Example clients or usage
└── .gitignore
```

## Current Development Status 2025-09-03

### WORKING CONTRACTS

- **concert_broker v0.3.0**: Basic template, builds successfully
- **sports_broker v0.2.0**: Advanced sports ticketing with team management, venue registration, event creation, and ticket purchasing
- **inktix_core v0.1.0**: Foundation template with common types and utilities

### FRONTEND APPLICATION

- **Next.js Frontend**: Modern React application with TypeScript
- **Wallet Integration**: Full Polkadot wallet connection (Polkadot.js, Talisman, SubWallet, Nova)
- **Smart Contract Manager**: Deploy and interact with contracts through web interface
- **Responsive Design**: Mobile-first UI with Tailwind CSS
- **Blockchain Integration**: Complete Web3 integration with mock and real contract support

### BUILD COMMANDS

```bash
# Build smart contracts
cd contracts/concert_broker && cargo contract build
cd contracts/sports_broker && cargo contract build

# Start frontend development server
cd frontend && npm install && npm run dev
```

### QUICK START

1. **Start Substrate Node** (for contract deployment):

   ```bash
   docker run --rm -p 9944:9944 parity/substrate:latest --dev --tmp --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe --execution=wasm --validator
   ```

2. **Start Frontend**:

   ```bash
   cd frontend && npm run dev
   ```

3. **Access Application**: Navigate to [http://localhost:3000](http://localhost:3000)

### FEATURES IMPLEMENTED

- Smart contract deployment and interaction
- Wallet connection and account management
- Contract method calling (get_total_teams, get_total_venues, etc.)
- Responsive UI for all screen sizes
- Mock mode for development and testing
- Real Substrate node integration
