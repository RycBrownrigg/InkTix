# Phase 1: On-Chain Module Validation - Context

**Gathered:** 2026-09-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Every contract module (venue/event, ticket purchase + user_tickets, pricing,
nft_management, anti_scalping, loyalty/advanced_team_loyalty, season_pass/
fantasy_sports/analytics/artist/xcm) is exercised against a live local
`ink-node`, in the dependency order fixed by `docs/v2_plan.md`'s Phase 1d
list, closing the gap between "compiles and passes off-chain unit tests"
(already done, Phase 1a/1c) and "actually works on-chain." Any bugs
surfaced are fixed within this phase; unit tests and CI must stay green
throughout.

Input-validation hardening (empty strings, zero IDs, bounds checks) is
explicitly Phase 3 scope (see PROJECT.md "Active" list and CONCERNS.md) —
not re-litigated here except where an "edge case" is actually about
on-chain runtime behavior rather than input sanitization (see Depth
decision below).

</domain>

<decisions>
## Implementation Decisions

### Validation Harness
- **D-01:** Use automated `#[ink_e2e::test]` Rust tests per module, not manual `cargo contract call` CLI invocations. The `ink_e2e = "6.0.0-beta.2"` dev-dependency already exists in `contracts/inktix/Cargo.toml` but is currently unused — this phase is what wires it up. — **Reversibility:** reversible — CLI-based validation could still be done ad hoc later; no code path depends on this choice, it only shapes how Phase 1's validation work gets written.
- **D-02:** Tests run against `scripts/dev-node.sh` (already exists, downloads/runs `ink-node --dev --tmp`) — no new node tooling needed.

### Recording
- **D-03:** Record per-module pass/fail results and any bugs found directly in `docs/v2_plan.md`'s existing Phase 1d module checklist (in place), since that file is the project's living source of truth per `.claude/CLAUDE.md`. Do not create a separate VALIDATION.md. — **Reversibility:** reversible.

### Validation Depth (risk-tiered)
- **D-04:** Low-risk modules (venue/event, nft_management, season_pass_management, fantasy_sports_management, analytics, artist_management, xcm_management) get happy-path validation only — one clean write/read round-trip per module is sufficient proof of on-chain correctness.
- **D-05:** The three flagged-risky modules get targeted **on-chain runtime** edge cases (not input-validation edge cases):
  - `pricing` — exercise the time-multiplier logic under `ink-node`'s manual-seal block production (blocks authored per-transaction, not on a timer) — v2_plan.md explicitly flags this as where "surprises" are expected.
  - `loyalty` / `advanced_team_loyalty` — double-`stake_on_team()` / restake on the same composite `Mapping<(Address, u32), TeamLoyaltyProfile>` key, not just a single write — this is the single riskiest storage item per CONCERNS.md.
  - `anti_scalping` — append to the `Vec<Address>` blacklist/whitelist more than once and re-check membership, to prove the on-chain Vec round-trip holds under repeated mutation, not just a single push.
- **D-06:** Explicitly do NOT add input-sanitization edge cases (empty strings, zero IDs, out-of-range values) in this phase for any module — that work is Phase 3's, and pulling it forward would duplicate effort once Phase 3 revisits these same entry points for payment enforcement.

### CI Scope
- **D-07:** On-chain validation (the new `ink_e2e` tests) stays a manual/local step for this phase — it is NOT wired into `.github/workflows/ci.yml`. The existing CI job (off-chain `cargo test` + `cargo contract build`) is what Phase 1's success criterion #3 ("CI is green") refers to; standing up a live `ink-node` in CI is new infrastructure work that should be scoped deliberately later, not folded in here. — **Reversibility:** reversible — CI integration can be added in a later phase without touching the tests themselves.

