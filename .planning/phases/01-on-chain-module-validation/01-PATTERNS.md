# Phase 1: On-Chain Module Validation - Pattern Map

**Mapped:** 2026-09-20
**Files analyzed:** ~8 (one net-new `e2e_tests` module/file per validation group, plus 2 possible support edits)
**Analogs found:** 8 / 8 (all analogs are within this single repo — no external project has a matching e2e harness)

## File Classification

All "files" in this phase are new `#[ink_e2e::test]` test groups. Per CONTEXT.md's "Claude's Discretion," exact
file/module split is left to planner/executor — this map assumes one `e2e_tests` sub-module per risk-tier grouping,
either inline in `src/lib.rs` (mirroring the existing `#[cfg(test)] mod tests { ... }` block) or as new files under
`contracts/inktix/src/tests/` (mirroring `core_tests.rs` / `sports_tests.rs` / `concert_tests.rs`). Both options
share the same analog and patterns below.

| New/Modified File (net-new test code) | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `e2e_tests::venue_event` (venue/event happy-path) | test | request-response (on-chain RPC round-trip) | `contracts/inktix/src/lib.rs:638-648` (`test_register_venue`) | role-match (same assertions, different backend: on-chain vs off-chain `#[ink::test]`) |
| `e2e_tests::ticket_purchase` (ticket + user_tickets happy-path) | test | request-response | `contracts/inktix/src/lib.rs:756-770` (`test_concert_purchase_limit`, `purchase_ticket` call site) | role-match |
| `e2e_tests::pricing` (manual-seal timing edge case) | test | request-response / timing-dependent | ink! official `flipper` example (read-only `.dry_run()` call) + `contracts/inktix/src/logic/core/pricing.rs` (`get_price_quote` signature) | partial (no existing timing-dependent test in-repo; framework skeleton borrowed from official example) |
| `e2e_tests::nft` (nft_management happy-path) | test | request-response | `contracts/inktix/src/lib.rs:638-648` pattern (generic write+read round-trip) | role-match |
| `e2e_tests::anti_scalping` (repeated Vec mutation edge case) | test | request-response / state-mutation | `contracts/inktix/src/lib.rs:729-752` (`test_create_concert_event`, exercises `get_anti_scalping_config`) | role-match |
| `e2e_tests::loyalty` (double `stake_on_team` composite-key edge case) | test | request-response / state-mutation | `contracts/inktix/src/lib.rs:638-648` pattern + `contracts/inktix/src/logic/sports/advanced_team_loyalty.rs` (message signatures) | role-match |
| `e2e_tests::season_pass_fantasy_analytics_artist_xcm` (5 happy-path modules) | test | request-response | `contracts/inktix/src/lib.rs:704-725` (`test_register_artist`/`test_verify_artist`, owner-gated verify pattern) | role-match |
| `contracts/inktix/src/tests/mod.rs` (possible new `e2e_tests` sub-mod declaration) | test | — | `contracts/inktix/src/tests/mod.rs` (itself, existing) | exact (same file, additive edit) |

**Note on "role":** every file in this phase is `test`; there is no controller/service/model work. Data flow is
uniformly `request-response` (ink_e2e submits an extrinsic or dry-run query and awaits a typed result), with the
`pricing` and `loyalty`/`anti_scalping` groups additionally being state-mutation/timing-sensitive per D-05.

## Pattern Assignments

### All `e2e_tests` groups — shared skeleton

**Analog:** `contracts/inktix/src/lib.rs:626-637` (existing `#[cfg(test)] mod tests { use super::*; ... }` block) for
in-repo test-module conventions, combined with ink!'s own official `ink_e2e` example (verified in RESEARCH.md Code
Examples section) for the async e2e-specific mechanics that don't exist anywhere in this repo yet.

**Module/feature-gate pattern** (new — model on existing `#[cfg(test)] mod tests` at `lib.rs:626-627`, gated
additionally behind the already-declared-but-unused `e2e-tests` Cargo feature at `contracts/inktix/Cargo.toml:21`):
```rust
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    // one #[ink_e2e::test] fn per module-under-test
}
```

**Instantiate + call + read-back pattern** (RESEARCH.md-verified from ink!'s official `flipper` example — no
InkTix-repo e2e analog exists since this is Wave 0 net-new code):
```rust
#[ink_e2e::test]
async fn it_works(mut client: Client) -> E2EResult<()> {
    let mut constructor = InkTixRef::new();
    let contract = client
        .instantiate("inktix", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let mut call_builder = contract.call_builder::<InkTix>();

    let register_venue = call_builder.register_venue(
        "Staples Center".to_string(), 20000, "Los Angeles".to_string(), VenueType::Arena,
    );
    let venue_res = client
        .call(&ink_e2e::alice(), &register_venue) // owner-gated: must be deploying account
        .submit()
        .await
        .expect("register_venue failed");
    let venue_id = venue_res.return_value().expect("register_venue returned Err");

    let get_venue = call_builder.get_venue(venue_id);
    let venue = client.call(&ink_e2e::alice(), &get_venue).dry_run().await?.return_value();
    assert!(venue.is_some());

    Ok(())
}
```

