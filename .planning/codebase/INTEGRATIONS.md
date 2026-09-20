# External Integrations

**Analysis Date:** 2026-09-20

## APIs & External Services

**Polkadot Blockchain RPC:**
- Service: Polkadot.js API via WebSocket RPC endpoints
- Primary Use: Query chain state, send extrinsics, subscribe to events
- Endpoints configured in `frontend/src/config/chains.ts`:
  - Westend Asset Hub (primary testnet): `wss://westend-asset-hub-rpc.polkadot.io`
  - Local dev: `ws://127.0.0.1:9944`
  - Westend relay: `wss://westend-rpc.polkadot.io`
  - Kusama: `wss://kusama-rpc.polkadot.io`
  - Polkadot: `wss://rpc.polkadot.io`
- SDK/Client: `@polkadot/api` (v16.4.6)
- Authentication: Wallet extension (injected signer)
- Implementation: `frontend/src/services/blockchain.ts` (BlockchainService singleton)

**Ink! Smart Contract ABI Metadata:**
- Service: Local ABI JSON loaded from filesystem
- Location: `frontend/src/sdk/abi/inktix.json`
- Used for: Contract interface definition, method encoding/decoding
- Client: `@polkadot/api-contract` (ContractPromise wrapper)
- Implementation: `frontend/src/sdk/contractProvider.ts`

## Data Storage

**Blockchain Storage (Primary):**
- Provider: Polkadot on-chain state
- Storage Type: Key-value pairs in Substrate storage
- Client: Polkadot.js query interface
- Use cases: Events, tickets, venues, teams, artists, balances, resale listings, NFT metadata
- Interaction: `frontend/src/services/blockchain.ts` (query.* calls)
- Smart contract logic: `contracts/inktix/src/storage/contract_storage.rs`

**Browser LocalStorage (Secondary):**
- Provider: Browser localStorage API
- Use cases:
  - Blockchain connection state (`blockchain_connected`, `blockchain_endpoint`)
  - Zustand state persistence (`inktix-store`)
  - Account/wallet preferences
- Cleared on logout; persisted across page reloads
- No sensitive data stored (keys only, not private keys)

**In-Memory State:**
- Provider: Zustand store (`frontend/src/store/`)
- Slices: `connectionSlice`, `walletSlice`, `contractSlice`, `dataSlice`
- Lifecycle: Initialized at app load, migrated from localStorage

**File Storage:**
- Not used. No backend file server. Static assets served via Nginx on VPS.
- Contract builds output to `contracts/inktix/target/ink/inktix.wasm`
- Frontend builds output to static HTML/CSS/JS via Next.js export

**Caching:**
- Query results: Cached in Zustand store (dataSlice)
- Contract metadata: Loaded once, cached in BlockchainService
- No external caching service (Redis, Memcached)

## Authentication & Identity

**Auth Provider:**
- Type: Wallet-based (self-custodial)
- Providers:
  - Polkadot.js browser extension (primary)
  - Talisman wallet
  - SubWallet
  - Nova wallet
- Implementation:
  - Extension discovery: `@polkadot/extension-dapp` (web3Enable)
  - Account enumeration: `web3Accounts()`
  - Signer injection: `web3FromAddress()` (injected signing capability)
- No centralized account database
- No email/password authentication
- Account format: SS58 encoded Polkadot address
- Transaction signing: Delegated to wallet extension via browser bridge

**Authorization:**
- Model: Owner-based (contract deployer is owner)
- Contract-level: `get_owner()` method checks caller identity
- Frontend: UI hides admin functions from non-owner accounts
- Implementation: `frontend/src/sdk/contractProvider.ts` (calls check `selectedAccount`)

## Monitoring & Observability

**Error Tracking:**
- Type: None (no external service)
- Implementation: Client-side console logging
- Browser DevTools inspection for debugging

**Logs:**
- Provider: Browser console (development) and stdout (backend if logging added)
- Approach:
  - `console.log()`, `console.error()` throughout codebase
  - Examples: `frontend/src/services/blockchain.ts` has extensive logging
  - No log aggregation service (Sentry, DataDog, etc.)
- Retention: Browser console history cleared on refresh (browser default)

**Performance Monitoring:**
- Type: None configured
- No external analytics service integrated

## CI/CD & Deployment

