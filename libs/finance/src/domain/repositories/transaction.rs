use crate::domain::{
  entities::{Transaction, TransactionCreateParams},
  repositories::repository::{PaginationResult, QueryParams, RepositoryResult},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransactionQueryParams {
  size: Option<usize>,
  cursor: Option<String>,
  show_deleted: Option<bool>,
  pub tags: Option<Vec<String>>,
}

impl TransactionQueryParams {
  pub fn new(
    tags: Option<Vec<String>>,
    size: Option<usize>,
    cursor: Option<String>,
    show_deleted: Option<bool>,
  ) -> Self {
    Self {
      size,
      cursor,
      tags,
      show_deleted,
    }
  }
}

impl QueryParams for TransactionQueryParams {
  fn get_size(&self) -> Option<usize> {
    self.size
  }

  fn get_cursor(&self) -> Option<String> {
    self.cursor.to_owned()
  }

  fn get_show_deleted(&self) -> Option<bool> {
    self.show_deleted
  }
}

#[async_trait(?Send)]
pub trait TransactionRepository {
  async fn create(&self, params: TransactionCreateParams) -> RepositoryResult<Transaction>;
  async fn retrieve(&self, transaction_id: String) -> RepositoryResult<Transaction>;
  async fn list(
    &self,
    params: TransactionQueryParams,
  ) -> RepositoryResult<PaginationResult<Transaction>>;
}
