use crate::models::merkle::MerkleNode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// An element in a Merkle proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofElement {
    /// Hash of the sibling node
    pub hash: Vec<u8>,
    /// True if this is the left sibling, false if right
    pub is_left: bool,
}

/// A complete Merkle proof for a specific commitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Index of the commitment being proved
    pub index: usize,
    /// The committed value
    pub value: Vec<u8>,
    /// The proof path (sibling hashes)
    pub proof: Vec<ProofElement>,
    /// The Merkle root
    pub root: Vec<u8>,
}

impl MerkleProof {
    /// Create a new Merkle proof
    pub fn new(index: usize, value: Vec<u8>, proof: Vec<ProofElement>, root: Vec<u8>) -> Self {
        Self {
            index,
            value,
            proof,
            root,
        }
    }

    /// Verify this proof is valid
    pub fn verify(&self) -> bool {
        let mut current_hash = {
            let mut hasher = Sha256::new();
            hasher.update(&self.value);
            hasher.finalize().to_vec()
        };

        for element in &self.proof {
            let mut hasher = Sha256::new();
            if element.is_left {
                hasher.update(&element.hash);
                hasher.update(&current_hash);
            } else {
                hasher.update(&current_hash);
                hasher.update(&element.hash);
            }
            current_hash = hasher.finalize().to_vec();
        }

        current_hash == self.root
    }
}

/// Generate a Merkle proof for a specific index
pub fn generate_proof(tree_root: &MerkleNode, target_index: usize, total_leaves: usize) -> Vec<ProofElement> {
    let mut proof = Vec::new();

    // Each stack entry: (node, start_index, end_index)
    let mut current_node = tree_root;
    let mut from = 0;
    let mut to = total_leaves - 1;
    let index = target_index;

    while !current_node.is_leaf() {
        let num_leaves = to - from + 1;
        let mid = from + (num_leaves - 1) / 2;
        // Determine if we step left or right at this layer
        if index <= mid {
            // Sibling is on the right
            let sibling = if let Some(ref right) = current_node.right {
                right
            } else {
                // Odd node: sibling is a duplicate of left
                current_node.left.as_ref().expect("Should have left child")
            };
            proof.push(ProofElement {
                hash: sibling.hash.clone(),
                is_left: false,
            });
            // Move down to left child
            current_node = current_node.left.as_ref().expect("Should have left child");
            to = mid;
        } else {
            // Sibling is on the left
            let sibling = if let Some(ref left) = current_node.left {
                left
            } else {
                current_node.right.as_ref().expect("Should have right child")
            };
            proof.push(ProofElement {
                hash: sibling.hash.clone(),
                is_left: true,
            });
            // Move down to right child
            current_node = current_node.right.as_ref().expect("Should have right child");
            from = mid + 1;
        }
    }
    proof
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::merkle::MerkleTree;

    #[test]
    fn test_proof_generation_and_verification() {
        let leaves = vec![
            MerkleNode::new_leaf(b"data0"),
            MerkleNode::new_leaf(b"data1"),
            MerkleNode::new_leaf(b"data2"),
            MerkleNode::new_leaf(b"data3"),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root().unwrap();

        // Generate proof for index 2
        let proof_elements = generate_proof(root, 2, 4);
        let proof = MerkleProof::new(
            2,
            b"data2".to_vec(),
            proof_elements,
            root.hash.clone(),
        );

        // Verify the proof
        assert!(proof.verify());
    }

    #[test]
    fn test_invalid_proof() {
        let leaves = vec![
            MerkleNode::new_leaf(b"data0"),
            MerkleNode::new_leaf(b"data1"),
        ];

        let tree = MerkleTree::from_leaves(leaves);
        let root = tree.root().unwrap();

        // Create an invalid proof (wrong value)
        let proof_elements = generate_proof(root, 0, 2);
        let proof = MerkleProof::new(
            0,
            b"wrong_data".to_vec(),
            proof_elements,
            root.hash.clone(),
        );

        assert!(!proof.verify());
    }
}