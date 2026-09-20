# Phase 1: On-Chain Module Validation - Research

**Researched:** 2026-09-20
**Domain:** ink! v6 / pallet-revive smart contract on-chain integration testing (`ink_e2e`)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Validation Harness**
- **D-01:** Use automated `#[ink_e2e::test]` Rust tests per module, not manual `cargo contract call` CLI invocations. The `ink_e2e = "6.0.0-beta.2"` dev-dependency already exists in `contracts/inktix/Cargo.toml` but is currently unused — this phase is what wires it up. — Reversibility: reversible — CLI-based validation could still be done ad hoc later; no code path depends on this choice, it only shapes how Phase 1's validation work gets written.
- **D-02:** Tests run against `scripts/dev-node.sh` (already exists, downloads/runs `ink-node --dev --tmp`) — no new node tooling needed.

**Recording**
- **D-03:** Record per-module pass/fail results and any bugs found directly in `docs/v2_plan.md`'s existing Phase 1d module checklist (in place), since that file is the project's living source of truth per `.claude/CLAUDE.md`. Do not create a separate VALIDATION.md. — Reversibility: reversible.

**Validation Depth (risk-tiered)**
- **D-04:** Low-risk modules (venue/event, nft_management, season_pass_management, fantasy_sports_management, analytics, artist_management, xcm_management) get happy-path validation only — one clean write/read round-trip per module is sufficient proof of on-chain correctness.
- **D-05:** The three flagged-risky modules get targeted **on-chain runtime** edge cases (not input-validation edge cases):
  - `pricing` — exercise the time-multiplier logic under `ink-node`'s manual-seal block production (blocks authored per-transaction, not on a timer) — v2_plan.md explicitly flags this as where "surprises" are expected.
  - `loyalty` / `advanced_team_loyalty` — double-`stake_on_team()` / restake on the same composite `Mapping<(Address, u32), TeamLoyaltyProfile>` key, not just a single write — this is the single riskiest storage item per CONCERNS.md.
  - `anti_scalping` — append to the `Vec<Address>` blacklist/whitelist more than once and re-check membership, to prove the on-chain Vec round-trip holds under repeated mutation, not just a single push.
- **D-06:** Explicitly do NOT add input-sanitization edge cases (empty strings, zero IDs, out-of-range values) in this phase for any module — that work is Phase 3's, and pulling it forward would duplicate effort once Phase 3 revisits these same entry points for payment enforcement.

**CI Scope**
- **D-07:** On-chain validation (the new `ink_e2e` tests) stays a manual/local step for this phase — it is NOT wired into `.github/workflows/ci.yml`. The existing CI job (off-chain `cargo test` + `cargo contract build`) is what Phase 1's success criterion #3 ("CI is green") refers to; standing up a live `ink-node` in CI is new infrastructure work that should be scoped deliberately later, not folded in here. — Reversibility: reversible — CI integration can be added in a later phase without touching the tests themselves.

### Claude's Discretion
- Exact `#[ink_e2e::test]` file/module organization (e.g., one test file per contract module vs. grouped by risk tier) is left to the planner/executor, following the existing Rust test conventions in `contracts/inktix/src/tests/`.

### Deferred Ideas (OUT OF SCOPE)
- **Wiring on-chain validation into CI** — explicitly deferred per D-07. Revisit once the `ink_e2e` tests exist and have proven stable locally; likely candidate for a later infrastructure phase or a Phase 2 follow-up once Passet Hub deployment raises the stakes on regression coverage.
- **Input-validation hardening across contract modules** — explicitly deferred per D-06 to Phase 3, which already owns this per PROJECT.md.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| VALIDATE-01 | Each contract module (venue/event, ticket purchase + user_tickets, pricing, nft_management, anti_scalping, loyalty/advanced_team_loyalty, season_pass/fantasy_sports/analytics/artist/xcm) is exercised against a live `ink-node`, in dependency order, with pass/fail recorded per module | Dependency-ordered message call chain worked out per module below (Architecture Patterns → Recommended Test Organization); exact message signatures read from `src/lib.rs` and `src/logic/**`; the "xcm" module resolved to `cross_chain_management` (Common Pitfalls: xcm_management is dead code) |
| VALIDATE-02 | All inline unit tests remain green after module validation | `cargo test --features std,sports,concert` (default features) already passes 14 tests (verified in `docs/v2_plan.md` 1c); `e2e-tests` Cargo feature gates the new tests so they don't run under plain `cargo test`, guaranteeing this stays true by construction (Architecture Patterns → Cargo feature gating) |
| VALIDATE-03 | CI is green on `redesign/polkadot-hub` | `.github/workflows/ci.yml` already triggers on push to `redesign/polkadot-hub`, pins `nightly-2026-01-15`, runs `cargo test` + `cargo contract build` with no `--features e2e-tests` flag — confirmed unaffected by this phase's new tests (Common Pitfalls: don't add `--features e2e-tests` to the CI job) |
</phase_requirements>

## Summary

This phase wires up the already-declared-but-unused `ink_e2e = "6.0.0-beta.2"` dev-dependency to close the gap between "compiles and passes off-chain unit tests" and "actually works on-chain." The mechanics are well-documented and low-risk: `#[ink_e2e::test]` async tests, gated behind an `e2e-tests` Cargo feature (the convention ink! itself uses, and the flag already sits unused in `contracts/inktix/Cargo.toml`), pointed at the persistent `scripts/dev-node.sh` instance via the `CONTRACTS_NODE_URL=ws://127.0.0.1:9944` environment variable rather than letting `ink_e2e` spawn its own node per test.

