use crate::api::state::AppState;
use crate::dto::{request::AddCommitmentRequest, response::*};
use crate::error::AppError;

use axum::{extract::State, Json};

/// Add a new commitment
pub async fn add_commitment(
    State(state): State<AppState>,
    Json(req): Json<AddCommitmentRequest>,
) -> Result<Json<AddCommitmentResponse>, AppError> {
    // Validate request
    req.validate()
        .map_err(|e| AppError::InvalidInput(e.to_string()))?;

    // Add commitment
    let (index, merkle_root) = state.storage.add_commitment(req.value).await?;

    Ok(Json(AddCommitmentResponse::new(index, merkle_root)))
}

/// Get all commitments
pub async fn get_all_commitments(
    State(state): State<AppState>,
) -> Result<Json<Vec<CommitmentResponse>>, AppError> {
    let commitments = state.storage.get_all_commitments().await?;
    let responses: Vec<CommitmentResponse> =
        commitments.into_iter().map(|c| c.into()).collect();

    Ok(Json(responses))
}

/// Get current root hash
pub async fn get_root(
    State(state): State<AppState>,
) -> Result<Json<RootResponse>, AppError> {
    let root_bytes = state.storage.get_root_hash().await?;
    let commitment_count = state.storage.commitment_count().await?;

    Ok(Json(RootResponse {
        root: hex::encode(&root_bytes),
        root_bytes,
        commitment_count,
    }))
}