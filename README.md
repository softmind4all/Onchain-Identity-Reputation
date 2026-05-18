# Stellar Onchain Identity & Reputation

A decentralized identity and reputation protocol built on [Stellar](https://stellar.org) using [Soroban](https://developers.stellar.org/docs/build/smart-contracts) smart contracts.

Soulbound tokens, verifiable credentials, and on-chain history combine to form a portable, privacy-respecting identity layer for the Stellar ecosystem.

---

## What It Does

| Layer | Contract | Purpose |
|---|---|---|
| Identity anchor | `soulbound-token` | Non-transferable token bound to one address |
| Credentials | `credential-registry` | Issue & verify W3C-style credentials on-chain |
| Reputation | `reputation-score` | Weighted score [0–1000] from 5 on-chain signals |
| Aggregation | `identity-aggregator` | Single query returns full identity profile |

---

## Inspiration & Prior Art

This protocol draws from three proven approaches in other ecosystems:

- **Gitcoin Passport** (Ethereum) — aggregating attestations ("stamps") from multiple sources into a single Sybil-resistance score. We adopt the same multi-source credential model.
- **Nomis Protocol** (multi-chain) — computing a numeric reputation score from 50+ on-chain metrics. We adapt this into a 5-signal weighted model suited to Stellar's data model.
- **Privado ID / Polygon ID** — ZK-based verifiable credentials with selective disclosure. Our credential registry is designed to support a ZK proof hook in a future iteration.

None of these exist on Stellar. This project brings the pattern natively to Soroban.

---

## Repository Structure

```
contracts/
  soulbound-token/        Non-transferable identity token (Rust/Soroban)
  credential-registry/    Verifiable credential issuance & verification
  reputation-score/       Oracle-fed scoring engine
  identity-aggregator/    Unified profile API
  shared/                 Shared error codes and constants

sdk/                      TypeScript SDK (@stellar-identity/sdk)
app/                      Next.js reference frontend
docs/                     Architecture and contributor guides
```

---

## Quick Start

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
```

### Build & test contracts

```bash
stellar contract build
cargo test
```

### Run the frontend

```bash
cd app
cp .env.example .env.local
npm install
npm run dev
```

---

## Scoring Model

Reputation scores are computed from five normalized signals (each 0–100):

| Signal | Weight |
|---|---|
| Credential count | 30% |
| Transaction volume | 25% |
| Account age | 15% |
| Governance votes | 15% |
| DeFi interactions | 15% |

Final score = Σ(signal × weight) / 100 → **range [0, 1000]**

---

## Roadmap

- [x] Contract scaffolding with skeleton code and tests
- [x] TypeScript SDK skeleton
- [x] Next.js frontend skeleton
- [ ] Implement cross-contract calls in identity-aggregator
- [ ] Build oracle service (Horizon API → `submit_signals`)
- [ ] Implement SDK client methods (simulation + submission)
- [ ] Wallet connection (Freighter)
- [ ] ZK proof hook for private credential disclosure
- [ ] Testnet deployment + deployment scripts
- [ ] Mainnet launch

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/architecture.md](docs/architecture.md).

All contract functions have `TODO` comments marking the next implementation step — these are the primary contribution targets.

---

## License

MIT
