/**
 * Horizon client — fetches raw account data from the Stellar Horizon REST API.
 * Testnet by default; override HORIZON_URL env var for mainnet.
 */

const HORIZON_URL = process.env.HORIZON_URL ?? "https://horizon-testnet.stellar.org";

/**
 * Fetch account details from Horizon.
 * @param {string} address - Stellar public key (G...)
 * @returns {Promise<object|null>}
 */
export async function fetchAccount(address) {
  const res = await fetch(`${HORIZON_URL}/accounts/${address}`);
  if (res.status === 404) return null;
  if (!res.ok) throw new Error(`Horizon error ${res.status}`);
  return res.json();
}

/**
 * Fetch recent transactions for an account.
 * @param {string} address
 * @param {number} limit
 * @returns {Promise<object[]>}
 */
export async function fetchTransactions(address, limit = 200) {
  const res = await fetch(
    `${HORIZON_URL}/accounts/${address}/transactions?limit=${limit}&order=desc`
  );
  if (!res.ok) throw new Error(`Horizon error ${res.status}`);
  const data = await res.json();
  return data._embedded?.records ?? [];
}

/**
 * Fetch operations (DeFi interactions proxy) for an account.
 * @param {string} address
 * @param {number} limit
 * @returns {Promise<object[]>}
 */
export async function fetchOperations(address, limit = 200) {
  const res = await fetch(
    `${HORIZON_URL}/accounts/${address}/operations?limit=${limit}&order=desc`
  );
  if (!res.ok) throw new Error(`Horizon error ${res.status}`);
  const data = await res.json();
  return data._embedded?.records ?? [];
}
