// Inline config type — mirrors StellarIdentityConfig from the SDK
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

/**
 * Returns the SDK config for the current environment.
 * Set contract addresses in .env.local after deployment.
 */
export function getSdkConfig(): StellarIdentityConfig {
  return {
    rpcUrl: process.env.NEXT_PUBLIC_RPC_URL ?? "https://soroban-testnet.stellar.org",
    networkPassphrase:
      process.env.NEXT_PUBLIC_NETWORK_PASSPHRASE ??
      "Test SDF Network ; September 2015",
    contracts: {
      soulboundToken: process.env.NEXT_PUBLIC_SOULBOUND_CONTRACT ?? "",
      credentialRegistry: process.env.NEXT_PUBLIC_CREDENTIAL_CONTRACT ?? "",
      reputationScore: process.env.NEXT_PUBLIC_REPUTATION_CONTRACT ?? "",
      identityAggregator: process.env.NEXT_PUBLIC_AGGREGATOR_CONTRACT ?? "",
    },
  };
}
