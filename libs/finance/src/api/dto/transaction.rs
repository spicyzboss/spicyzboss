use serde::{Deserialize, Serialize};

use crate::domain::entities::TransactionCreateParams;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCreateParamsDTO {
  pub category_id: String,
  pub amount: i32,
  pub tags: Option<Vec<String>>,
}

impl From<TransactionCreateParamsDTO> for TransactionCreateParams {
  fn from(params: TransactionCreateParamsDTO) -> Self {
    TransactionCreateParams {
      tags: params.tags,
      amount: params.amount,
      category_id: params.category_id,
    }
  }
}

impl From<TransactionCreateParams> for TransactionCreateParamsDTO {
  fn from(params: TransactionCreateParams) -> Self {
    TransactionCreateParamsDTO {
      tags: params.tags,
      amount: params.amount,
      category_id: params.category_id,
    }
  }
}
