# Requirements: InkTix v2 Migration

**Defined:** 2026-09-20
**Core Value:** The full ticket-purchase flow — browse, pay with real value, mint the ticket NFT, and let funds be withdrawn — works end-to-end against a live ink! v6 / `pallet-revive` chain (Passet Hub).

## v1 Requirements

Requirements for finishing the v2 migration. Each maps to a phase of `docs/v2_plan.md` (Phases 1d–4).

### On-Chain Validation

- [ ] **VALIDATE-01**: Each contract module (venue/event, ticket purchase + user_tickets, pricing, nft_management, anti_scalping, loyalty/advanced_team_loyalty, season_pass/fantasy_sports/analytics/artist/xcm) is exercised against a live `ink-node`, in dependency order, with pass/fail recorded per module
- [ ] **VALIDATE-02**: All inline unit tests remain green after module validation
- [ ] **VALIDATE-03**: CI is green on `redesign/polkadot-hub`

### Testnet Deployment

- [ ] **DEPLOY-01**: The live Passet Hub endpoint set (Substrate WSS, ETH-RPC if used, chain ID, token symbol, token decimals) is re-confirmed and recorded — not assumed from Phase 0's notes
- [ ] **DEPLOY-02**: A funded deploy account completes the `RevivedAccountMapping` step through a scripted, repeatable process
- [ ] **DEPLOY-03**: The contract is instantiated and callable on Passet Hub via CLI
- [ ] **DEPLOY-04**: A representative subset of the on-chain module validation is re-run against the live Passet Hub chain

### Payment Enforcement

- [ ] **PAYMENT-01**: User's `purchase_ticket` call is rejected when the transferred value doesn't match the price from `calculate_price` (payable, `U256`-compared)
- [ ] **PAYMENT-02**: An owner/organizer can withdraw accumulated contract funds via a dedicated settlement message
- [ ] **PAYMENT-03**: `calculate_loyalty_points`'s decimal assumption matches Passet Hub's actual token decimals
- [ ] **PAYMENT-04**: `anti_scalping` price-cap constraints are checked against the payable purchase path and gate it where applicable

### Frontend / SDK Integration

- [ ] **FRONTEND-01**: A new pallet-revive contract provider (PAPI/ink! SDK) is wired into the existing mock/real factory (`sdk/index.ts`)
- [ ] **FRONTEND-02**: A user can sign a contract call with an existing wallet extension (Polkadot.js, Talisman, SubWallet, or Nova) and have it survive `RevivedAccountMapping`
- [ ] **FRONTEND-03**: Deployment address capture handles H160-hex output correctly (not just AccountId32/SS58)
- [ ] **FRONTEND-04**: A user's H160 contract-side identity is displayable in the UI without an extra RPC round-trip
- [ ] **FRONTEND-05**: Token amounts display and submit correctly using chain-aware decimals (`ChainConfig.tokenDecimals`), not hardcoded Westend/WND values
- [ ] **FRONTEND-06**: XCM transfers to an H160 destination build an `AccountKey20` junction, not `AccountId32`
- [ ] **FRONTEND-07**: The three frontend files referencing deleted legacy contracts (`app/docs/page.tsx`, `ContractDeployment.tsx`, `utils/contractMethods.ts`) are rewritten against the new provider
- [ ] **FRONTEND-08**: Mock provider addresses are updated to H160 format for consistency with live testing

## v2 Requirements

Deferred to a future release — see Out of Scope for the reasoning behind each.

### Payments

- **PAYMENT-05**: Overpayment refund (accept `>=` price, transfer back the excess) instead of reject-on-mismatch

### Marketplace

- **MARKET-01**: Resale ticket purchase (`purchase_resale_ticket`) — buy-side of the resale marketplace

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Production cutover to Polkadot Hub mainnet (Phase 5) | Gated by Polkadot Hub mainnet GA timing, outside this project's control — becomes its own milestone once GA lands |
| Resale marketplace buy-side (v1) | New functionality, not something v1 had that the port needs to restore — v2 is a migration, not a feature expansion. Deferred to v2 Requirements above |
| Overpayment refund handling (v1) | Reject-on-mismatch avoids the reentrancy-ordering surface a refund transfer would introduce, with no reentrancy guards in place anywhere yet. Deferred to v2 Requirements above |
| viem/ethers + MetaMask-style wallets | Noted in `docs/v2_plan.md` as a possible later addition, not the primary path — PAPI/ink! SDK over the existing wallet extensions preserves current UX and avoids two parallel wallet models |
| New product features beyond payment enforcement | This is a toolchain/runtime migration — v1's feature set carries over as-is, no general feature expansion |

## Traceability

Populated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| VALIDATE-01 | — | Pending |
| VALIDATE-02 | — | Pending |
| VALIDATE-03 | — | Pending |
| DEPLOY-01 | — | Pending |
| DEPLOY-02 | — | Pending |
| DEPLOY-03 | — | Pending |
| DEPLOY-04 | — | Pending |
| PAYMENT-01 | — | Pending |
| PAYMENT-02 | — | Pending |
| PAYMENT-03 | — | Pending |
| PAYMENT-04 | — | Pending |
| FRONTEND-01 | — | Pending |
| FRONTEND-02 | — | Pending |
| FRONTEND-03 | — | Pending |
| FRONTEND-04 | — | Pending |
| FRONTEND-05 | — | Pending |
| FRONTEND-06 | — | Pending |
| FRONTEND-07 | — | Pending |
| FRONTEND-08 | — | Pending |

**Coverage:**
- v1 requirements: 19 total
- Mapped to phases: 0
- Unmapped: 19 ⚠️ (populated by roadmap creation)

---
*Requirements defined: 2026-09-20*
*Last updated: 2026-09-20 after initialization*