The single most important verified finding — not previously documented anywhere in this repo — is that **ink! v6's message dispatch reverts the entire extrinsic whenever an `#[ink(message)]` returns `Result::Err(_)`, regardless of whether the error type derives `#[ink::error]`.** This is read directly from ink!'s own codegen source (`crates/ink/codegen/src/generator/dispatch.rs`), not inferred. Since every InkTix message returns `Result<T, String>`, any dependency-chain misstep in a test (e.g. staking before registering the team) does not surface as a graceful `Ok(Err("Team not found"))` — it reverts the whole call, and the `ink_e2e::Error::CallExtrinsic(...)` that comes back does not cleanly expose the String message. Tests must therefore be written in strict, verified dependency order (this document works out that order per module, from direct reads of `src/lib.rs` and `src/logic/**`), and happy-path assertions should generally use `.submit().await.expect(...)` rather than defensive error matching, since D-06 explicitly excludes negative-path testing from this phase anyway.

Three further concrete findings shape the plan directly: (1) `anti_scalping`'s blacklist/whitelist are fields *inside* the whole-object `AntiScalpingConfig` that `configure_anti_scalping` overwrites wholesale — there is no dedicated "append" message, so D-05's "append twice" requirement means calling `configure_anti_scalping` twice with a config whose Vec has grown, not calling an append method that doesn't exist; (2) the module CONTEXT.md and `docs/v2_plan.md` both list "xcm" as a validation target, but `src/logic/core/xcm_management.rs` is declared, compiled, and completely dead — no `#[ink(message)]` anywhere calls into it; the actual on-chain-reachable "xcm" surface is `src/logic/sports/cross_chain_management.rs`'s `create_cross_chain_event` / `request_cross_chain_ticket_purchase` messages; (3) on this development machine, `scripts/dev-node.sh`'s hardcoded `BIN_PATH` does not match where the already-downloaded `ink-node` binary actually landed after extraction (`.tooling/ink-node/ink-node-mac/ink-node`, not `.tooling/ink-node/ink-node`), which will trigger a needless ~300MB re-download the first time the script runs unless fixed or worked around.

