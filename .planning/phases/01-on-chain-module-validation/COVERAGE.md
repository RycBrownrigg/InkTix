# Phase 1: API Coverage Decision

No external API integration: this phase wires up `ink_e2e` (an already-declared
dev-dependency of the contract crate) and runs its tests against a local `ink-node`
dev chain launched by `scripts/dev-node.sh` on `ws://127.0.0.1:9944` — a local dev-node
test harness, not a third-party API/SDK/service. No new external dependency is added,
no package-manager install occurs, `contracts/inktix/Cargo.toml` is in no plan's
`files_modified`, and no network service outside localhost is called by any task.

*Scanned: ROADMAP.md § Phase 1, 01-CONTEXT.md, 01-RESEARCH.md, and all three PLAN.md
bodies. 2026-09-20.*
