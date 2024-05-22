use async_trait::async_trait;

use crate::domain::entities::{Tag, TagCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{PaginationResult, TagQueryParams};

#[async_trait(?Send)]
pub trait TagService {
  async fn create(&self, params: TagCreateParams) -> Result<Tag, CommonError>;
  async fn retrieve(&self, tag_id: String) -> Result<Tag, CommonError>;
  async fn list(&self, params: TagQueryParams) -> Result<PaginationResult<Tag>, CommonError>;
}
