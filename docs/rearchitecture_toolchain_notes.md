# ink! v6 / pallet-revive toolchain notes (Phase 0)

Findings from validating the local build/deploy toolchain before writing real contract
code, per the v2 rearchitecture plan (branch `redesign/polkadot-hub`). All confirmed
working as of 2026-07-27.

## Install

```bash
rustup component add rust-src
cargo install --force --locked --version 6.0.0-beta.2 cargo-contract
```

`ink-node` (local dev chain) is a prebuilt binary, not a cargo install:
```bash
curl -sL -o ink-node.tar.gz \
  https://github.com/use-ink/ink-node/releases/download/v0.47.0/ink-node-mac-universal.tar.gz
tar xzf ink-node.tar.gz
xattr -dr com.apple.quarantine ink-node-mac/ink-node   # macOS Gatekeeper
./ink-node-mac/ink-node --dev --tmp                     # RPC on ws://127.0.0.1:9944
```

## Version pinning gotcha

`cargo-contract` is versioned `6.0.0-beta.2`, but the `ink` **library crate** on
crates.io only goes up to `6.0.0-beta.1` — they're versioned independently. Use
`ink = "6.0.0-beta.1"` in contract `Cargo.toml`s (this is what `cargo contract new`
scaffolds by default). Ignore the "not compatible" warning cargo-contract prints on
every command — it's a false positive comparing these two independent version numbers.

## Real blocker found: `-Zjson-target-spec`

Building against today's default nightly toolchain fails:
```
error: `.json` target specs require -Zjson-target-spec
```
This is a nightly Rust/Cargo strictness change (upstream PR rust-lang/cargo#16557)
that landed **after** cargo-contract v6.0.0-beta.2 was released (Jan 29 2026) and
**after** ink!'s maintaining team stopped active development (Jan 2026) — so nobody
has patched cargo-contract to pass the now-required flag. This is exactly the kind of
frozen-tooling friction flagged as a risk going into this migration.

**Workaround**: pin an older nightly toolchain from before the change landed, and
invoke cargo-contract through it:
```bash
rustup toolchain install nightly-2026-01-15 --profile minimal --component rust-src
cargo +nightly-2026-01-15 contract build --release
cargo +nightly-2026-01-15 contract instantiate --suri //Alice --url ws://127.0.0.1:9944 --args <...>
cargo +nightly-2026-01-15 contract call --contract <addr> --message <msg> --suri //Alice --url ws://127.0.0.1:9944 [-x] [--value <amount>]
```
Every real contract build/deploy/call command for this project should use
`+nightly-2026-01-15` until/unless a fixed cargo-contract release appears (unlikely
given the team's frozen status — treat this pin as long-term, not temporary).

## Confirmed v6 API changes (beyond what the migration docs already state)

- `self.env().transferred_value()` returns `ink::U256`, not `Balance`/`u128`. Any
  payable message doing payment-amount comparisons needs `U256`-aware arithmetic.
  This directly affects the `ticket-nft` contract's planned payment-enforcement
  redesign (Phase 3) — `dynamic_price` comparisons need to go through `U256`, not
  the current `u128`.
- `U256` is not in the `#[ink::contract]` module's implicit prelude — needs an
  explicit `ink::U256` path (or an import) inside contract code.
- Deployed contract addresses are `H160`-shaped Ethereum-style addresses
  (e.g. `0x5801b439a678d9d3a68b8019da6a4abfa507de11`), confirming the migration
  docs' note that contract identity moved from `AccountId` to `Address`/`H160`.
- First call from a fresh dev account triggers an implicit "map this account"
  prompt/extrinsic (`RevivedAccountMapping` — pallet-revive maps Substrate
  `AccountId`s to `H160` addresses on first use). Automate past this with
  `--skip-confirm` locally; on Passet Hub this may need to be scripted explicitly
  for a funded testnet account before Phase 2's deploy.

## Testnet target correction

The officially recommended Revive testnet is branded **Passet Hub**, not "Paseo Hub"
as assumed earlier in planning — `wss://testnet-passet-hub.polkadot.io`. Use this name
in `config/chains.ts` and docs going forward (chain ID 420420417 per earlier research
is unaffected, just the display name).

## Phase 0 gate: PASSED

Full round-trip confirmed against local `ink-node`: build → instantiate
(constructor arg `true`) → payable `deposit` tx (real balance transfer, 1 UNIT
Alice → contract) → state-changing `flip` tx → `get` query correctly returned
`false` (i.e. the flip was durably applied on-chain). Throwaway contract lived in
the session scratchpad, not committed to this repo.
