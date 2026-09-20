# Codebase Concerns

**Analysis Date:** 2026-09-20

## Tech Debt

### Missing Payment Enforcement (Critical)

**Issue:** The smart contract accepts ticket prices but has **zero `#[ink(payable)]` messages and zero `transferred_value()` calls**. `purchase_ticket` calculates a price but never validates that value was actually sent. Tickets are purchasable for free.

**Files:** `contracts/inktix/src/logic/core/ticket_management.rs` (purchase logic); `contracts/inktix/src/lib.rs` (line 174+, message handler)

**Impact:** Core business logic is non-functional. Revenue collection is impossible. The entire fee structure (currency conversion, anti-scalping caps, resale price limits) are unenforced bookkeeping.

**Fix approach:** 
- Phase 3 of v2_plan.md covers this: make `purchase_ticket` `#[ink(payable)]`, compare `self.env().transferred_value()` (U256) against calculated price.
- Adopt `U256` only at the transfer/comparison boundary; keep `u128` as internal storage type.
- **Blocking prerequisite:** Must live on Passet Hub with real value transfer (Phase 2 dependency).

### Missing Withdrawal / Fund Payout Mechanism (Critical)

**Issue:** There is **no withdraw/settlement message** anywhere in the contract. If payment enforcement is added, funds become trapped with no way to move them out.

**Files:** `contracts/inktix/src/lib.rs` (52 message handlers — none handle withdrawals)

**Impact:** Shipping `purchase_ticket` as payable without a corresponding owner/organizer-guarded withdraw path creates a fund trap. Once payment enforcement lands, organizers cannot access collected revenue.

**Fix approach:** Add owner-guarded `withdraw()` and possibly organizer-specific `claim_revenue()` methods in Phase 3 before merging payment enforcement.

### No Input Validation (Medium)

**Issue:** Input validation is nearly absent. Only one explicit check found across 22 files: `if name.is_empty()` in `event_management.rs`.

**Files:** All `contracts/inktix/src/logic/core/*.rs` and `contracts/inktix/src/logic/sports/*.rs` files

**Impact:** Accepts empty strings, zero/invalid IDs, capacity overflows, invalid currency rates, out-of-range pricing multipliers without bounds checking.

**Fix approach:** 
- Define a validation module with helper functions: `validate_positive()`, `validate_string_length()`, `validate_percentage()`, etc.
- Apply consistently at message entry points before state mutations.
- Target: Phase 3's payment enforcement pass should enforce input validity for all monetary operations.

## Known Bugs

### README Badge Mismatch

**Symptoms:** Badge states `ink! 5.1.1` but actual `contracts/inktix/Cargo.toml` specifies `ink = "6.0.0-beta.1"` (Phase 1 migration to v6/pallet-revive completed 2026-07-29).

**Files:** 
- `README.md` line 9: badge text
- `contracts/inktix/Cargo.toml` line 9: actual dependency

**Trigger:** Viewing repository; confuses developers about contract version/toolchain.

**Workaround:** Update README to reflect v6 migration status and note the ink! v6 beta / pallet-revive beta status.

### Platform-Specific Bash Bug (Minor)

**Symptoms:** Line 51 of `scripts/deploy-and-configure.sh` uses `grep -oP` (GNU grep with Perl regex), which fails silently on BSD grep (macOS). Address extraction falls through to "could not extract" error branch.

**Files:** `scripts/deploy-and-configure.sh`

**Trigger:** Running deploy script on macOS.

**Workaround:** None currently; script fails silently. Fix documented in Phase 2 of v2_plan.md as a concrete gap to fix before Passet Hub deployment.

## Security Considerations

### Composite Key Storage Layout Risk (Medium)

**Issue:** `team_loyalty_profiles: Mapping<(Address, u32), TeamLoyaltyProfile>` uses a composite tuple key `(Address, u32)`. Phase 1b spike (deferred to 1c) planned to validate `TypeInfo`/`StorageLayout` bounds on this shape before committing.

**Files:** `contracts/inktix/src/storage/contract_storage.rs` line 119; used in `contracts/inktix/src/logic/sports/advanced_team_loyalty.rs`

**Current mitigation:** 1c migration completed successfully; 14 inline tests pass. No empirical evidence of layout failure, but this was flagged as the "single riskiest storage item" in the port plan.

**Recommendations:** 
- Before Phase 2 deployment to Passet Hub, run explicit on-chain tests against a live chain that call `stake_on_team()` and verify the composite key round-trips correctly.
- If storage layout fails on-chain, fallback to `UserId([u8; 20])` newtype as documented in Phase 1b plan.

### No Reentrancy Guards (Medium)

