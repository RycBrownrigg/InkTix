# InkTix v2 Rearchitecture Plan

Living source of truth for the migration from ink! 5.1.1 / `pallet-contracts`
(Westend Asset Hub) to ink! v6 / `pallet-revive` (Passet Hub → Polkadot Hub).
Branch: `redesign/polkadot-hub`.

**Update this file at the end of every phase** — status, decisions, issues hit,
resolutions, and findings that affect later phases. This is what lets work resume
across sessions without reconstructing context from commit messages.

---

## Status

| Phase | Status | Scope |
|-------|--------|-------|
| **Phase 0** | ✅ Done — 2026-07-27 | Toolchain validation: confirm ink! v6 / cargo-contract v6 build → instantiate → call round-trip against a local `ink-node` |
| **Phase 1** | 🔄 In progress (1a, 1c done, 1b skipped — 2026-07-29; 1d remaining) | Port the unified contract to ink! v6 / `pallet-revive`: repo hygiene & toolchain pinning, mechanical type migration (contract compiles + all 14 tests pass + 103.6K release blob), module-by-module behavioral validation against a live `ink-node` (not yet run) |
| **Phase 2** | 🔜 Scoped, not started | Deploy the ported contract to Passet Hub testnet |
| **Phase 3** | 🔜 Scoped, not started | Add real payment enforcement to the unified contract (not a redesign — see [Why v2](#why-v2)) |
| **Phase 4** | 🔜 Scoped, not started | Frontend / SDK integration: new pallet-revive contract provider, H160 addressing, XCM junction-type change |
| **Phase 5** | 🔜 Scoped, not started | Production cutover to Polkadot Hub mainnet (new phase — cutover was implicit before, now has its own scope) |

Phase 1-5 scope below was reconstructed on 2026-07-28 from: the Phase 0 findings
(`docs/rearchitecture_toolchain_notes.md`), a live inventory of the contract's
storage coupling and the frontend's address/balance assumptions, and direct
inspection of the ink! v6 source. Load-bearing claims were spot-checked against the
actual repo files, not taken on faith. See [Open Questions](#open-questions) for what
still isn't decided.

---

## Why v2

v1 is deployed against Westend Asset Hub's classic `pallet-contracts` (Wasm)
execution environment. Polkadot's system chains are moving to `pallet-revive`
(PolkaVM, EVM-compatible bytecode) as the supported smart-contract environment going
forward — the branch name `redesign/polkadot-hub` reflects that target. ink! v6 builds
for `pallet-revive`, not `pallet-contracts`, so staying on ink! 5.1.1 means building
against a path Polkadot's own system chains are phasing out.

This is a toolchain/runtime migration, not a product pivot — v1's feature set and UX
goals carry over. But the runtime shift touches low-level assumptions across the whole
stack:

- **Contract identity:** ink! v6 keeps a 32-byte `AccountId` type, but
  `self.env().caller()` now returns `Address` (H160, Ethereum-style 20 bytes), with
  **no conversion back from `Address` to `AccountId`**. In practice this means
  contract-side user identity becomes `Address` everywhere, not a partial migration —
  see Phase 1b.
- **Value types:** `Balance`/`u128` → `ink::U256` in payable messages (price checks,
  transfers, anything touching `transferred_value()`)
- **Deploy target:** Westend Asset Hub → Passet Hub (pallet-revive testnet) → Polkadot
  Hub mainnet (once generally available)
- **Frontend:** `@polkadot/api-contract`'s `ContractPromise`/`CodePromise` have no
  pallet-revive equivalent at all — this isn't a config change, it's a new provider
  (see Phase 4)

ink! v6 and cargo-contract v6 are beta releases from a team whose active development
had paused as of Jan 2026 (per Phase 0 findings below) — so the plan front-loads
tooling risk: validate the full build → deploy → call round-trip *before* porting any
real contract code, rather than discovering toolchain breakage midway through a
rewrite.

**Correction (2026-07-28):** the original framing of Phase 3 as a "`ticket-nft`
contract payment-enforcement **redesign**" implied both (a) that payment enforcement
already exists and needs migrating to `U256`, and (b) that the unified contract was
being split into smaller contracts including a standalone `ticket-nft` one. Neither is
true. A direct inventory of `contracts/inktix/src/` found **zero `#[ink(payable)]`
messages and zero calls to `transferred_value()`/`.transfer()` anywhere** —
`dynamic_price` is bookkeeping only, never checked against real value sent. So Phase 3
is *adding* payment enforcement, not redesigning it. Separately, splitting the
contract was evaluated and explicitly deferred (see Phase 3 and the decomposition note
below) — v2 keeps one unified contract. The old Phase 3 name is corrected in this doc;
if you find the old phrasing elsewhere (commit messages, external notes), it no longer
reflects the plan.

One consequence of "payment enforcement doesn't exist yet": there's also **no
withdraw/payout message** anywhere in the contract. Adding payable `purchase_ticket`
without a way to get funds back out ships a fund trap — this is called out explicitly
in Phase 3.

*Gap: the product/business trigger for migrating now (vs. staying on v1 longer) isn't
captured anywhere yet. Low priority — there's no hard deadline (confirmed
2026-07-28), so this doesn't block planning or execution.*

---

## Open Questions

Genuinely unresolved items. Everything that had a clear answer as of 2026-07-28 has
been moved into the phase sections below and removed from this list — see each
phase's detail for the resolution and reasoning.

- **Target timeline / milestones** — none set. Confirmed acceptable: pacing is
  soundness-driven, not deadline-driven (2026-07-28).
- **Business/product motivation for migrating now** — still not captured anywhere.
  Low priority given the above, but worth a line once known.
- **Polkadot Hub mainnet GA timing** — outside this project's control. Phase 5 may sit
  open indefinitely; that's expected, not a sign of stalled work.

**Scope decision (2026-09-20):** the current tracked effort (Phases 1d–4) is v1 —
"done" means the full ticket flow working end-to-end against Passet Hub testnet, with
real payment enforcement. **Phase 5 (mainnet cutover) is explicitly out of scope for
v1**, not abandoned — it becomes its own milestone once Polkadot Hub reaches GA. This
avoids leaving the roadmap permanently open on a timeline nobody controls.

---

## Phase 0: Toolchain Validation — Details

**Goal:** prove the local dev toolchain works end-to-end before investing in contract
code changes.

**Setup**
- `cargo-contract` 6.0.0-beta.2, `ink` crate 6.0.0-beta.1 (see version-pinning
  decision below), `rust-src` component
- `ink-node` v0.47.0 — prebuilt binary (not a cargo install), macOS Gatekeeper
  quarantine needed clearing (`xattr -dr com.apple.quarantine`)
- `.gitignore` updated to exclude `.tooling/` for local dev binaries like `ink-node`

**Decisions made**
- Pin contract dependencies to `ink = "6.0.0-beta.1"` (what `cargo contract new`
  scaffolds by default) even though `cargo-contract` itself is `6.0.0-beta.2` — the two
  are versioned independently, and cargo-contract's "not compatible" warning on every
  command is a false positive comparing those two numbers, not a real incompatibility.
- Pin `nightly-2026-01-15` as the toolchain for all cargo-contract invocations,
  long-term rather than as a temporary workaround (see blocker below) — treat this as
  the standing build toolchain until/unless a fixed cargo-contract release appears.
- Use `wss://testnet-passet-hub.polkadot.io` and the name **Passet Hub** in
  `config/chains.ts` and docs going forward — corrects an earlier planning assumption
  of "Paseo Hub." Chain ID `420420417` from earlier research was unaffected.

**Issues encountered and resolutions**
1. **Version-mismatch warning** — cargo-contract 6.0.0-beta.2 vs. ink crate
   6.0.0-beta.1 triggered a "not compatible" warning on every command. Resolved by
   confirming the two crates version independently and the warning is spurious;
   documented so it isn't re-investigated later.
2. **Real blocker: nightly build failure** — building against the *default* nightly
   Rust toolchain failed with `` error: `.json` target specs require -Zjson-target-spec ``.
   Root cause: an upstream Rust/Cargo strictness change
   ([rust-lang/cargo#16557](https://github.com/rust-lang/cargo/pull/16557)) landed
   *after* cargo-contract v6.0.0-beta.2 froze and *after* ink!'s maintaining team
   paused active development (Jan 2026) — so no patched cargo-contract exists to pass
   the now-required flag. Resolved by pinning `nightly-2026-01-15` (predates the
   breaking change) and invoking every build/instantiate/call command through
   `cargo +nightly-2026-01-15 contract ...`. Flagged as long-term tooling friction,
   not a one-off fix, given the team's frozen status.

**Findings that affect later phases**
- `self.env().transferred_value()` now returns `ink::U256`, not `Balance`/`u128` —
  relevant to Phase 3's payment-enforcement work; any `dynamic_price` comparison
  against transferred value needs `U256`-aware arithmetic.
- `U256` is not in the `#[ink::contract]` module's implicit prelude — needs an explicit
  `ink::U256` import in contract code.
- Deployed contract addresses are H160-shaped (confirmed against a real deployed
  throwaway address, e.g. `0x5801b439a678d9d3a68b8019da6a4abfa507de11`) — confirms the
  identity shift from `AccountId` to `Address`/`H160`.
- First call from a fresh dev account triggers an implicit account-mapping step
  (`RevivedAccountMapping`, mapping a Substrate `AccountId` to an `H160` address on
  first use). Handled locally with `--skip-confirm`; flagged as needing to be scripted
  explicitly against a funded Passet Hub account before Phase 2's deploy.

**Result: Phase 0 gate PASSED.** Full round-trip confirmed against local `ink-node`:
build → instantiate (constructor arg `true`) → payable `deposit` tx (real balance
transfer, 1 UNIT Alice → contract) → state-changing `flip` tx → `get` query correctly
returned `false` (the flip was durably applied on-chain). The throwaway contract used
for this test lived in a session scratchpad and was not committed to the repo.

Full raw notes: [`rearchitecture_toolchain_notes.md`](./rearchitecture_toolchain_notes.md).

---

## Phase 1: Port the Unified Contract to ink! v6 / pallet-revive

**Goal:** `contracts/inktix` builds under ink! 6.0.0-beta.1 / cargo-contract
6.0.0-beta.2, all inline tests pass, and every message is behaviorally validated
against a local `ink-node`. Functional parity with v1 — no new business logic (that's
Phase 3).

**Decided (2026-07-28):** incremental, not big-bang — but since this is one Rust
crate under a single `#[ink::contract]` macro, "incremental" means staged
*compilation* (by Cargo feature: `core` only → `+sports` → `+concert`) and staged
*validation* module-by-module, not literally partial compilation of one crate.

Four sub-steps, not separate phases:

### 1a — Repo hygiene & toolchain pinning ✅ Done — 2026-07-29
No contract logic changes; keep these as isolated commits so a port failure is never
confused with a pipeline failure.

**Notes from doing this:**
- `inktix-deployment/deploy.sh` turned out to be a distinct, already-stale deployment
  path (targets Shibuya/Astar, PM2, `dist/` output — doesn't match the current
  documented deploy flow in CLAUDE.md at all). Only its `build_contracts()` function
  was touched (removed the three legacy contract build blocks, now builds
  `contracts/inktix` only) — the rest of the file (old Shibuya contract addresses in
  `create_env()`, etc.) is out of scope for this cleanup and likely worth a separate
  decision on whether the whole directory should be retired.
- `rust-toolchain.toml` was scoped to `contracts/inktix/` (not repo root), so CI's
  toolchain step sets `toolchain: nightly-2026-01-15` / `components: rust-src`
  explicitly rather than relying on auto-detection — the action doesn't reliably
  discover a toolchain file outside the repo root.
- `docker-compose.yml`'s `substrate` service was annotated as unsupported rather than
  removed, since it may still be useful for anyone working on v1 in parallel.
- Branch is now expected-red (`ink = "6.0.0-beta.1"` doesn't compile against v5-era
  code yet) — that's 1c's job, not a regression to chase.
- Ran `cargo check --features std,sports,concert` to confirm the version bump itself
  is sound (not just expected to fail for the right reason). Confirmed: dependency
  resolution succeeds, and all 26 errors are `expected AccountId, found H160` at
  every `self.env().caller()` call site — exactly the migration 1c/1b describe, not a
  bad pin. One thing to check first in 1b/1c, not yet root-caused: a small cluster of
  errors at the `#[ink(storage)] pub struct InkTix` definition itself (`lib.rs:58`)
  — `E0433 could not find marker in core`, `E0690 transparent struct needs at most
  one field`, `E0282 cannot infer type`. Plausibly cascading confusion from the
  AccountId/H160 mismatches elsewhere in the file rather than a separate root cause,
  but unconfirmed — worth resolving first since it changes the risk picture if it's
  real.

- **Delete the legacy contract crates** (`contracts/inktix_core/`,
  `contracts/sports_broker/`, `contracts/concert_broker/`) — decided 2026-07-28,
  already fully superseded by the unified contract per CLAUDE.md, and every future
  `grep` for `AccountId`/`caller()` during the port would otherwise turn up legacy
  noise. Fix what references them: `scripts/build.sh` (already stale — points at
  `contracts/inktix_core`), `inktix-deployment/deploy.sh`, and retire
  `docs/sports_broker_guide.md` / `docs/concert_broker_guide.md`. **Do not** touch the
  three frontend files that reference legacy contracts
  (`frontend/src/app/docs/page.tsx`, `ContractDeployment.tsx`,
  `utils/contractMethods.ts`) — those get rewritten wholesale in Phase 4; note them
  there so they aren't lost.
- **Add `contracts/inktix/rust-toolchain.toml`** pinning `nightly-2026-01-15` +
  `rust-src` — none exists today, so the pin currently lives only in this doc and
  decays into tribal knowledge. A pinned toolchain file makes `cargo contract build`
  just work without `+nightly-...` on every invocation.
- **Add `scripts/dev-node.sh`** to download `ink-node` v0.47.0 into the
  already-gitignored `.tooling/` if missing (platform-detect mac-universal / linux /
  linux-arm64 — confirmed no Docker image exists for `ink-node`, tarballs only) and
  run it. Mark `docker-compose.yml`'s `substrate` service
  (`parity/substrate-contracts-node:latest`) unsupported for v2 — it's a
  `pallet-contracts` node and can never host a revive contract, not just a stale
  reference.
- **Fix CI** (`.github/workflows/ci.yml`), four confirmed concrete gaps: `on: push:
  branches: [main]` means pushes to `redesign/polkadot-hub` never trigger CI today —
  add the branch; `toolchain: stable` needs to become the pinned nightly;
  `target: wasm32-unknown-unknown` is stale for a PolkaVM target; `cargo install
  cargo-contract --locked` is unpinned and would pull latest instead of
  `6.0.0-beta.2`.
- **Bump `contracts/inktix/Cargo.toml`** to `ink = "6.0.0-beta.1"` / `ink_e2e =
  "6.0.0-beta.2"`. The branch is expected to be red from this commit until 1c —
  that's normal, not a regression to chase.

### 1b — Identity spike — SKIPPED (2026-07-29)

Decided to skip the isolated throwaway-contract spike and go straight to 1c.
Rationale: `cargo check` against the real bumped `Cargo.toml` (done as part of
verifying 1a) already gave direct empirical signal — 26 `expected AccountId, found
H160` errors at every `caller()` call site, confirming the `Address`-everywhere
approach is necessary. Trade-off accepted knowingly: this skips the cheap isolated
check for whether `TypeInfo`/`StorageLayout` bounds hold on the composite
`Mapping<(Address, u32), _>` key before committing to the full port — if that bound
fails, it'll surface during 1c's real migration instead of before it, at higher cost
to unwind. If 1c hits a storage-layout wall, the fallback (`UserId([u8; 20])`
newtype) described in the original 1b plan is still the way out.

### 1b (original plan, not executed — kept for reference)
ink! v6 keeps a 32-byte `AccountId` type, but `caller()` returns `Address` (H160) with
**no way to convert back**. That means contract identity becomes `Address` everywhere
or the contract doesn't work — this has to be decided and validated before touching
the real contract, not discovered mid-port.

- Build a throwaway spike contract (same scratch methodology as Phase 0 — not
  committed) exercising: `Mapping<Address, Vec<u64>>`, `Mapping<(Address, u32), T>`
  (the composite key `contracts/inktix/src/storage/contract_storage.rs`'s
  `TeamLoyaltyProfile` mapping needs), and a struct with `Option<Address>` +
  `Vec<Address>` fields (matching the shapes actually used in
  `types/core/anti_scalping.rs` and `types/concert/artist.rs`). Build → instantiate →
  call against local `ink-node`.
- **Gate:** if it passes, adopt `Address` as the identity type everywhere in 1c. If a
  type-bound failure shows up on a specific shape (e.g. `TypeInfo`/`StorageLayout` on
  the composite tuple key), fall back to a `UserId([u8; 20])` newtype with derived
  codec/`TypeInfo`/`StorageLayout` for that spot only. If storage-layout generation
  itself breaks on the full shape — **stop and escalate**; that's the signal that the
  storage redesign this plan is deliberately avoiding (see Phase 3's decomposition
  note) might be unavoidable after all.
- Record the metadata's address encoding from this spike — Phase 4's contract
  provider needs to match it exactly.

### 1c — Mechanical type migration to first green build
- Global `AccountId → Address` across `src/types/**`, `src/storage/**`,
  `src/logic/**`, `src/lib.rs`. Fix every `AccountId::from([0u8; 32])` default
  constructor (becomes `Address::zero()`). Update the inline `#[ink::test]`s in
  `lib.rs` for v6's `default_accounts()` signature change (no generic param, returns
  `Address` fields).
- Compile in feature order: `core` only → `+sports` → `+concert` — this is the staged
  compilation the "incremental" decision maps to.
- **Explicitly out of scope here:** any `u128 → U256` change. Nothing calls
  `transferred_value()` yet (see [Why v2](#why-v2)), so pricing arithmetic stays
  untouched until Phase 3.
- **Measure the release blob size the moment it first builds**, and record it here.
  `CodeTooLarge` on the real chain (pallet-revive's size limits are stricter than
  `ink-node`'s permissive dev config) is the one risk that could force the contract
  split Phase 3 defers — catching it now, with the `sports`/`concert` Cargo features
  as a pressure valve (ship `core`+`sports`, defer `concert`, if needed), makes "keep
  it unified" a verified call instead of an assumed one.

**Done — 2026-07-29.** What actually happened, vs. the plan above:

- Global `AccountId → Address` rename: 118 occurrences across 22 files (close to the
  117/23 estimate from earlier research). All were pure type usages — imports, field
  types, fn params, `Mapping` keys/values, default constructors — no ambiguous cases.
  Done as a single word-boundary rename, then a separate pass fixing the 10
  `AccountId::from([0u8; 32])` sites to `Address::zero()` (can't blindly rename these —
  `H160`/`Address` is 20 bytes, not 32, so `Address::from([0u8; 32])` would be wrong).
- **`#[ink::test]` signature updates: turned out unnecessary.** None of the 13 inline
  tests actually call `default_accounts()` or `set_caller()` — they just call
  `InkTix::new()` and message methods directly, so they pick up whatever `caller()`
  returns in the off-chain test environment without needing changes. The "13 tests
  need updating" estimate from earlier research didn't hold up under direct
  inspection — recorded here so it isn't repeated as fact.
- **A real, separate bug was found and fixed, not predicted by this plan.** After the
  `Address` rename, 4 errors remained at the `#[ink(storage)] pub struct InkTix`
  definition (`E0433 could not find marker in core`, `E0690 transparent struct needs
  at most one field...`, `E0282 cannot infer type`) — the same cluster flagged as
  "unconfirmed, check first" when 1a was verified. Root cause, confirmed by fix:
  `src/types/mod.rs` declares `pub mod core;`, and `mod inktix { use crate::types::*;
  ... }` (the `#[ink::contract]`-annotated module) glob-imports it — so the bare name
  `core` inside that module's scope resolves to the local `crate::types::core` module
  instead of the real `core` crate. ink! v6's `#[ink::contract]` macro expansion emits
  an unqualified `core::marker::PhantomData` reference (for its new storage-pointer
  wrapper), which resolved to the wrong `core` and cascaded into all 4 errors. This is
  a general Rust footgun (a local item named `core`/`std`/`alloc` can shadow the
  extern-prelude crate name for any code textually inside a scope that glob-imports
  it) that predates ink! v6 and would trip up any macro emitting an unqualified `core`
  path in that scope — not an ink! bug. **Fix:** renamed `src/types/core/` →
  `src/types/core_types/` (`git mv` + updated `types/mod.rs` and 13 files' qualified
  `crate::types::core::...` references). Confirmed by re-running `cargo check`: the
  4 errors disappeared with no other change.
- Compiled clean (only pre-existing unused-import/variable warnings, nothing new) in
  all three feature combinations: `std` only, `std,sports`, `std,concert`, and
  `std,sports,concert`.
- `cargo test --features std,sports,concert`: **all 14 inline tests pass**, unchanged
  behavior from v1.
- `cargo contract build --release`: succeeds. **Blob size: 322.3K original → 103.6K
  optimized.** No `CodeTooLarge` signal at this size — comfortably small, no need yet
  to consider the `sports`/`concert` feature pressure valve. (Real chain size limits
  not yet cross-checked against this number — do that in Phase 2 once the live
  endpoint set is confirmed, not assumed safe just because it's small.)
- `u128 → U256`: untouched, as planned — confirmed no pricing/payment arithmetic was
  touched in this pass.

### 1d — Behavioral validation, module by module
This is where "incremental" actually lives day to day. Exercise each module against a
running local `ink-node` (via `scripts/dev-node.sh` + `cargo +nightly-2026-01-15
contract call`), in dependency order, recording pass/fail per module:

1. `venue_management` / `event_management` — no identity, no money; cheapest signal.
2. `ticket_management` purchase path + `user_tickets` — first real
   `Mapping<Address, _>` write/read round-trip.
3. `pricing` — the time-multiplier logic runs under `ink-node`'s manual-seal block
   production (blocks are authored per transaction, not on a timer); expect
   surprises around anything that assumes wall-clock block advancement.
4. `nft_management` — zero Balance/u128 arithmetic here, should be the cleanest port.
5. `anti_scalping` — the `Vec<Address>` blacklist/whitelist round-trip.
6. `loyalty` / `advanced_team_loyalty` — the composite `Mapping<(Address, u32), _>`
   key, the single riskiest storage item in the contract.
7. `season_pass_management`, `fantasy_sports_management`, `analytics`,
   `artist_management`, `xcm_management`.

**Exit gate for Phase 1:** every module above validated on-chain, all inline unit
tests green, blob size recorded and within budget, CI green on the branch.

---

## Phase 2: Deploy to Passet Hub

**Goal:** the ported (v1-equivalent) contract live on the public testnet, instantiated
and callable via CLI. No frontend, no new logic — a thin gate that surfaces
chain-specific reality (gas/storage costs, real network conditions) before building on
top of it.

**Scope**
- **Re-confirm the live endpoint set first** — don't trust Phase 0's note as
  permanent. Passet Hub is documented as a temporary testnet instance, and endpoint
  naming has reportedly shifted before. Confirm and record here: Substrate WSS
  endpoint, ETH-RPC endpoint (if used later), chain ID, token symbol, and **token
  decimals** (Phase 4's decimal handling depends on this).
- Fund a deploy account via the Polkadot faucet.
- Script the `RevivedAccountMapping` step explicitly for that funded account — Phase
  0 only used `--skip-confirm` locally; production/testnet deploy needs a real,
  repeatable equivalent.
- Extend `scripts/deploy-and-configure.sh`: add a network/`--url` parameter (currently
  hardcoded to `ws://127.0.0.1:9944` only), invoke through
  `+nightly-2026-01-15`, replace the local-node hint with `scripts/dev-node.sh`. Also
  **fix a live bug independent of this migration**: line 51's `grep -oP` is
  GNU-only and fails on BSD grep (confirmed on this machine today) — address capture
  silently falls through to the "could not extract" branch. Fix while adapting
  extraction to `0x`-shaped addresses anyway.
- Add a `passet-hub` entry to `frontend/src/config/chains.ts` (endpoint, chain ID,
  token, decimals) — config only, nothing consumes it until Phase 4.
- Re-run a representative subset of Phase 1d's module validation against the live
  chain. Expect gas/storage-deposit-limit differences from `ink-node`'s permissive dev
  defaults.

**Risks:** `CodeTooLarge` on the real chain (mitigation already staged in Phase 1c);
storage-deposit costs for a large (~185-field) storage struct; the testnet is
explicitly described as temporary, so treat any deployed address as disposable — never
hardcode it outside `.env.*` files.

**Depends on:** Phase 1 exit gate. **Unblocks:** Phase 3's on-chain money testing and
Phase 4's integration target.

---

## Phase 3: Payment Enforcement in the Unified Contract

*(Renamed from "ticket-nft contract payment-enforcement redesign" — see the
correction note in [Why v2](#why-v2). This is new functionality, not a migration of
existing logic, and it stays inside the current unified contract.)*

**Decomposition note (2026-07-28):** splitting ticket/NFT logic into a standalone
contract was evaluated and explicitly deferred. `purchase_ticket`,
`calculate_price`, and `mint_ticket_nft` all read/write a single flat
`InkTixStorage` struct (events, venues, anti-scalping config, tickets, analytics)
within one atomic borrow, with no existing module or storage boundary to split along.
Decomposing would mean introducing cross-contract calls with H160 addressing and a new
reentrancy surface, on top of a runtime that hasn't been proven yet. Keeping it
unified is the lower-risk call; Phase 1c's blob-size check (`CodeTooLarge`) is what
would force a reconsideration, not a preference. A future contract split remains a
possible v3 idea, not v2 scope.

**Goal:** ticket purchases move real value, safely.

**Scope**
- Make `purchase_ticket` `#[ink(payable)]`; compare `self.env().transferred_value()`
  (`U256`) against the price from `pricing::calculate_price`.
- **Convention: `U256` only at the transfer/comparison boundary.** Keep `u128` as the
  internal/storage/ABI type for prices — converting the full storage struct to `U256`
  would bloat metadata and the frontend's number handling for no real benefit;
  `U256::from(u128)` is infallible at the boundary where it's actually needed
  (`transferred_value()` comparison, `env().balance()`, `env().transfer()`
  arguments). Document this convention here so it doesn't erode over time.
- **Decided (2026-09-20): reject on mismatch**, not refund-the-excess. Simpler (no
  outbound transfer in the purchase path at all) and avoids the reentrancy-ordering
  surface a refund path would introduce, which matters here since **no reentrancy
  guards exist yet anywhere in the contract**. The frontend controls the exact value it
  sends (it calls `calculate_price` immediately before submitting), so a mismatch
  should be rare in practice. Refund-the-excess remains a fair v3+ UX improvement once
  the payable path itself has a security-review pass behind it.
- **Add a withdraw/settlement message.** There is currently no way to move funds out
  of the contract at all — shipping payable `purchase_ticket` without this ships a
  fund trap. At minimum: an owner/organizer-guarded withdraw path.
- Revisit `calculate_loyalty_points`'s hardcoded decimals assumption
  (`price / 1_000_000_000_000_000`) — it only becomes meaningful once payments are
  real; confirm it matches Passet Hub's actual token decimals (see Phase 2).
- **Decided (2026-09-20): defer the resale buy-side.** `resell_ticket` creates a
  listing today, but there is **no buy-side message at all**. Building
  `purchase_resale_ticket` is new functionality, not something v1 had and the port
  needs to restore — v2's stated scope is a toolchain/runtime migration, not a feature
  expansion (see [Why v2](#why-v2)). Listing-only carries over as-is; resale purchase
  is a candidate for a future milestone, tracked explicitly rather than left as a
  silently half-built marketplace.
- Check whether `anti_scalping` price-cap constraints should now gate the payable
  path, since they become enforceable for the first time here.

**Risks:** existential-deposit/minimum-balance interactions on transfers; reentrancy
on any refund path (avoid calling out before finalizing state); this is the
contract's first exposure to real value, so a focused security-review pass at this
phase's exit is worth the cost.

**Depends on:** Phase 2 (need a live chain to test real value transfer against).
**Unblocks:** Phase 4's purchase UX, which is meaningless without real payment.

---

## Phase 4: Frontend / SDK Integration

**Goal:** the Next.js app talks to the Passet Hub contract with H160 addresses and
real payment.

**Central decision:** `@polkadot/api-contract`'s `ContractPromise`/`CodePromise` are
`pallet-contracts`-only with no pallet-revive equivalent — this needs a new provider,
not a config tweak. Two routes considered:
- **PAPI / ink! SDK over the existing Substrate WSS endpoint, signed by the existing
  wallet extensions (Polkadot.js, Talisman, SubWallet, Nova) — recommended.**
  Preserves the current 4-wallet support and SS58 signing UX; `@polkadot/api` stays
  for balances and XCM.
- viem/ethers over the ETH-RPC endpoint (ink! v6 can emit a Solidity-ABI-shaped
  metadata variant), with MetaMask-style wallets — noted as a possible *later
  addition*, not the primary path, since it would mean supporting a second wallet
  model in parallel.

**Scope, roughly in order**
- **4a Chain config** — add `chainId` and an address-format field to
  `config/chains.ts`'s `ChainConfig`; replace the current `hasContractsPallet:
  boolean` with something that distinguishes pallet-contracts vs. pallet-revive
  chains. Update the capability probes in `services/blockchain.ts` accordingly
  (`api.tx.contracts` → `api.tx.revive`).
- **4b New provider** — `sdk/reviveContractProvider.ts` alongside the existing
  `sdk/contractProvider.ts`, selected by the existing mock/real factory in
  `sdk/index.ts` (this is additive, not a rewrite-in-place — the factory already
  switches providers). Reimplement the `Result<Result<T,E>, LangError>` unwrapping and
  the gas/storage-deposit dry-run flow for revive's call semantics.
  **Verify empirically here, early — not late:** does browser-extension signing
  survive `RevivedAccountMapping` the same way Phase 0's CLI SURI signing did? Phase 0
  only proved the CLI path. The "keep the existing wallet flow" plan above rests on
  this assumption; test it in the first pass at 4b, since it invalidates the whole
  provider design if wrong.
- **4c Address handling** — fix deployment address capture
  (currently assumes AccountId32/SS58 `.toString()` output, needs to become H160-hex
  aware); port the H160 derivation rule found during Phase 1b
  (`keccak256(pubkey)[12..]`, or truncate directly if eth-derived) to TypeScript so the
  UI can show a user's contract-side identity without an extra RPC round-trip.
- **4d Decimals** — replace hardcoded `/10^10`-style decimal math (currently
  Westend/WND-specific) with chain-aware lookups against `ChainConfig.tokenDecimals`
  (confirmed in Phase 2).
- **4e XCM junction type** — `services/xcm.ts` currently builds an `AccountId32`
  junction for the destination beneficiary; an H160 destination needs an
  `AccountKey20` junction instead. This is a structurally different XCM type, not a
  simple field-type swap.
- **4f Deferred legacy cleanup** — the three frontend files that reference deleted
  legacy contracts, noted and deliberately skipped in Phase 1a:
  `app/docs/page.tsx`, `ContractDeployment.tsx`, `utils/contractMethods.ts`.
- **4g Mock data** — cosmetic updates to `mockProvider.ts`'s hardcoded SS58 addresses,
  for consistency with H160 during migration testing. No structural changes needed —
  address fields are already typed as generic `string`.

**Depends on:** Phase 1's frozen metadata/address encoding (from the 1b spike) and
Phase 3's payable message signatures — starting 4b before Phase 3 lands means redoing
the purchase call path.

---

## Phase 5: Production Cutover

*(Out of scope for the current v1 effort — see the scope decision under
[Open Questions](#open-questions). Tracked here so the plan isn't lost, not as an
active phase.)*

**Goal:** retire the Westend Asset Hub deployment and run on Polkadot Hub mainnet (or
formally park at Passet Hub with a documented trigger, if mainnet isn't GA yet).

**Scope**
- Update `frontend/.env.production` (RPC endpoint, chain name, contract address) —
  baked in at static-export time, so this is a config change before a rebuild/deploy,
  not a script change; the existing deploy scripts need no modification.
- Correct README.md/CLAUDE.md's "Primary testnet: Westend Asset Hub" language.
- Decide and document a v1 decommission/redirect plan.
- Finish the `docker-compose.yml` revive-capable local-node setup if not already done
  in Phase 1a (a thin `Dockerfile` wrapping the `ink-node` binary, written once and
  reused from Phase 1's `scripts/dev-node.sh` recipe).
- Final CHANGELOG entry for the v2 migration.

**Risk:** Polkadot Hub mainnet GA timing is outside this project's control — this
phase may sit open indefinitely. That's expected; it isn't a sign the plan stalled.
