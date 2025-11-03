use crate::error::AppError;
use crate::models::{
    commitment::Commitment,
    merkle::{MerkleNode, MerkleTree},
};
use crate::storage::traits::CommitmentStorage;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory storage implementation
#[derive(Debug, Clone)]
pub struct MemoryStorage {
    commitments: Arc<RwLock<Vec<Commitment>>>,
    tree: Arc<RwLock<MerkleTree>>,
}

impl MemoryStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            commitments: Arc::new(RwLock::new(Vec::new())),
            tree: Arc::new(RwLock::new(MerkleTree::new())),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommitmentStorage for MemoryStorage {
    async fn add_commitment(&self, value: Vec<u8>) -> Result<(usize, Vec<u8>), AppError> {
        let mut commitments = self.commitments.write().await;
        let index = commitments.len();

        // Build leaves for all commitments including the new one
        let mut leaves = Vec::new();
        for c in commitments.iter() {
            leaves.push(MerkleNode::new_leaf(&c.value));
        }
        leaves.push(MerkleNode::new_leaf(&value));

        // Build merkle tree
        let tree = MerkleTree::from_leaves(leaves);
        let merkle_root = tree
            .root_hash()
            .ok_or(AppError::TreeBuildError("Failed to build tree".to_string()))?;

        // Store commitment
        let commitment = Commitment::new(index, value, merkle_root.clone());
        commitments.push(commitment);

        // Update tree
        *self.tree.write().await = tree;

        Ok((index, merkle_root))
    }

    async fn get_commitment(&self, index: usize) -> Result<Commitment, AppError> {
        let commitments = self.commitments.read().await;
        commitments
            .get(index)
            .cloned()
            .ok_or(AppError::NotFound(format!(
                "Commitment with index {} not found",
                index
            )))
    }

    async fn get_all_commitments(&self) -> Result<Vec<Commitment>, AppError> {
        let commitments = self.commitments.read().await;
        Ok(commitments.clone())
    }

    async fn get_tree(&self) -> Result<MerkleTree, AppError> {
        let tree = self.tree.read().await;
        Ok(tree.clone())
    }

    async fn get_root_hash(&self) -> Result<Vec<u8>, AppError> {
        let tree = self.tree.read().await;
        tree.root_hash()
            .ok_or(AppError::NotFound("No root hash available".to_string()))
    }

    async fn commitment_count(&self) -> Result<usize, AppError> {
        let commitments = self.commitments.read().await;
        Ok(commitments.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_get_commitment() {
        let storage = MemoryStorage::new();
        let data = b"test data".to_vec();

        let (index, _root) = storage.add_commitment(data.clone()).await.unwrap();
        assert_eq!(index, 0);

        let commitment = storage.get_commitment(index).await.unwrap();
        assert_eq!(commitment.value, data);
    }

    #[tokio::test]
    async fn test_multiple_commitments() {
        let storage = MemoryStorage::new();

        for i in 0..5 {
            let data = format!("data{}", i).into_bytes();
            storage.add_commitment(data).await.unwrap();
        }

        let count = storage.commitment_count().await.unwrap();
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_get_nonexistent_commitment() {
        let storage = MemoryStorage::new();
        let result = storage.get_commitment(999).await;
        assert!(result.is_err());
    }
}