**Primary recommendation:** Organize one `#[cfg(all(test, feature = "e2e-tests"))] mod e2e_tests` block per contract module inside `src/lib.rs` (following the file's existing `#[cfg(test)] mod tests { ... }` convention at the bottom of the file), run them with `CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo test --features e2e-tests -- --test-threads=1` against a `scripts/dev-node.sh` instance kept running for the whole session, and record pass/fail directly into `docs/v2_plan.md`'s Phase 1d checklist per D-03. Fix (or work around) the `dev-node.sh` binary-path mismatch before relying on it in Wave 0.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| On-chain message dispatch (venue/event/ticket/pricing/nft/anti-scalping/loyalty/season-pass/fantasy/analytics/artist/cross-chain) | API / Backend (contract logic) | Database / Storage (ink! `Mapping` state) | All validated behavior lives in `contracts/inktix/src/logic/**` and is invoked through `contracts/inktix/src/lib.rs`'s single `#[ink(message)]` surface — no frontend or off-chain service participates in this phase |
| Composite-key storage round-trip (`team_loyalty_profiles`) | Database / Storage | — | The risk is specifically about how `Mapping<(Address, u32), TeamLoyaltyProfile>` serializes/deserializes on a real chain's storage trie, not about the logic that reads/writes it |
| Block-timing-dependent pricing (`time_multiplier`) | API / Backend | — | Depends on `ink::env::block_timestamp()`, a runtime-provided value from the chain's timestamp pallet — the risk is chain behavior (manual-seal timestamp semantics), not application logic |
| Test harness / CI | Tooling (not a standard tier — local dev + CI runner) | — | `ink_e2e` tests run as a Rust test binary against a locally spawned `ink-node`; no browser, frontend server, or CDN involved anywhere in this phase |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `ink_e2e` | 6.0.0-beta.2 (pinned, already in `Cargo.toml` dev-dependencies) [VERIFIED: contracts/inktix/Cargo.toml:12] | On-chain integration test framework for ink! contracts | Official ink! testing framework; no alternative exists for pallet-revive contracts |
| `ink-node` | v0.47.0 (already installed via `scripts/dev-node.sh`, binary present in `.tooling/ink-node/`) [VERIFIED: scripts/dev-node.sh:12, local filesystem `.tooling/ink-node/ink-node-mac/ink-node` confirmed present and executable] | Local dev chain with `pallet-revive` support | Replaces `substrate-contracts-node`/pallet-contracts Docker image, which cannot host pallet-revive contracts — already the project's established choice (`docs/v2_plan.md` Phase 1a) |
| `cargo-contract` | 6.0.0-beta.2 (pinned in CI, confirmed installed locally: `cargo-contract-contract 6.0.0-beta.2-unknown-aarch64-apple-darwin`) [VERIFIED: local `cargo contract --version` output] | Contract build/test CLI wrapper | Already the project's pinned build tool; also usable as `cargo contract test --features e2e-tests` |

No new packages are introduced by this phase — `ink_e2e` is already declared as a dev-dependency and simply needs to be exercised. **No Package Legitimacy Audit is required** (see below).

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `e2e-tests` Cargo feature | already declared, empty (`e2e-tests = []`) [VERIFIED: contracts/inktix/Cargo.toml:22] | Convention flag that gates e2e test compilation out of default `cargo test` runs | Wrap every new `mod e2e_tests { ... }` block in `#[cfg(all(test, feature = "e2e-tests"))]` — this is the exact pattern ink!'s own example contracts use (see Code Examples) and is what keeps VALIDATE-02/VALIDATE-03 true by construction |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `#[ink_e2e::test]` full-node backend (default) | `#[ink_e2e::test(backend(runtime_only))]` / `drink`-style sandboxed backend | Sandboxed backends skip a real node entirely (faster, no manual-seal quirks) but D-01/D-02 explicitly lock in the full-node approach against `scripts/dev-node.sh`, precisely because the manual-seal timing risk (D-05) can only be observed against a real node — not applicable here |
| Manual `cargo contract call` CLI invocations | `#[ink_e2e::test]` (chosen, D-01) | CLI-based validation is faster to hack together one-off but isn't reusable; `docs/v2_plan.md` Phase 2 explicitly plans to "re-run a representative subset of Phase 1d's module validation" — only automated tests are re-runnable |

**Installation:** None required — `ink_e2e` is already a dev-dependency; no `cargo add` or `npm install` step needed.

**Version verification:** `ink = "6.0.0-beta.1"`, `ink_e2e = "6.0.0-beta.2"`, `cargo-contract 6.0.0-beta.2`, and `rust-toolchain.toml`'s `nightly-2026-01-15` were all read directly from `contracts/inktix/Cargo.toml` and `contracts/inktix/rust-toolchain.toml` this session [VERIFIED: contracts/inktix/Cargo.toml, contracts/inktix/rust-toolchain.toml]. Per `CONCERNS.md` and `.claude/CLAUDE.md`, these are hard pins — **do not bump any of these versions during this phase**; a version bump requires a full round-trip re-validation, which is out of this phase's scope.

## Package Legitimacy Audit

**Not applicable.** This phase installs no new external packages — `ink_e2e` is an existing, already-vetted dev-dependency (present in `Cargo.toml` since the v1c mechanical port), and the only "new" artifact is `ink-node`, which is already the project's established local dev-chain tool (`docs/v2_plan.md` Phase 1a) fetched by the existing `scripts/dev-node.sh`. No `npm install`/`pip install`/`cargo add` of a new crate occurs in this phase.

## Architecture Patterns

### System Architecture Diagram

```
 ┌─────────────────────────┐        CONTRACTS_NODE_URL=ws://127.0.0.1:9944
 │  cargo test --features   │ ─────────────────────────────────────────────┐
 │  e2e-tests                │                                              │
 │  (contracts/inktix/       │                                              ▼
 │   src/lib.rs e2e_tests    │                                   ┌──────────────────────┐
 │   modules, one per       │   1. client.instantiate(&alice)    │  ink-node --dev --tmp │
 │   contract module)       │──────────────────────────────────▶│  (scripts/dev-node.sh)│
 │                           │   2. client.call(&alice|&bob, ..) │  pallet-revive        │
 │                           │◀──────────────────────────────────│  manual-seal blocks   │
 └─────────────────────────┘   3. .submit()/.dry_run() results  └──────────────────────┘
              │                                                              │
              │ 4. assert on return_value() / storage reads                 │
              ▼                                                              ▼
   docs/v2_plan.md Phase 1d checklist                          InkTixStorage (Mapping<K,V>
   (pass/fail recorded, D-03)                                   fields — the thing being
                                                                  round-trip-verified)
```

Data flow for the primary validation path (any module): the test process instantiates the contract once per test (owner = `ink_e2e::alice()`), issues setup calls as the owner where `ensure_owner()` gates them, issues the module-under-test call as a non-owner account where applicable (`ink_e2e::bob()`), and reads back state through a getter message to confirm the round-trip. Every step happens over the same persistent `ink-node` process; no state is shared between separate `#[ink_e2e::test]` functions (each does a fresh `instantiate`).

### Recommended Test Organization

Follow the existing convention in `src/lib.rs` (`#[cfg(test)] mod tests { ... }` at the bottom of the `impl InkTix` block, using inline `#[ink::test]`s) by adding a sibling `#[cfg(all(test, feature = "e2e-tests"))] mod e2e_tests { ... }` block in the same location, OR — since D-05's three risk-tiered modules need materially more test code than the D-04 happy-path modules — split by file under `contracts/inktix/src/tests/` (`core_tests.rs`, `sports_tests.rs`, `concert_tests.rs` already exist as thin helper-struct files per the existing pattern) if the executor prefers separation. **This exact choice is explicitly left to the planner/executor per CONTEXT.md's Claude's Discretion.** Either way, every e2e test module needs:

```rust
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    // one #[ink_e2e::test] fn per module-under-test
}
```
[VERIFIED: github.com/use-ink/ink integration-tests/public/flipper/lib.rs and integration-tests/public/fallible-setter/lib.rs — both read this session via `gh api repos/use-ink/ink/contents/...`, confirming `use ink_e2e::ContractsBackend;`, `type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;`, and the bare (non-generic) `Client` parameter are the pattern used in ink!'s own shipped examples for the default full-node backend, not merely the tutorial-doc simplification]

### Dependency-Ordered Module Validation Chain

This is the concrete call sequence per module, worked out from direct reads of `src/lib.rs`'s `ensure_owner()` gating and each `src/logic/**` file's internal `.ok_or(...)` dependency checks this session:

1. **`venue_management` / `event_management`** (D-04, happy-path only)
   - `register_venue(...)` as **owner** (`ensure_owner` gated) [VERIFIED: contracts/inktix/src/lib.rs:96, "self.ensure_owner()?;" inside `register_venue`] → returns `venue_id`
   - `create_event(name, venue_id, date, capacity, base_price, EventCategory::Generic)` as **owner** [VERIFIED: contracts/inktix/src/lib.rs:129] — use `EventCategory::Generic` to avoid needing a team/season/artist dependency for this module in isolation
   - `get_venue(venue_id)`, `get_event(event_id)` (read-only, `.dry_run()`) to confirm round-trip

2. **`ticket_management` (`purchase_ticket`) + `user_tickets`** (D-04, happy-path; first real `Mapping<Address, _>` write/read)
   - Depends on step 1's event existing. `purchase_ticket(event_id, seat, CurrencyId::DOT)` as **bob** (not owner-gated) [VERIFIED: contracts/inktix/src/lib.rs:159-165, no `ensure_owner()` call in `purchase_ticket`] → returns `ticket_id`
   - `get_user_tickets(bob_address)` (`.dry_run()`) — confirms `user_tickets: Mapping<Address, Vec<u64>>` round-trip

3. **`pricing`** (D-05, targeted edge case — manual-seal timing)
   - Requires an event with `dynamic_pricing_enabled: true` (set by default in `create_event`, [VERIFIED: contracts/inktix/src/logic/core/event_management.rs "dynamic_pricing_enabled: true," in the `Event` struct literal])
   - Call `get_price_quote(event_id, seat, false)` (read-only) with the event's `date` field set relative to the chain's **actual current `block_timestamp()`** at test time (not a hardcoded constant) — see Common Pitfalls for why
   - The `stake_on_team`/loyalty and `anti_scalping` risk items below are independent of this step and don't gate it

4. **`nft_management`** (D-04, happy-path — "zero Balance/u128 arithmetic ... should be the cleanest port" per `docs/v2_plan.md`)
   - Depends on step 2's ticket. `mint_ticket_nft(ticket_id)` as **bob** (must be the ticket owner) [VERIFIED: contracts/inktix/src/logic/core/nft_management.rs, "if ticket.owner != caller { return Err(...) }"] → returns `token_id`
   - `verify_ticket_nft(token_id)` (read-only) to confirm round-trip

5. **`anti_scalping`** (D-05, targeted edge case — repeated `Vec<Address>` mutation)
   - **No dedicated append message exists.** `configure_anti_scalping(event_id, config: AntiScalpingConfig)` as **owner** [VERIFIED: contracts/inktix/src/lib.rs:217] fully **overwrites** the stored config each call [VERIFIED: contracts/inktix/src/logic/core/anti_scalping.rs, `storage.anti_scalping_configs.insert(event_id, &config)`]. To satisfy D-05's "append to the Vec more than once" requirement: call `configure_anti_scalping` once with `blacklisted_addresses: vec![addr_a]`, then call it again with `blacklisted_addresses: vec![addr_a, addr_b]` (constructed by reading back `get_anti_scalping_config` and pushing), then confirm via a third `get_anti_scalping_config` read that both addresses persisted. This proves the on-chain `Vec<Address>` round-trip under repeated mutation, per D-05's actual intent — not a literal "append" call, because none exists.

6. **`loyalty` / `advanced_team_loyalty`** (D-05, the riskiest item — composite key)
   - `register_team(name, city, SportType::...)` as **owner** (`ensure_owner` gated) [VERIFIED: contracts/inktix/src/lib.rs:311] → returns `team_id`
   - `create_team_loyalty_profile(team_id)` as **bob** (not owner-gated) [VERIFIED: contracts/inktix/src/logic/sports/advanced_team_loyalty.rs, no owner check] — this step is required; `stake_on_team` silently no-ops the profile-update branch if the profile doesn't already exist (`if let Some(mut profile) = storage.team_loyalty_profiles.get((user, team_id))`) [VERIFIED: contracts/inktix/src/logic/sports/advanced_team_loyalty.rs `stake_on_team` body]
   - `stake_on_team(team_id, amount_1, CurrencyId::DOT)` as **bob**, then **again** `stake_on_team(team_id, amount_2, CurrencyId::DOT)` as **bob** — this is the literal double-stake D-05 calls for
   - `get_team_loyalty_profile(bob_address, team_id)` (read-only) — assert `staked_amount == amount_1 + amount_2` and `loyalty_points` reflects both increments. This is the composite key `Mapping<(Address, u32), TeamLoyaltyProfile>` [VERIFIED: contracts/inktix/src/storage/contract_storage.rs:119, `pub team_loyalty_profiles: Mapping<(Address, u32), TeamLoyaltyProfile>,`] round-trip confirmation the phase's success criteria explicitly require.

7. **`season_pass_management`, `fantasy_sports_management`, `analytics`, `artist_management`, `xcm` (= `cross_chain_management`, see Common Pitfalls)** (D-04, happy-path only)
   - `season_pass_management`: `create_season_pass_package(name, team_id, season_id, price, currency, staking_requirement, benefits)` as owner [VERIFIED: contracts/inktix/src/lib.rs:344] — **`benefits` must be a non-empty `Vec`**; the logic does `benefits[0].clone()` with no bounds check [VERIFIED: contracts/inktix/src/logic/sports/season_pass_management.rs, `benefits: benefits[0].clone(),`] and an empty Vec would panic (trap), not return `Err` — see Common Pitfalls. Then `purchase_season_pass(package_id)` as bob.
   - `fantasy_sports_management`: `create_fantasy_league(...)` as bob (not owner-gated — confirmed no `ensure_owner()` at that call site) → `join_fantasy_league(league_id)` as another account → `create_fantasy_team(league_id, name)`.
   - `analytics`: `generate_analytics_report(report_type, start_date, end_date)` as **owner** [VERIFIED: contracts/inktix/src/lib.rs:524] → `get_analytics_report(report_id)` read-only.
   - `artist_management`: `register_artist(name)` as any caller (not owner-gated) → `verify_artist(artist_id)` as **owner** [VERIFIED: contracts/inktix/src/lib.rs:558] → `get_artist(artist_id)` read-only.
   - `xcm` (`cross_chain_management`): `create_cross_chain_event(event_id, target_chain)` as **owner** [VERIFIED: contracts/inktix/src/lib.rs:492], depends on step 1's event → `request_cross_chain_ticket_purchase(...)` as bob → `get_chain_connectivity(chain)` read-only. **Do not attempt to test `src/logic/core/xcm_management.rs`'s `send_xcm_message`/`get_xcm_analytics` — they have no message entry point and cannot be called on-chain at all** (see Common Pitfalls).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Node process lifecycle for tests | A custom node-spawn/teardown harness in test code | `CONTRACTS_NODE_URL=ws://127.0.0.1:9944` pointing at the already-running `scripts/dev-node.sh` process (D-02) | `ink_e2e` natively supports pointing at an externally-managed node via this env var [VERIFIED: github.com/use-ink/ink crates/e2e/macro/src/config.rs, `std::env::var("CONTRACTS_NODE_URL")`] — no custom process management needed |
| Nested-`Result` unwrapping for each call | Bespoke helper functions per test to unwrap `LangError` | The `ink_e2e` `CallBuilder`/`CallResult` API (`.submit().await?` then `.return_value()`) | This is exactly what the framework's `CallResult`/`InstantiationResult` types are for — reinventing it duplicates framework code for no benefit |

**Key insight:** There is very little to hand-roll here — this phase is almost entirely "use the framework as intended." The one place a naive implementation goes wrong is assuming Result-returning messages behave like they did off-chain (graceful `Ok(Err(...))`) — see the dispatch-revert finding in Common Pitfalls, which is the one piece of non-obvious framework behavior that must inform how every test in this phase is written.

## Common Pitfalls

### Pitfall 1: `Result::Err` from any `#[ink(message)]` reverts the whole extrinsic — not a graceful `Ok(Err(...))`
**What goes wrong:** A test author assumes (reasonably, based on how the off-chain unit tests behave, and how pallet-contracts/v5 semantics worked) that calling a message that internally hits an `Err(...)` path (e.g. calling `stake_on_team` before `create_team_loyalty_profile` on a team that doesn't exist) will still succeed at the extrinsic level and return `Ok(Result::Err("Team not found"))`, inspectable via `.return_value()`.
**Why it happens:** ink! v6's dispatch codegen checks `is_result_type!(#message_output) && is_result_err!(result)`; if both are true, `ReturnFlags::REVERT` is set and the contract's storage mutation is **not** pushed back — the extrinsic fails as a whole [VERIFIED: github.com/use-ink/ink crates/ink/codegen/src/generator/dispatch.rs, lines ~1159-1174, read directly via `gh api` this session: `let is_reverted = ::ink::is_result_type!(#message_output) && ::ink::is_result_err!(result); ... let mut flag = ::ink::env::ReturnFlags::REVERT; if !is_reverted { flag = ::ink::env::ReturnFlags::empty(); push_contract(contract, #mutates_storage); }`]. This applies to **every** InkTix message, since all of them return `Result<T, String>` — the revert is not conditional on `#[ink::error]` (confirmed by reading `crates/ink/src/result_info.rs`'s `is_result_type!`/`is_result_err!` macros, which pattern-match on any `Result<T, E>`, not a marker trait).
**How to avoid:** Since D-06 excludes negative-path testing from this phase, this mostly matters as a **debugging aid, not a test-design requirement** — build the dependency chains exactly as documented above (Architecture Patterns → Dependency-Ordered Module Validation Chain) so no call in the happy path ever hits an `Err` branch. If a `.submit().await` call unexpectedly returns `Err(ink_e2e::Error::CallExtrinsic(...))` during test-writing, the cause is almost always a missing setup step earlier in the chain, not a framework bug.
**Warning signs:** `.submit().await` returning `Err(ink_e2e::Error::CallExtrinsic(_, _))` where the test expected success; state that "should" have persisted from a prior call in the same test is missing on the next read.

### Pitfall 2: `xcm_management.rs` is dead code — the "xcm" validation item is actually `cross_chain_management.rs`
**What goes wrong:** Writing an e2e test that tries to call `send_xcm_message`, `get_xcm_message`, `get_xcm_analytics`, or `update_xcm_chain_connectivity` — these exist in `src/logic/core/xcm_management.rs` and are `pub`, but **no `#[ink(message)]` in `src/lib.rs` calls into them**, so there is no way to invoke them on-chain at all.
**Why it happens:** `src/logic/core/mod.rs` declares `pub mod xcm_management;` but `src/lib.rs`'s `use crate::logic::core::{ ... }` import list does not include `xcm_management` [VERIFIED: contracts/inktix/src/lib.rs:40-42, `use crate::logic::core::{anti_scalping, currency_management, event_management, nft_management, pricing, ticket_management, venue_management};` — `xcm_management` is absent from this list], and a repo-wide grep for `xcm_management` this session returned only its own `mod.rs` declaration [VERIFIED: `grep -rn "xcm_management" contracts/inktix/src/` returned exactly one hit: `src/logic/core/mod.rs:13: pub mod xcm_management;`]. The actual on-chain-reachable cross-chain surface is `src/logic/sports/cross_chain_management.rs`, wired to `create_cross_chain_event`, `request_cross_chain_ticket_purchase`, and `get_chain_connectivity` in `src/lib.rs`.
**How to avoid:** Validate `cross_chain_management`'s three messages for the "xcm" checklist item in `docs/v2_plan.md`. Do not attempt to write a test against `xcm_management.rs` — there is no entry point to call.
**Warning signs:** Searching `src/lib.rs` for a message name that would call into `xcm_management` and finding none.

### Pitfall 3: `scripts/dev-node.sh`'s `BIN_PATH` does not match where the already-downloaded binary actually is (on this machine)
**What goes wrong:** Running `scripts/dev-node.sh` as-is may trigger an unnecessary ~300MB re-download of `ink-node` even though it's already present locally.
**Why it happens:** The script does `tar xzf ... -C "$NODE_DIR" --strip-components=1` and then expects the binary at `"$NODE_DIR/ink-node"` [VERIFIED: scripts/dev-node.sh, `NODE_DIR="$TOOLING_DIR/ink-node"` and `BIN_PATH="$NODE_DIR/ink-node"`, lines defining `install_ink_node()`]. On this development machine, the binary that is actually present sits one level deeper: `.tooling/ink-node/ink-node-mac/ink-node` [VERIFIED: local filesystem, `ls -la .tooling/ink-node/` shows only a subdirectory `ink-node-mac/`, and `ls -la .tooling/ink-node/ink-node-mac/` shows the executable `ink-node` (288MB) and `eth-rpc` binaries there] — meaning `[ ! -x "$BIN_PATH" ]` evaluates true and `install_ink_node()` fires again.
**How to avoid:** Before relying on `scripts/dev-node.sh` in Wave 0, either (a) verify a fresh run resolves `BIN_PATH` correctly and the mismatch was a one-off leftover from an earlier script version / manual extraction, or (b) symlink/move `.tooling/ink-node/ink-node-mac/ink-node` → `.tooling/ink-node/ink-node` (and `eth-rpc` alongside it) as a workaround, or (c) fix the script's `--strip-components` count for the mac-universal asset. This was not root-caused to a specific tar layout bug this session (partial-download tar listing was inconclusive) — treat as a Wave 0 verification step, not a pre-diagnosed one-line fix.
**Warning signs:** `scripts/dev-node.sh` printing "ink-node not found — downloading..." despite `.tooling/ink-node` already containing multiple hundred MB of files.

### Pitfall 4: `season_pass_management::create_season_pass_package` panics (traps), doesn't `Err`, on an empty `benefits` Vec
**What goes wrong:** Calling `create_season_pass_package` with an empty `benefits: Vec<SeasonPassBenefits>` argument causes a Rust index-out-of-bounds panic inside the contract, not a `Result::Err`.
**Why it happens:** The implementation does `benefits: benefits[0].clone()` with no length check [VERIFIED: contracts/inktix/src/logic/sports/season_pass_management.rs, `benefits: benefits[0].clone(),` inside `create_season_pass_package`]. A panic traps the whole call (a harder failure mode than the `Result::Err` revert described in Pitfall 1 — `ink_e2e` will surface it as a different error variant, and the debug message may be less informative).
**How to avoid:** Always pass a non-empty `benefits` Vec (e.g. one element) when exercising this module in the happy-path test — this is a runtime-behavior finding worth noting for Phase 3 (which owns input-validation hardening per D-06) but must simply be avoided, not tested, in this phase.
**Warning signs:** A cryptic panic/trap message instead of a clean `Result::Err` when testing `season_pass_management`.

### Pitfall 5: Manual-seal block timestamps are real wall-clock time, but the exact advancement behavior between rapidly-produced blocks is unverified
**What goes wrong:** Assuming `time_multiplier`'s day-bucket logic (`>30 days out = 9000`, `1-7 days = 11500`, `<24h = 13000`, keyed off `ink::env::block_timestamp()` minus `event.date`) can be exercised deterministically by controlling block count, the way `#[ink::test]`'s off-chain environment allows manual timestamp injection.
**Why it happens:** `ink-node`'s manual-seal consensus authors a block per submitted transaction rather than on a fixed timer [CITED: web search of ink-node README/docs, "the consensus algorithm has been switched to manual-seal, whereby blocks are authored immediately at every transaction"], but this research could not find authoritative documentation on whether the timestamp inherent set on each manually-sealed block reflects real wall-clock time (the typical Substrate manual-seal default) or some other value. `docs/v2_plan.md` explicitly flags this as the item where "surprises" are expected.
**How to avoid:** Don't try to test specific day-boundary transitions (e.g. exactly 7 days vs 6 days, 23:59 vs 00:01) — that's input-validation-adjacent precision testing D-06 excludes anyway. Instead, set `event.date` to a fixed real-world offset from `chrono`/wall-clock "now" at test-write time (e.g., "3 days from whenever this test actually runs" computed inside the test itself from the chain's own current block timestamp, not a hardcoded constant), call `get_price_quote`, and assert the returned `time_multiplier` matches one of the five documented buckets — proving the on-chain codepath executes and returns *a* sane, non-panicking value under real (not off-chain-mocked) timing, which is what D-05 actually asks for.
**Warning signs:** A `time_multiplier` assertion that passes when run immediately after writing the test but silently starts failing hours/days later — a sign the test hardcoded an absolute timestamp instead of computing an offset from the chain's actual current time.

## Code Examples

### Basic `#[ink_e2e::test]` skeleton (instantiate + call + read-back)
```rust
// Source: github.com/use-ink/ink integration-tests/public/flipper/lib.rs (ink! v6 official example,
// read via `gh api repos/use-ink/ink/contents/...` this session)
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn it_works(mut client: Client) -> E2EResult<()> {
        // given
        let mut constructor = FlipperRef::new(false);
        let contract = client
            .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<Flipper>();

        let get = call_builder.get();
        let get_res = client.call(&ink_e2e::bob(), &get).submit().await?;
        assert!(!get_res.return_value());

        // when
        let flip = call_builder.flip();
        let _flip_res = client
            .call(&ink_e2e::bob(), &flip)
            .submit()
            .await
            .expect("flip failed");

        // then
        let get = call_builder.get();
        let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(get_res.return_value());

        Ok(())
    }
}
```

### Result-returning message pattern (for InkTix's `Result<T, String>` messages)
```rust
// Source: github.com/use-ink/ink integration-tests/public/fallible-setter/lib.rs (ink! v6 official
// example — the closest shipped example to InkTix's Result<T, E>-returning message shape; note this
// example's Error derives #[ink::error], which InkTix's plain String does not — the revert-on-Err
// behavior documented in Pitfall 1 applies identically either way, per the dispatch.rs source read)
#[ink_e2e::test]
async fn it_works(mut client: Client) -> E2EResult<()> {
    let mut constructor = FallibleSetterRef::new(0);
    let contract = client
        .instantiate("fallible_setter", &ink_e2e::bob(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<FallibleSetter>();

    let set = call_builder.try_set(1);
    let set_res = client
        .call(&ink_e2e::bob(), &set)
        .submit()
        .await
        .expect("set failed");
    assert!(set_res.return_value().is_ok()); // happy path: return_value() gives the inner Result<T, E>

    Ok(())
}
```

### Running the tests
```bash
# Terminal 1: keep a persistent node running for the whole test session (D-02)
./scripts/dev-node.sh

# Terminal 2: point ink_e2e at that running node instead of spawning its own
CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo test --features e2e-tests
# or, equivalently, via cargo-contract's wrapper:
CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo contract test --features e2e-tests
```
[VERIFIED: github.com/use-ink/ink `.github/workflows/ci.yml`, `CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo contract test --features e2e-tests --manifest-path ...` — read directly via `gh api` this session; `CONTRACTS_NODE_URL` behavior confirmed via `crates/e2e/macro/src/config.rs`'s `Node::url()` reading `std::env::var("CONTRACTS_NODE_URL")`]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|---------------|--------|
| v5 / pallet-contracts: `ink_e2e::test]` generic `<Client: E2EBackend>` signature shown in some doc pages | v6 official example contracts use the concrete (non-generic) `Client` parameter for the default full-node backend | ink! v6 (current) | Use the concrete `Client` form shown in the Code Examples above — it's what ink!'s own shipped examples use, not the more generic tutorial-doc form (both compile, but the concrete form matches this repo's existing off-chain test style more closely) |

**Deprecated/outdated:** Nothing else specific to this phase — the project is already on ink! v6 / pallet-revive throughout; there is no v5-era code path being touched.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `ink-node`'s manual-seal block production sets each block's timestamp inherent to real wall-clock time (the typical Substrate manual-seal default), rather than some fixed/incrementing synthetic value | Common Pitfalls → Pitfall 5 | If wrong, `time_multiplier` assertions built around "now + N days" would need to be redesigned around whatever the actual timestamp source is; low risk since D-05's actual requirement is just "returns a sane non-panicking value," which holds either way |
| A2 | The `scripts/dev-node.sh` binary-path mismatch observed on this development machine (Pitfall 3) is either a one-off leftover from a prior extraction or a real `--strip-components` bug — this research did not download the full ink-node release tarball to confirm which | Common Pitfalls → Pitfall 3 | If it's a real script bug (not a leftover), every future fresh clone/environment will hit the same needless re-download; low-medium risk — worst case is a slow Wave 0 setup step, not a correctness issue |

## Open Questions

1. **Does `.dry_run()` vs `.submit()` matter for the read-only getter calls used to confirm round-trips (e.g. `get_venue`, `get_team_loyalty_profile`)?**
   - What we know: ink!'s own examples use `.dry_run()` for pure reads and `.submit()` for state-mutating calls (see Code Examples).
   - What's unclear: Whether `.dry_run()` reads reflect state from the immediately-preceding `.submit()`'d write in the same test (i.e., does dry-run see uncommitted-vs-committed state correctly) — this matters directly for the double-stake composite-key assertion in step 6 of the Dependency-Ordered Module Validation Chain.
   - Recommendation: Use `.submit()` for the final confirming read in the loyalty/anti-scalping tests (guarantees querying committed on-chain state) even though it's slightly slower; reserve `.dry_run()` for genuinely disposable intermediate reads.

2. **Exact keypair-to-`Address` (H160) conversion helper for asserting caller identity in tests** (e.g. confirming `get_owner()` equals the deploying `ink_e2e::alice()` account's `Address`)
   - What we know: `ink_e2e::alice()`/`ink_e2e::bob()` return subxt signer keypairs; InkTix's contract-side identity type is `ink::primitives::Address` (H160), consistent with the rest of the v2 migration.
   - What's unclear: The exact helper method/conversion path from an `ink_e2e` keypair to its `Address` was not pinned down this session (not needed for the happy-path validation chain above, since none of the D-04/D-05 assertions require comparing a caller's own address against a returned value — `get_owner()` isn't part of any module's validation target here).
   - Recommendation: Not blocking for this phase's scope; if a future test needs it, check `ink_e2e`'s `AccountKeyring`/`Keypair` API docs or the `account_id()`/similar method at execution time via Context7 or the same `gh api repos/use-ink/ink` source-read technique used throughout this research.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `nightly-2026-01-15` Rust toolchain | All contract builds/tests | ✓ [VERIFIED: `rustup show` output this session] | nightly-2026-01-15-aarch64-apple-darwin | — (hard pin, no fallback per CONCERNS.md) |
| `cargo-contract` 6.0.0-beta.2 | `cargo contract build`/`test` | ✓ [VERIFIED: `cargo contract --version` this session] | 6.0.0-beta.2-unknown-aarch64-apple-darwin | — |
| `ink-node` v0.47.0 binary | `scripts/dev-node.sh` | ✓ present locally, but at a path `scripts/dev-node.sh` doesn't currently resolve correctly (Pitfall 3) | v0.47.0 (per script's pinned `INK_NODE_VERSION`) | Manual path fix/symlink, or accept a one-time re-download |
| Port 9944 (local ink-node RPC) | `CONTRACTS_NODE_URL` target | ✓ free at time of research (no process currently bound) [VERIFIED: `lsof -i :9944` returned nothing this session] | — | — |

**Missing dependencies with no fallback:** None — everything this phase needs is already present on the development machine, modulo the Pitfall 3 path-resolution issue (workaround available).

**Missing dependencies with fallback:** `ink-node` binary path mismatch — workaround is a manual symlink/move, or a tolerable one-time re-download.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `ink_e2e` 6.0.0-beta.2 (async Rust test framework, `#[ink_e2e::test]` macro) |
| Config file | None dedicated — configuration is the `e2e-tests` Cargo feature flag (`contracts/inktix/Cargo.toml`) plus the `CONTRACTS_NODE_URL` environment variable at invocation time |
| Quick run command | `CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo test --features e2e-tests <module_name>::e2e_tests -- --test-threads=1` (single module, against a running `dev-node.sh`) |
| Full suite command | `CONTRACTS_NODE_URL=ws://127.0.0.1:9944 cargo test --features e2e-tests -- --test-threads=1` (all e2e tests; `--test-threads=1` recommended since tests share one chain's nonce/state sequencing even though each instantiates its own contract) |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|--------------------|-------------|
| VALIDATE-01 (venue/event) | `register_venue` + `create_event` round-trip | e2e (on-chain) | `cargo test --features e2e-tests venue_event_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (ticket) | `purchase_ticket` + `user_tickets` round-trip | e2e (on-chain) | `cargo test --features e2e-tests ticket_purchase_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (pricing) | `time_multiplier` under manual-seal | e2e (on-chain) | `cargo test --features e2e-tests pricing_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (nft) | `mint_ticket_nft` round-trip | e2e (on-chain) | `cargo test --features e2e-tests nft_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (anti-scalping) | Repeated `configure_anti_scalping` Vec mutation | e2e (on-chain) | `cargo test --features e2e-tests anti_scalping_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (loyalty) | Double `stake_on_team` composite-key round-trip | e2e (on-chain) | `cargo test --features e2e-tests loyalty_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-01 (season-pass/fantasy/analytics/artist/xcm) | One happy-path round-trip each | e2e (on-chain) | `cargo test --features e2e-tests <module>_e2e_test -x` | ❌ Wave 0 |
| VALIDATE-02 | 14 existing inline unit tests stay green | unit (off-chain) | `cargo test --features std,sports,concert` | ✅ already exists |
| VALIDATE-03 | CI green on `redesign/polkadot-hub` | CI | GitHub Actions `contracts` job (unchanged) | ✅ already exists |

### Sampling Rate
- **Per task commit:** Run the single module's e2e test just written, against the running `dev-node.sh` session.
- **Per wave merge:** Run the full `cargo test --features e2e-tests` suite plus `cargo test --features std,sports,concert` (off-chain) to confirm VALIDATE-02 hasn't regressed.
- **Phase gate:** Full e2e suite green + off-chain suite green + `cargo contract build` succeeds + `docs/v2_plan.md`'s Phase 1d checklist fully filled in (D-03) before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] All `e2e_tests` modules — none exist yet; this entire phase's test code is net-new (the `e2e-tests` Cargo feature flag and `ink_e2e` dependency exist, but zero tests currently use them)
- [ ] `scripts/dev-node.sh` binary-path verification/fix (Pitfall 3) — confirm it resolves to the already-downloaded binary before relying on it, or accept the one-time re-download

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-------------------|
| V2 Authentication | No | No wallet/user auth surface touched in this phase — tests use `ink_e2e`'s well-known dev keypairs (`alice`/`bob`), not production auth |
| V3 Session Management | No | Not applicable — contract calls, not sessions |
| V4 Access Control | Yes | Existing `ensure_owner()` guard, already implemented [VERIFIED: contracts/inktix/src/lib.rs:80-85] — this phase's tests exercise (but do not add) owner-gated vs. non-owner-gated call paths per the dependency chain above; no new access-control code is written |
| V5 Input Validation | No (explicitly deferred, D-06) | Phase 3 owns input-validation hardening; this phase deliberately does not add or test input validation |
| V6 Cryptography | No | No cryptographic code touched — `ink_e2e`'s signing is handled entirely by its own subxt-based keypair infrastructure, not custom code |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|----------------------|
| Owner-only message called by non-owner | Elevation of Privilege | Already mitigated by `ensure_owner()` [VERIFIED: contracts/inktix/src/lib.rs:80-85] — this phase validates the happy (owner-caller) path only, per D-06; negative-path (non-owner rejection) testing is explicitly out of scope here and belongs to a future hardening pass |
| Test code accidentally committing real secrets/keys | Information Disclosure | `ink_e2e`'s dev accounts (`alice`, `bob`, etc.) are well-known, publicly-documented test keypairs — never use real deploy-account keys in e2e test code |

This phase introduces no new attack surface (it adds tests, not contract logic) — the Security Domain section is included per config (`security_enforcement: true`) but there is minimal new ground to cover.

## Sources

### Primary (HIGH confidence)
- `contracts/inktix/Cargo.toml`, `contracts/inktix/rust-toolchain.toml`, `contracts/inktix/src/lib.rs`, `contracts/inktix/src/logic/**/*.rs`, `contracts/inktix/src/storage/contract_storage.rs`, `contracts/inktix/src/types/core_types/anti_scalping.rs`, `.github/workflows/ci.yml`, `scripts/dev-node.sh` — all read directly this session via the `Read`/`Bash` tools
- `github.com/use-ink/ink` — `crates/ink/codegen/src/generator/dispatch.rs`, `crates/ink/src/result_info.rs`, `crates/e2e/macro/src/config.rs`, `integration-tests/public/flipper/lib.rs`, `integration-tests/public/fallible-setter/lib.rs`, `integration-tests/public/contract-transfer/lib.rs`, `.github/workflows/ci.yml` — all fetched directly via `gh api repos/use-ink/ink/contents/...` and read in full this session; this is the actual source code of the framework being used, the strongest possible source for framework-behavior claims
- Local development environment: `rustup show`, `cargo contract --version`, `ls -la .tooling/ink-node/`, `lsof -i :9944` — all run this session

### Secondary (MEDIUM confidence)
- `use.ink/docs/v6/contract-testing/end-to-end-e2e-testing/`, `use.ink/docs/v6/getting-started/testing-your-contract/` — official ink! v6 documentation, fetched via WebFetch this session, corroborated the `#[ink_e2e::test]` pattern and `cargo contract test --features e2e-tests` command but did not fully resolve `CONTRACTS_NODE_URL` details (resolved instead via the primary GitHub source reads above)

### Tertiary (LOW confidence)
- WebSearch results on `ink-node` manual-seal timestamp behavior (Pitfall 5 / Assumption A1) — no authoritative primary source found confirming exact timestamp-inherent semantics under manual-seal; flagged as an assumption, not a verified fact

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — versions read directly from pinned config files, no new packages introduced
- Architecture: HIGH — dependency chains and message signatures read directly from source; the revert-on-Err and dead-code findings are verified from ink!'s own framework source and this repo's own code, respectively
- Pitfalls: HIGH for Pitfalls 1, 2, 4 (all verified from source); MEDIUM for Pitfall 3 (verified the symptom, not the root cause); LOW/ASSUMED for Pitfall 5 (manual-seal timestamp semantics)

**Research date:** 2026-09-20
**Valid until:** 2026-10-20 (30 days — the ink! v6 beta ecosystem is still actively evolving even though this project pins exact versions; re-verify if any toolchain version pin changes)