### Claude's Discretion
- Exact `#[ink_e2e::test]` file/module organization (e.g., one test file per contract module vs. grouped by risk tier) is left to the planner/executor, following the existing Rust test conventions in `contracts/inktix/src/tests/`.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Migration plan and validation scope (authoritative)
- `docs/v2_plan.md` §"1d — Behavioral validation, module by module" — locks the module order, the manual-seal timing risk on `pricing`, the composite-key risk on `loyalty`, and the Phase 1 exit gate (all modules validated, unit tests green, blob size recorded, CI green). This is where D-03's recording happens.
- `docs/v2_plan.md` §Phase 2 — confirms Phase 2 will "re-run a representative subset of Phase 1d's module validation against the live chain," which is why D-01 (automated ink_e2e tests over manual CLI) matters for reusability.

### Codebase concerns (background/rationale)
- `.planning/codebase/CONCERNS.md` §"Composite Key Storage Layout Risk" — the `stake_on_team()` round-trip risk behind D-05.
- `.planning/codebase/CONCERNS.md` §"Phase 1 Behavioral Validation Incomplete" — enumerates all 7 module groups and the specific on-chain-vs-off-chain risk classes (storage layout, Address/H160 serialization, timing assumptions).
- `.planning/codebase/CONCERNS.md` §"No Input Validation" — the input-validation gap explicitly deferred out of this phase (D-06).

### Project scope guardrails
- `.planning/PROJECT.md` — "Active" requirements list confirms input-validation hardening is Phase 3 scope, not Phase 1.
- `.planning/ROADMAP.md` §Phase 1 — the three success criteria this phase must satisfy (module validation with explicit `stake_on_team()` confirmation; unit tests green; CI green).

### Toolchain
- `scripts/dev-node.sh` — the local `ink-node` launcher these tests run against; no changes needed.
- `contracts/inktix/rust-toolchain.toml` — pins `nightly-2026-01-15`; do not change without a full round-trip re-validation per CONCERNS.md's "Ink! v6 Beta Toolchain" entry.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ink_e2e = "6.0.0-beta.2"` dev-dependency — already declared in `contracts/inktix/Cargo.toml`, currently unused. This phase is what activates it.
- `contracts/inktix/src/tests/core_tests.rs`, `sports_tests.rs`, `concert_tests.rs` — existing off-chain `#[ink::test]` helper pattern (e.g., `CoreTestHelpers`) to follow stylistically for new `ink_e2e` test organization.
- `contracts/inktix/Cargo.toml` has an existing but unused `e2e-tests = []` feature flag — leftover scaffolding, not currently wired to anything; the planner should decide whether to use it to gate the new e2e tests or leave default-feature-gated.

### Established Patterns
- Contract logic is organized by domain under `contracts/inktix/src/logic/{core,sports,concert}/` — one file per module (e.g., `pricing.rs`, `anti_scalping.rs`, `advanced_team_loyalty.rs`), matching the module list this phase validates.
- All contract methods return `Result<T, String>`, checked with `?` — `ink_e2e` tests will need to unwrap both the outer `Result<Result<T, E>, LangError>` (revive dispatch) and the inner contract-level `Result`.

### Integration Points
- `contracts/inktix/src/lib.rs` — the single `#[ink(message)]` surface (61 messages) that `ink_e2e` tests call into; no per-module contract entry points exist separately.
- `.github/workflows/ci.yml` — the `contracts` job (`cargo test` + `cargo contract build`) is what stays green per D-07; not modified in this phase.

</code_context>

<specifics>
## Specific Ideas

No particular UI/UX or visual references — this is a backend/contract validation phase with no frontend surface.

</specifics>

<deferred>
## Deferred Ideas

- **Wiring on-chain validation into CI** — explicitly deferred per D-07. Revisit once the `ink_e2e` tests exist and have proven stable locally; likely candidate for a later infrastructure phase or a Phase 2 follow-up once Passet Hub deployment raises the stakes on regression coverage.
- **Input-validation hardening across contract modules** — explicitly deferred per D-06 to Phase 3, which already owns this per PROJECT.md.

### Reviewed Todos (not folded)
None — no pending todos matched this phase (`todo.match-phase` returned 0 matches).

</deferred>

---

*Phase: 1-On-Chain Module Validation*
*Context gathered: 2026-09-20*
