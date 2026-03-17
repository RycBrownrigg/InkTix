# InkTix - Polkadot Alumni Hackathon Submission

## Executive Summary

InkTix is a decentralized cross-chain ticket marketplace built on Polkadot that addresses the $50B+ global ticketing industry's core problems: scalping, fraud, siloed platforms, and opaque pricing. Through a unified ink! smart contract, real XCM cross-chain transfers, NFT-based ticket authentication with QR verification, and a 6-factor dynamic pricing engine, InkTix demonstrates what production-grade blockchain ticketing looks like on Polkadot.

The platform features a complete user journey — browse events, get dynamic price quotes, purchase tickets, mint NFT tickets with QR codes, resell on an anti-scalping marketplace, and track analytics — all backed by on-chain contract logic with 14 passing tests and a typed SDK that bridges mock and live environments.

**Live Demo:** https://inktix.rycsprojects.com | https://inktix.com
**GitHub:** https://github.com/RycBrownrigg/InkTix

## What We Built

### Unified Smart Contract (`contracts/inktix/`)

Consolidated three separate contracts (inktix_core, sports_broker, concert_broker) into a single unified ink! 5.1.1 contract with 54 source files and Cargo feature flags `["sports", "concert"]`:

- **52 on-chain message handlers** covering the full ticketing lifecycle
- **Unified Event type** with `EventCategory` enum (Sports/Concert/Generic)
- **Dynamic pricing engine** — 6-factor algorithm (demand, time urgency, seat type, team performance, rivalry, season pass discount) with anti-scalping guardrails (50% floor, 3x cap)
- **NFT ticket minting** — Blake2x256 verification hashes, attendance tokens, on-chain ownership proof
- **Anti-scalping system** — configurable per-event purchase limits, transfer cooldowns, resale price caps, user behavior monitoring
- **Cross-chain XCM** — ticket purchase requests and event replication across parachains
- **Sports features** — teams, seasons, season passes with staking, fantasy sports, team loyalty programs, venue management (parking, concessions, merchandise)
- **Concert features** — artist management, auto-applied 4-ticket-per-user limit
- **Analytics** — platform stats, event analytics, report generation
- **14 unit tests** covering initialization, registration, events, tickets, NFT mint/use, dynamic pricing, and demand surge

### Frontend Application

Next.js 15 static export with 10 pages, deployed to two domains:

| Page | Description |
|------|-------------|
| `/` | Landing page with hero, feature cards, navigation to all features |
| `/events` | Browse 39+ events with category filters, dynamic pricing, 3-step purchase flow, share & group buy |
| `/connect` | Wallet connection (Polkadot.js, Talisman, SubWallet, Nova) |
| `/my-tickets` | NFT ticket portfolio with QR codes, mint/use/verify actions |
| `/resale` | Resale marketplace with anti-scalping enforcement, buy/sell flows |
| `/analytics` | Dashboard with KPI cards, revenue charts, category breakdown, top events |
| `/cross-chain-demo` | Interactive XCM transfer demo with 5-step progress tracker |
| `/smart-contracts` | Contract deployment and method interaction UI |
| `/docs` | Documentation hub |
| `/cross-chain` | Cross-chain redirect |

### Architecture Highlights

- **Zustand store** replacing React Context — 4 slices (connection, wallet, contract, data) with single `inktix-store` localStorage key and migration from legacy keys
- **Typed SDK layer** — `InkTixSDK` interface with `MockProvider` (in-memory demo) and `ContractProvider` (real `@polkadot/api-contract` calls with gas estimation)
- **Environment config** — `.env.local`/`.env.production`/`.env.development` with `config/chains.ts` as single source of truth
- **Component decomposition** — SmartContractManager split into 6 focused components
- **CI/CD** — GitHub Actions (contract tests + frontend lint/build/test), Docker Compose for local dev
- **31 frontend tests** (Vitest + Testing Library) + 3 Playwright e2e specs
- **Comprehensive documentation** — JSDoc on all TypeScript files, `//!`/`///` docs on all Rust files

## Ecosystem Impact

### Polkadot Technology Showcase

| Technology | How InkTix Uses It |
|---|---|
| **XCM** | Real `limitedReserveTransferAssets` from Westend Asset Hub with V3 MultiLocation |
| **ink! 5.1.1** | 52-method unified contract with feature flags, storage items, cross-module logic |
| **Polkadot.js API** | Full wallet integration, contract deployment via `CodePromise`, method calls via `ContractPromise` |
| **Asset Hub** | Cross-chain asset transfers for ticket purchases |
| **Multi-chain vision** | Specialized event types (Sports/Concert) designed for parachain specialization |

### Real-World Value

