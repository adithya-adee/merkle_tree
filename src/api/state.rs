use crate::storage::traits::CommitmentStorage;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<dyn CommitmentStorage>,
}

impl AppState {
    pub fn new(storage: Arc<dyn CommitmentStorage>) -> Self {
        Self { storage }
    }
}