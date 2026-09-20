# Codebase Structure

**Analysis Date:** 2026-09-20

## Directory Layout

```
inktix-hackathon-version/
├── .planning/codebase/           # Codebase analysis documents (ARCHITECTURE.md, STRUCTURE.md, etc.)
├── .tooling/                     # Local binaries (ink-node, cargo-contract) — gitignored
├── contracts/                    # Rust smart contracts
│   ├── inktix/                   # Unified ink! v6 contract (PRIMARY)
│   │   ├── Cargo.toml            # ink! v6.0.0-beta.1, features: sports, concert
│   │   ├── rust-toolchain.toml   # nightly-2026-01-15 + rust-src component
│   │   └── src/
│   │       ├── lib.rs            # Entry point: InkTix contract definition + 14 inline tests
│   │       ├── logic/            # Business logic by domain
│   │       │   ├── core/         # venue, event, ticket, pricing, anti-scalping, nft, xcm
│   │       │   ├── sports/       # teams, seasons, season passes, fantasy, loyalty, analytics
│   │       │   └── concert/      # artist management
│   │       ├── storage/          # InkTixStorage unified struct (~185 fields)
│   │       ├── types/            # Data types
│   │       │   ├── core_types/   # (renamed from core/ to avoid shadow issue in ink! v6)
│   │       │   ├── sports/
│   │       │   └── concert/
│   │       ├── utils/            # Helper functions
│   │       └── tests/            # Integration tests (if any)
│   ├── concert_broker/           # Legacy contract — TO BE DELETED in Phase 1a
│   ├── sports_broker/            # Legacy contract — TO BE DELETED in Phase 1a
│   └── inktix_core/              # Legacy contract — TO BE DELETED in Phase 1a
├── frontend/                     # Next.js 15 static export app
│   ├── src/
│   │   ├── app/                  # Route segments (App Router)
│   │   │   ├── layout.tsx        # Root layout: metadata, fonts, BlockchainWrapper
│   │   │   ├── page.tsx          # Landing/home page
│   │   │   ├── events/           # Event browsing and details
│   │   │   ├── my-tickets/       # User's ticket collection
│   │   │   ├── resale/           # Resale marketplace
│   │   │   ├── connect/          # Wallet connection
│   │   │   ├── analytics/        # Platform stats dashboard
│   │   │   ├── smart-contracts/  # Contract deployment UI
│   │   │   ├── cross-chain/      # XCM functionality
│   │   │   ├── cross-chain-demo/ # XCM demo
│   │   │   ├── docs/             # Documentation pages
│   │   │   └── globals.css       # Global Tailwind directives
│   │   ├── components/           # Reusable React components
│   │   │   ├── BlockchainWrapper.tsx      # Hydration guard
│   │   │   ├── BlockchainWallet.tsx       # Wallet UI
│   │   │   ├── ClientOnly.tsx             # Client-side render guard
│   │   │   ├── WalletConnect.tsx          # Wallet connection flow
│   │   │   ├── SmartContractManager.tsx   # Stub (full logic in subdir)
│   │   │   └── smart-contracts/          # Decomposed contract UI
│   │   │       ├── index.tsx              # Composition point
│   │   │       ├── ContractDeployment.tsx # Deploy UI + feedback
│   │   │       ├── ContractInteraction.tsx # Method call builder
│   │   │       ├── ContractStatus.tsx     # Live status
│   │   │       ├── ContractInfo.tsx       # Address + metadata
│   │   │       ├── ContractRegistry.tsx   # Deployed address registry
│   │   │       ├── CrossChainPanel.tsx    # XCM UI
│   │   │       └── __tests__/
│   │   ├── hooks/                # Custom React hooks
│   │   │   └── useBlockchain.ts   # Backward-compatible store wrapper
│   │   ├── store/                # Zustand state management
│   │   │   ├── index.ts           # Store factory + initialization
│   │   │   ├── persistence.ts     # localStorage migration helper
│   │   │   └── slices/
│   │   │       ├── connectionSlice.ts  # RPC connection state
│   │   │       ├── walletSlice.ts      # Account selection + injected providers
│   │   │       ├── contractSlice.ts    # Contract address + deployment state
│   │   │       └── dataSlice.ts        # Loaded entities (events, venues, etc.)
│   │   ├── services/             # Singleton service layer
│   │   │   ├── blockchain.ts      # BlockchainService: Polkadot.js lifecycle + RPC calls
│   │   │   └── xcm.ts            # XCM utility: builds transfer calls
│   │   ├── sdk/                  # Contract interaction abstraction
│   │   │   ├── index.ts           # Factory: createInkTixSDK()
│   │   │   ├── inktixContract.ts  # InkTixSDK interface definition
│   │   │   ├── contractProvider.ts # Polkadot.js-backed implementation
│   │   │   ├── mockProvider.ts    # Test/dev mock data implementation
│   │   │   ├── types.ts           # SDK types (Event, Venue, Ticket, etc.)
│   │   │   ├── abi/               # Contract ABI / metadata (if imported at runtime)
│   │   │   └── __tests__/
│   │   ├── config/               # Static configuration
│   │   │   └── chains.ts          # Chain registry + env var helpers
│   │   ├── types/                # TypeScript type definitions
│   │   │   └── (component-specific types)
│   │   ├── lib/                  # Utility functions
│   │   └── utils/                # Helpers (contractMethods, methodArgs)
│   │       ├── contractMethods.ts # Exported message / query method lists
│   │       └── methodArgs.ts      # Argument schema helpers
│   ├── public/                   # Static assets (images, fonts)
│   │   └── InkTix_logo.png
│   ├── e2e/                      # Playwright end-to-end tests
│   ├── next.config.js            # Static export, basePath, webpack fallback
│   ├── tsconfig.json             # TypeScript config with @/* path alias
│   ├── tailwind.config.js        # Tailwind CSS theme (inktix-blue, orange, purple)
│   ├── package.json              # Dependencies: @polkadot/api, zustand, next, tailwindcss
│   └── .env.local                # Dev config (NEXT_PUBLIC_MOCK_MODE=true)
│   └── .env.production           # Prod config (Westend AssetHub in v1, v2 TBD)
├── docs/                         # Project documentation
│   ├── v2_plan.md                # Living rearchitecture plan (PRIMARY)
│   ├── rearchitecture_toolchain_notes.md
│   ├── sports_broker_guide.md    # Legacy — to retire
│   ├── concert_broker_guide.md   # Legacy — to retire
│   └── (other docs)
├── scripts/                      # Build and deployment scripts
│   ├── deploy-and-configure.sh   # Build contract + deploy locally + configure frontend
│   ├── deploy_inktix_to_rycsprojects.sh # Build + deploy frontend to VPS
│   ├── dev-node.sh               # (to be created) Download + run ink-node locally
│   ├── build.sh                  # (legacy, points at inktix_core) — TO BE UPDATED
│   └── nginx/                    # Nginx config for production VPS
├── inktix-deployment/            # Old deployment tooling
│   └── deploy.sh                 # Legacy deployment path (Shibuya/Astar, PM2)
├── tests/                        # Standalone test suites (if any)
├── examples/                     # Usage examples
├── README.md                     # Project overview
├── CLAUDE.md                     # Instructions for Claude (this file's source)
├── CHANGELOG.md                  # Version history
└── LICENSE

```

