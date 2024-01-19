use serde::{Deserialize, Serialize};

use crate::domain::entities::Transaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct D1Transaction {
  pub id: String,
  pub r#type: String,
  pub amount: i32,
  pub tags: Option<String>,
  pub created_at: String,
  pub updated_at: String,
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
      r#type: transaction.r#type,
      amount: transaction.amount,
      created_at: transaction.created_at,
      updated_at: transaction.updated_at,
      tags,
    }
  }
}
