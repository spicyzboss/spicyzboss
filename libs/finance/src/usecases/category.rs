use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::{Category, CategoryCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{CategoryQueryParams, CategoryRepository, PaginationResult};
use crate::domain::services::CategoryService;

#[derive(Clone)]
pub struct CategoryUsecase {
  pub repository: Arc<dyn CategoryRepository>,
}

impl CategoryUsecase {
  pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
    CategoryUsecase { repository }
  }
}

#[async_trait(?Send)]
impl CategoryService for CategoryUsecase {
  async fn create(&self, params: CategoryCreateParams) -> Result<Category, CommonError> {
    self
      .repository
      .create(params.clone())
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn retrieve(&self, category_id: String) -> Result<Category, CommonError> {
    self
      .repository
      .retrieve(category_id)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn list(
    &self,
    params: CategoryQueryParams,
  ) -> Result<PaginationResult<Category>, CommonError> {
    self
      .repository
      .list(params)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }
}
