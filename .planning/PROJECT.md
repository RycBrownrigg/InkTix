# InkTix v2 Migration

## What This Is

InkTix is a decentralized event ticketing platform on the Polkadot ecosystem — ink! smart contracts for on-chain ticket/event/NFT management, cross-chain transfers via XCM, and wallet-based authentication, with a Next.js frontend. It's currently deployed on Westend Asset Hub (ink! 5.1.1 / `pallet-contracts`). This project tracks finishing the migration to ink! v6 / `pallet-revive`, targeting Passet Hub testnet (Polkadot Hub mainnet once GA).

## Core Value

The full ticket-purchase flow — browse, pay with real value, mint the ticket NFT, and let funds be withdrawn — works end-to-end against a live ink! v6 / `pallet-revive` chain (Passet Hub).

## Requirements

### Validated

- ✓ Event, venue, and ticket data model with NFT minting — existing (v1, carries into v2 port)
- ✓ Wallet-based auth via Polkadot.js, Talisman, SubWallet, Nova extensions — existing
- ✓ Cross-chain transfers via XCM (`limitedReserveTransferAssets`) — existing
- ✓ Resale listing (sell-side) — existing
- ✓ Analytics dashboard, sports loyalty/season-pass/fantasy features, concert artist management — existing
- ✓ ink! v6 / `pallet-revive` toolchain round-trip (build → instantiate → call) validated locally — Phase 0, 2026-07-27
- ✓ Contract mechanically ported to ink! v6 (`AccountId` → `Address` throughout, compiles clean, all 14 inline tests pass, 103.6K release blob) — Phase 1a/1c, 2026-07-29

### Active

- [ ] Contract behaviorally validated module-by-module against a live `ink-node` (Phase 1d — venue/event, ticket purchase + user_tickets, pricing, nft_management, anti_scalping, loyalty modules, season pass/fantasy/analytics/artist/xcm)
- [ ] Contract deployed and callable on Passet Hub testnet (Phase 2)
- [ ] `purchase_ticket` made payable; real value compared against `calculate_price`, rejecting on mismatch (Phase 3)
- [ ] Withdraw/settlement message added — currently no way to move funds out of the contract at all (Phase 3)
- [ ] New pallet-revive contract provider for the frontend (PAPI/ink! SDK), replacing `@polkadot/api-contract` which has no pallet-revive equivalent (Phase 4)
- [ ] H160 address handling end-to-end in the frontend (deployment capture, user identity display) (Phase 4)
- [ ] XCM junction type updated from `AccountId32` to `AccountKey20` for H160 destinations (Phase 4)
- [ ] Chain-aware token decimal handling, replacing hardcoded Westend/WND-specific math (Phase 4)

### Out of Scope

- **Production cutover to Polkadot Hub mainnet (Phase 5)** — gated by Polkadot Hub mainnet GA timing, which is outside this project's control. Becomes its own milestone once GA lands, rather than leaving this roadmap permanently open. Decided 2026-09-20.
- **Resale marketplace buy-side** (`purchase_resale_ticket`) — new functionality, not something v1 had that the port needs to restore. v2's scope is a toolchain/runtime migration, not a feature expansion. Listing-only carries over as-is. Decided 2026-09-20.
- **Overpayment refund handling** — reject-on-mismatch was chosen instead of accept-and-refund-excess, to avoid the reentrancy-ordering surface a refund transfer would introduce (no reentrancy guards exist anywhere in the contract yet). Remains a candidate v3+ UX improvement once the payable path has a security-review pass behind it. Decided 2026-09-20.
- **viem/ethers + MetaMask-style wallets** — noted in `docs/v2_plan.md` as a possible later addition, not the primary integration path. PAPI/ink! SDK over the existing Substrate WSS endpoint, signed by the existing 4 wallet extensions, is the chosen route — it preserves current UX and avoids supporting two wallet models in parallel.

## Context

- Full phase-by-phase detail, decisions, and findings live in `docs/v2_plan.md` — the project's own living source of truth for this migration (predates this GSD project; this project tracks execution against its Phases 1d–4). Update both documents together when scope or decisions shift.
- Codebase mapped in `.planning/codebase/` (2026-09-20). Notable flags from `CONCERNS.md`:
  - Zero `#[ink(payable)]` messages and zero `transferred_value()`/`.transfer()` calls anywhere in the contract today — tickets are functionally free until Phase 3 lands.
  - No reentrancy guards anywhere in the contract.
  - Sparse test coverage: 14 contract unit tests, 3 frontend tests, 0 on-chain integration tests; e2e test plan is paused.
  - Minimal input validation (1 check across 22 files).
  - README still describes ink! 5.1.1 in places — drifted from the current branch state.
- Toolchain risk: ink! v6 (`6.0.0-beta.1`) and `cargo-contract` (`6.0.0-beta.2`) are beta releases from a team whose active development had paused as of Jan 2026. `nightly-2026-01-15` is pinned as the long-term build toolchain (no patched cargo-contract exists for newer nightlies) — this is standing friction, not a temporary workaround.
- This is a public repository the user references from their personal website — documentation and public-facing decisions should be written accordingly.
- No hard deadline. Pacing is soundness-driven, confirmed 2026-07-28 and reconfirmed at project kickoff (2026-09-20).

## Constraints

- **Tech stack**: ink! v6 / `pallet-revive` only — `pallet-contracts`-era tooling (`@polkadot/api-contract`'s `ContractPromise`/`CodePromise`) has no equivalent and must be replaced, not adapted.
- **Toolchain**: Rust `nightly-2026-01-15` is a hard pin for all contract builds — a Rust/Cargo strictness change after this date breaks `cargo-contract` v6.0.0-beta.2, and no fix is expected given the ink! team's paused status.
- **Compatibility**: preserve the existing 4 wallet extensions and SS58 signing UX — this is why PAPI/ink! SDK was chosen over an EVM-wallet route.
- **Scope discipline**: this is a toolchain/runtime migration, not a product pivot — v1's feature set carries over. New functionality is limited to what real payment requires (payable purchase + withdrawal), not general feature expansion.
- **Timeline**: none — soundness-driven, not deadline-driven.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| v1 scope = Phases 1d–4 only; Phase 5 (mainnet cutover) deferred to a future milestone | Mainnet GA timing is outside project control; including it leaves the roadmap permanently open | — Pending |
| Overpayment policy: reject on mismatch, not refund | Avoids reentrancy-ordering surface with no guards yet in place; frontend already sends the exact price from `calculate_price` | — Pending |
| Resale marketplace buy-side: deferred, listing-only carries over | New functionality, not a migration item — v2 scope is migration, not feature expansion | — Pending |
| PAPI/ink! SDK over existing wallet extensions, not viem/MetaMask | Preserves current 4-wallet UX and SS58 signing; avoids supporting two wallet models in parallel | — Pending |
| Keep the unified `contracts/inktix` contract, don't split it | Everything shares one atomic storage borrow with no clean module boundary; splitting adds cross-contract H160 calls and a new reentrancy surface on an unproven runtime. `CodeTooLarge` would be the trigger to reconsider, not a preference — blob came in at 103.6K, comfortably under risk | ✓ Good — validated in Phase 1c |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-09-20 after initialization*
