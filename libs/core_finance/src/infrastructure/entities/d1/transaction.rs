use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::domain::entities::Transaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct D1Transaction {
  pub id: String,
  pub transaction_type_id: i32,
  pub amount: i32,
  pub created_at: String,
  pub updated_at: String,
  pub tag: Option<String>,
  pub r#type: Option<String>,
}

impl From<Vec<D1Transaction>> for Transaction {
  fn from(transactions: Vec<D1Transaction>) -> Transaction {
    transactions
      .into_iter()
      .fold(BTreeMap::new(), |mut acc, transaction| {
        acc
          .entry(transaction.id.clone())
          .or_insert_with(|| Transaction {
            id: transaction.id,
            r#type: transaction.r#type,
            amount: transaction.amount,
            tags: Some(Vec::new()),
            created_at: transaction.created_at,
            updated_at: transaction.updated_at,
          })
          .tags
          .as_mut()
          .map(|tags| if let Some(tag) = transaction.tag { tags.push(tag) });

        acc
      })
      .into_iter()
      .map(|(_, v)| v)
      .collect::<Vec<Transaction>>()
      .get(0)
      .unwrap()
      .clone()
  }
}