## Directory Purposes

**`.planning/codebase/`:**
- Purpose: Codebase analysis documents generated by `/gsd-map-codebase` command
- Contains: ARCHITECTURE.md, STRUCTURE.md, STACK.md, INTEGRATIONS.md, CONCERNS.md, CONVENTIONS.md, TESTING.md

**`contracts/inktix/`:**
- Purpose: The unified smart contract (PRIMARY in v2)
- Contains: Rust source, feature flags (sports, concert), inline tests
- Key files: `lib.rs` (contract definition), `storage/contract_storage.rs` (unified state)

**`contracts/{concert_broker,sports_broker,inktix_core}/`:**
- Purpose: Legacy contracts from v1
- Status: Marked for deletion in Phase 1a of v2 rearchitecture
- Do NOT delete manually; wait for coordinated cleanup

**`frontend/src/app/`:**
- Purpose: Route-based UI using Next.js App Router
- Contains: Route segments (folders with page.tsx, layout.tsx), global CSS
- Pattern: Folder-per-route; each page.tsx is a route handler

**`frontend/src/components/`:**
- Purpose: Reusable React components
- Contains: Smart contract UI (smart-contracts/), wallet UI, guards (ClientOnly, BlockchainWrapper)
- Special: `smart-contracts/` subfolder decomposes the monolithic SmartContractManager

