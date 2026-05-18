//! # Identity Aggregator Contract
//!
//! The top-level entry point that ties together soulbound tokens,
//! credentials, and reputation scores into a unified identity profile.
//!
//! ## Design
//! - Stores references (contract addresses) to the three sibling contracts
//! - Provides a single `profile` query that cross-calls all three
//! - Acts as the public-facing API for dApps and the SDK
//!
//! ## Contributor Guide
//! - `initialize`  → set sibling contract addresses
//! - `profile`     → aggregate view: token status + credential list + score
//! - `has_token`   → quick check: does this address hold a soulbound token?
//!
//! ## Extension Points (TODO for contributors)
//! - Cross-contract calls to soulbound-token and credential-registry
//!   (currently returns stubs — implement after deploying sibling contracts)
//! - ZK proof verification hook for private credential disclosure
//! - Delegation: allow one address to act on behalf of another

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// ── Storage keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    SoulboundContract,
    CredentialContract,
    ReputationContract,
}

// ── Data types ────────────────────────────────────────────────────────────────

/// Aggregated identity profile returned to callers.
#[contracttype]
#[derive(Clone)]
pub struct IdentityProfile {
    pub subject: Address,
    /// True if the subject holds a valid soulbound token
    pub has_soulbound: bool,
    /// Reputation score [0, 1000]
    pub reputation_score: u32,
    /// Number of active credentials
    pub credential_count: u32,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct IdentityAggregator;

#[contractimpl]
impl IdentityAggregator {
    /// Initialize with addresses of the three sibling contracts.
    pub fn initialize(
        env: Env,
        admin: Address,
        soulbound_contract: Address,
        credential_contract: Address,
        reputation_contract: Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::SoulboundContract, &soulbound_contract);
        env.storage()
            .instance()
            .set(&DataKey::CredentialContract, &credential_contract);
        env.storage()
            .instance()
            .set(&DataKey::ReputationContract, &reputation_contract);
    }

    /// Returns the aggregated identity profile for `subject`.
    ///
    /// TODO: replace stub values with actual cross-contract calls once
    /// sibling contracts are deployed. See docs/cross-contract-calls.md.
    pub fn profile(env: Env, subject: Address) -> IdentityProfile {
        // TODO: call soulbound-token contract → has_token(subject)
        // TODO: call reputation-score contract → score(subject)
        // TODO: call credential-registry contract → count active credentials

        // Stub implementation — replace with cross-contract invocations
        let _ = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::SoulboundContract);

        IdentityProfile {
            subject,
            has_soulbound: false,   // TODO
            reputation_score: 0,    // TODO
            credential_count: 0,    // TODO
        }
    }

    /// Quick soulbound token existence check.
    pub fn has_token(_env: Env, _subject: Address) -> bool {
        // TODO: cross-contract call to soulbound-token
        false
    }

    /// Update a sibling contract address (admin only, for upgrades).
    pub fn update_contract(env: Env, key: DataKey, new_address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        // Only allow updating the three contract keys, not Admin itself
        match key {
            DataKey::SoulboundContract
            | DataKey::CredentialContract
            | DataKey::ReputationContract => {
                env.storage().instance().set(&key, &new_address);
            }
            _ => panic!("invalid key"),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    fn setup() -> (Env, IdentityAggregatorClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(IdentityAggregator, ());
        let client = IdentityAggregatorClient::new(&env, &id);
        let admin = Address::generate(&env);
        let sb = Address::generate(&env);
        let cr = Address::generate(&env);
        let rs = Address::generate(&env);
        client.initialize(&admin, &sb, &cr, &rs);
        (env, client)
    }

    #[test]
    fn profile_returns_stub() {
        let (env, client) = setup();
        let user = Address::generate(&env);
        let profile = client.profile(&user);
        assert_eq!(profile.subject, user);
        assert!(!profile.has_soulbound);
        assert_eq!(profile.reputation_score, 0);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn double_init_fails() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let sb = Address::generate(&env);
        let cr = Address::generate(&env);
        let rs = Address::generate(&env);
        client.initialize(&admin, &sb, &cr, &rs);
    }
}