**Off-chain assertion style to mirror** (from `contracts/inktix/src/lib.rs:638-648`, `test_register_venue`):
```rust
let venue_id = contract.register_venue(
    "Staples Center".to_string(), 20000, "Los Angeles".to_string(), VenueType::Arena,
).unwrap();
assert_eq!(venue_id, 1);
let venue = contract.get_venue(venue_id).unwrap();
assert_eq!(venue.name, "Staples Center");
assert_eq!(venue.capacity, 20000);
```
Use this same "call, `.unwrap()`/assert return value, call getter, assert fields" shape inside each `#[ink_e2e::test]`
fn — only the call mechanics (`.submit().await`/`.dry_run().await` + `.return_value()`) differ from the off-chain
`#[ink::test]` style.

---

### Owner-gated vs non-owner-gated call pattern

**Analog:** `contracts/inktix/src/lib.rs:80-85` (`ensure_owner`) and its call sites (`register_venue` at line 96,
`verify_artist` gated per RESEARCH.md `lib.rs:558`).

**Pattern:** Owner-gated messages (`register_venue`, `create_event`, `register_team`, `configure_anti_scalping`,
`verify_artist`, `generate_analytics_report`, `create_cross_chain_event`) must be called with `&ink_e2e::alice()`
(the deploying account, matching `storage.owner = Self::env().caller()` set in the constructor at `lib.rs:68`).
Non-owner-gated messages (`purchase_ticket`, `mint_ticket_nft`, `create_team_loyalty_profile`, `stake_on_team`,
`create_fantasy_league`, `register_artist`) can use `&ink_e2e::bob()`. This directly determines which signer each
`client.call(&ink_e2e::<account>(), ...)` in the new tests must use — get it wrong and the call reverts per
RESEARCH.md Pitfall 1, not a graceful error.

```rust
// ensure_owner pattern — contracts/inktix/src/lib.rs:80-85
fn ensure_owner(&self) -> Result<(), String> {
    if self.env().caller() != self.storage.owner {
        return Err("Only the owner can call this function".to_string());
    }
    Ok(())
}
```

---

### `pricing` group (D-05 — manual-seal timing edge case)

**Analog:** No in-repo test touches `get_price_quote`; use the shared skeleton above plus
`contracts/inktix/src/logic/core/pricing.rs`'s `get_price_quote(event_id, seat, false)` signature (per
RESEARCH.md line 154). Compute `event.date` as an offset from the chain's actual current `block_timestamp()` at
test-run time (not a hardcoded constant) — see RESEARCH.md Pitfall 5.

---

### `anti_scalping` group (D-05 — repeated Vec mutation)

**Analog:** `contracts/inktix/src/lib.rs:729-752` (`test_create_concert_event`, which already demonstrates reading
back `get_anti_scalping_config` after `create_concert_event` auto-configures it):
```rust
// Verify anti-scalping was auto-configured — contracts/inktix/src/lib.rs:748-751
let config = contract.get_anti_scalping_config(event_id).unwrap();
assert_eq!(config.max_tickets_per_user, 4);
assert!(config.anti_bot_measures);
```
For the e2e double-mutation test: call `configure_anti_scalping(event_id, config)` twice as owner — first with
`blacklisted_addresses: vec![addr_a]`, then with the read-back config's Vec extended to `vec![addr_a, addr_b]` — then
assert both addresses persist via a third `get_anti_scalping_config` read. No dedicated append message exists
(RESEARCH.md step 5) — this mirrors the existing repo pattern of "call, then re-read the same getter to confirm."

---

### `loyalty` / `advanced_team_loyalty` group (D-05 — composite key, double stake)

**Analog:** `contracts/inktix/src/lib.rs:652-661` (`test_register_team`) for the setup-call shape, combined with
`contracts/inktix/src/logic/sports/advanced_team_loyalty.rs`'s `stake_on_team`/`create_team_loyalty_profile`/
`get_team_loyalty_profile` signatures (per RESEARCH.md step 6):
```rust
// contracts/inktix/src/lib.rs:652-661 — setup-call + getter-assert shape to mirror
let team_id = contract.register_team(
    "Lakers".to_string(), "Los Angeles".to_string(), SportType::Basketball,
).unwrap();
assert_eq!(team_id, 1);
let team = contract.get_team(team_id).unwrap();
assert_eq!(team.name, "Lakers");
```
Sequence for the e2e test: `register_team` (owner) → `create_team_loyalty_profile(team_id)` (bob) →
`stake_on_team(team_id, amount_1, CurrencyId::DOT)` (bob) → `stake_on_team(team_id, amount_2, CurrencyId::DOT)`
(bob, again) → `get_team_loyalty_profile(bob_address, team_id)` and assert
`staked_amount == amount_1 + amount_2`. Use `.submit()` (not `.dry_run()`) for the final confirming read per
RESEARCH.md Open Question 1, to guarantee committed on-chain state.

