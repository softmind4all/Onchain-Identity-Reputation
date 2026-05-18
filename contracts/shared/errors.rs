/// Shared error codes used across all contracts.
/// Using a common enum prevents collisions when contracts are composed.
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    NotFound = 3,
    AlreadyExists = 4,
    Expired = 5,
    InvalidInput = 6,
}
