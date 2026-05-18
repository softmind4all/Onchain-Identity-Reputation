import { SorobanRpc, Keypair } from "@stellar/stellar-sdk";
import type { Signals, StellarIdentityConfig } from "./types";

/**
 * Client for the reputation-score Soroban contract.
 *
 * ## Usage
 * ```ts
 * const client = new ReputationScoreClient(config);
 * const score = await client.score("G...");
 * ```
 */
export class ReputationScoreClient {
  private server: SorobanRpc.Server;
  private contractId: string;

  constructor(private config: StellarIdentityConfig) {
    this.server = new SorobanRpc.Server(config.rpcUrl);
    this.contractId = config.contracts.reputationScore;
  }

  /**
   * Returns the reputation score [0, 1000] for `subject`.
   * TODO: simulate `score` invocation and parse u32 result.
   */
  async score(_subject: string): Promise<number> {
    throw new Error("not implemented");
  }

  /**
   * Returns the raw signal breakdown for `subject`.
   * TODO: simulate `signals` invocation and parse Signals struct.
   */
  async signals(_subject: string): Promise<Signals> {
    throw new Error("not implemented");
  }

  /**
   * Submit signal data. Requires oracle keypair.
   * TODO: build and submit `submit_signals` transaction.
   */
  async submitSignals(
    _oracleKeypair: Keypair,
    _subject: string,
    _signals: Omit<Signals, "updatedAt">
  ): Promise<string> {
    throw new Error("not implemented");
  }
}
