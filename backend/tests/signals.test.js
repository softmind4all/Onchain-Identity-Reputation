import { computeSignals } from "../src/signals.js";

const account = (daysAgo) => ({
  last_modified_time: new Date(Date.now() - 86_400_000 * daysAgo).toISOString(),
});

describe("computeSignals", () => {
  it("returns zeros for empty data", () => {
    const s = computeSignals(account(0), [], []);
    expect(s.txVolume).toBe(0);
    expect(s.defiInteractions).toBe(0);
  });

  it("caps txVolume at 100", () => {
    const txs = new Array(600).fill({});
    const s = computeSignals(account(30), txs, []);
    expect(s.txVolume).toBe(100);
  });

  it("counts only DeFi operation types", () => {
    const ops = [
      { type: "invoke_host_function" },
      { type: "payment" }, // not DeFi
      { type: "manage_sell_offer" },
    ];
    const s = computeSignals(account(30), [], ops);
    expect(s.defiInteractions).toBe(1); // 2/200 * 100 = 1
  });

  it("normalizes account age correctly", () => {
    // 365 days = 50% of 2-year cap
    const s = computeSignals(account(365), [], []);
    expect(s.accountAgeDays).toBe(50);
  });
});
