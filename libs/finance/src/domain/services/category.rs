use async_trait::async_trait;

use crate::domain::entities::{Category, CategoryCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{CategoryQueryParams, PaginationResult};

#[async_trait(?Send)]
pub trait CategoryService {
  async fn create(&self, params: CategoryCreateParams) -> Result<Category, CommonError>;
  async fn retrieve(&self, category_id: String) -> Result<Category, CommonError>;
  async fn list(
    &self,
    params: CategoryQueryParams,
  ) -> Result<PaginationResult<Category>, CommonError>;
}
