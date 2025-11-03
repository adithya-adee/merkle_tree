use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A node in the Merkle tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    /// Create a new leaf node from raw data
    pub fn new_leaf(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Self {
            hash: hasher.finalize().to_vec(),
            left: None,
            right: None,
        }
    }

    /// Create a new parent node from two children
    pub fn new_parent(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        Self {
            hash: hasher.finalize().to_vec(),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    /// Get the root hash
    pub fn root_hash(&self) -> &[u8] {
        &self.hash
    }

    /// Check if this is a leaf node
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// Merkle tree structure
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaf_count: usize,
}

impl MerkleTree {
    /// Create a new empty Merkle tree
    pub fn new() -> Self {
        Self {
            root: None,
            leaf_count: 0,
        }
    }

    /// Build a Merkle tree from a list of leaf nodes
    pub fn from_leaves(leaves: Vec<MerkleNode>) -> Self {
        let leaf_count = leaves.len();
        let root = Self::build_tree(leaves);
        Self { root, leaf_count }
    }

    /// Get the root node
    pub fn root(&self) -> Option<&MerkleNode> {
        self.root.as_ref()
    }

    /// Get the root hash
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|r| r.hash.clone())
    }

    /// Get the number of leaves in the tree
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }

    /// Build the tree from leaves (internal recursive function)
    fn build_tree(leaves: Vec<MerkleNode>) -> Option<MerkleNode> {
        if leaves.is_empty() {
            return None;
        }

        let mut current_level = leaves;

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(MerkleNode::new_parent(
                        chunk[0].clone(),
                        chunk[1].clone(),
                    ));
                } else {
                    // Odd number of nodes, duplicate the last one
                    next_level.push(MerkleNode::new_parent(
                        chunk[0].clone(),
                        chunk[0].clone(),
                    ));
                }
            }

            current_level = next_level;
        }

        current_level.into_iter().next()
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_creation() {
        let data = b"test data";
        let leaf = MerkleNode::new_leaf(data);
        
        assert!(leaf.is_leaf());
        assert_eq!(leaf.hash.len(), 32); // SHA-256 produces 32 bytes
    }

    #[test]
    fn test_parent_creation() {
        let left = MerkleNode::new_leaf(b"left");
        let right = MerkleNode::new_leaf(b"right");
        let parent = MerkleNode::new_parent(left.clone(), right.clone());

        assert!(!parent.is_leaf());
        assert_eq!(parent.left.as_ref().unwrap().hash, left.hash);
        assert_eq!(parent.right.as_ref().unwrap().hash, right.hash);
    }

    #[test]
    fn test_tree_building() {
        let leaves = vec![
            MerkleNode::new_leaf(b"data1"),
            MerkleNode::new_leaf(b"data2"),
            MerkleNode::new_leaf(b"data3"),
        ];

        let tree = MerkleTree::from_leaves(leaves);
        
        assert_eq!(tree.leaf_count(), 3);
        assert!(tree.root().is_some());
        assert!(tree.root_hash().is_some());
    }

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new();
        
        assert_eq!(tree.leaf_count(), 0);
        assert!(tree.root().is_none());
        assert!(tree.root_hash().is_none());
    }
}