**Issue:** Once Phase 3 adds payable `purchase_ticket` and `withdraw()` functions, the contract will transfer value. No mutex/reentrancy guard patterns observed in the codebase.

**Files:** All of `contracts/inktix/src/` (no guards present)

**Current mitigation:** No cross-contract calls yet, and ink! messages are atomic; reentrancy is only a risk if the contract calls out before finalizing state.

**Recommendations:**
- Phase 3 must finalize all state writes **before** any `env().transfer()` call.
- Document this invariant for all future payment/transfer messages.
- Consider adopting a check-effects-interactions pattern in code review.

### No Explicit ACL Beyond Owner Check (Medium)

**Issue:** Only `ensure_owner()` guard exists. Sports/concert features have no organizer/artist role-based access control.

**Files:** `contracts/inktix/src/lib.rs` lines 80–85 (owner check only)

**Current mitigation:** All sensitive messages that call organizer-specific logic (`register_venue`, `create_event`, etc.) currently check only `self.ensure_owner()`. For a multi-tenant platform, this is too coarse.

**Recommendations:**
- Introduce role types: `Owner`, `Organizer`, `Artist`, `Venue Manager`.
- Store role assignments in storage and check before sensitive operations.
- Phase 3 / Phase 4 scope: add full ACL before revenue-affecting messages are live.

### No Rate Limiting (Low)

**Issue:** No per-user or per-call throttling on any message. A single caller can invoke `purchase_ticket` unlimited times in one block (though anti-scalping purchase limits apply).

**Files:** All message handlers in `contracts/inktix/src/logic/`

**Current mitigation:** Substrate block size and gas limits provide implicit rate limiting. Anti-scalping purchase caps add business-logic limits.

**Recommendations:** If anti-scalping is meant to prevent bot-driven mass purchasing, document the explicit limits (purchase_count per period, time_between_purchases) and verify they're enforced end-to-end in Phase 3.

## Performance Bottlenecks

### Monolithic Storage Struct (Medium)

**Issue:** 180+ fields in `InkTixStorage` (`contracts/inktix/src/storage/contract_storage.rs`). No modular separation. Single atomic borrow-all-or-nothing pattern during message execution.

**Files:** `contracts/inktix/src/storage/contract_storage.rs` lines 33–187

**Cause:** Unified contract design (Phase 3 decomposition note: splitting was evaluated and deferred). All core, sports, and concert logic reads/writes the same flat struct.

**Improvement path:**
- Current state is acceptable for single-chain workloads; Phase 1c confirmed blob size is 103.6K (well under limits).
- If per-module storage isolation becomes a bottleneck (contention on hot fields like `user_tickets`, `loyalty_profiles`), a future refactor could separate storage by feature (`CoreStorage`, `SportsStorage`, `ConcertStorage`) and use sub-modules.
- Defer to Phase 4 / v3 unless profiling shows contention.

### No Pagination on `get_all_events` (Medium)

**Issue:** `get_all_events` message handler likely returns all event records in a single response, unbounded.

**Files:** `contracts/inktix/src/lib.rs` (message handler); exact implementation in `contracts/inktix/src/logic/core/event_management.rs`

**Cause:** No limit/offset parameters in the message signature.

**Improvement path:**
- Add optional `limit` and `offset` parameters to return paginated results.
- Frontend should batch requests for large datasets.
- Priority: Medium (only hits performance if event count exceeds thousands, unlikely for testnet-scale).

## Fragile Areas

### Ink! v6 Beta Toolchain (High)

**Files:** 
- `contracts/inktix/Cargo.toml` line 9: `ink = "6.0.0-beta.1"`
- `contracts/inktix/rust-toolchain.toml`: pins `nightly-2026-01-15`
- `.github/workflows/ci.yml`: CI config

**Why fragile:** 
- ink! v6 and cargo-contract v6.0.0-beta.2 are beta releases from a team whose active development paused Jan 2026 (per Phase 0 notes).
- A real blocker was found in Phase 0: default nightly Rust fails with `.json target specs require -Zjson-target-spec` error. Fixed by pinning `nightly-2026-01-15`, which predates the upstream Rust/Cargo strictness change. **No patched cargo-contract exists**, so this pin is permanent, not a workaround.
- If the pinned nightly becomes unavailable or if upstream Rust makes further breaking changes, the build pipeline breaks with no forward path.

**Safe modification:** 
- Never bump `nightly-*` version without confirming the full build → instantiate → call round-trip succeeds against a live `ink-node`.
- Document any toolchain issues discovered; check Phase 0 notes (`docs/rearchitecture_toolchain_notes.md`) before troubleshooting.
- Monitor ink! repo for v6 stable release; upgrade aggressively once available to reduce beta risk.