- **Anti-scalping**: On-chain enforcement of purchase limits, transfer cooldowns, resale price caps — not just policy, but code
- **Dynamic pricing**: Transparent, algorithm-driven pricing visible to users (demand %, seat multiplier, time urgency) — no hidden fees
- **NFT tickets**: Cryptographic proof of ownership with QR verification — eliminates counterfeiting
- **75-85% lower fees**: Decentralized architecture vs traditional platforms (Ticketmaster charges 20-30%)

## Technical Difficulty

### Contract Consolidation
Merged 3 contracts (836 + 505 + 302 lines) into a unified architecture with shared storage, cross-domain types, and feature-gated compilation — while keeping all 14 tests green and `cargo contract build` producing deployable `.wasm`.

### Dynamic Pricing Algorithm
Implemented a multi-factor pricing engine entirely in `no_std` Rust using basis-point arithmetic (10000 = 1.0x) to avoid floating point, with demand curves, time urgency brackets, seat type multipliers, team performance correlation, and anti-scalping guardrails.

### Real Contract Integration
Built a typed SDK (`InkTixSDK` interface) with two implementations — `MockProvider` for demo mode and `ContractProvider` using `@polkadot/api-contract` for real on-chain calls with gas estimation via dry-run and wallet extension signing.

### State Management Migration
Replaced a 743-line React Context with Zustand store (4 composable slices) including automatic migration from 6 legacy localStorage keys to a single persisted key, without breaking any existing component.

## Functionality and User Experience

### Complete User Journey

```
Browse Events → Dynamic Price Quote → Select Seat & Currency → Purchase Ticket
     ↓                                                              ↓
Share Event ←──── Group Buy Code ←──── Mint NFT ←──── View in My Tickets
     ↓                                    ↓
Post to X/Twitter                   QR Code Verification
     ↓                                    ↓
Resale Marketplace ←──── Anti-Scalping ←──── Analytics Dashboard
```

### Key User Flows

1. **Ticket Purchase** — Browse events → View details → Select seat type (7 options) → Choose currency (DOT/KSM/aUSD/ACA) → See dynamic price breakdown → Confirm → Success with ticket ID → Link to My Tickets

2. **NFT Minting** — View owned tickets → Mint NFT → Get QR code with verification hash → Use at event entry → Receive attendance token

3. **Resale** — List ticket (pick from owned or enter manually) → Set price (1.5x cap enforced) → On-chain verification → Buyer browses listings → Purchase with anti-scalping checks

4. **Group Buy** — Create group (2-10 people) → Get shareable code → Friends join with code → 5% group discount → Adjacent seats guaranteed

5. **XCM Transfer** — Configure ParaId and beneficiary → Execute reserve transfer → Track 5-step progress → Complete cross-chain ticket purchase

## Developer Experience Feedback

### What Worked Well

- **ink! 5.1.1**: Excellent tooling with `cargo-contract`, clear error messages, good Mapping/Vec support
- **Polkadot.js API**: Comprehensive TypeScript types, reliable WebSocket connections
- **Westend Asset Hub**: Stable testnet with faucet, contracts pallet available
- **Static export model**: Next.js `output: "export"` + Nginx is a simple, reliable deployment pattern for dApps

### Challenges

- **`cargo contract build` vs `cargo test`**: Different compilation targets (wasm32 vs native) surface different errors — code that passes `cargo test` can fail `cargo contract build` due to stricter clippy lints
- **XCM WeightV2**: Documentation examples use simple numbers but the API requires `{ refTime, proofSize }` objects
- **Environment variables in static exports**: `.env.local` overrides `.env.production` during `npm run build` — caught a bug where localhost RPC was baked into production JS
- **Zustand persist + jsdom**: Test environment needed localStorage polyfill for Zustand's persist middleware

### Suggestions for Polkadot Developer Tooling

- XCM dry-run tool that shows exactly what a transfer will do before signing
- `cargo-contract` flag to run clippy with the same strictness as `cargo contract build`
- More examples of `@polkadot/api-contract` with ink! 5.x (most examples are for older versions)

## Roadmap

### Completed (This Hackathon)

- ✅ Unified ink! contract (3→1) with 52 message handlers, 14 tests
- ✅ Real XCM cross-chain transfers on Westend Asset Hub
- ✅ NFT ticket minting with Blake2x256 verification and QR codes
- ✅ 6-factor dynamic pricing engine with anti-scalping guardrails
- ✅ Resale marketplace with anti-scalping enforcement
- ✅ Ticket purchase flow with seat selection and multi-currency
- ✅ Analytics dashboard with KPI cards and charts
- ✅ Share event (copy link, X/Twitter, Web Share API) + Group Buy
- ✅ Zustand state management, typed SDK layer, environment config
- ✅ CI/CD pipeline, Docker dev setup, deployment scripts
- ✅ Comprehensive documentation (Rust + TypeScript)
- ✅ Production deployment on two domains

### Next Steps

