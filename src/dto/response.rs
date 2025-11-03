use crate::crypto::proof::MerkleProof;
use crate::models::commitment::Commitment;
use serde::Serialize;

/// Response after adding a commitment
#[derive(Debug, Serialize)]
pub struct AddCommitmentResponse {
    /// The assigned index
    pub index: usize,
    /// The Merkle root hash (hex encoded)
    pub merkle_root: String,
    /// The Merkle root hash (raw bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_root_bytes: Option<Vec<u8>>,
}

impl AddCommitmentResponse {
    pub fn new(index: usize, merkle_root: Vec<u8>) -> Self {
        Self {
            index,
            merkle_root: hex::encode(&merkle_root),
            merkle_root_bytes: Some(merkle_root),
        }
    }
}

/// Response containing a commitment
#[derive(Debug, Serialize)]
pub struct CommitmentResponse {
    pub commitment: Commitment,
    /// Value as hex string for readability
    pub value_hex: String,
    /// Root as hex string for readability
    pub root_hex: String,
}

impl From<Commitment> for CommitmentResponse {
    fn from(commitment: Commitment) -> Self {
        Self {
            value_hex: hex::encode(&commitment.value),
            root_hex: hex::encode(&commitment.merkle_root),
            commitment,
        }
    }
}

/// Response containing a Merkle proof
#[derive(Debug, Serialize)]
pub struct ProofResponse {
    pub proof: MerkleProof,
    /// Proof visualization for debugging
    pub proof_hex: Vec<String>,
    /// Root as hex string
    pub root_hex: String,
}

impl From<MerkleProof> for ProofResponse {
    fn from(proof: MerkleProof) -> Self {
        Self {
            proof_hex: proof
                .proof
                .iter()
                .map(|p| hex::encode(&p.hash))
                .collect(),
            root_hex: hex::encode(&proof.root),
            proof,
        }
    }
}

/// Response for root hash query
#[derive(Debug, Serialize)]
pub struct RootResponse {
    /// Root hash (hex encoded)
    pub root: String,
    /// Root hash (raw bytes)
    pub root_bytes: Vec<u8>,
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