### Phase 1 Behavioral Validation Incomplete (Medium)

**What's not tested:** Phase 1d (behavioral validation module-by-module against local `ink-node`) is scoped but not started. Exit gate requires every module below to be validated on-chain:
1. `venue_management` / `event_management`
2. `ticket_management` purchase + `user_tickets` round-trip
3. `pricing` (time-multiplier logic under manual-seal blocks)
4. `nft_management`
5. `anti_scalping` (Vec<Address> blacklist/whitelist)
6. `loyalty` / `advanced_team_loyalty` (composite key, riskiest item)
7. `season_pass_management`, `fantasy_sports_management`, `analytics`, `artist_management`, `xcm_management`

**Files:** All of `contracts/inktix/src/logic/**/*.rs`

**Risk:** Mechanical compile-time type migration (1c) is complete, but on-chain behavior unknown. A module may compile but fail at runtime due to:
- Ink! v6 changed semantics for storage iteration, query results, or transaction finality.
- Address/H160 round-trips don't survive serialization/deserialization on real chain.
- Time-multiplier logic assumptions break under real block timing (not manual-seal).

**Test coverage:** 14 inline `#[ink::test]` tests pass (unit-level), but these run off-chain. No on-chain integration tests yet.

### Address Identity Mismatch in Frontend (High)

**What's fragile:** Frontend (`frontend/src/`) still references legacy contract addressing (SS58/AccountId) in three files that import deleted contracts:
- `frontend/src/app/docs/page.tsx` — documentation references legacy contract addresses
- `frontend/src/components/smart-contracts/ContractDeployment.tsx` — UI references legacy addresses  
- `frontend/src/utils/contractMethods.ts` — utility functions expect SS58 addressing

Meanwhile, pallet-revive contracts use H160 (Ethereum-style 20-byte) addresses. The Phase 4 integration work will wholesale rewrite these, but until Phase 4 completes, the frontend cannot communicate with a pallet-revive contract.

**Files:** 
- `frontend/src/sdk/contractProvider.ts` line 444: still uses `@polkadot/api-contract` `ContractPromise`, which is `pallet-contracts`-only
- `frontend/src/services/blockchain.ts` line 148: checks `api.tx.contracts` (old pallet), not `api.tx.revive` (new pallet)

**Safe modification:** Do not use these files to deploy/interact with pallet-revive contracts. Phase 4 creates new `reviveContractProvider.ts`. Ensure addresses are H160-formatted (`0x...` 40-char hex), never SS58.

## Scaling Limits

### Contract Blob Size Budget (Medium)

**Current capacity:** 103.6K (release blob after Phase 1c). No CodeTooLarge errors on dev `ink-node` (permissive limits).

**Limit:** pallet-revive on real Passet Hub / Polkadot Hub has not yet been cross-checked against this size. Real limits unknown.

**Scaling path:** 
- Phase 2 must confirm the live chain's `CodeTooLarge` limit and measure gas/storage-deposit costs.
- If the real limit is <103K, Phase 1c staged the pressure valve: build with `core` only, defer `sports`/`concert` features.
- Current design (all-in-one) is lower-risk than cross-contract calls would be; keep it unified unless real evidence forces a split.

### Unbounded Storage Allocations (Medium)

**Issue:** `Vec<Address>`, `Vec<u64>`, `Vec<u32>` fields in storage (e.g., `user_tickets`, `league_participants`, team/venue-specific lists) can grow indefinitely.

**Files:** `contracts/inktix/src/storage/contract_storage.rs` lines 54, 91, 111, 127, etc.

**Current mitigation:** No explicit max-length validation. Substrate's storage-deposit system will eventually charge so much gas that appending becomes prohibitive, but this is implicit, not enforced.

**Scaling path:**
- Phase 3 input validation work should add reasonable bounds: e.g., max 10,000 tickets per user, max 100 participants per fantasy league.
- Document these limits in Phase 3's input validation pass.

## Dependencies at Risk

### Polkadot.js API Transition (Medium)

**Risk:** Frontend uses `@polkadot/api` ^16.4.6 and `@polkadot/api-contract` ^16.5.4, both tied to `pallet-contracts` semantics. Phase 4 (pallet-revive integration) will need to:
- Drop `api-contract` entirely (no pallet-revive equivalent).
- Switch to PAPI / ink! SDK over WSS, or viem/ethers over ETH-RPC.
- Keep `@polkadot/api` for balance queries and XCM.

**Files:** 
- `frontend/package.json` lines 18–19
- `frontend/src/services/blockchain.ts` lines 18–19: imports

