# Technology Stack

**Analysis Date:** 2026-09-20

## Languages

**Primary:**
- TypeScript 5.9.2 - Frontend (React/Next.js application)
- Rust (nightly-2026-01-15) - Smart contracts (ink! v6)

**Secondary:**
- JavaScript - Build configuration and scripts
- WASM (WebAssembly) - Compiled output from Rust contracts

## Runtime

**Environment:**
- Node.js 20+ (development)
- Browser (modern WebGL-capable browser for frontend)
- Polkadot Substrate Runtime (network layer via RPC)

**Package Manager:**
- npm (Node.js package manager)
- Lockfile: `frontend/package-lock.json` (present, committed)

## Frameworks

**Core:**
- Next.js 15.5.0 - Full-stack React framework (static export mode)
- React 19.1.1 - UI component library
- ink! 6.0.0-beta.1 - Smart contract framework (Rust)

**Testing:**
- Vitest 3.2.4 - Unit test runner (jsdom environment)
- @playwright/test 1.58.2 - E2E test framework
- ink_e2e 6.0.0-beta.2 - Smart contract end-to-end tests

**Build/Dev:**
- cargo-contract 6.0.0-beta.2 - Smart contract build tool (Rust)
- Webpack (via Next.js) - JavaScript bundler
- TypeScript 5.9.2 - Type checking
- PostCSS 8.5.6 - CSS transformation
- Autoprefixer 10.4.21 - CSS vendor prefixing

## Key Dependencies

**Blockchain/Polkadot.js:**
- @polkadot/api 16.4.6 - Polkadot.js core API client
- @polkadot/api-contract 16.5.4 - Contract interaction library
- @polkadot/extension-dapp 0.61.6 - Wallet extension bridge (dApp side)
- @polkadot/extension-inject 0.61.6 - Wallet extension types
- @polkadot/types 16.4.6 - Type definitions for Polkadot runtime
- @polkadot/util 13.5.6 - Polkadot.js utility functions
- @polkadot/util-crypto 13.5.6 - Cryptographic utilities
- @polkadot/wasm-crypto 7.5.1 - WASM crypto implementation

**State Management:**
- zustand 5.0.11 - Lightweight state management with persistence

**UI Components:**
- lucide-react 0.542.0 - Icon library
- class-variance-authority 0.7.1 - Component variant utilities
- clsx 2.1.1 - Utility for combining CSS classes
- qrcode.react 4.2.0 - QR code generation

**Styling:**
- tailwindcss 3.4.17 - Utility-first CSS framework
- @tailwindcss/aspect-ratio 0.4.2 - Aspect ratio plugin
- @tailwindcss/forms 0.5.10 - Form styling plugin
- @tailwindcss/typography 0.5.16 - Typography plugin

**Development/Testing:**
- @testing-library/react 16.3.2 - React component testing utilities
- @testing-library/jest-dom 6.9.1 - DOM matchers
- jsdom 25.0.1 - DOM implementation for Node.js
- @typescript-eslint/eslint-plugin 8.41.0 - TypeScript linting rules
- @typescript-eslint/parser 8.41.0 - TypeScript parser for ESLint
- eslint 9.34.0 - JavaScript linter
- eslint-config-next 15.5.2 - Next.js ESLint config

**Types:**
- @types/node 24.3.0 - Node.js type definitions
- @types/react 19.1.11 - React type definitions
- @types/react-dom 19.1.8 - React DOM type definitions

## Configuration

**Environment:**
- Environment configuration via `.env.local`, `.env.development`, `.env.production`
- Key variables:
  - `NEXT_PUBLIC_RPC_ENDPOINT` - Blockchain RPC WebSocket URL
  - `NEXT_PUBLIC_LOCAL_RPC_ENDPOINT` - Local dev node endpoint
  - `NEXT_PUBLIC_CHAIN_NAME` - Chain identifier (e.g., "Westend AssetHub")
  - `NEXT_PUBLIC_CONTRACT_ADDRESS` - Pre-deployed contract address
  - `NEXT_PUBLIC_MOCK_MODE` - Enable mock provider for development
  - `NEXT_PUBLIC_BASE_PATH` - Base path for static export (e.g., "/inktix")

**Build:**
- `frontend/next.config.js` - Next.js configuration (static export, base path support)
- `frontend/tsconfig.json` - TypeScript compiler configuration
- `frontend/.eslintrc.json` - ESLint rules
- `contracts/inktix/rust-toolchain.toml` - Rust toolchain pinning (nightly-2026-01-15)
- `contracts/inktix/Cargo.toml` - Rust dependencies and features
- `frontend/vitest.config.ts` - Unit test configuration
- `frontend/playwright.config.ts` - E2E test configuration
- `.github/workflows/ci.yml` - GitHub Actions CI pipeline

**Tailwind CSS:**
- Custom theme colors: `inktix-blue`, `inktix-orange`, `inktix-purple`
- Configured in `frontend/src/app/layout.tsx` or tailwind config
- Output CSS mode with static export

## Platform Requirements

**Development:**
- Node.js 22+ (tested with v25.2.1)
- Rust 1.92+ with nightly toolchain
- `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- Polkadot wallet extension (Polkadot.js, Talisman, SubWallet, Nova)
- Modern browser with WebSocket support

**Production:**
- Deployment target: VPS at 135.148.61.99 (Debian, Nginx)
- Production URL: `https://rycsprojects.com/inktix/`
- Requires RPC endpoint connectivity to Westend Asset Hub
- Static files served via Nginx (no backend server required)

## Tooling

**CI/CD:**
- GitHub Actions workflow (``.github/workflows/ci.yml`)
- Runs on: `ubuntu-latest`
- Contract build: `cargo contract build`
- Contract tests: `cargo test` (14 unit tests)
- Frontend build: `npm run build`
- Frontend tests: `npx vitest run` and `next lint`

**Local Development:**
- Docker Compose support (`docker-compose.yml`)
- Parity Substrate contracts node (via Docker)
- Hot reload via `npm run dev` on port 3000

---

*Stack analysis: 2026-09-20*
