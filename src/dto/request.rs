use serde::Deserialize;

/// Request to add a new commitment
#[derive(Debug, Deserialize)]
pub struct AddCommitmentRequest {
    /// The data to commit (as byte array)
    pub value: Vec<u8>,
}

impl AddCommitmentRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.value.is_empty() {
            return Err("Value cannot be empty".to_string());
        }
        if self.value.len() > 1_000_000 {
            // 1MB limit
            return Err("Value too large (max 1MB)".to_string());
        }
        Ok(())
    }
}