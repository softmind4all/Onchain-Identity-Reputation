/**
 * Computes normalized reputation signals [0, 100] from raw Horizon data.
 * These values are submitted to the reputation-score Soroban contract.
 *
 * Scoring logic mirrors the on-chain weights:
 *   credential_count  30%
 *   tx_volume         25%
 *   account_age_days  15%
 *   governance_votes  15%
 *   defi_interactions 15%
 */

const ACCOUNT_AGE_CAP_DAYS = 365 * 2; // 2 years = max age score
const TX_CAP = 500;
const DEFI_CAP = 200;

/**
 * @param {object} account - Horizon account object
 * @param {object[]} transactions - recent transactions
 * @param {object[]} operations - recent operations
 * @returns {{ txVolume: number, accountAgeDays: number, defiInteractions: number }}
 */
export function computeSignals(account, transactions, operations) {
  const accountAgeDays = account
    ? Math.floor(
        (Date.now() - new Date(account.last_modified_time ?? 0).getTime()) /
          86_400_000
      )
    : 0;

  const txVolume = Math.min(transactions.length, TX_CAP);

  // Proxy for DeFi: count invoke_host_function and manage_sell_offer operations
  const defiOps = operations.filter((op) =>
    ["invoke_host_function", "manage_sell_offer", "manage_buy_offer", "liquidity_pool_deposit"].includes(op.type)
  );

  return {
    txVolume: normalize(txVolume, TX_CAP),
    accountAgeDays: normalize(accountAgeDays, ACCOUNT_AGE_CAP_DAYS),
    defiInteractions: normalize(defiOps.length, DEFI_CAP),
  };
}

function normalize(value, cap) {
  return Math.min(100, Math.round((value / cap) * 100));
}
