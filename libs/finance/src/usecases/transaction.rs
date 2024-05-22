use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::{Transaction, TransactionCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{
  PaginationResult, TransactionQueryParams, TransactionRepository,
};
use crate::domain::services::TransactionService;

#[derive(Clone)]
pub struct TransactionUsecase {
  pub repository: Arc<dyn TransactionRepository>,
}

impl TransactionUsecase {
  pub fn new(repository: Arc<dyn TransactionRepository>) -> Self {
    TransactionUsecase { repository }
  }
}

#[async_trait(?Send)]
impl TransactionService for TransactionUsecase {
  async fn create(&self, params: TransactionCreateParams) -> Result<Transaction, CommonError> {
    self
      .repository
      .create(params.clone())
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn retrieve(&self, transaction_id: String) -> Result<Transaction, CommonError> {
    self
      .repository
      .retrieve(transaction_id)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn list(
    &self,
    params: TransactionQueryParams,
  ) -> Result<PaginationResult<Transaction>, CommonError> {
    self
      .repository
      .list(params)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }
}
