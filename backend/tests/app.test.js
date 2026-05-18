import request from "supertest";
import { jest } from "@jest/globals";

// Mock horizon module before importing app
jest.unstable_mockModule("../src/horizon.js", () => ({
  fetchAccount: jest.fn(),
  fetchTransactions: jest.fn(),
  fetchOperations: jest.fn(),
}));

const { app } = await import("../src/app.js");
const { fetchAccount, fetchTransactions, fetchOperations } = await import("../src/horizon.js");

const VALID_ADDRESS = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";

describe("GET /health", () => {
  it("returns ok", async () => {
    const res = await request(app).get("/health");
    expect(res.status).toBe(200);
    expect(res.body.status).toBe("ok");
  });
});

describe("GET /signals/:address", () => {
  it("returns 400 for invalid address", async () => {
    const res = await request(app).get("/signals/notanaddress");
    expect(res.status).toBe(400);
  });

  it("returns 404 when account not found", async () => {
    fetchAccount.mockResolvedValue(null);
    fetchTransactions.mockResolvedValue([]);
    fetchOperations.mockResolvedValue([]);
    const res = await request(app).get(`/signals/${VALID_ADDRESS}`);
    expect(res.status).toBe(404);
  });

  it("returns signals for a valid account", async () => {
    fetchAccount.mockResolvedValue({ last_modified_time: new Date(Date.now() - 86_400_000 * 100).toISOString() });
    fetchTransactions.mockResolvedValue(new Array(50).fill({}));
    fetchOperations.mockResolvedValue([
      { type: "invoke_host_function" },
      { type: "payment" },
    ]);
    const res = await request(app).get(`/signals/${VALID_ADDRESS}`);
    expect(res.status).toBe(200);
    expect(res.body.address).toBe(VALID_ADDRESS);
    expect(res.body.signals.txVolume).toBe(10); // 50/500 * 100
    expect(res.body.signals.defiInteractions).toBe(1); // 1/200 * 100 = 0.5 → 1
    expect(res.body.signals.accountAgeDays).toBeGreaterThan(0);
  });
});
