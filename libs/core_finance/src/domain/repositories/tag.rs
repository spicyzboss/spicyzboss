use crate::domain::{
  entities::{Tag, TagCreateParams},
  repositories::repository::{
    PaginationResult, QueryParams, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET,
  },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQueryParams {
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

impl QueryParams for TagQueryParams {
  fn limit(&self) -> i64 {
    self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
  }

  fn offset(&self) -> i64 {
    self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
  }
}

#[async_trait(?Send)]
pub trait TagRepository {
  async fn create(&self, new_tag: TagCreateParams) -> RepositoryResult<Tag>;
  async fn retrieve(&self, transaction_id: String) -> RepositoryResult<Tag>;
  async fn list(&self, params: TagQueryParams) -> RepositoryResult<PaginationResult<Tag>>;
}
