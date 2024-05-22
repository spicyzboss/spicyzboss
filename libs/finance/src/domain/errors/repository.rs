use crate::domain::errors::CommonError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryError {
  pub message: String,
  pub code: u16,
}

impl RepositoryError {
  pub fn new(message: String, code: u16) -> Self {
    RepositoryError { message, code }
  }
}

impl Into<CommonError> for RepositoryError {
  fn into(self) -> CommonError {
    CommonError {
      message: self.message,
      code: self.code,
    }
  }
}
