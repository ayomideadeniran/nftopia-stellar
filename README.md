<p align="center">
  <img src="nftopia-frontend/public/nftopia-04.svg" alt="NFTopia" width="420" />
</p>

# NFTopia Monorepo

NFTopia is a multi-app, cross-platform project for creating, managing, and trading NFTs. This monorepo includes:
- A Next.js web frontend
- A Nest.js backend API (Stellar/Soroban integration)
- A React Native mobile app (Expo)
- Soroban (Stellar) smart contracts for on-chain NFT operations

This README provides a project-level overview, setup instructions, and development guidance across all components.

## Overview
- Purpose: Deliver a full-stack NFT platform with secure minting, marketplace features, and rich user experiences.
- Chains: Primary chain is Stellar with Soroban smart contracts. Migration is underway to Stellar across all apps.
- Apps: Web (Next.js), Backend (Nest.js), Mobile (Expo RN), On-chain Contracts (Rust + Soroban).

## Repository Structure
```
nftopia-stellar/
├── nftopia-frontend/     # Next.js web app (i18n, wallet, marketplace UI)
├── nftopia-backend/      # Nest.js API (auth, NFTs, collections, Stellar/Soroban)
├── nftopia-mobile-app/   # Expo React Native app (Stellar wallet integration)
└── nftopia-stellar/      # Soroban (Stellar) smart contracts (Rust)
```

## Tech Stack
- Frontend: `Next.js`, `Tailwind`, `zustand`
- Backend: `Nest.js`, `TypeORM`, `PostgreSQL`, `BullMQ`, `JWT`, `stellar-sdk`
- Mobile: `React Native` + `Expo`, NativeWind
- On-chain (Stellar): `Rust`, `Soroban`, `soroban-sdk`

## Prerequisites
- Node.js v18+
- pnpm (recommended) or npm
- PostgreSQL 14+ (backend)
- Redis (backend queues)
- Stellar wallet for web testing (Freighter). Mobile can deep-link to Stellar wallets (e.g., xBull, Lobstr).
- Rust 1.75+, Soroban CLI (contracts)

## Environment Configuration
- Frontend: `apps/frontend/.env.local` (see `nftopia-frontend/.env.example` if present)
- Backend: `apps/backend/.env` (copy from `nftopia-backend/.env.example`)
  - Required: `SOROBAN_RPC_URL`, `STELLAR_NETWORK`, `SOROBAN_NFT_CONTRACT_ID`, `SOROBAN_MARKETPLACE_CONTRACT_ID`, `SOROBAN_AUCTION_CONTRACT_ID`, `STELLAR_ACCOUNT_PUBLIC_KEY`, `STELLAR_SECRET_KEY`, `JWT_SECRET`, DB vars
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
  - Stellar wallet connection (Freighter)
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
  - JWT-based authentication and challenge/nonce signing (Stellar SEP-0010-style)
  - NFT mint endpoints (upload to Firebase, metadata to IPFS via NFT.Storage)
  - Collections, transactions, auctions, stats
  - Event listeners/log processing for Soroban contracts (marketplace, auction, transaction)

#### Common API Endpoints
- Mint NFT via file upload:
  - `POST /nfts/mint/:userId/:collectionId`
  - Multipart form with `file`; JSON fields: `title`, `description`, `price`, `currency`
- Mint NFT from image URL (JWT required):
  - `POST /nfts/mint/from-url?collectionId=<id>`
  - JSON body: `{ "title": "...", "description": "...", "imageUrl": "...", "price": 1.23, "currency": "XLM" }`

### Mobile App (Expo React Native)
- Directory: `nftopia-mobile-app`
- Commands:
  - Install: `pnpm install`
  - Start: `pnpm start`
  - Android: `pnpm android`
  - iOS: `pnpm ios`
- Features:
  - Wallet connectivity via Stellar-compatible wallets (e.g., Freighter web, deep-links to xBull/Lobstr)
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
- Connect Stellar wallet (Freighter or compatible)
- Backend issues a challenge/nonce
- User signs the challenge/nonce with their Stellar key
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
- Ensure Soroban RPC and contract IDs are configured (`SOROBAN_*`, `STELLAR_*`)
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