**Impact:** This is a known, intentional dependency replacement, not a surprise bug. Phase 4 plan covers it. Flagged here to note that v1 frontend code is not forward-compatible with pallet-revive contracts.

### Unstable Ink! Beta Version (High)

**Risk:** Upgrading `ink = "6.0.0-beta.1"` without explicit testing will silently break the contract. Beta releases don't follow semver stability guarantees.

**Files:** `contracts/inktix/Cargo.toml` line 9

**Mitigation:** Pin to exact version `6.0.0-beta.1` indefinitely until v6 stable is released. When upgrading, repeat Phase 0's full round-trip validation.

**Migration plan:** Watch the [ink! repository](https://github.com/paritytech/ink) for v6 stable release; immediately bump and re-validate contract on a live chain before merging.

## Missing Critical Features

### Payment Enforcement Not Yet Implemented

**Problem:** Described in "Tech Debt" section above. The entire revenue model is non-functional. Tickets are free.

**Blocks:** Any monetization, revenue reporting, or withdrawal UX until Phase 3 completes.

### Resale Marketplace Buy-Side Incomplete

**Problem:** `resell_ticket` creates listings, but there is **no buy-side message**. The marketplace is half-built.

**Files:** `contracts/inktix/src/logic/core/ticket_management.rs` — only sell-side is implemented

**Blocks:** Any test/use of the resale flow until a `purchase_resale()` or `accept_resale_listing()` message is added.

**Fix approach:** Phase 3 should either (a) build the complete payable buy-side, or (b) explicitly deprecate/remove the half-built sell-side to avoid confusion.

## Test Coverage Gaps

### Minimal Frontend Test Suite (Low)

**What's not tested:** UI components, state management, contract calls, wallet connection logic.

**Files:** Only 3 test files:
- `frontend/src/components/smart-contracts/__tests__/ContractDeployment.test.tsx`
- `frontend/src/sdk/__tests__/mockProvider.test.ts`
- `frontend/src/store/__tests__/connectionSlice.test.ts`

**Risk:** Frontend features like event filtering, pricing display, ticket resale, analytics charts have zero test coverage. Regressions are not caught until manual testing.

**Priority:** Low for now (hackathon-phase, dynamic product). Medium before production launch. Add tests incrementally with Phase 4 (frontend integration with pallet-revive).

### No On-Chain Integration Tests (Medium)

**What's not tested:** Behavior of each contract module when deployed to a live chain. Phase 1d (validation against `ink-node`) is scoped but not run.

**Files:** All of `contracts/inktix/src/logic/**/*.rs`

**Risk:** A module compiles and passes unit tests but fails on-chain due to storage layout mismatches, address identity issues (Address/H160 serialization), or timing assumptions (pricing logic assumes wall-clock block advancement, not manual-seal).

**Test coverage:** 14 unit tests run off-chain only.

**Priority:** High. Phase 1d **must run before Phase 2 deployment** to Passet Hub. Unblocks Phase 3 and Phase 4.

### E2E Tests Paused (Medium)

**What's not tested:** Full user flows: connect wallet → browse events → purchase ticket → view on-chain → transfer → resale. Playwright e2e tests exist but are not integrated into CI.

**Files:** 
- `frontend/e2e/wallet-connect.spec.ts`
- `frontend/e2e/buy-ticket.spec.ts`
- `frontend/e2e/deploy-contract.spec.ts`

**Current status:** Per memory note "project_status.md" — e2e test plan paused.

**Risk:** Critical flows are never exercised end-to-end. UI changes, contract changes, and wallet extension updates can break flows without detection.

**Priority:** Defer until Phase 4 (frontend integration). At that point, e2e tests should be:
- Integrated into CI/CD
- Run against both mock mode and live Passet Hub testnet
- Capture wallet extension interactions explicitly (e2g., MetaMask rejection, user cancellation)

### Anti-Scalping Enforcement Not Validated (Low)

**What's not tested:** Purchase limits, transfer cooldowns, and resale price caps are stored but never actually enforced. No test exercises the boundary case where a user hits the purchase limit.

**Files:** `contracts/inktix/src/logic/core/anti_scalping.rs` (logic); `contracts/inktix/src/logic/core/ticket_management.rs` (purchase enforcement — missing)

**Risk:** Anti-scalping config is silently ignored. Marketing claims anti-scalping features, but they don't work.

**Priority:** High for Phase 3. When payment enforcement is added, anti-scalping price-cap constraints must be gated on the payable path. Add unit tests and on-chain integration tests at that time.

---

*Concerns audit: 2026-09-20*
