use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
  pub id: String,
  pub r#type: String,
  pub amount: i32,
  pub tags: Option<Vec<String>>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionCreateParams {
  pub transaction_type_id: i32,
  pub amount: i32,
  pub tags: Option<Vec<i32>>,
}
