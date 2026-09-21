# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

InkTix is a decentralized event ticketing platform built on the Polkadot ecosystem. It combines a Next.js frontend with ink! smart contracts (Rust) for on-chain ticket management, cross-chain transfers via XCM, and wallet-based authentication.

## Build & Run Commands

### Deployment
```bash
./scripts/deploy_inktix_to_rycsprojects.sh  # Build frontend + deploy to VPS via SSH
./scripts/deploy-and-configure.sh            # Build contract + deploy to local node + configure frontend
```

### Local Substrate Node (standalone)
```bash
docker run --rm -p 9944:9944 parity/substrate-contracts-node:latest --dev --tmp --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe
```

## Git & Docs Workflow

- **Push on every commit.** This project has no per-phase branches (`git.branching_strategy: "none"` in `.planning/config.json`) — GSD planning and execution commits land directly on `redesign/polkadot-hub`. After any commit (including automated GSD `docs(...)`/`chore(...)` commits), immediately `git push` so `origin/redesign/polkadot-hub` stays in sync. Don't batch pushes up for later.
- **Keep README.md current.** At the end of each completed phase (execution, not planning), refresh README.md's version/test-count badges and the "v2 Rearchitecture (In Progress)" section's status line to match the latest phase status. Use `docs/v2_plan.md`'s `## Status` table as the source of truth. Match the existing badge/table/section style — this is a content refresh, not a redesign.

## Architecture

### v2 Rearchitecture (in progress)

The project is migrating from ink! 5.1.1 / `pallet-contracts` (Westend Asset Hub) to ink! v6 / `pallet-revive` (Passet Hub → Polkadot Hub), on branch `redesign/polkadot-hub`. **`docs/v2_plan.md` is the living source of truth** for this migration — architecture rationale, phase-by-phase status, decisions, issues, and resolutions. Read it before doing any v2-related work, and update it at the end of every phase (don't let findings live only in commit messages or conversation — they get lost across sessions otherwise).

### Frontend (`frontend/src/`)
- **Next.js 15 with App Router** — static export mode (`output: "export"`, base path `/inktix`)
- **Zustand Store** (`store/`) — replaces BlockchainContext. Slices: `connectionSlice`, `walletSlice`, `contractSlice`, `dataSlice`. Single localStorage key `inktix-store` with migration from old keys.
- **useBlockchain hook** (`hooks/useBlockchain.ts`) — backward-compatible wrapper around Zustand store
- **BlockchainService** (`services/blockchain.ts`) — singleton managing Polkadot.js API connection
- **Smart Contract Components** (`components/smart-contracts/`) — decomposed from monolithic SmartContractManager into: `ContractStatus`, `ContractDeployment`, `ContractInteraction`, `ContractRegistry`, `CrossChainPanel`, `ContractInfo`, composed by `index.tsx`
- **SDK Layer** (`sdk/`) — typed contract interface (`InkTixSDK`) with `MockProvider` and `ContractProvider` implementations. Factory in `index.ts` returns mock or real based on `NEXT_PUBLIC_MOCK_MODE`.
- **Chain Config** (`config/chains.ts`) — single source of truth for all chain/endpoint config. Replaces hardcoded endpoints.
- **XCM Service** (`services/xcm.ts`) — handles `limitedReserveTransferAssets` using V3 MultiLocation format
- **Utilities** (`utils/contractMethods.ts`, `utils/methodArgs.ts`) — extracted from SmartContractManager

### Environment Configuration
- `frontend/.env.local` — local dev config (`NEXT_PUBLIC_MOCK_MODE=true`)
- `frontend/.env.production` — production config (Westend AssetHub)
- Key variables: `NEXT_PUBLIC_RPC_ENDPOINT`, `NEXT_PUBLIC_MOCK_MODE`, `NEXT_PUBLIC_CONTRACT_ADDRESS`

### Key Design Decisions
- **No backend server** — contracts are the source of truth; frontend is a static site served by Nginx
- **Wallet-based auth** — supports Polkadot.js, Talisman, SubWallet, Nova extensions
- **Primary testnet**: Westend Asset Hub
- **Tailwind CSS** with custom theme: `inktix-blue`, `inktix-orange`, `inktix-purple` color tokens
- **Path alias**: `@/*` maps to `src/*` in TypeScript

### Deployment Target
- VPS at `135.148.61.99` (Debian, Nginx)
- Deploy script does atomic swap for zero-downtime updates
- Production URL: `https://rycsprojects.com/inktix/`
