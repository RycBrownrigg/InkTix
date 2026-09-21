---
phase: quick-260921-g2j
plan: 01
subsystem: ci
tags: [ci, contracts, ink!, frontend, npm, lockfile]
status: complete
dependency-graph:
  requires: []
  provides:
    - "Green CI on redesign/polkadot-hub"
    - "contracts/inktix compiles clean under -D warnings"
    - "frontend npm ci works from a resynced lockfile"
  affects:
    - "contracts/inktix/src/**"
    - "frontend/package-lock.json"
tech-stack:
  added: []
  patterns:
    - "Scoped #[allow(...)] over crate-level allow for known macro/tooling false positives"
    - "Discard binding (`let _ = x;`) to silence unused-param warnings on ink! message args without changing the published ABI label"
    - "TODO(v2-gap): prefix for greppable, durable markers on genuine-but-out-of-scope defects"
key-files:
  created: []
  modified:
    - contracts/inktix/src/lib.rs
    - contracts/inktix/src/logic/core/event_management.rs
    - contracts/inktix/src/logic/core/ticket_management.rs
    - contracts/inktix/src/logic/core/venue_management.rs
    - contracts/inktix/src/logic/sports/cross_chain_management.rs
    - contracts/inktix/src/logic/sports/fantasy_sports_management.rs
    - contracts/inktix/src/logic/sports/loyalty.rs
    - contracts/inktix/src/tests/core_tests.rs
    - contracts/inktix/src/tests/sports_tests.rs
    - contracts/inktix/src/tests/concert_tests.rs
    - frontend/package-lock.json
decisions:
  - "Compiler output is authoritative over planning findings: fixed one additional dead import (lib.rs:13 `use ink::primitives::Address;`) and one additional cfg-gate (lib.rs's `venue` import, only reachable under `#[cfg(feature = \"sports\")]`) that planning's static read of the code missed but `cargo test -D warnings` / `cargo contract build -D warnings` surfaced."
  - "Silenced dead_code on three test-helper structs in contracts/inktix/src/tests/ (never constructed by any inline lib.rs test) with scoped #[allow(dead_code)] — these only appeared once the crate compiled clean and cargo test reached that compilation unit; fixing was in-scope per the plan's own iterate-until-clean instruction, not a new file added outside the mandate."
  - "Accepted npm's peer/dev metadata-flag churn on ~25 unrelated lockfile entries from regenerating with local npm 11 rather than chasing an exact-match diff against whatever npm version originally produced the lockfile — no package version changed, and the plan explicitly named the Node 26 local vs Node 20 CI difference as the expected source of divergence, with the live CI run as the actual confirming signal."
metrics:
  duration: "~40 minutes"
  completed: "2026-09-21"
actuals:
  tokens: 34000
  tasks: 3
  commits: 2
---

# Phase quick-260921-g2j Plan 01: Fix CI failures on redesign/polkadot-hub Summary

Cleared 19 pre-existing `-D warnings` compiler errors in the ink! v6 contract crate behavior-neutrally and resynced a pruned `frontend/package-lock.json` so `npm ci` no longer fails with `EUSAGE` — both CI jobs now pass on the pushed commit.

## What Was Built

