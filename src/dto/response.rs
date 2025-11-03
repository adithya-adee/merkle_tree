use crate::crypto::proof::MerkleProof;
use crate::models::commitment::Commitment;
use serde::Serialize;

/// Response after adding a commitment
#[derive(Debug, Serialize)]
pub struct AddCommitmentResponse {
    /// The assigned index
    pub index: usize,
    /// The Merkle root hash (raw bytes)
    pub merkle_root: Vec<u8>,
}

impl AddCommitmentResponse {
    pub fn new(index: usize, merkle_root: Vec<u8>) -> Self {
        Self {
            index,
            merkle_root,
        }
    }
}

/// Response containing a commitment
#[derive(Debug, Serialize)]
pub struct CommitmentResponse {
    pub commitment: Commitment,
    /// Value as bytes
    pub value: Vec<u8>,
    /// Root as bytes
    pub root: Vec<u8>,
}

impl From<Commitment> for CommitmentResponse {
    fn from(commitment: Commitment) -> Self {
        Self {
            value: commitment.value.clone(),
            root: commitment.merkle_root.clone(),
            commitment,
        }
    }
}

/// Response containing a Merkle proof
#[derive(Debug, Serialize)]
pub struct ProofResponse {
    pub proof: MerkleProof,
    /// Root as bytes
    pub root: Vec<u8>,
}

impl From<MerkleProof> for ProofResponse {
    fn from(proof: MerkleProof) -> Self {
        Self {
            root: proof.root.clone(),
            proof,
        }
    }
}

/// Response for root hash query
#[derive(Debug, Serialize)]
pub struct RootResponse {
    /// Root hash (raw bytes)
    pub root: Vec<u8>,
    /// Number of commitments in the tree
    pub commitment_count: usize,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub commitment_count: usize,
}