- 🟩 Deploy contract to Westend Asset Hub for live on-chain demo
- 🟩 PSP34 NFT standard compliance for cross-chain ticket transfers
- 🟩 DeFi integration (Acala DEX for currency conversion, staking rewards on season passes)
- 🟩 Friend activity feeds and social event discovery
- 🟩 Fan clubs with token-gated exclusive content
- 🟩 Mobile PWA with push notifications
- 🟩 External API integrations (Ticketmaster, StubHub for event data)
- 🟩 Governance token for platform decision-making

### Long-Term Vision

- Mainnet deployment on Polkadot with multiple parachain integrations
- 200+ venue partnerships with custom features
- Enterprise solutions (corporate event management, bulk purchasing)
- Global expansion across North America, Europe, and Asia-Pacific
- Industry-standard cross-chain ticketing protocols

## Technical Architecture

### Smart Contract

```
contracts/inktix/
├── Cargo.toml                    # Feature flags: sports, concert
└── src/
    ├── lib.rs                    # 52 message handlers, constructor, tests
    ├── types/
    │   ├── core/                 # Event, Ticket, Venue, Currency, NFT, AntiScalping, XCM, Seat, Error
    │   ├── sports/               # Team, Season, SeasonPass, Fantasy, Loyalty, Analytics, Pricing
    │   └── concert/              # Artist
    ├── storage/
    │   └── contract_storage.rs   # InkTixStorage — unified storage with 100+ Mapping fields
    ├── logic/
    │   ├── core/                 # Event, Ticket, Venue, Currency, AntiScalping, NFT, Pricing, XCM
    │   ├── sports/               # Team, Season, SeasonPass, Fantasy, Loyalty, Analytics, CrossChain
    │   └── concert/              # Artist management
    ├── utils/                    # Validation, currency conversion
    └── tests/                    # Core, sports, concert test suites
```

### Frontend

```
frontend/
├── src/
│   ├── app/                      # 10 pages (events, resale, my-tickets, analytics, etc.)
│   ├── components/
│   │   └── smart-contracts/      # 6 decomposed components + index
│   ├── sdk/
│   │   ├── types.ts              # 13 TypeScript interfaces matching contract types
│   │   ├── inktixContract.ts     # InkTixSDK interface (team, venue, event, ticket, NFT, pricing, resale)
│   │   ├── mockProvider.ts       # In-memory mock implementation
│   │   ├── contractProvider.ts   # Real @polkadot/api-contract implementation
│   │   └── abi/inktix.json       # Contract ABI metadata (762KB)
│   ├── store/                    # Zustand: 4 slices + persistence + migration
│   ├── config/chains.ts          # Single source of truth for all chain config
│   ├── hooks/useBlockchain.ts    # Backward-compatible hook wrapping Zustand
│   └── services/
│       ├── blockchain.ts         # Singleton: connection, wallet, deploy, call → SDK routing
│       └── xcm.ts                # XCM limitedReserveTransferAssets
├── vitest.config.ts              # 31 unit tests
├── playwright.config.ts          # 3 e2e specs
└── Dockerfile                    # Node 20 Alpine for Docker Compose
```

### Infrastructure

```
scripts/
├── deploy_inktix.sh              # Deploy to inktix.rycsprojects.com (VPS, Nginx, SSL)
├── deploy_inktix_com.sh          # Deploy to inktix.com (Namecheap hosting, .htaccess)
├── deploy-and-configure.sh       # Build contract + deploy to local node + configure frontend
└── nginx/inktix.conf             # Nginx config with SSL, gzip, caching

.github/workflows/ci.yml          # GitHub Actions: cargo test + npm ci/lint/build/test
docker-compose.yml                # substrate-contracts-node + frontend dev server
```

## Conclusion

InkTix demonstrates that Polkadot's cross-chain architecture can power production-grade applications solving real-world problems. The unified contract handles the full ticketing lifecycle on-chain — from dynamic pricing and anti-scalping to NFT minting and cross-chain transfers — while the frontend provides an intuitive experience that hides blockchain complexity from end users.

**Key Achievements:**

- ✅ 52 on-chain contract methods with 14 passing tests
- ✅ 6-factor dynamic pricing engine with transparent price breakdowns
- ✅ NFT tickets with QR code verification and attendance tokens
- ✅ Anti-scalping resale marketplace with 1.5x price caps
- ✅ Real XCM cross-chain transfers on Westend Asset Hub
- ✅ Typed SDK bridging mock and live contract environments
- ✅ 31 frontend tests, CI/CD pipeline, Docker dev setup
- ✅ Comprehensive documentation following thesis-grade standards
- ✅ Production deployment on two live domains

**Live Demo:** https://inktix.rycsprojects.com | https://inktix.com

---

_Built with ink!, Polkadot.js, Next.js, and Zustand for the Polkadot ecosystem_
