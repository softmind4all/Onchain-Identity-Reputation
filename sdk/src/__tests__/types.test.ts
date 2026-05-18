import type { StellarIdentityConfig, IdentityProfile, Signals } from "../types";

describe("SDK types", () => {
  it("StellarIdentityConfig has required fields", () => {
    const config: StellarIdentityConfig = {
      rpcUrl: "https://soroban-testnet.stellar.org",
      networkPassphrase: "Test SDF Network ; September 2015",
      contracts: {
        soulboundToken: "C...",
        credentialRegistry: "C...",
        reputationScore: "C...",
        identityAggregator: "C...",
      },
    };
    expect(config.rpcUrl).toBeDefined();
    expect(config.contracts.soulboundToken).toBeDefined();
  });

  it("IdentityProfile has required fields", () => {
    const profile: IdentityProfile = {
      subject: "G...",
      hasSoulbound: false,
      reputationScore: 0,
      credentialCount: 0,
    };
    expect(profile.reputationScore).toBe(0);
  });

  it("Signals has all five signal fields", () => {
    const signals: Signals = {
      txVolume: 50,
      credentialCount: 10,
      governanceVotes: 5,
      accountAgeDays: 100,
      defiInteractions: 20,
      updatedAt: BigInt(0),
    };
    expect(Object.keys(signals)).toHaveLength(6);
  });
});
