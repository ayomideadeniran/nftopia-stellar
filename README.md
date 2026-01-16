<p align="center">
  <img src="nftopia-frontend/public/nftopia-04.svg" alt="NFTopia" width="420" />
</p>

# NFTopia Monorepo

NFTopia is a multi-app, cross-platform project for creating, managing, and trading NFTs. This monorepo includes:
- A Next.js web frontend
- A Nest.js backend API (Starknet integration)
- A React Native mobile app (Expo)
- Soroban (Stellar) smart contracts for on-chain NFT operations

This README provides a project-level overview, setup instructions, and development guidance across all components.

## Overview
- Purpose: Deliver a full-stack NFT platform with secure minting, marketplace features, and rich user experiences.
- Chains: Actively integrates with Starknet (frontend/backend). Soroban (Stellar) contracts are included for current and future on-chain capabilities.
- Apps: Web (Next.js), Backend (Nest.js), Mobile (Expo RN), On-chain Contracts (Rust + Soroban).

## Repository Structure
```
nftopia-stellar/
├── nftopia-frontend/     # Next.js web app (i18n, wallet, marketplace UI)
├── nftopia-backend/      # Nest.js API (auth, NFTs, collections, Starknet)
├── nftopia-mobile-app/   # Expo React Native app (ArgentX/Braavos integration)
└── nftopia-stellar/      # Soroban (Stellar) smart contracts (Rust)
```

## Tech Stack
- Frontend: `Next.js`, `Tailwind`, `zustand`, `@starknet-react`
- Backend: `Nest.js`, `TypeORM`, `PostgreSQL`, `BullMQ`, `JWT`, `Starknet.js`
- Mobile: `React Native` + `Expo`, NativeWind
- On-chain (Stellar): `Rust`, `Soroban`, `soroban-sdk`

## Prerequisites
- Node.js v18+
- pnpm (recommended) or npm
- PostgreSQL 14+ (backend)
- Redis (backend queues)
- Starknet wallet (ArgentX/Braavos) for web/mobile testing
- Rust 1.75+, Soroban CLI (contracts)

## Environment Configuration
- Frontend: `apps/frontend/.env.local` (see `nftopia-frontend/.env.example` if present)
- Backend: `apps/backend/.env` (copy from `nftopia-backend/.env.example`)
  - Required: `STARKNET_RPC_URL`, `STARKNET_NFT_CONTRACT`, `STARKNET_MARKETPLACE_CONTRACT`, `STARKNET_AUCTION_CONTRACT`, `STARKNET_ACCOUNT_ADDRESS`, `STARKNET_PRIVATE_KEY`, `JWT_SECRET`, DB vars
- Mobile: `nftopia-mobile-app/.env` (copy from `.env.example`)
- Soroban: `nftopia-stellar/.env` (copy `.env.example`)

## Quick Start

### Frontend (Next.js)
- Directory: `nftopia-frontend`
- Commands:
  - Install: `pnpm install`
  - Dev: `pnpm dev` (defaults to port `5000`)
  - Build: `pnpm build`
  - Start: `pnpm start`
  - Tests: `pnpm test`
  - Validate i18n: `pnpm validate-translations`
- Features:
  - i18n with EN/FR/ES/DE
  - Starknet wallet connection (ArgentX/Braavos)
  - Creator dashboard with minting UI and file upload

### Backend (Nest.js)
- Directory: `nftopia-backend`
- Setup:
  - Ensure PostgreSQL and Redis are running
  - Install: `pnpm install`
  - Copy env: `cp .env.example .env` and configure values
- Commands:
  - Dev: `pnpm start:dev` (typical Nest script; alternatively `pnpm start` if configured)
  - Tests: `pnpm test`
- Highlights:
  - JWT-based authentication and nonce signing (Starknet typed data)
  - NFT mint endpoints (upload to Firebase, metadata to IPFS via NFT.Storage)
  - Collections, transactions, auctions, stats
  - Event listeners for Starknet contracts (marketplace, auction, transaction)

#### Common API Endpoints
- Mint NFT via file upload:
  - `POST /nfts/mint/:userId/:collectionId`
  - Multipart form with `file`; JSON fields: `title`, `description`, `price`, `currency`
- Mint NFT from image URL (JWT required):
  - `POST /nfts/mint/from-url?collectionId=<id>`
  - JSON body: `{ "title": "...", "description": "...", "imageUrl": "...", "price": 1.23, "currency": "STK" }`

### Mobile App (Expo React Native)
- Directory: `nftopia-mobile-app`
- Commands:
  - Install: `pnpm install`
  - Start: `pnpm start`
  - Android: `pnpm android`
  - iOS: `pnpm ios`
- Features:
  - Wallet connectivity (ArgentX/Braavos)
  - Mobile-optimized minting and marketplace browsing

### Soroban Contracts (Stellar)
- Directory: `nftopia-stellar`
- Purpose: On-chain contracts for NFT minting, management, marketplace escrow, royalties, and analytics hooks.
- Setup:
  - Install Rust and WASM target: `rustup target add wasm32-unknown-unknown`
  - Soroban CLI: `cargo install --locked --git https://github.com/stellar/rs-soroban-cli`
  - Env: copy `.env.example` to `.env`
- Build: `cargo build --target wasm32-unknown-unknown --release`
- Local RPC:
  - `soroban rpc serve --network testnet --hostname 127.0.0.1 --port 8000`
- Deploy example:
  - `soroban contract deploy --wasm target/wasm32-unknown-unknown/release/<contract>.wasm --source WALLET_SECRET_SEED --network testnet --rpc-url $SOROBAN_RPC_URL`
- Test suite:
  - Per-contract: `cargo test`
  - Workspace: `cargo test --all`

## Authentication Flow (Web)
- Connect wallet (ArgentX/Braavos)
- Backend issues a nonce
- User signs typed data message with nonce
- Backend verifies signature and issues JWT

## Internationalization
- Locales: EN, FR, ES, DE
- Validation script: `pnpm validate-translations`
- Locale routing: `/[locale]/...`

## Testing & QA
- Frontend: `jest`, React Testing Library; run `pnpm test`
- Backend: `jest`; run `pnpm test`
- Soroban: `cargo test` per contract or `--all` for workspace

## Development Tips
- Keep envs in sync with backend validation (`src/config/validation.ts`)
- For Starknet listeners, ensure required `STARKNET_*` vars are set
- Use the provided scripts and avoid modifying unrelated modules

## Contributing
- Fork and branch: `git checkout -b feat/your-feature`
- Follow Conventional Commits
- Add tests for changes
- Open PRs with clear scope and benchmarks if applicable

## License
MIT License. See individual package LICENSE files where applicable.

## Support
- Discord: `discord.gg/nftopia`
- Issues: open in the appropriate subproject directory