**`frontend/src/store/`:**
- Purpose: Zustand state management
- Contains: Store factory (index.ts), slices (connection, wallet, contract, data), persistence helpers
- Key: Single localStorage key `inktix-store`; old keys migrated on first load

**`frontend/src/hooks/`:**
- Purpose: Custom React hooks
- Contains: `useBlockchain()` (backward-compatible wrapper around store)

**`frontend/src/services/`:**
- Purpose: Singleton service layer
- Contains: `BlockchainService` (Polkadot.js lifecycle + RPC methods), `xcm.ts` (XCM helpers)

**`frontend/src/sdk/`:**
- Purpose: Contract interaction abstraction
- Contains: Factory (index.ts), interface (inktixContract.ts), implementations (contractProvider, mockProvider), types
- Pattern: Pluggable providers; both implement `InkTixSDK` interface

**`frontend/src/config/`:**
- Purpose: Static configuration
- Contains: Chain registry (RPC endpoints, token decimals, capabilities)
- Pattern: Environment-driven via NEXT_PUBLIC_* vars at build time

**`docs/`:**
- Purpose: Project documentation
- Key: `v2_plan.md` is the living source of truth for rearchitecture status, decisions, and blockers
- Update: At the end of each phase/session; do not leave findings in commit messages only

**`scripts/`:**
- Purpose: Build and deployment automation
- Key scripts:
  - `deploy-and-configure.sh` — local build + deploy flow (PRIMARY)
  - `deploy_inktix_to_rycsprojects.sh` — production VPS deployment
  - `dev-node.sh` — (to be created) ink-node local setup

## Key File Locations

**Entry Points:**

| File | Purpose |
|------|---------|
| `frontend/src/app/layout.tsx` | Root layout: mounts BlockchainWrapper, sets global CSS |
| `frontend/src/app/page.tsx` | Landing/home page with hero section and feature cards |
| `contracts/inktix/src/lib.rs:38..inktix{}` | Smart contract module: message definitions, constructor, logic |
| `scripts/deploy-and-configure.sh` | Orchestrates local build and deployment |

**Configuration:**

| File | Purpose |
|------|---------|
| `frontend/.env.local` | Dev config (NEXT_PUBLIC_MOCK_MODE=true) |
| `frontend/.env.production` | Prod config (RPC endpoint, contract address) |
| `frontend/src/config/chains.ts` | Chain registry + helpers (getActiveChainConfig, isMockMode) |
| `contracts/inktix/Cargo.toml` | Contract dependencies and feature flags |
| `contracts/inktix/rust-toolchain.toml` | Pinned nightly toolchain for cargo-contract |
| `.github/workflows/ci.yml` | GitHub Actions CI (build, test) |

**Core Logic:**

| File | Purpose |
|------|---------|
| `frontend/src/store/index.ts` | Zustand store factory + initialization |
| `frontend/src/store/slices/*.ts` | Store slices (connection, wallet, contract, data) |
| `frontend/src/services/blockchain.ts` | BlockchainService singleton |
| `frontend/src/sdk/contractProvider.ts` | Polkadot.js-backed contract calls |
| `frontend/src/sdk/mockProvider.ts` | Mock data for development |
| `contracts/inktix/src/storage/contract_storage.rs` | Unified on-chain state struct |
| `contracts/inktix/src/logic/core/*` | Core domain modules (venue, event, ticket, etc.) |

**Testing:**

| File | Purpose |
|------|---------|
| `contracts/inktix/src/lib.rs:~2050..` | 14 inline #[ink::test] unit tests |
| `frontend/e2e/` | Playwright end-to-end tests (paused during v2 migration) |
| `frontend/src/store/__tests__/` | Zustand store tests |
| `frontend/src/sdk/__tests__/` | SDK tests |

## Naming Conventions

**Files:**

