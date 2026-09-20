# Coding Conventions

**Analysis Date:** 2026-09-20

## Naming Patterns

**Files:**
- TypeScript/React: `camelCase` for files (e.g., `blockchain.ts`, `useBlockchain.ts`, `ContractDeployment.tsx`)
- React components: `PascalCase` (e.g., `ContractDeployment.tsx`, `BlockchainWrapper.tsx`)
- Test files: `*.test.ts`, `*.test.tsx` or `*.spec.ts` (e.g., `connectionSlice.test.ts`, `mockProvider.test.ts`)
- Rust source: `snake_case` (e.g., `ticket_management.rs`, `venue_management.rs`)
- Rust modules: Organized by feature domain (e.g., `logic/core/`, `logic/sports/`)

**Functions:**
- TypeScript: `camelCase` (e.g., `connectToNetwork()`, `getActiveChainConfig()`, `useBlockchain()`)
- React hooks: Prefix with `use` in `camelCase` (e.g., `useBlockchain`, `useInkTixStore`)
- React components: `PascalCase` (e.g., `ContractDeployment`, `BlockchainService`)
- Rust: `snake_case` (e.g., `register_venue()`, `purchase_ticket()`, `transfer_ticket()`)
- Rust struct methods: `snake_case` with verb prefixes (e.g., `ensure_owner()`, `calculate_loyalty_points()`)

**Variables:**
- TypeScript: `camelCase` for all local variables and state (e.g., `isConnected`, `selectedAccount`, `contractAddress`)
- Boolean flags: Prefix with `is`, `has`, or `can` (e.g., `isConnected`, `isWalletConnected`, `hasContractsPallet`)
- Rust: `snake_case` for local vars, module state (e.g., `dynamic_price`, `ticket_id`, `event_id`)
- Rust constants: SCREAMING_SNAKE_CASE (e.g., `PERSISTED_KEYS`)

