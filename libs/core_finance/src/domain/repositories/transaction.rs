use crate::domain::{
  entities::{Transaction, TransactionCreateParams},
  repositories::repository::{
    PaginationResult, QueryParams, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET,
  },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionQueryParams {
  pub limit: Option<i64>,
  pub offset: Option<i64>,
  pub tags: Option<Vec<String>>,
}

impl QueryParams for TransactionQueryParams {
  fn limit(&self) -> i64 {
    self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
  }

  fn offset(&self) -> i64 {
    self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
  }
}

#[async_trait(?Send)]
pub trait TransactionRepository {
  async fn create(&self, new_transaction: TransactionCreateParams)
    -> RepositoryResult<Transaction>;
  async fn retrieve(&self, transaction_id: String) -> RepositoryResult<Transaction>;
  async fn list(
    &self,
    params: TransactionQueryParams,
  ) -> RepositoryResult<PaginationResult<Transaction>>;
}
