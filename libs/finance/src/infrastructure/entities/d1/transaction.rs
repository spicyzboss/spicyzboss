use serde::{Deserialize, Serialize};

use crate::domain::entities::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub struct D1Transaction {
  pub id: String,
  pub category: String,
  pub amount: i32,
  pub tags: Option<String>,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}

impl From<D1Transaction> for Transaction {
  fn from(transaction: D1Transaction) -> Transaction {
    let tags = match transaction.tags {
      Some(tags) => Some(
        tags
          .split(',')
          .map(|tag| tag.to_string())
          .collect::<Vec<String>>(),
      ),
      None => None,
    };

    Transaction {
      id: transaction.id,
      category: transaction.category,
      amount: transaction.amount,
      created_at: transaction.created_at,
      updated_at: transaction.updated_at,
      deleted_at: transaction.deleted_at,
      tags,
    }
  }
}
