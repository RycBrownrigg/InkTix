---
gsd_state_version: 1.0
milestone: v2
milestone_name: Migration
current_phase: 01
current_phase_name: On-Chain Module Validation
status: executing
stopped_at: Phase 1 context gathered
last_updated: "2026-09-21T17:10:25.133Z"
last_activity: 2026-09-21
last_activity_desc: "Completed quick task 260921-g2j: Fix CI failures on redesign/polkadot-hub"
state_head: 8d4e4c48ecc1e4445426adbc9e72957eeae3d65d
progress:
  total_phases: 4
  completed_phases: 0
  total_plans: 3
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-09-20)

**Core value:** The full ticket-purchase flow — browse, pay with real value, mint the ticket NFT, and let funds be withdrawn — works end-to-end against a live ink! v6 / `pallet-revive` chain (Passet Hub).
**Current focus:** Phase 01 — On-Chain Module Validation

## Current Position

Phase: 01 (On-Chain Module Validation) — EXECUTING
Plan: 1 of 3
Status: Executing Phase 01
Last activity: 2026-09-21 — Completed quick task 260921-g2j: Fix CI failures on redesign/polkadot-hub

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Velocity:**

- Total plans completed: 0
- Average duration: - min
- Total execution time: 0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| - | - | - | - |

**Recent Trend:**

- Last 5 plans: -
- Trend: -

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Project init]: v1 scope = `docs/v2_plan.md` Phases 1d–4 only; Phase 5 (mainnet cutover) deferred to a future milestone
- [Project init]: Overpayment policy is reject-on-mismatch, not refund (avoids reentrancy-ordering surface with no guards yet in place)
- [Project init]: Resale marketplace buy-side deferred; listing-only carries over
- [Project init]: PAPI/ink! SDK chosen over viem/MetaMask for Phase 4 — preserves existing 4-wallet UX

### Pending Todos

- [`TODO(v2-gap)` @ `contracts/inktix/src/logic/core/venue_management.rs:116`] Gap A — `purchase_parking_pass` is an unimplemented stub: accepts all params, discards them, persists nothing, increments the wrong (`"venue"` instead of `"parking_pass"`) ID counter. Found during the 260921-g2j CI-fix quick task while investigating an "unused variable" warning. Suggest scheduling with venue/sports feature completion.
- [`TODO(v2-gap)` @ `contracts/inktix/src/lib.rs:468`] Gap B — `purchase_concession_credits` drops the caller's `CurrencyId` and hardcodes `"DOT"`; `CurrencyId` has no string conversion yet. Found during 260921-g2j. Suggest Phase 3 (payment enforcement).
- [`TODO(v2-gap)` @ `contracts/inktix/src/logic/core/event_management.rs:43`, lower confidence] Gap C — Sports event `sport_type` is destructured but never validated against the team/season `sport_type`. Found during 260921-g2j. Suggest Phase 1 (on-chain module validation) or a dedicated data-integrity pass, pending a product decision on intended validation semantics.

### Blockers/Concerns

- [CONCERNS.md] Zero `#[ink(payable)]` messages and zero `transferred_value()`/`.transfer()` calls exist yet — tickets are functionally free until Phase 3 lands. Not a blocker for Phase 1/2, but the reason Phase 3 exists.
- [CONCERNS.md] No reentrancy guards anywhere in the contract — relevant once Phase 3 adds payable `purchase_ticket` and withdraw messages; state must finalize before any `env().transfer()` call.
- [CONCERNS.md] ink! v6 (`6.0.0-beta.1`) / `cargo-contract` (`6.0.0-beta.2`) are beta releases from a team whose active development paused as of Jan 2026; `nightly-2026-01-15` is a long-term toolchain pin, not a temporary workaround.
- [CONCERNS.md] Composite key `Mapping<(Address, u32), TeamLoyaltyProfile>` is flagged as the single riskiest storage item — Phase 1 validation must explicitly exercise `stake_on_team()` on-chain.
- [CONCERNS.md] Zero on-chain integration tests exist today (14 inline unit tests are off-chain only); Phase 1 closes this gap.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260921-g2j | Fix CI failures on redesign/polkadot-hub: resolve 19 -D-warnings-promoted-to-errors in contracts/inktix and resync frontend/package-lock.json | 2026-09-21 | 2164fdec | [260921-g2j-fix-ci-failures-on-redesign-polkadot-hub](./quick/260921-g2j-fix-ci-failures-on-redesign-polkadot-hub/) |

## Deferred Items

Items acknowledged and deferred at milestone close, most recent first:

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| *(none)* | | | | |

## Session Continuity

Last session: 2026-09-20T15:57:38.931Z
Stopped at: Phase 1 context gathered
Resume file: .planning/phases/01-on-chain-module-validation/01-CONTEXT.md
