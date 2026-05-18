/**
 * Home page — entry point for the identity dashboard.
 *
 * TODO for contributors:
 * 1. Add wallet connection (Freighter / WalletConnect)
 * 2. On connect, call IdentityAggregatorClient.profile(address)
 * 3. Render <IdentityCard> with the returned profile
 */
export default function Home() {
  return (
    <main style={{ fontFamily: "sans-serif", maxWidth: 640, margin: "80px auto", padding: "0 16px" }}>
      <h1>Stellar Onchain Identity</h1>
      <p>Connect your Stellar wallet to view your identity profile and reputation score.</p>

      {/* TODO: replace with <WalletConnectButton /> component */}
      <button disabled style={{ padding: "12px 24px", fontSize: 16, cursor: "not-allowed" }}>
        Connect Wallet (coming soon)
      </button>

      {/* TODO: render <IdentityCard profile={...} /> once wallet is connected */}
    </main>
  );
}
