import { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE, Keypair } from "@stellar/stellar-sdk";
import type { TokenData, StellarIdentityConfig } from "./types";

/**
 * Client for the soulbound-token Soroban contract.
 *
 * ## Usage
 * ```ts
 * const client = new SoulboundTokenClient(config);
 * const token = await client.get("G...");
 * ```
 */
export class SoulboundTokenClient {
  private server: SorobanRpc.Server;
  private contractId: string;

  constructor(private config: StellarIdentityConfig) {
    this.server = new SorobanRpc.Server(config.rpcUrl);
    this.contractId = config.contracts.soulboundToken;
  }

  /**
   * Returns token metadata for `owner`, or null if no token exists.
   *
   * TODO: implement full simulation + parse XDR response into TokenData.
   * See https://developers.stellar.org/docs/build/guides/transactions
   */
  async get(_owner: string): Promise<TokenData | null> {
    // TODO: build and simulate a `get` invocation against this.contractId
    throw new Error("not implemented — see TODO in SoulboundTokenClient.get");
  }

  /**
   * Returns true if the token is locked (always true for soulbound tokens).
   */
  async locked(): Promise<boolean> {
    // TODO: simulate `locked` invocation
    return true;
  }

  /**
   * Mint a soulbound token. Requires admin keypair.
   *
   * TODO: build, sign, and submit the `mint` transaction.
   */
  async mint(_adminKeypair: Keypair, _recipient: string, _uri: string): Promise<string> {
    // TODO: TransactionBuilder → addOperation(contract.call("mint", ...)) → sign → submit
    throw new Error("not implemented — see TODO in SoulboundTokenClient.mint");
  }
}
