# Roadmap: InkTix v2 Migration

## Overview

This roadmap tracks finishing the migration of InkTix's unified `contracts/inktix`
contract from ink! 5.1.1 / `pallet-contracts` to ink! v6 / `pallet-revive`, targeting
Passet Hub testnet. The mechanical port (type migration, compile, unit tests) is
already done (Phase 0, 1a, 1c in `docs/v2_plan.md`, predating this GSD project). What
remains is proving the contract actually *behaves* correctly on a live chain, getting
it deployed to a public testnet, making ticket purchases move real value safely, and
rewiring the frontend to speak the new provider/addressing model. The four phases below
follow the dependency chain already validated in `docs/v2_plan.md` (Phases 1d -> 2 -> 3
-> 4): you can't deploy code you haven't behaviorally validated, you can't test real
payment without a live chain to test it against, and the frontend integration is
meaningless without a payable contract to call. Phase 5 (mainnet cutover) is explicitly
out of scope for this milestone — see PROJECT.md.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: On-Chain Module Validation** - Every contract module is proven to behave correctly against a live `ink-node`, not just compile
- [ ] **Phase 2: Passet Hub Testnet Deployment** - The ported contract is live, instantiated, and callable on public testnet
- [ ] **Phase 3: Payment Enforcement** - Ticket purchases move real value safely, and organizers can withdraw it
- [ ] **Phase 4: Frontend & SDK Integration** - The Next.js app talks to the live contract with correct addressing, signing, decimals, and XCM junction types

## Phase Details

### Phase 1: On-Chain Module Validation
**Goal**: Every contract module (venue/event, ticket purchase, pricing, nft, anti-scalping, loyalty, sports/concert extras) is behaviorally validated against a live `ink-node` — closing the gap between "compiles and passes off-chain unit tests" (already done) and "actually works on-chain."
**Depends on**: Nothing (first tracked phase — builds on the already-completed mechanical port, Phase 0/1a/1c in `docs/v2_plan.md`)
**Requirements**: VALIDATE-01, VALIDATE-02, VALIDATE-03
**Success Criteria** (what must be TRUE):
  1. Each contract module (venue/event, ticket purchase + user_tickets, pricing, nft_management, anti_scalping, loyalty/advanced_team_loyalty, season_pass/fantasy_sports/analytics/artist/xcm) is exercised against a live local `ink-node`, in dependency order, with pass/fail recorded per module — including explicit confirmation that the composite `Mapping<(Address, u32), TeamLoyaltyProfile>` key round-trips correctly via `stake_on_team()` (the flagged riskiest storage item)
  2. All inline unit tests (`cargo test --features std,sports,concert`) remain green after any fixes made during on-chain validation
  3. CI is green on `redesign/polkadot-hub` (correct branch trigger, pinned nightly toolchain, correct target)
**Plans**: TBD

### Phase 2: Passet Hub Testnet Deployment
**Goal**: The ported (v1-equivalent) contract is live on the public Passet Hub testnet, instantiated and callable via CLI — surfacing chain-specific reality (gas/storage costs, real network conditions) before Phase 3 tests real money against it.
**Depends on**: Phase 1
**Requirements**: DEPLOY-01, DEPLOY-02, DEPLOY-03, DEPLOY-04
**Success Criteria** (what must be TRUE):
  1. The live Passet Hub endpoint set (Substrate WSS, ETH-RPC if used, chain ID, token symbol, token decimals) is reconfirmed against the actual network — not assumed from Phase 0's notes — and recorded in `docs/v2_plan.md` and `frontend/src/config/chains.ts`
  2. A funded deploy account completes the `RevivedAccountMapping` step through a scripted, repeatable process (not ad hoc `--skip-confirm`)
  3. The contract is instantiated on Passet Hub, and a CLI call against it returns the expected on-chain state
  4. A representative subset of Phase 1's module validations (e.g., ticket purchase, loyalty composite-key write, pricing) is re-run against the live Passet Hub chain and passes
**Plans**: TBD

### Phase 3: Payment Enforcement
**Goal**: Ticket purchases move real value, safely — closing the "tickets are functionally free" gap flagged in CONCERNS.md, without shipping a fund trap.
**Depends on**: Phase 2 (need a live chain to test real value transfer against)
**Requirements**: PAYMENT-01, PAYMENT-02, PAYMENT-03, PAYMENT-04
**Success Criteria** (what must be TRUE):
  1. Calling `purchase_ticket` with a `transferred_value()` that doesn't match the `U256`-compared price from `calculate_price` is rejected on the live chain
  2. An owner/organizer can call a dedicated withdraw/settlement message and successfully move accumulated contract funds out to their account
  3. `calculate_loyalty_points`'s decimal constant matches Passet Hub's actual token decimals (confirmed in Phase 2) and produces correct point values for a known price input
  4. A purchase attempt that violates `anti_scalping` price-cap constraints is rejected at the payable purchase call site
**Plans**: TBD

### Phase 4: Frontend & SDK Integration
**Goal**: The Next.js app talks to the Passet Hub contract with H160 addresses and real payment — replacing `@polkadot/api-contract`'s pallet-contracts-only provider with a new pallet-revive-aware one.
**Depends on**: Phase 3 (payable message signatures — starting integration before Phase 3 lands means redoing the purchase call path) and Phase 1's frozen address/metadata encoding
**Requirements**: FRONTEND-01, FRONTEND-02, FRONTEND-03, FRONTEND-04, FRONTEND-05, FRONTEND-06, FRONTEND-07, FRONTEND-08
**Success Criteria** (what must be TRUE):
  1. A full purchase-ticket call completes against the live Passet Hub contract through the new pallet-revive provider (`sdk/reviveContractProvider.ts`), selected via the existing mock/real factory in `sdk/index.ts`, signed by an existing wallet extension (Polkadot.js, Talisman, SubWallet, or Nova), and survives `RevivedAccountMapping`
  2. A freshly deployed contract's H160 address is captured and displayed correctly (not mis-parsed as SS58/AccountId32), and a connected user's H160 contract-side identity is shown in the UI without an extra RPC round-trip
  3. Token amounts (prices, balances) display and submit using `ChainConfig.tokenDecimals` for the connected chain, not hardcoded Westend/WND math
  4. An XCM transfer to an H160 destination builds and submits an `AccountKey20` junction (not `AccountId32`) and succeeds
  5. The three frontend files that referenced deleted legacy contracts (`app/docs/page.tsx`, `ContractDeployment.tsx`, `utils/contractMethods.ts`) are rewritten against the new provider with no remaining legacy-contract references, and mock provider addresses are updated to H160 format
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. On-Chain Module Validation | 0/TBD | Not started | - |
| 2. Passet Hub Testnet Deployment | 0/TBD | Not started | - |
| 3. Payment Enforcement | 0/TBD | Not started | - |
| 4. Frontend & SDK Integration | 0/TBD | Not started | - |
