use crate::error::AppError;
use crate::models::{commitment::Commitment, merkle::MerkleTree};
use async_trait::async_trait;

/// Storage trait for commitment and Merkle tree operations
#[async_trait]
pub trait CommitmentStorage: Send + Sync {
    /// Add a new commitment and return its index
    async fn add_commitment(&self, value: Vec<u8>) -> Result<(usize, Vec<u8>), AppError>;

    /// Get a commitment by its index
    async fn get_commitment(&self, index: usize) -> Result<Commitment, AppError>;

    /// Get all commitments
    async fn get_all_commitments(&self) -> Result<Vec<Commitment>, AppError>;

    /// Get the current Merkle tree
    async fn get_tree(&self) -> Result<MerkleTree, AppError>;

    /// Get the current root hash
    async fn get_root_hash(&self) -> Result<Vec<u8>, AppError>;

    /// Get the total number of commitments
    async fn commitment_count(&self) -> Result<usize, AppError>;
}