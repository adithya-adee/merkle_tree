use crate::api::state::AppState;
use crate::crypto::proof::{generate_proof, MerkleProof};
use crate::dto::response::{CommitmentResponse, ProofResponse};
use crate::error::AppError;

use axum::{
    extract::{Path, State},
    Json,
};

/// Get a specific commitment by index
pub async fn get_commitment(
    State(state): State<AppState>,
    Path(index): Path<usize>,
) -> Result<Json<CommitmentResponse>, AppError> {
    let commitment = state.storage.get_commitment(index).await?;
    Ok(Json(commitment.into()))
}

/// Get Merkle proof for a commitment
pub async fn get_proof(
    State(state): State<AppState>,
    Path(index): Path<usize>,
) -> Result<Json<ProofResponse>, AppError> {
    // Get commitment
    let commitment = state.storage.get_commitment(index).await?;

    // Get tree
    let tree = state.storage.get_tree().await?;
    let root = tree.root().ok_or(AppError::NotFound(
        "Merkle tree root not found".to_string(),
    ))?;

    // Generate proof
    let proof_elements = generate_proof(root, index, tree.leaf_count());

    let proof = MerkleProof::new(
        index,
        commitment.value,
        proof_elements,
        root.hash.clone(),
    );

    Ok(Json(proof.into()))
}

/// Verify a Merkle proof
pub async fn verify_proof(
    State(_state): State<AppState>,
    Json(proof): Json<MerkleProof>,
) -> Result<Json<bool>, AppError> {
    let is_valid = proof.verify();
    Ok(Json(is_valid))
}