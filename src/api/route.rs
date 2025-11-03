use crate::{api::{handlers, state::AppState}, dto::response::HealthResponse};
use axum::{
    Json, Router, extract::State, routing::{get, post}
};
// use tower_http::trace::TraceLayer;

/// Create the application router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(|State(state): State<AppState>| async move {
            let commitment_count = state.storage.commitment_count().await.unwrap_or(0);
            Json(HealthResponse {
                status: "healthy".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                commitment_count,
            })
        }))

        // Commitment endpoints
        .route("/api/v1/commitments", post(handlers::commitment::add_commitment))
        .route("/api/v1/commitments", get(handlers::commitment::get_all_commitments))
        .route("/api/v1/commitments/{index}", get(handlers::proof::get_commitment))

        // Proof endpoints
        .route("/api/v1/proof/{index}", get(handlers::proof::get_proof))
        .route("/api/v1/proof/verify", post(handlers::proof::verify_proof))

        // Root endpoint
        .route("/api/v1/root", get(handlers::commitment::get_root))

        // Add tracing middleware
        // .layer(TraceLayer::new_for_http())
        .with_state(state)
}