**Task 1 — Contract warnings cleared under `-D warnings` (commit `cf2aa06f`):**
- Removed 8 dead imports (7 anticipated by planning, plus `lib.rs:13`'s `use ink::primitives::Address;`, found only by the compiler once the other 7 were cleared).
- Underscore-prefixed `home_team` (side-effect existence check) and the six unused parameters of `VenueManagement::purchase_parking_pass` (an internal, non-ABI helper).
- Added `let _ = event_id;` / `let _ = currency;` discard bindings inside two `#[ink(message)]` functions instead of renaming the parameters, preserving the published ink! ABI arg labels the frontend SDK consumes.
- Added a scoped `#[allow(unexpected_cfgs)]` directly above `#[ink::contract]` for the `ink_abi` cfg false positive (known ink! v6 macro/toolchain interaction, not project-code sloppiness).
- Gated the `venue` module import with `#[cfg(feature = "sports")]` — it's only referenced from sports-gated code, and this was flagged as unused specifically in the `cargo contract build` (release/WASM) compile pass, not the `cargo test` pass.
- Silenced `dead_code` on three test-helper structs in `src/tests/{core,sports,concert}_tests.rs` (`#[allow(dead_code)]`, scoped) — these helpers exist for future test use per their own doc comments and were never constructed; this warning only surfaced once the crate compiled clean and `cargo test` proceeded to the `#[cfg(test)] mod tests;` compilation unit.
- Documented three genuine logic gaps in place with `TODO(v2-gap):` markers (exactly 3, verified by `grep -c`) instead of silently suppressing them. See **Follow-up work surfaced** below.
- `RUSTFLAGS="-D warnings" cargo test`: 14/14 passing, zero warnings.
- `RUSTFLAGS="-D warnings" cargo contract build`: succeeds, zero warnings.

**Task 2 — Frontend lockfile resynced (commit `2164fdec`):**
- `npm install --package-lock-only` added resolved entries for `@polkadot-api/substrate-client@0.1.4` and `smoldot@2.0.26` (both pre-existing transitive deps of `@polkadot/api@^16.4.6`, not new direct dependencies) — each with a `registry.npmjs.org` `resolved` URL and an `integrity` hash.
- No package version changed anywhere else in the lockfile (verified via diff on `"version"` lines — only the two new entries appear).
- `package.json` untouched.
- `npm ci`, `npm run lint`, `npm run build`, and `npx vitest run` all exit 0.

**Task 3 — Committed, pushed, and confirmed green (this task made no additional commits — Tasks 1 and 2 were already committed individually):**
- Re-ran both gates on the combined working tree; no interaction issues between the contract and frontend changes.
- Pushed `redesign/polkadot-hub` (`8d4e4c48..2164fdec`).
- Watched GitHub Actions run [35640297128](https://github.com/RycBrownrigg/InkTix/actions/runs/35640297128) to completion: **both jobs succeeded** (`Smart Contracts` 3m17s, `Frontend` 1m15s). `headSha` matched the pushed commit `2164fdec`. Final `conclusion: success`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - blocking issue, per task's own explicit "iterate until clean" instruction] Additional dead import beyond planning's 7-site table**
- **Found during:** Task 1, initial `RUSTFLAGS="-D warnings" cargo test` baseline run
- **Issue:** `lib.rs:13` `use ink::primitives::Address;` (crate-root level, outside `pub mod inktix`) was unused — shadowed by the ink! macro's own environment-generated `Address` type once accessed via `use super::*;` inside the module. Planning's static read of the flagged sites didn't catch this one (it wasn't in the original 19-site list read from the CI log alone).
- **Fix:** Removed the import. `Address` remains available inside `pub mod inktix` via the ink! macro's own generated scope.
- **Files modified:** `contracts/inktix/src/lib.rs`
- **Commit:** `cf2aa06f`

**2. [Rule 1 - bug, root-cause fix] `venue` import unused specifically in `cargo contract build`'s release/WASM pass**
- **Found during:** Task 1, second verification pass — `cargo test` was clean but `cargo contract build` printed one residual warning (not promoted to error, since cargo-contract's internal build appeared not to inherit the shell's `RUSTFLAGS` for that specific compile unit) for `crate::types::core_types::venue` being unused.
- **Issue:** `venue::ConcessionCreditType::General` is only referenced inside `purchase_concession_credits`, which is entirely gated by `#[cfg(feature = "sports")]`. The plain (ungated) `use` statement was structurally correct but incomplete — it should have matched the gating of its only call site.
- **Fix:** Added `#[cfg(feature = "sports")]` directly above the `venue` import, matching its actual usage. This is always correct regardless of build pipeline/feature-flag quirks, rather than relying on the fact that the warning happened not to be promoted to an error in this instance.
- **Files modified:** `contracts/inktix/src/lib.rs`
- **Commit:** `cf2aa06f`

**3. [Rule 3 - blocking issue, per task's own explicit "iterate until clean" instruction] Dead-code warnings on unused test helpers, surfaced only after the first fix round**
- **Found during:** Task 1, second `cargo test` run (after the initial 19 warnings cleared, `cargo test` proceeded further into the `#[cfg(test)] mod tests;` unit and surfaced 6 new `-D dead-code` errors on `CoreTestHelpers`, `SportsTestHelpers`, `ConcertTestHelpers` and their methods)
- **Issue:** These structs/methods in `src/tests/{core,sports,concert}_tests.rs` are reusable assertion helpers per their own doc comments ("Primary tests live inline in lib.rs") but are never actually called by any inline test yet.
- **Fix:** Added scoped `#[allow(dead_code)]` on each struct and impl block. No test assertions changed; the 14 existing tests pass unmodified, as required.
- **Files modified:** `contracts/inktix/src/tests/core_tests.rs`, `contracts/inktix/src/tests/sports_tests.rs`, `contracts/inktix/src/tests/concert_tests.rs`
- **Commit:** `cf2aa06f`

None of the above required Rule 4 (architectural change) — all were narrow, behavior-neutral fixes within the task's own explicitly-permitted edit categories (import removal/gating, scoped allow, discard binding, comment).

## Follow-up work surfaced

Three genuine logic gaps were found while clearing lint noise and are **not fixed** in this quick task — each is a behavior change requiring its own testing/scheduling, per the plan's explicit prohibition on repairing them here.

**Gap A — `VenueManagement::purchase_parking_pass` is an unimplemented stub**
- **File/line:** `contracts/inktix/src/logic/core/venue_management.rs:116-138` (marker at line 116)
- **What's missing:** Accepts `buyer`, `pass_type`, `valid_from`, `valid_until`, `lot_name`, `currency`, discards all six, constructs no `ParkingPass`, writes nothing to storage. Also calls `storage.get_next_id("venue")` instead of the dedicated `"parking_pass"` counter — so it both stores nothing and increments the wrong counter.
- **What already exists to support it:** The `ParkingPass` struct (`types/core_types/venue.rs:483`) and the `parking_passes` / `user_parking_passes` / `venue_parking_passes` mappings (`storage/contract_storage.rs:135,140,143`) are already in place. The sibling `purchase_concession_credits` (lines directly below) is the correct reference pattern — it builds and inserts its record properly.
- **Suggested phase:** Phase 2 or 3 (wherever venue/sports feature completion is scheduled) — this should land alongside real payment wiring (Gap B) since both are in the same feature area.

**Gap B — `InkTix::purchase_concession_credits` ignores the caller's currency**
- **File/line:** `contracts/inktix/src/lib.rs:468-480` (marker at line 468)
- **What's missing:** The `#[ink(message)]` accepts `currency: CurrencyId` from the caller but hardcodes `"DOT".to_string()` downstream — the user's chosen currency is silently dropped.
- **What already exists to support it:** `CurrencyId` (`types/core_types/currency.rs:12`) has five variants but no `Display`/`to_string` impl yet — wiring this needs a small conversion function that doesn't exist. Related: `purchase_parking_pass` (`lib.rs:450`) similarly accepts `event_id` and never forwards it, unsurprising since Gap A's callee ignores everything anyway.
- **Suggested phase:** Phase 3 (the plan's context notes real payment/currency handling belongs there) — this is exactly the kind of currency-plumbing gap that phase should close.

**Gap C — Sports `sport_type` never validated (lower confidence, flagged as a question not a defect)**
- **File/line:** `contracts/inktix/src/logic/core/event_management.rs:43-56` (marker at line 43, in the `EventCategory::Sports` match arm)
- **What's missing:** `sport_type` is destructured from `EventCategory::Sports` and never used. Both `Team` and `Season` carry their own `sport_type` field, and the arm already fetches the home team, away team, and season — so a consistency check (event/team/season sports agree) looks like it was intended and never written.
- **What already exists to support it:** All three entities (`home_team`, `away_team`, `season`) are already fetched at this point in the function; only the comparison logic itself is missing. Validation semantics (reject on mismatch vs. warn) need a product decision first.
- **Suggested phase:** Phase 1 (On-Chain Module Validation) or a dedicated data-integrity pass — since this is about the trustworthiness of already-existing sports event data, it fits naturally alongside the on-chain validation work currently in progress, but confidence is lower than Gaps A/B so it may warrant a quick discussion before scheduling.

## Self-Check: PASSED

- All 11 modified files found on disk.
- Both commits (`cf2aa06f`, `2164fdec`) found in `git log --oneline --all`.
- CI run `35640297128` confirmed `conclusion: success` with `headSha` matching the pushed commit `2164fdec`.