**Hosting:**
- Platform: VPS at `135.148.61.99` (Debian Linux, Nginx)
- Deployment method: SSH via deploy script
- Scripts:
  - `./scripts/deploy_inktix_to_rycsprojects.sh` - Build frontend + deploy to VPS
  - `./scripts/deploy-and-configure.sh` - Build contract + deploy + configure frontend
- Atomic swap deployment for zero-downtime updates
- Production URL: `https://rycsprojects.com/inktix/`

**CI Pipeline:**
- Provider: GitHub Actions
- Trigger: Push to `main` or `redesign/polkadot-hub` branches
- Workflow: `.github/workflows/ci.yml`
- Jobs:
  - **contracts**: Rust build, cargo-contract build, unit tests (14 tests)
  - **frontend**: Node.js setup, npm lint, npm build, vitest run
- No artifact upload or release automation configured

**Container Registry:**
- Provider: Docker (local only for development)
- Config: `docker-compose.yml`
- Services: `substrate` (parity contracts node), `frontend` (Next.js dev server)
- Production: Static builds deployed via rsync/SSH (no container orchestration)

**Secrets Management:**
- Location: GitHub repo settings (if any GitHub secrets configured)
- VPS deployments: SSH key-based authentication
- RPC endpoints: Environment variables (no secrets stored in code)
- No integration with HashiCorp Vault, AWS Secrets Manager, or similar

## Webhooks & Callbacks

**Incoming Webhooks:**
- Type: None
- No webhook listeners for external events

**Outgoing Webhooks:**
- Type: None
- No webhooks to external services
- Blockchain events: Subscribed to via Polkadot.js on-chain event stream (not webhooks)

**Blockchain Event Subscriptions:**
- Implementation: `frontend/src/services/blockchain.ts` (listen for contract events)
- Events: Tickets purchased, transfers, resale listings, NFT mints
- Pattern: Polkadot.js query subscriptions (reactive)

## Cross-Chain (XCM) Integration

**Service:** XCM Cross-Chain Transfer
- Protocol: `polkadotXcm.limitedReserveTransferAssets` (V3 MultiLocation)
- Supported Transfers:
  - Westend Asset Hub → Any Polkadot parachain (with XCM support)
  - Fungible assets only (native tokens)
- Implementation: `frontend/src/services/xcm.ts` (limitedReserveTransferAssets function)
- SDK: @polkadot/api (xcm extrinsic construction)
- Use cases: Transfer tickets or event currencies across chains
- Fee handling: Configurable `feeAssetItem` parameter
- Weight limits: Hardcoded as `{ refTime: 6000000000, proofSize: 0 }`

## Environment Configuration

**Required Environment Variables:**

Frontend (`.env.local`, `.env.production`):
- `NEXT_PUBLIC_RPC_ENDPOINT` - RPC WebSocket URL (wss://... format)
- `NEXT_PUBLIC_LOCAL_RPC_ENDPOINT` - Local dev node endpoint (optional)
- `NEXT_PUBLIC_CHAIN_NAME` - Display name (e.g., "Westend AssetHub")
- `NEXT_PUBLIC_CONTRACT_ADDRESS` - Deployed contract address (optional, runtime input if missing)
- `NEXT_PUBLIC_MOCK_MODE` - "true" to enable mock provider, "false" for real
- `NEXT_PUBLIC_BASE_PATH` - Base path for static export (e.g., "/inktix")

**Secrets Location:**
- VPS deployment key: Stored on CI/CD runner or VPS (SSH key auth)
- Polkadot.js signer: Never stored — always in wallet extension (user-controlled)
- No `.env` secrets checked into git (honor .gitignore)

## Smart Contract Deployment

**On-Chain Deployment:**
- Chain: Westend Asset Hub (primary testnet)
- Method: `code.tx.new()` extrinsic via Polkadot.js
- Gas limits: `refTime: 300_000_000_000, proofSize: 1_000_000`
- Endowment: Configurable via UI (minimum required by chain)
- Signer: Connected wallet account
- Implementation: `frontend/src/services/blockchain.ts` (deployContract method)

**Contract Features Deployment:**
- Rust feature flags: `sports`, `concert` (both enabled by default)
- Compiled binary: `contracts/inktix/target/ink/inktix.wasm`
- Metadata: Generated by cargo-contract at build time
- ABI layout: Exported to JSON for frontend SDK

---

*Integration audit: 2026-09-20*
