/**
 * Oracle REST API
 *
 * GET /health              — liveness check
 * GET /signals/:address    — compute reputation signals for a Stellar address
 *
 * TODO for contributors:
 * - POST /signals/:address  — trigger on-chain submission via Soroban SDK
 * - Add API key middleware for write endpoints
 */

import express from "express";
import { fetchAccount, fetchTransactions, fetchOperations } from "./horizon.js";
import { computeSignals } from "./signals.js";

export const app = express();
app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

app.get("/signals/:address", async (req, res) => {
  const { address } = req.params;

  // Basic Stellar address validation (G... 56 chars)
  if (!/^G[A-Z2-7]{55}$/.test(address)) {
    return res.status(400).json({ error: "invalid Stellar address" });
  }

  try {
    const [account, transactions, operations] = await Promise.all([
      fetchAccount(address),
      fetchTransactions(address),
      fetchOperations(address),
    ]);

    if (!account) {
      return res.status(404).json({ error: "account not found" });
    }

    const signals = computeSignals(account, transactions, operations);
    return res.json({ address, signals });
  } catch (err) {
    console.error(err);
    return res.status(502).json({ error: "upstream error" });
  }
});
