use serde::{Deserialize, Serialize};

/// A commitment represents a piece of data committed to the Merkle tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Commitment {
    /// Unique sequential index
    pub index: usize,
    /// The committed data
    pub value: Vec<u8>,
    /// The Merkle root at the time of commitment
    pub merkle_root: Vec<u8>,
}

impl Commitment {
    /// Create a new commitment
    pub fn new(index: usize, value: Vec<u8>, merkle_root: Vec<u8>) -> Self {
        Self {
            index,
            value,
            merkle_root,
        }
    }

    /// Get the commitment index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the committed value
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Get the Merkle root at commitment time
    pub fn merkle_root(&self) -> &[u8] {
        &self.merkle_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_creation() {
        let value = vec![1, 2, 3, 4];
        let root = vec![5, 6, 7, 8];
        let commitment = Commitment::new(0, value.clone(), root.clone());

        assert_eq!(commitment.index(), 0);
        assert_eq!(commitment.value(), &value);
        assert_eq!(commitment.merkle_root(), &root);
    }

    #[test]
    fn test_commitment_serialization() {
        let commitment = Commitment::new(42, vec![1, 2, 3], vec![4, 5, 6]);
        
        let json = serde_json::to_string(&commitment).unwrap();
        let deserialized: Commitment = serde_json::from_str(&json).unwrap();
        
        assert_eq!(commitment, deserialized);
    }
}