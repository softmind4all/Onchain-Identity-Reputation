# Architecture

## Overview

The system is composed of four Soroban smart contracts and a TypeScript SDK.

```
┌─────────────────────────────────────────────────────┐
│                   dApp / Frontend                   │
│              (Next.js + @stellar-identity/sdk)      │
└────────────────────────┬────────────────────────────┘
                         │
              ┌──────────▼──────────┐
              │  identity-aggregator │  ← single entry point
              └──┬──────┬──────┬───┘
                 │      │      │
    ┌────────────▼─┐ ┌──▼──────▼──────┐ ┌──────────────────┐
    │  soulbound-  │ │  credential-   │ │  reputation-     │
    │  token       │ │  registry      │ │  score           │
    └──────────────┘ └────────────────┘ └──────────────────┘
```

## Contracts

### soulbound-token
- One token per address, permanently non-transferable
- Admin mints; admin can revoke (fraud/abuse)
- Stores a URI pointing to the off-chain DID document

### credential-registry
- Whitelisted issuers publish credentials for subjects
- Credentials have a type, content hash, and optional expiry
- Inspired by Gitcoin Passport "stamps" and W3C Verifiable Credentials

### reputation-score
- Trusted oracles push 5 normalized signals per address
- Score = weighted sum of signals, range [0, 1000]
- Weights: credentials 30%, tx volume 25%, account age 15%, governance 15%, DeFi 15%
- Inspired by Nomis Protocol's multi-metric scoring

### identity-aggregator
- Stores references to the three sibling contracts
- Provides a single `profile` query for dApps
- Extension point for ZK proof verification and delegation

## Scoring Model

| Signal             | Weight | Max raw value |
|--------------------|--------|---------------|
| Credential count   | 30%    | 100           |
| Transaction volume | 25%    | 100           |
| Account age (days) | 15%    | 100           |
| Governance votes   | 15%    | 100           |
| DeFi interactions  | 15%    | 100           |

Final score = Σ(signal × weight) / 100 → range [0, 1000]

## Off-chain Components

- **Oracle service** (TODO): reads Stellar Horizon API, computes signals, calls `submit_signals`
- **Issuer service** (TODO): KYC / credential issuance backend
- **DID documents**: stored on IPFS, URI referenced in soulbound token

## Roadmap

- [ ] Implement cross-contract calls in identity-aggregator
- [ ] Build oracle service (Node.js + Horizon API)
- [ ] Add ZK proof hook for private credential disclosure
- [ ] Wallet connection in frontend (Freighter)
- [ ] Mainnet deployment scripts
