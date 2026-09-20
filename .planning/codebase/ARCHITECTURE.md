<!-- refreshed: 2026-09-20 -->
# Architecture

**Analysis Date:** 2026-09-20

## System Overview

InkTix is a decentralized event ticketing platform combining a static-export Next.js frontend with a unified ink! v6 smart contract on the Polkadot ecosystem. The system is undergoing a v2 rearchitecture: migrating from ink! 5.1.1/pallet-contracts (Westend Asset Hub) to ink! v6/pallet-revive (Passet Hub testnet → Polkadot Hub mainnet).

```text
┌─────────────────────────────────────────────────────────────────────┐
│                      Browser (React/Next.js 15)                     │
│  `frontend/src/app/*` (Route segments + pages)                      │
│  `frontend/src/components/*` (UI layer)                             │
└─────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────┐
│          React Hooks & State Management (Zustand)                   │
│  `frontend/src/hooks/useBlockchain.ts` (wrapper hook)               │
│  `frontend/src/store/` (inktix-store with 4 slices)                 │
│  └─ connectionSlice, walletSlice, contractSlice, dataSlice          │
└─────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────┐
│               Service Layer (Singleton + Factory)                   │
│  `frontend/src/services/blockchain.ts` (BlockchainService)          │
│  `frontend/src/services/xcm.ts` (XCM helper)                        │
│  `frontend/src/config/chains.ts` (Chain registry)                   │
└─────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────┐
│              SDK Layer (Provider Pattern)                           │
│  `frontend/src/sdk/index.ts` (Factory)                              │
│  `frontend/src/sdk/contractProvider.ts` (Polkadot.js wrapper)       │
│  `frontend/src/sdk/mockProvider.ts` (Mock for development)          │
│  `frontend/src/sdk/inktixContract.ts` (InkTixSDK interface)          │
└─────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────┐
│    Polkadot Ecosystem (Blockchain Runtime)                          │
│  Westend AssetHub (v1) → Passet Hub testnet (v2) →                 │
│  Polkadot Hub mainnet (v2 future)                                   │
│                                                                      │
│  Unified ink! v6 Contract: `contracts/inktix/src/lib.rs`            │
│  ├─ Core: venue, event, ticket, pricing, anti-scalping, nft, xcm   │
│  ├─ Sports: teams, seasons, season passes, fantasy, loyalty         │
│  └─ Concert: artist management                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| **Page Components** | Route-based UI, entry points for features | `frontend/src/app/page.tsx`, `app/events/page.tsx`, etc. |
| **Smart Contract Components** | Deployment, interaction, status UI | `frontend/src/components/smart-contracts/` |
| **useBlockchain Hook** | Backward-compatible store wrapper, initialization | `frontend/src/hooks/useBlockchain.ts` |
| **Zustand Store** | Client-side state (connection, wallet, contract, data) | `frontend/src/store/index.ts` + slices |
| **BlockchainService** | Polkadot.js API management, wallet/account handling | `frontend/src/services/blockchain.ts` |
| **SDK Providers** | Contract method invocation (real or mock) | `frontend/src/sdk/contractProvider.ts`, `mockProvider.ts` |
| **InkTix Contract** | Ticketing logic, NFTs, cross-chain (on-chain) | `contracts/inktix/src/lib.rs` |
| **Contract Storage** | All on-chain state (unified struct) | `contracts/inktix/src/storage/contract_storage.rs` |
| **Contract Logic** | Modular business rules by domain | `contracts/inktix/src/logic/core/*`, `logic/sports/*`, `logic/concert/*` |

## Pattern Overview

**Overall:** Modular layered architecture with provider pattern for contract interaction, Zustand for state, and functional service singleton for blockchain operations.

**Key Characteristics:**
- **No backend server** — contracts are the source of truth; frontend is a static site
- **Provider factory pattern** — `createInkTixSDK()` returns `MockProvider` or `ContractProvider` based on environment
- **Zustand slices** — separate concerns (connection, wallet, contract data, app data) into composable pieces
- **Singleton BlockchainService** — one instance per app session manages Polkadot.js API lifecycle
- **Environment-driven behavior** — `NEXT_PUBLIC_MOCK_MODE`, `NEXT_PUBLIC_RPC_ENDPOINT`, `NEXT_PUBLIC_CONTRACT_ADDRESS` bake config at build time
- **Static export** — Next.js with `output: "export"` deploys as flat HTML/JS to Nginx
- **Wallet-based auth** — no backend auth; identity is derived from Substrate account / H160 address
- **Feature-gated contract** — Cargo features `sports` and `concert` enable/disable modules at build time

## Layers

**Browser / React Layer:**
- Purpose: User interface rendering and interaction
- Location: `frontend/src/app/`, `frontend/src/components/`
- Contains: Page components, reusable UI components, layouts, route segments
- Depends on: useBlockchain hook
- Used by: End users via browser

**Hook & State Layer:**
- Purpose: Centralized state management and reactive data binding
- Location: `frontend/src/hooks/`, `frontend/src/store/`
- Contains: `useBlockchain` hook wrapper, Zustand store with 4 slices, persistence logic
- Depends on: BlockchainService, SDK factory
- Used by: React components

**Service Layer:**
- Purpose: Singleton lifecycle management, RPC operations, helper services
- Location: `frontend/src/services/`, `frontend/src/config/`
- Contains: BlockchainService (API connection, account management), XCM service, chain registry
- Depends on: Polkadot.js API
- Used by: Store initialization, SDK providers

**SDK / Contract Interaction Layer:**
- Purpose: Abstract contract method invocation behind a unified interface
- Location: `frontend/src/sdk/`
- Contains: InkTixSDK interface, ContractProvider (pallet-contracts/revive), MockProvider (test data)
- Depends on: BlockchainService, Polkadot.js contract module
- Used by: Store (dataSlice)

**Smart Contract Layer:**
- Purpose: On-chain business logic and state storage
- Location: `contracts/inktix/src/`
- Contains: Unified contract definition, modular logic (core/sports/concert), storage struct, types
- Depends on: ink! v6 runtime, pallet-revive
- Used by: SDK providers via contract calls

## Data Flow

### Primary Request Path: Ticket Purchase

1. **User clicks "Purchase Ticket" in event detail** (`frontend/src/app/events/[eventId]/page.tsx`)
2. **React component calls `useBlockchain()` hook** → reads `selectedAccount`, `contractAddress` from store
3. **Hook invokes `callContract('purchaseTicket', [eventId, seatNumber, ...])`** (`frontend/src/hooks/useBlockchain.ts:~95`)
4. **Store's `contractSlice` routes to SDK** → `createInkTixSDK()` factory returns live or mock provider
5. **ContractProvider builds message call** → encodes args, sets gas/storage-deposit, signs with account (`frontend/src/sdk/contractProvider.ts`)
6. **Call dispatched via Polkadot.js** → `api.tx.revive.call()` (v2) or `api.tx.contracts.call()` (v1) for pallet-contracts
7. **Smart contract executes on-chain** → validates caller, checks anti-scalping, mints ticket NFT, updates storage (`contracts/inktix/src/logic/core/ticket_management.rs`)
8. **Transaction confirmed** → SDK unwraps result, store updates `balance`, `userTickets`, UI re-renders

### State Synchronization (User Connects Wallet)

1. **User clicks "Connect Wallet"** → `BlockchainWrapper` mounts `BlockchainProvider` (pass-through)
2. **Component calls `useBlockchain().connectWallet()`**
3. **Store's `walletSlice` calls `BlockchainService.connectWallet()`**
4. **Service injects Polkadot.js extension, lists accounts** → user picks one
5. **Store updates `selectedAccount`, triggers `loadBalance()`**
6. **`connectionSlice` attempts `connectToNetwork()` with stored endpoint**
7. **BlockchainService instantiates `ApiPromise`**, subscribes to finalized heads
8. **Store initializes SDK with live provider** (if contract address set)
9. **`dataSlice.loadMockData()` fetches initial event list** (or live via contract query)

**State Management:**
- Zustand store persists connection state to localStorage (`inktix-store` key)
- Old localStorage keys migrated on first load (`persistence.ts`)
- Each store slice is a factory function returning actions + state
- Computed data (e.g., balance formatting) lives in components, not store

## Key Abstractions

**InkTixSDK Interface:**
- Purpose: Unified contract surface for both real and mock backends
- Examples: `registerVenue()`, `purchaseTicket()`, `getPlatformStats()` in `frontend/src/sdk/inktixContract.ts`
- Pattern: Async factory pattern — `createInkTixSDK()` returns implementation

**BlockchainService Singleton:**
- Purpose: Centralized Polkadot.js API lifecycle
- Methods: `connectWallet()`, `restoreConnection()`, `selectAccount()`, `callContract()`, etc.
- Location: `frontend/src/services/blockchain.ts`
- Accessed via: `BlockchainService.getInstance()`

**Contract Storage Struct:**
- Purpose: Single unified on-chain state bucket avoiding cross-contract calls
- Shape: ~185 fields in `InkTixStorage` covering venues, events, tickets, loyalty, fantasy, artist, etc.
- Location: `contracts/inktix/src/storage/contract_storage.rs`
- Pattern: Fields always present (no conditional compilation of storage for ink! layout stability)

**Zustand Store Slices:**
- Purpose: Separate concerns into independently testable pieces
- Slices: `connectionSlice` (RPC state), `walletSlice` (accounts), `contractSlice` (contract metadata), `dataSlice` (loaded entities)
- Location: `frontend/src/store/slices/*.ts`
- Pattern: Each slice is a factory `(set, get, api) => ({ state, actions })`

**Chain Config Registry:**
- Purpose: Single source of truth for RPC endpoints, token decimals, chain capabilities
- Entries: Westend AssetHub, Passet Hub (testnet), Polkadot (future), local dev node
- Location: `frontend/src/config/chains.ts`
- Pattern: Lazy-loaded from environment variables, fallback to defaults

## Entry Points

**Frontend (Browser):**
- Location: `frontend/src/app/layout.tsx` (root layout)
- Triggers: Page navigation, app load
- Responsibilities: Wraps tree in `ClientOnly` guard, mounts `BlockchainWrapper`, loads global CSS

**Smart Contract:**
- Location: `contracts/inktix/src/lib.rs:38..inktix{}` (contract module)
- Triggers: Constructor call (deployment), message calls (state changes), queries (read-only)
- Responsibilities: Storage management, permission checks (`ensure_owner`), delegating to logic modules

**Deployment Entrypoint:**
- Script: `scripts/deploy-and-configure.sh` (builds contract, deploys, stores address in frontend config)

## Architectural Constraints

- **Threading:** Single-threaded async event loop (Next.js / browser); ink! contract runs synchronously in PolkaVM
- **Global state:** `BlockchainService` is a singleton; no shared mutable state outside of Zustand store and service
- **Circular imports:** Zustand slices avoid circular references by passing `set, get` functions; no dynamic imports
- **Static export:** Next.js `output: "export"` disallows server-side rendering, dynamic routes must be pre-generated or use client components
- **Contract deployment:** v2 targets pallet-revive with H160 (20-byte) addresses; v1 used AccountId32 (32-byte) for pallet-contracts
- **Feature gating:** Cargo features `sports`, `concert` gate logic modules at compile time; no runtime branch on features
- **Browser extension auth:** Wallet selection via Polkadot.js / Talisman / SubWallet / Nova extensions; no seedphrase stored in UI

## Anti-Patterns

### Hardcoded RPC Endpoints in Components

**What happens:** Components fetch RPC endpoint from string literals instead of `config/chains.ts`
**Why it's wrong:** Makes it impossible to redeploy to a different chain without recompiling; silently ignores environment overrides
**Do this instead:** Always read from `getDefaultEndpoint()` or `getActiveChainConfig()` in `frontend/src/config/chains.ts`

### Direct Polkadot.js API Calls in React Components

**What happens:** Component imports `ApiPromise` directly and calls `.rpc.chain_getHead()` without going through service layer
**Why it's wrong:** Breaks encapsulation; multiple components may create duplicate API instances; hard to mock for tests
**Do this instead:** Call through `BlockchainService.getInstance()` methods or store actions

### Storing Sensitive Data in Zustand Store

**What happens:** Private key, seed phrase, or raw account objects persisted to localStorage
**Why it's wrong:** Breach vector; keys are unencrypted on disk; violates wallet extension security model
**Do this instead:** Never store secrets; wallet selection is ephemeral; `BlockchainService` holds API instance reference only

### Duplicating Contract Metadata in Frontend

**What happens:** ABI or message signatures hardcoded in TypeScript instead of loading from contract deployment
**Why it's wrong:** Drift between contract code and UI ABI causes silent failures; prevents seamless contract upgrades
**Do this instead:** Keep `InkTixSDK` interface in sync with contract; regenerate from contract metadata during Phase 4

## Error Handling

**Strategy:** Layered error wrapping with `ContractCallResult<T> = { success, data?, error? }` at SDK boundary.

**Patterns:**
- **SDK boundary errors** — wrap Polkadot.js errors (network timeouts, encoding failures) into `ContractCallResult.error: string`
- **Contract revert** — if-let on returned `Result<Result<T, E>, LangError>` from revive; unwrap both layers to user-facing message
- **Wallet errors** — `connectWallet()` catches extension unavailable, user rejection; returns success:false + error message
- **State mismatch** — `loadBalance()` silently fails if no account selected; components check `balance === null` before rendering

## Cross-Cutting Concerns

**Logging:**
- Approach: `console.log/warn/error` in development; no structured logging library yet
- Locations: BlockchainService (`console.log("Connected to RPC")`), store initialization, SDK calls
- Future: Phase 4+ may add error telemetry

**Validation:**
- Approach: Type-checked at TS compile time; minimal runtime validation
- Contract-side: `ensure_owner()` guards, saturating arithmetic on prices (no panics)
- Frontend-side: `EventData` type ensures shape before rendering

**Authentication:**
- Approach: Derived from Substrate wallet account via extension; no backend auth
- Token model: Polkadot.js extension holds signing capability; frontend requests signature, never has key
- Contract-side: `self.env().caller()` (ink! 5) → `self.env().caller()` returns `Address` (ink! v6); no role-based access yet (only owner check)

---

*Architecture analysis: 2026-09-20*
