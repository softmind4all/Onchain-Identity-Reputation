export interface IdentityProfile {
  subject: string;
  hasSoulbound: boolean;
  reputationScore: number;
  credentialCount: number;
}

interface Props {
  profile: IdentityProfile;
}

/**
 * Displays an aggregated identity profile.
 *
 * TODO for contributors:
 * - Style with Tailwind or CSS modules
 * - Add credential list (fetch from CredentialRegistryClient)
 * - Add score breakdown chart (signals from ReputationScoreClient)
 * - Add "Request Credential" flow
 */
export function IdentityCard({ profile }: Props) {
  return (
    <div>
      <h2>Identity Profile</h2>
      <p><strong>Address:</strong> {profile.subject}</p>
      <p><strong>Soulbound Token:</strong> {profile.hasSoulbound ? "✓ Issued" : "✗ Not issued"}</p>
      <p><strong>Reputation Score:</strong> {profile.reputationScore} / 1000</p>
      <p><strong>Active Credentials:</strong> {profile.credentialCount}</p>
    </div>
  );
}
