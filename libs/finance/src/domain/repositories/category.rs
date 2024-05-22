use crate::domain::{
  entities::{Category, CategoryCreateParams},
  repositories::repository::{PaginationResult, QueryParams, RepositoryResult},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryQueryParams {
  pub size: Option<usize>,
  pub cursor: Option<String>,
  pub show_deleted: Option<bool>,
}

impl CategoryQueryParams {
  pub fn new(size: Option<usize>, cursor: Option<String>, show_deleted: Option<bool>) -> Self {
    Self {
      size,
      cursor,
      show_deleted,
    }
  }
}

impl QueryParams for CategoryQueryParams {
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
pub trait CategoryRepository {
  async fn create(&self, params: CategoryCreateParams) -> RepositoryResult<Category>;
  async fn retrieve(&self, category_id: String) -> RepositoryResult<Category>;
  async fn list(&self, params: CategoryQueryParams)
    -> RepositoryResult<PaginationResult<Category>>;
}
