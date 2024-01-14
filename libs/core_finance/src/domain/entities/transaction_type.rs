use crate::domain::entities::{Tags, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionType {
  pub transaction_id: String,
  pub tag_id: i32,
  pub created_at: String,
  pub updated_at: String,
  pub transaction: Option<Box<Transaction>>,
  pub tags: Option<Box<Tags>>,
}
