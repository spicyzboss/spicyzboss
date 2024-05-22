use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::{Tag, TagCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{PaginationResult, TagQueryParams, TagRepository};
use crate::domain::services::TagService;

#[derive(Clone)]
pub struct TagUsecase {
  pub repository: Arc<dyn TagRepository>,
}

impl TagUsecase {
  pub fn new(repository: Arc<dyn TagRepository>) -> Self {
    TagUsecase { repository }
  }
}

#[async_trait(?Send)]
impl TagService for TagUsecase {
  async fn create(&self, params: TagCreateParams) -> Result<Tag, CommonError> {
    self
      .repository
      .create(params.clone())
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn retrieve(&self, tag_id: String) -> Result<Tag, CommonError> {
    self
      .repository
      .retrieve(tag_id)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }

  async fn list(&self, params: TagQueryParams) -> Result<PaginationResult<Tag>, CommonError> {
    self
      .repository
      .list(params)
      .await
      .map_err(|e| -> CommonError { e.into() })
  }
}
