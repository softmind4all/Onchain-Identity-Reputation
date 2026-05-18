# Contributing

Thank you for your interest in contributing to Stellar Onchain Identity & Reputation.

## Project Structure

```
contracts/          Soroban smart contracts (Rust)
  soulbound-token/  Non-transferable identity token
  credential-registry/ Verifiable credential issuance & verification
  reputation-score/ Weighted scoring engine
  identity-aggregator/ Unified profile API (cross-contract)
  shared/           Shared constants and error codes

sdk/                TypeScript SDK for dApp integration
app/                Next.js reference frontend
docs/               Architecture and contributor guides
```

## Getting Started

### Prerequisites

- Rust + `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Stellar CLI: `cargo install --locked stellar-cli`
- Node.js 20+

### Build contracts

```bash
stellar contract build
```

### Run contract tests

```bash
cargo test
```

### Run the frontend

```bash
cd app && npm install && npm run dev
```

## How to Contribute

1. Browse open issues — look for `good first issue` or `help wanted` labels.
2. Comment on the issue to claim it before starting work.
3. Fork the repo and create a branch: `git checkout -b feat/your-feature`.
4. Write code and tests. Every contract function must have at least one test.
5. Open a pull request against `main`. Fill in the PR template.

## Contract Contribution Guidelines

- Each contract function must have a doc comment explaining its purpose.
- All new functions must include at least one `#[test]`.
- Do not remove existing tests.
- Keep `no_std` — do not add `std` dependencies to contracts.

## SDK Contribution Guidelines

- Implement the `TODO` stubs in `sdk/src/` using `@stellar/stellar-sdk`.
- Each client method should handle simulation + submission in one call.
- Export any new public types from `sdk/src/index.ts`.

## Code of Conduct

Be respectful. Constructive feedback only. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
