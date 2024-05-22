use async_trait::async_trait;

use crate::domain::entities::{Transaction, TransactionCreateParams};
use crate::domain::errors::CommonError;
use crate::domain::repositories::{PaginationResult, TransactionQueryParams};

#[async_trait(?Send)]
pub trait TransactionService {
  async fn create(&self, params: TransactionCreateParams) -> Result<Transaction, CommonError>;
  async fn retrieve(&self, transaction_id: String) -> Result<Transaction, CommonError>;
  async fn list(
    &self,
    params: TransactionQueryParams,
  ) -> Result<PaginationResult<Transaction>, CommonError>;
}