---

### `season_pass` / `fantasy_sports` / `analytics` / `artist_management` / `xcm` (`cross_chain_management`) group (D-04)

**Analog:** `contracts/inktix/src/lib.rs:704-725` (`test_register_artist` + `test_verify_artist`) for the
owner-vs-non-owner two-step verify pattern:
```rust
// contracts/inktix/src/lib.rs:706-714
let artist_id = contract.register_artist("Taylor Swift".to_string()).unwrap();
assert_eq!(artist_id, 1);
let artist = contract.get_artist(artist_id).unwrap();
assert_eq!(artist.name, "Taylor Swift");
assert!(!artist.verified);

// contracts/inktix/src/lib.rs:717-724
contract.verify_artist(artist_id).unwrap();
let artist = contract.get_artist(artist_id).unwrap();
assert!(artist.verified);
```
Apply the same two-step (mutate as correct signer, re-read, assert) shape to each of the five D-04 modules per
RESEARCH.md step 7's exact message sequences. **Important:** for `season_pass_management`, always pass a non-empty
`benefits` Vec (RESEARCH.md Pitfall 4 — empty Vec panics, doesn't `Err`). For "xcm," target
`cross_chain_management`'s `create_cross_chain_event`/`request_cross_chain_ticket_purchase`/`get_chain_connectivity`
— never `xcm_management.rs`, which has no message entry point (RESEARCH.md Pitfall 2).

---

## Shared Patterns

### Test module organization & feature gating
**Source:** `contracts/inktix/src/lib.rs:21-22` (`#[cfg(test)] mod tests;`) and `contracts/inktix/src/tests/mod.rs`
(existing `pub mod core_tests;` / feature-gated `sports_tests`/`concert_tests` declarations)
**Apply to:** Every new e2e test group — wrap in `#[cfg(all(test, feature = "e2e-tests"))]`, matching the existing
`#[cfg(test)]` / `#[cfg(feature = "sports")]` nesting convention already used throughout `lib.rs`.

### Owner/caller identity
**Source:** `contracts/inktix/src/lib.rs:80-85` (`ensure_owner`)
**Apply to:** Every e2e test group — determines whether `&ink_e2e::alice()` (owner/deployer) or `&ink_e2e::bob()`
(non-owner) is the correct signer for each call.

### Result-unwrapping at the message boundary
**Source:** RESEARCH.md-verified ink! v6 dispatch behavior (`crates/ink/codegen/src/generator/dispatch.rs`) — not
present in this repo's own code, but governs every `.submit().await` call written in this phase.
**Apply to:** All e2e tests — any `Err(_)` from a contract message reverts the whole extrinsic (no graceful
`Ok(Err(...))`); tests must follow the exact dependency order from RESEARCH.md's "Dependency-Ordered Module
Validation Chain" so no happy-path call ever hits an `Err` branch.

### Cargo feature flag (already declared, unused)
**Source:** `contracts/inktix/Cargo.toml:21` (`e2e-tests = []`)
**Apply to:** All e2e test modules — this is the flag that keeps VALIDATE-02 (existing 14 unit tests stay green)
and VALIDATE-03 (CI stays green, since `.github/workflows/ci.yml` never passes `--features e2e-tests`) true by
construction; no CI file changes needed (D-07).

## No Analog Found

None. Every planned test group has at least a role-match analog within `contracts/inktix/src/lib.rs`'s existing
`#[cfg(test)] mod tests` block or `contracts/inktix/src/tests/*.rs` helper files; the async/`ink_e2e`-specific
mechanics (not present anywhere in this repo) are covered by the RESEARCH.md-verified official ink! `flipper` /
`fallible-setter` example skeletons, which this map treats as the framework-mechanics analog layered on top of the
in-repo assertion-style analog.

## Metadata

**Analog search scope:** `contracts/inktix/src/lib.rs` (inline `#[cfg(test)] mod tests` block, lines 626-910),
`contracts/inktix/src/tests/*.rs` (helper-struct files), `contracts/inktix/Cargo.toml` (feature flags),
`contracts/inktix/src/logic/**` (message signatures referenced by RESEARCH.md), RESEARCH.md's own verified
ink! official-example excerpts (used only where no in-repo analog exists).
**Files scanned:** 6 (`lib.rs`, `tests/mod.rs`, `tests/core_tests.rs`, `tests/sports_tests.rs`,
`tests/concert_tests.rs`, `Cargo.toml`)
**Pattern extraction date:** 2026-09-20
