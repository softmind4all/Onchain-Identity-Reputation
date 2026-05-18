import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { IdentityCard } from "../src/components/IdentityCard";

const profile = {
  subject: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5",
  hasSoulbound: true,
  reputationScore: 750,
  credentialCount: 3,
};

describe("IdentityCard", () => {
  it("renders the subject address", () => {
    render(<IdentityCard profile={profile} />);
    expect(screen.getByText(profile.subject)).toBeInTheDocument();
  });

  it("shows soulbound token status", () => {
    render(<IdentityCard profile={profile} />);
    expect(screen.getByText(/✓ Issued/)).toBeInTheDocument();
  });

  it("shows reputation score", () => {
    render(<IdentityCard profile={profile} />);
    expect(screen.getByText(/750 \/ 1000/)).toBeInTheDocument();
  });

  it("shows credential count", () => {
    render(<IdentityCard profile={profile} />);
    expect(screen.getByText(/Active Credentials:/)).toBeInTheDocument();
  });
});
