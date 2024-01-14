use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CommonError {
  pub message: String,
  pub code: u16,
}
