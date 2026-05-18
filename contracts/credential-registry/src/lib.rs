//! # Credential Registry Contract
//!
//! Issues and verifies W3C-style Verifiable Credentials anchored on Stellar.
//!
//! ## Design (inspired by Gitcoin Passport "stamps" + Privado ID attestations)
//! - Issuers are whitelisted by admin
//! - Each credential has a type, subject, issuer, expiry, and a content hash
//! - Credentials are keyed by (subject, credential_type) — one active credential
//!   per type per subject
//! - Expired credentials are treated as invalid without needing explicit removal
//!
//! ## Contributor Guide
//! - `add_issuer`   → admin whitelists a trusted issuer address
//! - `issue`        → whitelisted issuer creates a credential for a subject
//! - `revoke`       → issuer or admin revokes a credential
//! - `verify`       → returns true if credential exists and is not expired
//! - `get`          → fetch full credential data

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String,
};

// ── Storage keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    Issuer(Address),
    Credential(Address, String), // (subject, credential_type)
}

// ── Data types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub issuer: Address,
    pub subject: Address,
    /// e.g. "KYC", "ProofOfHumanity", "GitContributor", "StellarDeveloper"
    pub credential_type: String,
    /// SHA-256 hash of the off-chain credential document
    pub content_hash: Bytes,
    pub issued_at: u64,
    /// 0 = no expiry
    pub expires_at: u64,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct CredentialRegistry;

#[contractimpl]
impl CredentialRegistry {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Whitelist a trusted credential issuer.
    pub fn add_issuer(env: Env, issuer: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::Issuer(issuer), &true);
    }

    /// Issue a verifiable credential to `subject`.
    pub fn issue(
        env: Env,
        issuer: Address,
        subject: Address,
        credential_type: String,
        content_hash: Bytes,
        expires_at: u64,
    ) {
        issuer.require_auth();
        let is_trusted: bool = env
            .storage()
            .instance()
            .get(&DataKey::Issuer(issuer.clone()))
            .unwrap_or(false);
        if !is_trusted {
            panic!("issuer not whitelisted");
        }

        let cred = Credential {
            issuer: issuer.clone(),
            subject: subject.clone(),
            credential_type: credential_type.clone(),
            content_hash,
            issued_at: env.ledger().timestamp(),
            expires_at,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Credential(subject.clone(), credential_type.clone()), &cred);
        env.events()
            .publish((symbol_short!("issue"),), (subject, credential_type));
    }

    /// Revoke a credential. Callable by the original issuer or admin.
    pub fn revoke(env: Env, caller: Address, subject: Address, credential_type: String) {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        let key = DataKey::Credential(subject.clone(), credential_type.clone());

        let cred: Credential = env.storage().persistent().get(&key).expect("not found");
        if caller != admin && caller != cred.issuer {
            panic!("unauthorized");
        }
        env.storage().persistent().remove(&key);
        env.events()
            .publish((symbol_short!("revoke"),), (subject, credential_type));
    }

    /// Returns true if the credential exists and has not expired.
    pub fn verify(env: Env, subject: Address, credential_type: String) -> bool {
        let key = DataKey::Credential(subject, credential_type);
        match env.storage().persistent().get::<DataKey, Credential>(&key) {
            Some(cred) => {
                cred.expires_at == 0 || cred.expires_at > env.ledger().timestamp()
            }
            None => false,
        }
    }

    pub fn get(env: Env, subject: Address, credential_type: String) -> Credential {
        env.storage()
            .persistent()
            .get(&DataKey::Credential(subject, credential_type))
            .expect("credential not found")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Bytes, Env, String};

    fn setup() -> (Env, Address, Address, CredentialRegistryClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(CredentialRegistry, ());
        let client = CredentialRegistryClient::new(&env, &id);
        let admin = Address::generate(&env);
        let issuer = Address::generate(&env);
        client.initialize(&admin);
        client.add_issuer(&issuer);
        (env, admin, issuer, client)
    }

    #[test]
    fn issue_and_verify() {
        let (env, _admin, issuer, client) = setup();
        let subject = Address::generate(&env);
        let ctype = String::from_str(&env, "KYC");
        let hash = Bytes::from_slice(&env, &[0u8; 32]);
        client.issue(&issuer, &subject, &ctype, &hash, &0u64);
        assert!(client.verify(&subject, &ctype));
    }

    #[test]
    fn expired_credential_is_invalid() {
        let (env, _admin, issuer, client) = setup();
        let subject = Address::generate(&env);
        let ctype = String::from_str(&env, "KYC");
        let hash = Bytes::from_slice(&env, &[0u8; 32]);
        // expires in the past
        client.issue(&issuer, &subject, &ctype, &hash, &1u64);
        assert!(!client.verify(&subject, &ctype));
    }

    #[test]
    #[should_panic(expected = "issuer not whitelisted")]
    fn untrusted_issuer_fails() {
        let (env, _admin, _issuer, client) = setup();
        let rogue = Address::generate(&env);
        let subject = Address::generate(&env);
        let ctype = String::from_str(&env, "KYC");
        let hash = Bytes::from_slice(&env, &[0u8; 32]);
        client.issue(&rogue, &subject, &ctype, &hash, &0u64);
    }
}
