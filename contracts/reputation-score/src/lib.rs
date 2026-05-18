//! # Reputation Score Contract
//!
//! Computes and stores a numeric reputation score for each identity.
//!
//! ## Design (inspired by Nomis Protocol's 50+ metric approach)
//! - Score is a u32 in range [0, 1000]
//! - Score is composed of weighted signals submitted by trusted oracles
//! - Signals: transaction_volume, credential_count, governance_votes,
//!   account_age_days, defi_interactions
//! - Oracles are whitelisted; they push signal updates on-chain
//! - Final score = weighted sum, clamped to [0, 1000]
//!
//! ## Contributor Guide
//! - `add_oracle`      → admin registers a trusted data oracle
//! - `submit_signals`  → oracle pushes raw signal values for a subject
//! - `score`           → read the current score for an address
//! - `signals`         → read raw signal breakdown for an address

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

// ── Weights (basis points, sum = 1000) ───────────────────────────────────────
// Adjust these to tune the scoring model.
const W_TX_VOLUME: u32 = 250;
const W_CREDENTIALS: u32 = 300;
const W_GOV_VOTES: u32 = 150;
const W_ACCOUNT_AGE: u32 = 150;
const W_DEFI: u32 = 150;

// ── Storage keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    Oracle(Address),
    Signals(Address),
}

// ── Data types ────────────────────────────────────────────────────────────────

/// Raw signals submitted by oracles. Each value is normalized [0, 100].
#[contracttype]
#[derive(Clone, Default)]
pub struct Signals {
    /// Normalized transaction volume score [0, 100]
    pub tx_volume: u32,
    /// Number of valid credentials (capped at 100 for scoring)
    pub credential_count: u32,
    /// Governance votes cast (capped at 100)
    pub governance_votes: u32,
    /// Account age in days (capped at 100 → ~3 months = max)
    pub account_age_days: u32,
    /// DeFi protocol interactions (capped at 100)
    pub defi_interactions: u32,
    /// Last updated ledger timestamp
    pub updated_at: u64,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct ReputationScore;

#[contractimpl]
impl ReputationScore {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Register a trusted oracle that can submit signal data.
    pub fn add_oracle(env: Env, oracle: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::Oracle(oracle), &true);
    }

    /// Oracle submits updated signals for a subject address.
    pub fn submit_signals(
        env: Env,
        oracle: Address,
        subject: Address,
        tx_volume: u32,
        credential_count: u32,
        governance_votes: u32,
        account_age_days: u32,
        defi_interactions: u32,
    ) {
        oracle.require_auth();
        let trusted: bool = env
            .storage()
            .instance()
            .get(&DataKey::Oracle(oracle))
            .unwrap_or(false);
        if !trusted {
            panic!("oracle not authorized");
        }

        let signals = Signals {
            tx_volume: tx_volume.min(100),
            credential_count: credential_count.min(100),
            governance_votes: governance_votes.min(100),
            account_age_days: account_age_days.min(100),
            defi_interactions: defi_interactions.min(100),
            updated_at: env.ledger().timestamp(),
        };
        env.storage()
            .persistent()
            .set(&DataKey::Signals(subject.clone()), &signals);
        env.events()
            .publish((symbol_short!("update"),), (subject, Self::compute(&signals)));
    }

    /// Returns the computed reputation score [0, 1000].
    pub fn score(env: Env, subject: Address) -> u32 {
        let signals: Signals = env
            .storage()
            .persistent()
            .get(&DataKey::Signals(subject))
            .unwrap_or_default();
        Self::compute(&signals)
    }

    /// Returns the raw signal breakdown for a subject.
    pub fn signals(env: Env, subject: Address) -> Signals {
        env.storage()
            .persistent()
            .get(&DataKey::Signals(subject))
            .unwrap_or_default()
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    fn compute(s: &Signals) -> u32 {
        (s.tx_volume * W_TX_VOLUME
            + s.credential_count * W_CREDENTIALS
            + s.governance_votes * W_GOV_VOTES
            + s.account_age_days * W_ACCOUNT_AGE
            + s.defi_interactions * W_DEFI)
            / 100
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    fn setup() -> (Env, Address, Address, ReputationScoreClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(ReputationScore, ());
        let client = ReputationScoreClient::new(&env, &id);
        let admin = Address::generate(&env);
        let oracle = Address::generate(&env);
        client.initialize(&admin);
        client.add_oracle(&oracle);
        (env, admin, oracle, client)
    }

    #[test]
    fn default_score_is_zero() {
        let (env, _admin, _oracle, client) = setup();
        let user = Address::generate(&env);
        assert_eq!(client.score(&user), 0);
    }

    #[test]
    fn perfect_signals_give_max_score() {
        let (env, _admin, oracle, client) = setup();
        let user = Address::generate(&env);
        client.submit_signals(&oracle, &user, &100, &100, &100, &100, &100);
        assert_eq!(client.score(&user), 1000);
    }

    #[test]
    fn partial_signals_score_correctly() {
        let (env, _admin, oracle, client) = setup();
        let user = Address::generate(&env);
        // Only credentials filled (100) → 300/1000
        client.submit_signals(&oracle, &user, &0, &100, &0, &0, &0);
        assert_eq!(client.score(&user), 300);
    }

    #[test]
    #[should_panic(expected = "oracle not authorized")]
    fn unauthorized_oracle_fails() {
        let (env, _admin, _oracle, client) = setup();
        let rogue = Address::generate(&env);
        let user = Address::generate(&env);
        client.submit_signals(&rogue, &user, &50, &50, &50, &50, &50);
    }
}
