/// Well-known credential type strings.
/// Use these constants in both contracts and the SDK to avoid typos.
pub mod credential_types {
    pub const KYC: &str = "KYC";
    pub const PROOF_OF_HUMANITY: &str = "ProofOfHumanity";
    pub const STELLAR_DEVELOPER: &str = "StellarDeveloper";
    pub const GIT_CONTRIBUTOR: &str = "GitContributor";
    pub const DAO_MEMBER: &str = "DaoMember";
    pub const DEFI_USER: &str = "DeFiUser";
}

/// Score range constants.
pub const SCORE_MIN: u32 = 0;
pub const SCORE_MAX: u32 = 1000;