**Types:**
- TypeScript interfaces: `PascalCase` with I prefix optional (e.g., `ChainConfig`, `UseBlockchainReturn`, `ContractCallResult<T>`)
- Rust structs: `PascalCase` (e.g., `InkTix`, `Ticket`, `Event`, `Venue`)
- Rust enums: `PascalCase` (e.g., `EventCategory`, `SportType`, `GameType`)
- Generic type parameters: Single uppercase letters (e.g., `<T>`, or descriptive (e.g., `<T = any>`)

## Code Style

**Formatting:**
- TypeScript/JavaScript: 2-space indentation (implicit via Next.js defaults)
- Rust: 4-space indentation (implicit via Rust conventions)
- Line length: Soft limit ~100 chars for readability; enforce via linter
- Trailing semicolons: Required in TypeScript/JavaScript

**Linting:**
- TypeScript: ESLint with Next.js config (`eslint-config-next`)
- Config file: `frontend/.eslintrc.json`
- Rule set: `next/core-web-vitals` (enforces Next.js best practices)
- No custom rule overrides currently in place
- Rust: Implicit `rustfmt` via Cargo (see `Cargo.toml` edition = "2021")

**Type Safety:**
- TypeScript: `strict: true` in `tsconfig.json` — all implicit `any` disallowed
- Path aliases: `@/*` → `./src/*` for cleaner imports across the codebase
- Rust: All public functions explicitly typed with return types (no type inference for public APIs)

## Import Organization

**Order:**
1. External packages from npm/crates (e.g., `import { create } from "zustand"`)
2. Polkadot.js and Web3 libraries (e.g., `import { ApiPromise } from "@polkadot/api"`)
3. Internal project imports (e.g., `import { InkTixStore } from "../store"`)
4. Relative imports from sibling/parent directories (e.g., `import { useInkTixStore } from "../store"`)

**Path Aliases:**
- `@/*` used throughout the codebase for absolute imports (e.g., `import { useBlockchain } from "@/hooks/useBlockchain"`)
- Reduces relative path complexity and improves refactoring safety

**Barrel Files:**
- `src/store/index.ts` exports the combined Zustand store
- `src/components/smart-contracts/index.tsx` composes smart contract sub-components
- `src/sdk/index.ts` exports the SDK factory function

## Error Handling

**TypeScript/React Patterns:**
- Result wrapper type: `ContractCallResult<T>` with `{ success: boolean; data?: T; error?: string; message?: string; txHash?: string; }`
- Exceptions: Wrapped in `try-catch` blocks; errors logged to console (e.g., `console.error("Failed to initialize crypto:", error)`)
- Silent failures: Caught errors often swallowed with no-ops (acceptable for non-critical paths like wallet connection retries)
- Promise-based: All async operations return structured results, not raw exceptions

**Rust Patterns:**
- Error type: `Result<T, String>` throughout contract logic (e.g., `Result<u32, String>`, `Result<(), String>`)
- Error propagation: `?` operator used for early return (e.g., `let event = storage.events.get(event_id).ok_or("Event not found")?;`)
- Validation: Checks performed inline with early returns (e.g., `if !event.active { return Err("Event is not active".to_string()); }`)
- Error messages: String descriptions sent to caller (e.g., `"Only the owner can call this function"`)

## Logging

**Framework:** `console` API in TypeScript/React (no logger library)

**Patterns:**
- Connection lifecycle: `console.log()` for info (e.g., "Attempting to connect to:", "Actually connected to chain:")
- Errors: `console.error()` for exceptions (e.g., "Failed to initialize crypto:", error)
- Debug: Sparse; used for chain info verification (e.g., "✅ Contracts pallet is available")
- Location: Often in service initialization and state transitions

**Best Practice:**
- Log at entry points (constructor, message handlers) rather than deep in utility functions
- Include context (endpoint, chain name) in connection logs
- Keep production-ready logs minimal; development logs acceptable in non-critical paths

## Comments

**When to Comment:**
- Document module purpose at file top with JSDoc/doc comments
- Explain WHY, not WHAT (e.g., "Check if contracts pallet is available" explains intent, not just `if (this.api.tx.contracts)`)
- Mark section boundaries with visual separators for long functions (e.g., `// =============================================================================`)

**JSDoc/TSDoc:**
- Enabled throughout TypeScript/React codebase
- Module-level docs: `@module`, `@exports`, `@interface`, `@function` tags
- Example in `config/chains.ts`:
  ```typescript
  /**
   * Chain configuration and environment helpers for supported Polkadot networks.
   *
   * @module config/chains
   * @exports ChainConfig, CHAINS, getActiveChainConfig, ...
   */
  ```
- Rust: Doc comments with `///` for public items, `//!` for module-level docs

**Comment Style:**
- Single-line: `//` (TypeScript, Rust)
- Multi-line: `/* ... */` or stacked `//` for both languages
- Section markers: Rust uses `// =============================================================================` for major sections

## Function Design

**Size:** 
- Target 20-40 lines for TypeScript functions; split larger logic into helpers
- Rust functions often 30-100 lines due to error handling verbosity

**Parameters:**
- TypeScript: Prefer object/interface parameters for >2 args (e.g., `interface ConnectOptions { endpoint: string; timeout?: number }`)
- Rust: Positional parameters fine; complex structs passed by reference

**Return Values:**
- TypeScript: Wrapped in result types (`ContractCallResult<T>`) for async, direct types for sync
- Rust: `Result<T, String>` for all public contract methods; success path returns value or `()`
- Never return bare `null` or `undefined` in error cases; use error result wrapper

## Module Design

**Exports:**
- `export` for public APIs (functions, types, interfaces)
- Private module functions not exported; kept in same file
- TypeScript: Explicit `export interface`, `export class`, `export function`
- Rust: `pub` for contract messages and helper structs; private helpers remain module-scoped

**Barrel Files:**
- `store/index.ts`: Re-exports store type and hook with combined slice composition
- `components/smart-contracts/index.tsx`: Composes decomposed contract components
- Reduces import depth from consumers (e.g., `import { useInkTixStore } from "@/store"` vs. `from "@/store/index"`)

**Slice Pattern (Zustand):**
- Each domain gets a slice file (e.g., `connectionSlice.ts`, `walletSlice.ts`, `contractSlice.ts`, `dataSlice.ts`)
- Slice exports type (`ConnectionSlice`) and factory (`createConnectionSlice`)
- Factory receives Zustand `(set, get, api)` and returns partial store object
- Example in `store/slices/connectionSlice.ts` pattern (see TESTING.md for test examples)

---

*Convention analysis: 2026-09-20*
