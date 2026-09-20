# Testing Patterns

**Analysis Date:** 2026-09-20

## Test Framework

**Runner:**
- Vitest 3.2.4
- Config: `frontend/vitest.config.ts`
- Test environment: jsdom (browser-like DOM for React component testing)

**Assertion Library:**
- Testing Library (`@testing-library/react` 16.3.2) for React component testing
- Vitest built-in `expect()` for all assertions
- Additional: `@testing-library/jest-dom` 6.9.1 for DOM matchers (e.g., `.toBeDisabled()`, `.toHaveValue()`)

**Run Commands:**
```bash
npm test                    # Run all tests once
npm test -- --watch       # Watch mode (re-run on file change)
npm test -- --coverage    # Run with coverage report
npm test -- --ui          # Launch test UI dashboard
```

**E2E Testing:**
- Framework: Playwright 1.58.2
- Command: `npm run test:e2e`
- Config: Implicit Playwright default (no config file checked in)
- Tests: Not currently active (paused per project status)

## Test File Organization

**Location:**
- Frontend: Co-located in `__tests__/` subdirectory adjacent to source
  - Example: `src/store/__tests__/connectionSlice.test.ts`
  - Example: `src/components/smart-contracts/__tests__/ContractDeployment.test.tsx`
  - Example: `src/sdk/__tests__/mockProvider.test.ts`
- Contracts: Test modules inline in source tree (e.g., `contracts/inktix/src/tests/core_tests.rs`)

**Naming:**
- Pattern: `*.test.ts`, `*.test.tsx`
- Follows: Source file name with `.test` suffix
- Example: `connectionSlice.ts` → `connectionSlice.test.ts`

**Structure:**
```
frontend/src/
├── store/
│   ├── index.ts
│   ├── slices/
│   │   └── connectionSlice.ts
│   └── __tests__/
│       └── connectionSlice.test.ts
├── components/
│   ├── smart-contracts/
│   │   ├── ContractDeployment.tsx
│   │   └── __tests__/
│   │       └── ContractDeployment.test.tsx
└── sdk/
    ├── index.ts
    └── __tests__/
        └── mockProvider.test.ts
```

## Test Structure

**Suite Organization:**
```typescript
import { describe, it, expect, beforeEach, vi } from "vitest";

describe("ComponentName", () => {
  let fixture: any;

  beforeEach(() => {
    // Setup shared test state, reset mocks
    fixture = new ComponentName();
  });

  describe("Feature/Method Group", () => {
    it("should perform expected behavior", () => {
      // Arrange
      const input = { /* ... */ };
      
      // Act
      const result = fixture.doSomething(input);
      
      // Assert
      expect(result).toBe(expected);
    });
  });
});
```

**Key Patterns:**

1. **Describe Blocks:** Nested hierarchy (top-level feature, then sub-features)
   - Example in `mockProvider.test.ts`: `describe("MockProvider")` → `describe("Team Management")` → `it("should register a team")`

2. **BeforeEach Hooks:** Reset shared state and mocks between tests
   - Example in `connectionSlice.test.ts`:
     ```typescript
     beforeEach(() => {
       useInkTixStore.setState({
         isConnected: false,
         isConnecting: false,
         // ... reset all fields
       });
     });
     ```

3. **Test Naming:** "should [expected behavior]" pattern
   - `it("should start disconnected")`
   - `it("should update connection state")`
   - `it("should register a team and return an id")`

## Mocking

**Framework:** `vitest` mocking with `vi.mock()` and `vi.fn()`

**Patterns:**

1. **Service Mocking:**
   ```typescript
   vi.mock("../../services/blockchain", () => ({
     BlockchainService: {
       getInstance: () => ({
         connectToNetwork: vi.fn().mockResolvedValue({ success: true }),
         connectWallet: vi.fn().mockResolvedValue({
           success: true,
           data: [{ address: "...", meta: { name: "Alice" } }]
         }),
         // ... other methods
       }),
     },
   }));
   ```
   (From `connectionSlice.test.ts`)

2. **Hook Mocking:**
   ```typescript
   vi.mock("../../../contexts/BlockchainContext", () => ({
     useBlockchain: () => ({
       isDeployingContract: false,
       deployContract: vi.fn().mockResolvedValue({
         success: true,
         data: "0xmockaddress",
       }),
     }),
   }));
   ```
   (From `ContractDeployment.test.tsx`)

3. **Function Mocks:**
   ```typescript
   const mockCallback = vi.fn();
   render(<ContractDeployment onContractTypeChange={mockCallback} />);
   expect(mockCallback).toHaveBeenCalled();
   ```

**What to Mock:**
- External services (BlockchainService, API calls)
- Context providers and hooks
- Browser APIs (localStorage, if not already polyfilled)
- Components used as children/dependencies

