use crate::domain::{
  entities::{Tag, TagCreateParams},
  repositories::repository::{PaginationResult, QueryParams, RepositoryResult},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagQueryParams {
  pub size: Option<usize>,
  pub cursor: Option<String>,
  pub show_deleted: Option<bool>,
}

impl TagQueryParams {
  pub fn new(size: Option<usize>, cursor: Option<String>, show_deleted: Option<bool>) -> Self {
    Self {
      size,
      cursor,
      show_deleted,
    }
  }
}

impl QueryParams for TagQueryParams {
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
pub trait TagRepository {
  async fn create(&self, params: TagCreateParams) -> RepositoryResult<Tag>;
  async fn retrieve(&self, tag_id: String) -> RepositoryResult<Tag>;
  async fn list(&self, params: TagQueryParams) -> RepositoryResult<PaginationResult<Tag>>;
}
