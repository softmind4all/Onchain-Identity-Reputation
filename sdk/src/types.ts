// Mirrors the Soroban contract types for use in TypeScript clients.

export interface TokenData {
  uri: string;
  issuedAt: bigint;
}

export interface Credential {
  issuer: string;
  subject: string;
  credentialType: string;
  contentHash: Uint8Array;
  issuedAt: bigint;
  expiresAt: bigint; // 0 = no expiry
}

export interface Signals {
  txVolume: number;
  credentialCount: number;
  governanceVotes: number;
  accountAgeDays: number;
  defiInteractions: number;
  updatedAt: bigint;
}

export interface IdentityProfile {
  subject: string;
  hasSoulbound: boolean;
  reputationScore: number;
  credentialCount: number;
}

export interface StellarIdentityConfig {
  rpcUrl: string;
  networkPassphrase: string;
  contracts: {
    soulboundToken: string;
    credentialRegistry: string;
    reputationScore: string;
    identityAggregator: string;
  };
}