**What NOT to Mock:**
- Zustand store (test against real store state)
- Built-in DOM APIs (use jsdom env; jsdom handles these)
- Utility functions (test with real implementations unless they're expensive/external)

## Fixtures and Factories

**Test Data:**

1. **Mock Store State:** Zustand state reset in `beforeEach`:
   ```typescript
   useInkTixStore.setState({
     isConnected: false,
     accounts: [{ address: "...", meta: { name: "Alice" } }],
     balance: "5000",
   });
   ```

2. **Mock Contract Results:**
   ```typescript
   const mockResult = {
     success: true,
     data: "0x1234",
     txHash: "0xmocktx",
   };
   ```

3. **Provider Mock Data:** MockProvider includes seeded data
   - Teams: Lakers, Celtics
   - Artists: Drake, The Weeknd
   - Venues: Crypto.com Arena, Staples Center
   - Events: "Lakers vs Warriors", concert events
   - Accessible via `getTeam(1)`, `getVenue(1)`, `getEvent(1)`

**Location:**
- Inline in test files (no separate fixtures directory)
- Mock data defined in `beforeEach` for per-test isolation
- Shared mock setup in `test-setup.ts` (localStorage polyfill)

## Coverage

**Requirements:** No coverage enforcement currently in place

**View Coverage:**
```bash
npm test -- --coverage
```

**Coverage Report:** Generates in `coverage/` directory (implicit from Vitest)

**Current Test Files (3 total):**
- `src/store/__tests__/connectionSlice.test.ts` (44 tests)
- `src/sdk/__tests__/mockProvider.test.ts` (24 tests)
- `src/components/smart-contracts/__tests__/ContractDeployment.test.tsx` (3 tests)

## Test Types

**Unit Tests:**
- Scope: Individual functions, hooks, store slices
- Approach: Isolated with mocked dependencies
- Examples:
  - `connectionSlice.test.ts`: Tests store state setters and getters
  - `mockProvider.test.ts`: Tests SDK operations with in-memory data
  - `ContractDeployment.test.tsx`: Tests component rendering and user interaction

**Integration Tests:**
- Scope: Component + store interactions (not yet implemented)
- Approach: Test full component lifecycle with Zustand store
- Blockers: Complex Polkadot.js API setup; currently mocked instead

**E2E Tests:**
- Framework: Playwright
- Scope: End-to-end user flows (currently paused)
- Command: `npm run test:e2e`
- Status: Not enforced; infrastructure exists but tests not written

## Common Patterns

**Async Testing:**
```typescript
it("should connect to network", async () => {
  const result = await service.connectToNetwork("wss://...");
  expect(result.success).toBe(true);
});
```
- All async operations return `Promise` or `Promise<ContractCallResult<T>>`
- Always `await` the promise before asserting

**Error Testing:**
```typescript
it("should return error for non-existent team", async () => {
  const result = await provider.getTeam(999);
  expect(result.success).toBe(false);
});
```
- Errors returned in result wrapper (never thrown)
- Check `.success === false` and optionally `.error` message

**React Component Testing:**
```typescript
import { render, screen, fireEvent } from "@testing-library/react";

it("renders deployment form", () => {
  render(<ContractDeployment onContractTypeChange={vi.fn()} />);
  expect(screen.getByText("Contract File (.wasm)")).toBeTruthy();
});

it("allows changing endowment value", () => {
  render(<ContractDeployment onContractTypeChange={vi.fn()} />);
  const input = screen.getByDisplayValue("1.0");
  fireEvent.change(input, { target: { value: "2.5" } });
  expect(input).toHaveValue(2.5);
});
```
- Use `screen` queries (`getByText`, `getByRole`, `getByDisplayValue`) instead of `container`
- Use `fireEvent` for user interactions
- Reset store state between tests to avoid cross-test pollution

**Store State Verification:**
```typescript
it("should update connection state", () => {
  useInkTixStore.getState().setIsConnected(true);
  expect(useInkTixStore.getState().isConnected).toBe(true);
});
```
- Access store state with `useInkTixStore.getState()`
- Call store actions directly (no rendering needed for unit tests)

## Test Setup

**File:** `frontend/src/test-setup.ts`

**Contents:**
- Imports `@testing-library/jest-dom/vitest` for DOM matchers
- Polyfills `localStorage` if not available in jsdom
- Detects if localStorage works; falls back to in-memory polyfill if broken

**Vitest Configuration:**
- Entry point: `frontend/vitest.config.ts`
- Settings:
  - `test.environment: "jsdom"` — Browser-like DOM
  - `test.globals: true` — No need to import `describe`, `it`, `expect`
  - `test.setupFiles: ["./src/test-setup.ts"]` — Run setup before tests
  - `test.include: ["src/**/*.{test,spec}.{ts,tsx}"]` — Find test files

## Rust Contract Tests

**Structure:**
- Helpers in `src/tests/core_tests.rs`, `src/tests/sports_tests.rs`, `src/tests/concert_tests.rs`
- Inline tests in source files with `#[cfg(test)]` blocks (feature-gated for sports/concert)
- Example helper in `core_tests.rs`:
  ```rust
  pub struct CoreTestHelpers;
  
  impl CoreTestHelpers {
    pub fn verify_empty_contract_state(totals: (u32, u32, u32, u64, u32, u32)) {
      assert_eq!(totals, (0, 0, 0, 0, 0, 0));
    }
  }
  ```

**Running:**
```bash
cargo test                 # Run all tests
cargo test --features sports,concert  # Run with features enabled
```

---

*Testing analysis: 2026-09-20*