| Pattern | Example | Where |
|---------|---------|-------|
| snake_case | `contract_storage.rs`, `blockchain.ts` | All files |
| PascalCase | `ContractDeployment.tsx`, `BlockchainService` | React components, classes |
| index.ts | `sdk/index.ts`, `components/smart-contracts/index.ts` | Composition/export points |
| .test.ts/.spec.ts | `store/__tests__/index.test.ts` | Test files |
| .local / .production | `.env.local`, `.env.production` | Environment files |

**Directories:**

| Pattern | Example | Purpose |
|---------|---------|---------|
| plural | `components/`, `services/`, `hooks/` | Collections of related items |
| kebab-case | `smart-contracts/`, `cross-chain/`, `my-tickets/` | Route segments |
| snake_case | `contract_storage.rs`, `team_management.rs` | Rust modules (convention) |
| camelCase | `connectionSlice.ts`, `walletSlice.ts` | Store slices |

**Exports & Modules:**

| Pattern | Example | Convention |
|---------|---------|-----------|
| Barrel files | `components/smart-contracts/index.tsx` | Re-export all from subdir |
| Path aliases | `@/sdk`, `@/config` | All imports use `@/*` prefix |
| camelCase | `useBlockchain`, `createInkTixSDK` | Functions / hooks |
| PascalCase | `BlockchainService`, `InkTixSDK` | Classes / interfaces |

## Where to Add New Code

**New Feature (End-to-End):**

1. **Smart Contract**
   - Implement logic in `contracts/inktix/src/logic/core/` (or sports/concert if domain-specific)
   - Update `contracts/inktix/src/storage/contract_storage.rs` for new state
   - Add types to `contracts/inktix/src/types/core_types/` (or respective subfolder)
   - Add inline `#[ink::test]` in logic module
   - Export public messages in `contracts/inktix/src/lib.rs`

2. **Frontend SDK**
   - Add method signature to `frontend/src/sdk/inktixContract.ts` (InkTixSDK interface)
   - Implement in `frontend/src/sdk/contractProvider.ts` (calls contract) and `mockProvider.ts` (mock data)
   - Add types to `frontend/src/sdk/types.ts`

3. **Frontend Store**
   - Add state to appropriate slice in `frontend/src/store/slices/` (or create new slice if cross-cutting)
   - Add initialize/action to slice
   - Re-export from `frontend/src/store/index.ts` if creating new slice

4. **Frontend UI**
   - Create page in `frontend/src/app/[route]/page.tsx` OR component in `frontend/src/components/`
   - Use `useBlockchain()` hook to access state and actions
   - Render based on `isConnected`, `selectedAccount`, data from store

**New Component/Module:**

- **Shared UI component** → `frontend/src/components/MyComponent.tsx` (export from index.ts if generic)
- **Domain-specific logic module** → `contracts/inktix/src/logic/core/my_module.rs` (add to logic/mod.rs)
- **Type definition** → `frontend/src/types/` (or `contracts/inktix/src/types/*/` for contract types)
- **Utility function** → `frontend/src/utils/` or `frontend/src/lib/`
- **Store slice** → `frontend/src/store/slices/mySlice.ts` (add to index.ts composition)

**Utilities & Helpers:**

- **Formatting / validation** → `frontend/src/utils/`
- **Polkadot.js helpers** → `frontend/src/lib/` (e.g., address encoding/decoding in Phase 4)
- **Contract helpers** → `contracts/inktix/src/utils/` (e.g., price calculation, ID generation)

## Special Directories

**`.tooling/`:**
- Purpose: Local dev binaries (ink-node, cargo-contract)
- Generated: Downloaded by `scripts/dev-node.sh`
- Committed: No (in .gitignore)

**`.next/` and `frontend/out/`:**
- Purpose: Next.js build output
- Generated: By `npm run build`
- Committed: No (in .gitignore)

**`node_modules/`:**
- Purpose: npm dependencies
- Generated: By `npm install`
- Committed: No (in .gitignore); use `package-lock.json`

**`frontend/public/`:**
- Purpose: Static assets (images, fonts) served at `/` root
- Committed: Yes (version-controlled assets)
- Usage: Reference as `/InkTix_logo.png` in JSX

**`frontend/e2e/`:**
- Purpose: Playwright end-to-end tests
- Status: Paused during v2 migration
- Run: `npm run test:e2e`

---

*Structure analysis: 2026-09-20*
