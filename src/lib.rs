// Public API modules
pub mod api;
pub mod crypto;
pub mod dto;
pub mod error;
pub mod models;
pub mod storage;

// Re-exports for convenience
pub use error::AppError;
pub use models::{commitment::Commitment, merkle::MerkleTree};
pub use storage::traits::CommitmentStorage;