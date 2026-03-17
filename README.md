# InkTix: Cross-Chain Ticket Marketplace on Polkadot

<p align="center">
  <img src="./docs/InkTix_logo.png" alt="InkTix Logo" width="425">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-v1.0.0-blue" alt="version" />
  <img src="https://img.shields.io/badge/ink!-5.1.1-orange" alt="ink!" />
  <img src="https://img.shields.io/badge/Next.js-15-black" alt="Next.js" />
  <img src="https://img.shields.io/badge/tests-14_contract_|_31_frontend-green" alt="tests" />
  <img src="https://img.shields.io/badge/license-Apache_2.0-green" alt="license" />
</p>

> Decentralized event ticketing with dynamic pricing, NFT tickets, anti-scalping, and cross-chain transfers — built with ink! smart contracts on Polkadot.

**Live Demo:** [inktix.rycsprojects.com](https://inktix.rycsprojects.com) | [inktix.com](https://inktix.com)

---

## What is InkTix?

InkTix is a cross-chain ticket marketplace that solves the ticketing industry's biggest problems — scalping, fraud, opaque pricing, and siloed platforms — using Polkadot's multi-chain architecture.

**For fans:** Browse events, see transparent dynamic pricing, buy tickets with multiple currencies, and get fraud-proof NFT tickets with QR codes.

**For organizers:** On-chain anti-scalping enforcement, real-time analytics, multi-currency revenue, and cross-chain event distribution.

**For the ecosystem:** A production-grade demonstration of ink! smart contracts, XCM cross-chain transfers, and Polkadot.js frontend integration.

---

## Features

| Feature | Description |
|---------|-------------|
| **Dynamic Pricing** | 6-factor algorithm (demand, time, seat type, team performance, rivalry, season pass) with transparent breakdown |
| **NFT Tickets** | Blake2x256 verification hashes, QR codes, attendance tokens, on-chain ownership proof |
| **Anti-Scalping** | Configurable purchase limits, transfer cooldowns, resale price caps (1.5x max), bot detection |
| **Resale Marketplace** | List and buy resale tickets with anti-scalping enforcement and price transparency |
| **Cross-Chain (XCM)** | Real `limitedReserveTransferAssets` from Westend Asset Hub to destination parachains |
| **Multi-Currency** | Pay with DOT, KSM, aUSD, ACA, or LDOT with on-chain exchange rates |
| **Group Buy** | Create group codes, share with friends, adjacent seats, 5% group discount |
| **Analytics Dashboard** | Platform KPIs, revenue charts, category breakdown, top events, demand heatmap |
| **Share Events** | Copy link, post to X/Twitter, Web Share API on mobile |

---

## Quick Start

### Prerequisites

- [Node.js 22+](https://nodejs.org/) (developed with v25.2.1)
- [Rust 1.92+](https://rustup.rs/) with `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- [cargo-contract 5.0+](https://github.com/paritytech/cargo-contract) (`cargo install cargo-contract`)
- A Polkadot wallet extension ([Polkadot.js](https://polkadot.js.org/extension/), [Talisman](https://talisman.xyz/), or [SubWallet](https://subwallet.app/))

### Run Locally

```bash
# Clone the repo
git clone https://github.com/RycBrownrigg/InkTix.git
cd InkTix

# Start the frontend
cd frontend
npm install
npm run dev
# → http://localhost:3000

# In another terminal, run contract tests
cd contracts/inktix
cargo test
```

### Run with Docker (one command)

```bash
docker compose up
# Starts a local Substrate node + frontend dev server
```

### Build the Contract

```bash
cd contracts/inktix
cargo contract build           # Debug build
cargo contract build --release # Release build → target/ink/inktix.wasm
cargo test                     # Run 14 unit tests
```

---

## Project Structure

```
InkTix/
├── contracts/inktix/              # Unified ink! 5.1.1 smart contract
│   └── src/
│       ├── lib.rs                 # 52 message handlers, constructor, inline tests
│       ├── types/                 # Core, Sports, Concert type definitions
│       ├── storage/               # InkTixStorage — 100+ Mapping fields
│       ├── logic/                 # Core, Sports, Concert business logic
│       ├── utils/                 # Validation, currency conversion
│       └── tests/                 # Core, Sports, Concert test suites
│
├── frontend/                      # Next.js 15 static export
│   └── src/
│       ├── app/                   # 10 pages (events, resale, my-tickets, analytics, ...)
│       ├── sdk/                   # Typed SDK: InkTixSDK interface, MockProvider, ContractProvider
│       ├── store/                 # Zustand: 4 slices + persistence + migration
│       ├── config/                # Chain config (endpoints, tokens, mock mode)
│       ├── hooks/                 # useBlockchain backward-compatible hook
│       ├── services/              # BlockchainService singleton, XCM service
│       ├── components/            # Smart contract UI components
│       └── utils/                 # Contract method helpers
│
├── scripts/                       # Deployment and setup scripts
│   ├── deploy_inktix.sh           # Deploy to inktix.rycsprojects.com
│   ├── deploy_inktix_com.sh       # Deploy to inktix.com
│   ├── deploy-and-configure.sh    # Build contract + deploy to local node
│   └── nginx/inktix.conf          # Nginx config with SSL, gzip, caching
│
├── .github/workflows/ci.yml       # CI: cargo test + npm lint/build/test
├── docker-compose.yml             # Substrate node + frontend
├── CLAUDE.md                      # AI assistant context
└── docs/
    ├── product_specification.md   # Full product spec
    ├── HACKATHON_SUBMISSION.md    # Hackathon submission details
    └── system_architecture.md     # Architecture documentation
```

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                     User's Browser                           │
│                                                              │
│  Next.js 15 Static App          Polkadot Wallet Extension    │
│  ┌───────────────────┐           ┌──────────────────┐        │
│  │ Zustand Store     │           │ Polkadot.js      │        │
│  │ (4 slices)        │           │ Talisman         │        │
│  │                   │           │ SubWallet        │        │
│  │ SDK Layer         │◄─────────►│ Nova             │        │
│  │ ├ MockProvider    │  signing  └──────────────────┘        │
│  │ └ ContractProvider│                                       │
│  └────────┬──────────┘                                       │
│           │                                                  │
└───────────┼──────────────────────────────────────────────────┘
            │ WebSocket (wss://)
            ▼
┌───────────────────────────────────────┐
│        Westend Asset Hub              │
│                                       │
│  ┌─────────────────────────────┐      │
│  │    InkTix Contract (ink!)   │      │
│  │                             │      │
│  │  Events · Tickets · Venues  │      │
│  │  NFTs · Pricing · Resale    │      │
│  │  Teams · Artists · Loyalty  │      │
│  │  XCM · Analytics            │      │
│  └─────────────────────────────┘      │
│                                       │
│  XCM ──► Other Parachains             │
└───────────────────────────────────────┘
```

---

## Smart Contract

The unified contract (`contracts/inktix/`) consolidates three legacy contracts into one with Cargo feature flags:

```toml
[features]
default = ["std", "sports", "concert"]
sports = []   # Teams, seasons, fantasy sports, loyalty programs
concert = []  # Artist management, concert-specific anti-scalping
```

### Key Message Handlers (52 total)

| Category | Methods |
|----------|---------|
| **Events** | `create_event`, `get_event`, `get_all_events`, `update_event_status` |
| **Tickets** | `purchase_ticket`, `get_ticket`, `transfer_ticket`, `resell_ticket` |
| **Pricing** | `get_price_quote`, `set_dynamic_pricing`, `update_team_performance` |
| **NFT** | `mint_ticket_nft`, `verify_ticket_nft`, `use_ticket_nft`, `transfer_nft` |
| **Venues** | `register_venue`, `get_venue`, `update_venue_capacity` |
| **Anti-Scalping** | `configure_anti_scalping`, `get_anti_scalping_config` |
| **Sports** | `register_team`, `create_season_pass_package`, `create_fantasy_league`, `stake_on_team` |
| **Concert** | `register_artist`, `verify_artist`, `create_concert_event` |
| **Analytics** | `get_platform_stats`, `generate_analytics_report`, `get_event_analytics` |
| **XCM** | `create_cross_chain_event`, `request_cross_chain_ticket_purchase` |
| **Currency** | `get_supported_currencies`, `update_currency_rate` |

---

## Deployment

### Deploy to inktix.rycsprojects.com

```bash
./scripts/deploy_inktix.sh
# Builds frontend → uploads via SCP → atomic swap → Nginx reload
```

### Deploy to inktix.com

```bash
./scripts/deploy_inktix_com.sh
# Builds frontend → uploads via SCP (port 21098) → .htaccess setup
```

### Deploy Contract to Local Node

```bash
./scripts/deploy-and-configure.sh
# Builds contract → deploys to ws://127.0.0.1:9944 → writes address to .env.local
```

---

## Testing

```bash
# Contract tests (14 tests)
cd contracts/inktix && cargo test

# Frontend unit tests (31 tests)
cd frontend && npm test

# Frontend e2e tests (3 specs)
cd frontend && npm run test:e2e

# Lint
cd frontend && npm run lint
```

---

## Environment Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `NEXT_PUBLIC_RPC_ENDPOINT` | Substrate RPC WebSocket URL | `wss://westend-asset-hub-rpc.polkadot.io` |
| `NEXT_PUBLIC_MOCK_MODE` | Use mock data instead of contract calls | `true` (dev), `false` (prod) |
| `NEXT_PUBLIC_CONTRACT_ADDRESS` | Deployed contract address | (empty) |
| `NEXT_PUBLIC_BASE_PATH` | URL base path (empty for root) | (empty) |

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Smart Contract | Rust 1.92, ink! 5.1.1, cargo-contract 5.0.3 |
| Frontend | Next.js 15.5, React 19.1, TypeScript 5.9, Tailwind CSS 3.4 |
| State Management | Zustand 5.0 with persist middleware |
| Blockchain API | @polkadot/api 16.4, @polkadot/api-contract 16.5, @polkadot/extension-dapp 0.61 |
| QR Codes | qrcode.react |
| Testing | Vitest 3.2, Testing Library 16.3, Playwright 1.58 |
| CI/CD | GitHub Actions |
| Deployment | Nginx (VPS), Apache/LiteSpeed (shared hosting) |
| Dev Environment | Docker Compose (substrate-contracts-node + Node 20 Alpine) |

---

## Contributing

Contributions are welcome. Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Run tests (`cargo test` and `npm test`)
4. Submit a pull request

---

## Documentation

- [Product Specification](./docs/product_specification.md) — Full product spec with market analysis
- [System Architecture](./docs/system_architecture.md) — Technical architecture details
- [Hackathon Submission](./docs/HACKATHON_SUBMISSION.md) — Polkadot Alumni Hackathon submission
- [CLAUDE.md](./CLAUDE.md) — AI assistant context and build commands

---

## License

Apache 2.0 — see [LICENSE](./LICENSE) for details.

---

<p align="center">
  Built with ink!, Polkadot.js, Next.js, and Zustand for the Polkadot ecosystem
</p>
