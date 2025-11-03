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
pub fn generate_proof(tree: &MerkleNode, target_index: usize, total_leaves: usize) -> Vec<ProofElement> {
    fn helper(node: &MerkleNode, idx: usize, begin: usize, leaves_count: usize) -> Vec<ProofElement> {
        if node.left.is_none() && node.right.is_none() {
            // Leaf node, end of proof path
            return Vec::new();
        }
        // Figure out the size of the left subtree
        let left_count = (leaves_count + 1) / 2;
        let right_count = leaves_count - left_count;
        let right_begin = begin + left_count;

        // Depending on which subtree target is in, recurse accordingly
        if idx < begin + left_count {
            // Target is in left subtree
            let left = node.left.as_ref().unwrap();
            let right = node.right.as_ref().unwrap_or(left); // duplicate left if missing
            let mut proof = helper(left, idx, begin, left_count);
            proof.push(ProofElement {
                hash: right.hash.clone(),
                is_left: false,
            });
            proof
        } else {
            // Target is in right subtree
            let left = node.left.as_ref().unwrap();
            let right = node.right.as_ref().unwrap_or(left); // duplicate left if missing (should never happen here, but for symmetry)
            let mut proof = helper(right, idx, right_begin, right_count);
            proof.push(ProofElement {
                hash: left.hash.clone(),
                is_left: true,
            });
            proof
        }
    }
    helper(tree, target_index, 0, total_leaves)
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

    #[test]
    fn test_proof_three_odd_commitments_all_indices() {
        let leaves = vec![
            MerkleNode::new_leaf(b"aaa"),
            MerkleNode::new_leaf(b"bbb"),
            MerkleNode::new_leaf(b"ccc"),
        ];
        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root().unwrap();
        // Generate/verify for index 0
        let proof0 = MerkleProof::new(
            0,
            b"aaa".to_vec(),
            generate_proof(root, 0, 3),
            root.hash.clone(),
        );
        assert!(proof0.verify());
        // Generate/verify for index 1
        let proof1 = MerkleProof::new(
            1,
            b"bbb".to_vec(),
            generate_proof(root, 1, 3),
            root.hash.clone(),
        );
        assert!(proof1.verify());
        // Generate/verify for index 2
        let proof2 = MerkleProof::new(
            2,
            b"ccc".to_vec(),
            generate_proof(root, 2, 3),
            root.hash.clone(),
        );
        assert!(proof2.verify());
    }
}