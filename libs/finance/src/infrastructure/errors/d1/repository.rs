use worker::D1Error;

use crate::domain::errors::RepositoryError;
use axum::http::StatusCode;

pub struct D1RepositoryError(RepositoryError);

impl D1RepositoryError {
  pub fn into_inner(self) -> RepositoryError {
    self.0
  }
}

impl From<RepositoryError> for D1RepositoryError {
  fn from(error: RepositoryError) -> D1RepositoryError {
    D1RepositoryError(RepositoryError::new(
      error.message,
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    ))
  }
}

impl From<String> for D1RepositoryError {
  fn from(error: String) -> D1RepositoryError {
    D1RepositoryError(RepositoryError::new(
      error,
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    ))
  }
}

impl From<worker::Error> for D1RepositoryError {
  fn from(error: worker::Error) -> D1RepositoryError {
    D1RepositoryError(RepositoryError::new(
      error.to_string(),
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    ))
  }
}

impl From<D1Error> for D1RepositoryError {
  fn from(error: D1Error) -> D1RepositoryError {
    D1RepositoryError(RepositoryError::new(
      error.to_string(),
      StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    ))
  }
}
