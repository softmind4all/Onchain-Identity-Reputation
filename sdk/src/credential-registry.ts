import { SorobanRpc, Keypair } from "@stellar/stellar-sdk";
import type { Credential, StellarIdentityConfig } from "./types";

/**
 * Client for the credential-registry Soroban contract.
 *
 * ## Usage
 * ```ts
 * const client = new CredentialRegistryClient(config);
 * const isValid = await client.verify("G...", "KYC");
 * ```
 */
export class CredentialRegistryClient {
  private server: SorobanRpc.Server;
  private contractId: string;

  constructor(private config: StellarIdentityConfig) {
    this.server = new SorobanRpc.Server(config.rpcUrl);
    this.contractId = config.contracts.credentialRegistry;
  }

  /**
   * Returns true if the credential exists and has not expired.
   * TODO: simulate `verify` invocation and parse bool result.
   */
  async verify(_subject: string, _credentialType: string): Promise<boolean> {
    throw new Error("not implemented");
  }

  /**
   * Fetch full credential data.
   * TODO: simulate `get` and parse Credential XDR.
   */
  async get(_subject: string, _credentialType: string): Promise<Credential> {
    throw new Error("not implemented");
  }

  /**
   * Issue a credential. Requires issuer keypair.
   * TODO: build and submit `issue` transaction.
   */
  async issue(
    _issuerKeypair: Keypair,
    _subject: string,
    _credentialType: string,
    _contentHash: Uint8Array,
    _expiresAt: bigint
  ): Promise<string> {
    throw new Error("not implemented");
  }
}
