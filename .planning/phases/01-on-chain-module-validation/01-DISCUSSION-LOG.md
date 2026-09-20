# Phase 1: On-Chain Module Validation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-20
**Phase:** 1-On-Chain Module Validation
**Areas discussed:** Validation Harness, Recording, Validation Depth, CI Scope

---

## Validation Harness

| Option | Description | Selected |
|--------|-------------|----------|
| ink_e2e automated tests | Write `#[ink_e2e::test]` functions per module. Repeatable, re-runnable for Phase 2's "representative subset" requirement, can be wired into CI later. More upfront effort per module. | ✓ |
| Manual CLI calls | Use `cargo contract call` against `dev-node.sh` directly, as v2_plan.md's prose implies. Fast to start, but not repeatable. | |
| Hybrid | Manual CLI for cheap modules, ink_e2e for risky ones. | |

**User's choice:** ink_e2e automated tests
**Notes:** `ink_e2e = "6.0.0-beta.2"` was already an unused dev-dependency in `contracts/inktix/Cargo.toml` — this phase activates it rather than leaving it dead weight.

---

## Recording

| Option | Description | Selected |
|--------|-------------|----------|
| Update docs/v2_plan.md directly | Check off each module in the existing Phase 1d list in-place, since that file is the project's living source of truth per CLAUDE.md. | ✓ |
| New VALIDATION.md doc | A dedicated results doc kept separate from the narrative plan file. | |
| Just test output / CI | Rely on ink_e2e test names/pass-fail status as the record. | |

**User's choice:** Update docs/v2_plan.md directly
**Notes:** None.

---

## Validation Depth

| Option | Description | Selected |
|--------|-------------|----------|
| Risk-tiered | Happy path for low-risk modules; targeted ON-CHAIN edge cases (not input validation) for pricing (manual-seal timing), loyalty (composite-key double-stake), anti_scalping (Vec round-trip under repeated pushes). | ✓ |
| Happy path only, everywhere | Fastest; defers all edge exploration, including timing/storage-layout risk, to later phases. | |
| Happy path + edge cases, everywhere | Most thorough, but pulls Phase 3's input-validation hardening forward, duplicating effort. | |

**User's choice:** Risk-tiered
**Notes:** This question was reframed mid-discussion. The initial framing ("happy path only" vs "happy path + edge cases") conflated two different things: input-sanitization edge cases (explicitly Phase 3 scope) and on-chain-runtime-behavior edge cases (squarely Phase 1's job, since v2_plan.md itself flags manual-seal timing and composite-key risk as areas to expect surprises). The user asked for this distinction to be clarified before choosing, which produced the risk-tiered framing.

---

## CI Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Manual/local only | CI's existing green bar (off-chain cargo test + build) is what Phase 1's success criterion #3 refers to. Standing up ink-node in CI is new infra work, scoped deliberately later. | ✓ |
| Add to CI now | Stand up ink-node in a CI job and run the new ink_e2e tests there too. | |

**User's choice:** Manual/local only
**Notes:** None.

---

## Claude's Discretion

- Exact `#[ink_e2e::test]` file/module organization (one file per module vs. grouped by risk tier) — left to planner/executor, following the existing `contracts/inktix/src/tests/` conventions.
- Whether to use the existing unused `e2e-tests` Cargo feature flag to gate the new tests, or leave them under default features — left to planner/executor.

## Deferred Ideas

- Wiring on-chain validation into CI (deferred per CI Scope decision) — candidate for a later infrastructure phase or a Phase 2 follow-up.
- Input-validation hardening across contract modules (deferred per Validation Depth decision) — already owned by Phase 3 per PROJECT.md.
