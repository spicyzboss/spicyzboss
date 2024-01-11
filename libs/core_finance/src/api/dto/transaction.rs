use serde::{Deserialize, Serialize};

use crate::domain::entities::TransactionCreateParams;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionCreateParamsDTO {
  pub transaction_type_id: i32,
  pub amount: i32,
  pub tags: Option<Vec<i32>>,
}

impl From<TransactionCreateParamsDTO> for TransactionCreateParams {
  fn from(params: TransactionCreateParamsDTO) -> Self {
    TransactionCreateParams {
      tags: params.tags,
      amount: params.amount,
      transaction_type_id: params.transaction_type_id,
    }
  }
}

impl From<TransactionCreateParams> for TransactionCreateParamsDTO {
  fn from(params: TransactionCreateParams) -> Self {
    TransactionCreateParamsDTO {
      tags: params.tags,
      amount: params.amount,
      transaction_type_id: params.transaction_type_id,
    }
  }
}
