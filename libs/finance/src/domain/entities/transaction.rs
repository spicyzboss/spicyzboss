use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  pub id: String,
  pub category: String,
  pub amount: i32,
  pub tags: Option<Vec<String>>,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCreateParams {
  pub category_id: String,
  pub amount: i32,
  pub tags: Option<Vec<String>>,
}
