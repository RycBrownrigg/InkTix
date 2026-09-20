<!-- GSD:project-start source:PROJECT.md -->

## Project

**InkTix v2 Migration**

InkTix is a decentralized event ticketing platform on the Polkadot ecosystem — ink! smart contracts for on-chain ticket/event/NFT management, cross-chain transfers via XCM, and wallet-based authentication, with a Next.js frontend. It's currently deployed on Westend Asset Hub (ink! 5.1.1 / `pallet-contracts`). This project tracks finishing the migration to ink! v6 / `pallet-revive`, targeting Passet Hub testnet (Polkadot Hub mainnet once GA).

**Core Value:** The full ticket-purchase flow — browse, pay with real value, mint the ticket NFT, and let funds be withdrawn — works end-to-end against a live ink! v6 / `pallet-revive` chain (Passet Hub).

### Constraints

- **Tech stack**: ink! v6 / `pallet-revive` only — `pallet-contracts`-era tooling (`@polkadot/api-contract`'s `ContractPromise`/`CodePromise`) has no equivalent and must be replaced, not adapted.
- **Toolchain**: Rust `nightly-2026-01-15` is a hard pin for all contract builds — a Rust/Cargo strictness change after this date breaks `cargo-contract` v6.0.0-beta.2, and no fix is expected given the ink! team's paused status.
- **Compatibility**: preserve the existing 4 wallet extensions and SS58 signing UX — this is why PAPI/ink! SDK was chosen over an EVM-wallet route.
- **Scope discipline**: this is a toolchain/runtime migration, not a product pivot — v1's feature set carries over. New functionality is limited to what real payment requires (payable purchase + withdrawal), not general feature expansion.
- **Timeline**: none — soundness-driven, not deadline-driven.

<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->

## Technology Stack

## Languages

- TypeScript 5.9.2 - Frontend (React/Next.js application)
- Rust (nightly-2026-01-15) - Smart contracts (ink! v6)
- JavaScript - Build configuration and scripts
- WASM (WebAssembly) - Compiled output from Rust contracts

## Runtime

- Node.js 20+ (development)
- Browser (modern WebGL-capable browser for frontend)
- Polkadot Substrate Runtime (network layer via RPC)
- npm (Node.js package manager)
- Lockfile: `frontend/package-lock.json` (present, committed)

## Frameworks

- Next.js 15.5.0 - Full-stack React framework (static export mode)
- React 19.1.1 - UI component library
- ink! 6.0.0-beta.1 - Smart contract framework (Rust)
- Vitest 3.2.4 - Unit test runner (jsdom environment)
- @playwright/test 1.58.2 - E2E test framework
- ink_e2e 6.0.0-beta.2 - Smart contract end-to-end tests
- cargo-contract 6.0.0-beta.2 - Smart contract build tool (Rust)
- Webpack (via Next.js) - JavaScript bundler
- TypeScript 5.9.2 - Type checking
- PostCSS 8.5.6 - CSS transformation
- Autoprefixer 10.4.21 - CSS vendor prefixing

## Key Dependencies

- @polkadot/api 16.4.6 - Polkadot.js core API client
- @polkadot/api-contract 16.5.4 - Contract interaction library
- @polkadot/extension-dapp 0.61.6 - Wallet extension bridge (dApp side)
- @polkadot/extension-inject 0.61.6 - Wallet extension types
- @polkadot/types 16.4.6 - Type definitions for Polkadot runtime
- @polkadot/util 13.5.6 - Polkadot.js utility functions
- @polkadot/util-crypto 13.5.6 - Cryptographic utilities
- @polkadot/wasm-crypto 7.5.1 - WASM crypto implementation
- zustand 5.0.11 - Lightweight state management with persistence
- lucide-react 0.542.0 - Icon library
- class-variance-authority 0.7.1 - Component variant utilities
- clsx 2.1.1 - Utility for combining CSS classes
- qrcode.react 4.2.0 - QR code generation
- tailwindcss 3.4.17 - Utility-first CSS framework
- @tailwindcss/aspect-ratio 0.4.2 - Aspect ratio plugin
- @tailwindcss/forms 0.5.10 - Form styling plugin
- @tailwindcss/typography 0.5.16 - Typography plugin
- @testing-library/react 16.3.2 - React component testing utilities
- @testing-library/jest-dom 6.9.1 - DOM matchers
- jsdom 25.0.1 - DOM implementation for Node.js
- @typescript-eslint/eslint-plugin 8.41.0 - TypeScript linting rules
- @typescript-eslint/parser 8.41.0 - TypeScript parser for ESLint
- eslint 9.34.0 - JavaScript linter
- eslint-config-next 15.5.2 - Next.js ESLint config
- @types/node 24.3.0 - Node.js type definitions
- @types/react 19.1.11 - React type definitions
- @types/react-dom 19.1.8 - React DOM type definitions

## Configuration

- Environment configuration via `.env.local`, `.env.development`, `.env.production`
- Key variables:
- `frontend/next.config.js` - Next.js configuration (static export, base path support)
- `frontend/tsconfig.json` - TypeScript compiler configuration
- `frontend/.eslintrc.json` - ESLint rules
- `contracts/inktix/rust-toolchain.toml` - Rust toolchain pinning (nightly-2026-01-15)
- `contracts/inktix/Cargo.toml` - Rust dependencies and features
- `frontend/vitest.config.ts` - Unit test configuration
- `frontend/playwright.config.ts` - E2E test configuration
- `.github/workflows/ci.yml` - GitHub Actions CI pipeline
- Custom theme colors: `inktix-blue`, `inktix-orange`, `inktix-purple`
- Configured in `frontend/src/app/layout.tsx` or tailwind config
- Output CSS mode with static export

## Platform Requirements

- Node.js 22+ (tested with v25.2.1)
- Rust 1.92+ with nightly toolchain
- `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- Polkadot wallet extension (Polkadot.js, Talisman, SubWallet, Nova)
- Modern browser with WebSocket support
- Deployment target: VPS at 135.148.61.99 (Debian, Nginx)
- Production URL: `https://rycsprojects.com/inktix/`
- Requires RPC endpoint connectivity to Westend Asset Hub
- Static files served via Nginx (no backend server required)

## Tooling

- GitHub Actions workflow (``.github/workflows/ci.yml`)
- Runs on: `ubuntu-latest`
- Contract build: `cargo contract build`
- Contract tests: `cargo test` (14 unit tests)
- Frontend build: `npm run build`
- Frontend tests: `npx vitest run` and `next lint`
- Docker Compose support (`docker-compose.yml`)
- Parity Substrate contracts node (via Docker)
- Hot reload via `npm run dev` on port 3000

<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->

## Conventions

## Naming Patterns

- TypeScript/React: `camelCase` for files (e.g., `blockchain.ts`, `useBlockchain.ts`, `ContractDeployment.tsx`)
- React components: `PascalCase` (e.g., `ContractDeployment.tsx`, `BlockchainWrapper.tsx`)
- Test files: `*.test.ts`, `*.test.tsx` or `*.spec.ts` (e.g., `connectionSlice.test.ts`, `mockProvider.test.ts`)
- Rust source: `snake_case` (e.g., `ticket_management.rs`, `venue_management.rs`)
- Rust modules: Organized by feature domain (e.g., `logic/core/`, `logic/sports/`)
- TypeScript: `camelCase` (e.g., `connectToNetwork()`, `getActiveChainConfig()`, `useBlockchain()`)
- React hooks: Prefix with `use` in `camelCase` (e.g., `useBlockchain`, `useInkTixStore`)
- React components: `PascalCase` (e.g., `ContractDeployment`, `BlockchainService`)
- Rust: `snake_case` (e.g., `register_venue()`, `purchase_ticket()`, `transfer_ticket()`)
- Rust struct methods: `snake_case` with verb prefixes (e.g., `ensure_owner()`, `calculate_loyalty_points()`)
- TypeScript: `camelCase` for all local variables and state (e.g., `isConnected`, `selectedAccount`, `contractAddress`)
- Boolean flags: Prefix with `is`, `has`, or `can` (e.g., `isConnected`, `isWalletConnected`, `hasContractsPallet`)
- Rust: `snake_case` for local vars, module state (e.g., `dynamic_price`, `ticket_id`, `event_id`)
- Rust constants: SCREAMING_SNAKE_CASE (e.g., `PERSISTED_KEYS`)
- TypeScript interfaces: `PascalCase` with I prefix optional (e.g., `ChainConfig`, `UseBlockchainReturn`, `ContractCallResult<T>`)
- Rust structs: `PascalCase` (e.g., `InkTix`, `Ticket`, `Event`, `Venue`)
- Rust enums: `PascalCase` (e.g., `EventCategory`, `SportType`, `GameType`)
- Generic type parameters: Single uppercase letters (e.g., `<T>`, or descriptive (e.g., `<T = any>`)

## Code Style

- TypeScript/JavaScript: 2-space indentation (implicit via Next.js defaults)
- Rust: 4-space indentation (implicit via Rust conventions)
- Line length: Soft limit ~100 chars for readability; enforce via linter
- Trailing semicolons: Required in TypeScript/JavaScript
- TypeScript: ESLint with Next.js config (`eslint-config-next`)
- Config file: `frontend/.eslintrc.json`
- Rule set: `next/core-web-vitals` (enforces Next.js best practices)
- No custom rule overrides currently in place
- Rust: Implicit `rustfmt` via Cargo (see `Cargo.toml` edition = "2021")
- TypeScript: `strict: true` in `tsconfig.json` — all implicit `any` disallowed
- Path aliases: `@/*` → `./src/*` for cleaner imports across the codebase
- Rust: All public functions explicitly typed with return types (no type inference for public APIs)

## Import Organization

- `@/*` used throughout the codebase for absolute imports (e.g., `import { useBlockchain } from "@/hooks/useBlockchain"`)
- Reduces relative path complexity and improves refactoring safety
- `src/store/index.ts` exports the combined Zustand store
- `src/components/smart-contracts/index.tsx` composes smart contract sub-components
- `src/sdk/index.ts` exports the SDK factory function

## Error Handling

- Result wrapper type: `ContractCallResult<T>` with `{ success: boolean; data?: T; error?: string; message?: string; txHash?: string; }`
- Exceptions: Wrapped in `try-catch` blocks; errors logged to console (e.g., `console.error("Failed to initialize crypto:", error)`)
- Silent failures: Caught errors often swallowed with no-ops (acceptable for non-critical paths like wallet connection retries)
- Promise-based: All async operations return structured results, not raw exceptions
- Error type: `Result<T, String>` throughout contract logic (e.g., `Result<u32, String>`, `Result<(), String>`)
- Error propagation: `?` operator used for early return (e.g., `let event = storage.events.get(event_id).ok_or("Event not found")?;`)
- Validation: Checks performed inline with early returns (e.g., `if !event.active { return Err("Event is not active".to_string()); }`)
- Error messages: String descriptions sent to caller (e.g., `"Only the owner can call this function"`)

## Logging

- Connection lifecycle: `console.log()` for info (e.g., "Attempting to connect to:", "Actually connected to chain:")
- Errors: `console.error()` for exceptions (e.g., "Failed to initialize crypto:", error)
- Debug: Sparse; used for chain info verification (e.g., "✅ Contracts pallet is available")
- Location: Often in service initialization and state transitions
- Log at entry points (constructor, message handlers) rather than deep in utility functions
- Include context (endpoint, chain name) in connection logs
- Keep production-ready logs minimal; development logs acceptable in non-critical paths

## Comments

- Document module purpose at file top with JSDoc/doc comments
- Explain WHY, not WHAT (e.g., "Check if contracts pallet is available" explains intent, not just `if (this.api.tx.contracts)`)
- Mark section boundaries with visual separators for long functions (e.g., `// =============================================================================`)
- Enabled throughout TypeScript/React codebase
- Module-level docs: `@module`, `@exports`, `@interface`, `@function` tags
- Example in `config/chains.ts`:
- Rust: Doc comments with `///` for public items, `//!` for module-level docs
- Single-line: `//` (TypeScript, Rust)
- Multi-line: `/* ... */` or stacked `//` for both languages
- Section markers: Rust uses `// =============================================================================` for major sections

## Function Design

- Target 20-40 lines for TypeScript functions; split larger logic into helpers
- Rust functions often 30-100 lines due to error handling verbosity
- TypeScript: Prefer object/interface parameters for >2 args (e.g., `interface ConnectOptions { endpoint: string; timeout?: number }`)
- Rust: Positional parameters fine; complex structs passed by reference
- TypeScript: Wrapped in result types (`ContractCallResult<T>`) for async, direct types for sync
- Rust: `Result<T, String>` for all public contract methods; success path returns value or `()`
- Never return bare `null` or `undefined` in error cases; use error result wrapper

## Module Design

- `export` for public APIs (functions, types, interfaces)
- Private module functions not exported; kept in same file
- TypeScript: Explicit `export interface`, `export class`, `export function`
- Rust: `pub` for contract messages and helper structs; private helpers remain module-scoped
- `store/index.ts`: Re-exports store type and hook with combined slice composition
- `components/smart-contracts/index.tsx`: Composes decomposed contract components
- Reduces import depth from consumers (e.g., `import { useInkTixStore } from "@/store"` vs. `from "@/store/index"`)
- Each domain gets a slice file (e.g., `connectionSlice.ts`, `walletSlice.ts`, `contractSlice.ts`, `dataSlice.ts`)
- Slice exports type (`ConnectionSlice`) and factory (`createConnectionSlice`)
- Factory receives Zustand `(set, get, api)` and returns partial store object
- Example in `store/slices/connectionSlice.ts` pattern (see TESTING.md for test examples)

<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->

## Architecture

## System Overview

```text

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

- **No backend server** — contracts are the source of truth; frontend is a static site
- **Provider factory pattern** — `createInkTixSDK()` returns `MockProvider` or `ContractProvider` based on environment
- **Zustand slices** — separate concerns (connection, wallet, contract data, app data) into composable pieces
- **Singleton BlockchainService** — one instance per app session manages Polkadot.js API lifecycle
- **Environment-driven behavior** — `NEXT_PUBLIC_MOCK_MODE`, `NEXT_PUBLIC_RPC_ENDPOINT`, `NEXT_PUBLIC_CONTRACT_ADDRESS` bake config at build time
- **Static export** — Next.js with `output: "export"` deploys as flat HTML/JS to Nginx
- **Wallet-based auth** — no backend auth; identity is derived from Substrate account / H160 address
- **Feature-gated contract** — Cargo features `sports` and `concert` enable/disable modules at build time

## Layers

- Purpose: User interface rendering and interaction
- Location: `frontend/src/app/`, `frontend/src/components/`
- Contains: Page components, reusable UI components, layouts, route segments
- Depends on: useBlockchain hook
- Used by: End users via browser
- Purpose: Centralized state management and reactive data binding
- Location: `frontend/src/hooks/`, `frontend/src/store/`
- Contains: `useBlockchain` hook wrapper, Zustand store with 4 slices, persistence logic
- Depends on: BlockchainService, SDK factory
- Used by: React components
- Purpose: Singleton lifecycle management, RPC operations, helper services
- Location: `frontend/src/services/`, `frontend/src/config/`
- Contains: BlockchainService (API connection, account management), XCM service, chain registry
- Depends on: Polkadot.js API
- Used by: Store initialization, SDK providers
- Purpose: Abstract contract method invocation behind a unified interface
- Location: `frontend/src/sdk/`
- Contains: InkTixSDK interface, ContractProvider (pallet-contracts/revive), MockProvider (test data)
- Depends on: BlockchainService, Polkadot.js contract module
- Used by: Store (dataSlice)
- Purpose: On-chain business logic and state storage
- Location: `contracts/inktix/src/`
- Contains: Unified contract definition, modular logic (core/sports/concert), storage struct, types
- Depends on: ink! v6 runtime, pallet-revive
- Used by: SDK providers via contract calls

## Data Flow

### Primary Request Path: Ticket Purchase

### State Synchronization (User Connects Wallet)

- Zustand store persists connection state to localStorage (`inktix-store` key)
- Old localStorage keys migrated on first load (`persistence.ts`)
- Each store slice is a factory function returning actions + state
- Computed data (e.g., balance formatting) lives in components, not store

## Key Abstractions

- Purpose: Unified contract surface for both real and mock backends
- Examples: `registerVenue()`, `purchaseTicket()`, `getPlatformStats()` in `frontend/src/sdk/inktixContract.ts`
- Pattern: Async factory pattern — `createInkTixSDK()` returns implementation
- Purpose: Centralized Polkadot.js API lifecycle
- Methods: `connectWallet()`, `restoreConnection()`, `selectAccount()`, `callContract()`, etc.
- Location: `frontend/src/services/blockchain.ts`
- Accessed via: `BlockchainService.getInstance()`
- Purpose: Single unified on-chain state bucket avoiding cross-contract calls
- Shape: ~185 fields in `InkTixStorage` covering venues, events, tickets, loyalty, fantasy, artist, etc.
- Location: `contracts/inktix/src/storage/contract_storage.rs`
- Pattern: Fields always present (no conditional compilation of storage for ink! layout stability)
- Purpose: Separate concerns into independently testable pieces
- Slices: `connectionSlice` (RPC state), `walletSlice` (accounts), `contractSlice` (contract metadata), `dataSlice` (loaded entities)
- Location: `frontend/src/store/slices/*.ts`
- Pattern: Each slice is a factory `(set, get, api) => ({ state, actions })`
- Purpose: Single source of truth for RPC endpoints, token decimals, chain capabilities
- Entries: Westend AssetHub, Passet Hub (testnet), Polkadot (future), local dev node
- Location: `frontend/src/config/chains.ts`
- Pattern: Lazy-loaded from environment variables, fallback to defaults

## Entry Points

- Location: `frontend/src/app/layout.tsx` (root layout)
- Triggers: Page navigation, app load
- Responsibilities: Wraps tree in `ClientOnly` guard, mounts `BlockchainWrapper`, loads global CSS
- Location: `contracts/inktix/src/lib.rs:38..inktix{}` (contract module)
- Triggers: Constructor call (deployment), message calls (state changes), queries (read-only)
- Responsibilities: Storage management, permission checks (`ensure_owner`), delegating to logic modules
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

### Direct Polkadot.js API Calls in React Components

### Storing Sensitive Data in Zustand Store

### Duplicating Contract Metadata in Frontend

## Error Handling

- **SDK boundary errors** — wrap Polkadot.js errors (network timeouts, encoding failures) into `ContractCallResult.error: string`
- **Contract revert** — if-let on returned `Result<Result<T, E>, LangError>` from revive; unwrap both layers to user-facing message
- **Wallet errors** — `connectWallet()` catches extension unavailable, user rejection; returns success:false + error message
- **State mismatch** — `loadBalance()` silently fails if no account selected; components check `balance === null` before rendering

## Cross-Cutting Concerns

- Approach: `console.log/warn/error` in development; no structured logging library yet
- Locations: BlockchainService (`console.log("Connected to RPC")`), store initialization, SDK calls
- Future: Phase 4+ may add error telemetry
- Approach: Type-checked at TS compile time; minimal runtime validation
- Contract-side: `ensure_owner()` guards, saturating arithmetic on prices (no panics)
- Frontend-side: `EventData` type ensures shape before rendering
- Approach: Derived from Substrate wallet account via extension; no backend auth
- Token model: Polkadot.js extension holds signing capability; frontend requests signature, never has key
- Contract-side: `self.env().caller()` (ink! 5) → `self.env().caller()` returns `Address` (ink! v6); no role-based access yet (only owner check)

<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->

## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->

## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:

- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->

## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
