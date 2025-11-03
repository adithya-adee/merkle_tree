use merkle_tree::{api, storage::memory::MemoryStorage};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create storage
    let storage = Arc::new(MemoryStorage::new());

    // Create application state
    let state = api::state::AppState::new(storage);

    // Create router
    let app = api::route::create_router(state);

    // Start server
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);
    println!("📝 API documentation available at /health");

    axum::serve(listener, app).await.unwrap();
}
