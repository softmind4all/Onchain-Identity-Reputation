//! # Soulbound Token Contract
//!
//! A non-transferable identity token bound to a single Stellar address.
//! Inspired by ERC-5192 (Ethereum) but adapted for Soroban.
//!
//! ## Design
//! - One token per address (enforced on mint)
//! - Transfer is permanently disabled — the token is "soul-bound"
//! - Admin can revoke tokens (e.g. for fraud/abuse)
//! - Metadata URI points to off-chain DID document (IPFS / Stellar TOML)
//!
//! ## Contributor Guide
//! - `mint`   → issue a new identity token to an address
//! - `revoke` → admin-only burn
//! - `get`    → read token metadata for an address
//! - `locked` → always returns true (ERC-5192 compatibility signal)

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String,
};

// ── Storage keys ────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    Token(Address), // address → TokenData
}

// ── Data types ───────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct TokenData {
    /// W3C DID or IPFS URI pointing to the identity document
    pub uri: String,
    /// Unix timestamp of issuance
    pub issued_at: u64,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct SoulboundToken;

#[contractimpl]
impl SoulboundToken {
    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Mint a soulbound token to `recipient`.
    /// Each address may hold at most one token.
    pub fn mint(env: Env, recipient: Address, uri: String) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Token(recipient.clone());
        if env.storage().persistent().has(&key) {
            panic!("token already exists for this address");
        }

        let data = TokenData {
            uri,
            issued_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &data);
        env.events()
            .publish((symbol_short!("mint"),), (recipient,));
    }

    /// Revoke (burn) a token. Admin only.
    pub fn revoke(env: Env, owner: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Token(owner.clone());
        if !env.storage().persistent().has(&key) {
            panic!("token not found");
        }
        env.storage().persistent().remove(&key);
        env.events()
            .publish((symbol_short!("revoke"),), (owner,));
    }

    /// Returns token metadata for `owner`, or panics if none exists.
    pub fn get(env: Env, owner: Address) -> TokenData {
        env.storage()
            .persistent()
            .get(&DataKey::Token(owner))
            .expect("token not found")
    }

    /// Returns true — soulbound tokens are always locked (non-transferable).
    pub fn locked(_env: Env) -> bool {
        true
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::Env;

    fn setup() -> (Env, Address, SoulboundTokenClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(SoulboundToken, ());
        let client = SoulboundTokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        (env, admin, client)
    }

    #[test]
    fn mint_and_get() {
        let (env, _admin, client) = setup();
        let user = Address::generate(&env);
        let uri = String::from_str(&env, "ipfs://Qm...");
        client.mint(&user, &uri);
        let token = client.get(&user);
        assert_eq!(token.uri, uri);
    }

    #[test]
    #[should_panic(expected = "token already exists")]
    fn double_mint_fails() {
        let (env, _admin, client) = setup();
        let user = Address::generate(&env);
        let uri = String::from_str(&env, "ipfs://Qm...");
        client.mint(&user, &uri);
        client.mint(&user, &uri); // must panic
    }

    #[test]
    fn revoke_removes_token() {
        let (env, _admin, client) = setup();
        let user = Address::generate(&env);
        client.mint(&user, &String::from_str(&env, "ipfs://Qm..."));
        client.revoke(&user);
        // token should be gone — next call panics
    }

    #[test]
    fn locked_is_always_true() {
        let (_env, _admin, client) = setup();
        assert!(client.locked());
    }
}
