use std::sync::Arc;

use crate::{
  domain::{
    entities::{Transaction, TransactionCreateParams},
    repositories::{
      PaginationResult, RepositoryResult, TransactionQueryParams, TransactionRepository,
    },
  },
  infrastructure::{errors::D1RepositoryError, D1Transaction},
};

use async_trait::async_trait;
use futures::future;
use uuid::Uuid;
use worker::D1Database;

pub struct TransactionD1Repository {
  pub db: Arc<D1Database>,
}

impl TransactionD1Repository {
  pub fn new(db: Arc<D1Database>) -> Self {
    TransactionD1Repository { db }
  }
}

#[async_trait(?Send)]
impl TransactionRepository for TransactionD1Repository {
  async fn create(
    &self,
    new_transaction: TransactionCreateParams,
  ) -> RepositoryResult<Transaction> {
    let id = Uuid::new_v4().to_string();

    let create_transaction_statement = self.db.prepare(
      "
      INSERT INTO transactions (id, amount, transaction_type_id)
      VALUES (?1, ?2, ?3);
      ",
    );

    let transaction_query = create_transaction_statement
      .bind(&[
        id.clone().into(),
        new_transaction.amount.into(),
        new_transaction.transaction_type_id.into(),
      ])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let transaction_query_result = transaction_query.run().await;

    match transaction_query_result {
      Ok(_) => {}
      Err(e) => return Err(D1RepositoryError::from(e).into_inner()),
    }

    // TODO: This is a bad way to do this, should batch these statements
    if let Some(tags) = new_transaction.tags {
      let transaction_tag_queries = tags
        .iter()
        .map(|tag| {
          let create_transaction_tag_statement = self.db.prepare(
            "
            INSERT INTO transactions_tags (transaction_id, tag_id)
            VALUES (?1, ?2);
            ",
          );

          create_transaction_tag_statement
            .bind(&[id.clone().into(), tag.clone().into()])
            .map_err(|v| D1RepositoryError::from(v).into_inner())
            .unwrap()
        })
        .map(|query| async move { query.run().await });

      future::join_all(transaction_tag_queries).await;
    }

    let result = self
      .retrieve(id)
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(result.into())
  }

  async fn retrieve(&self, transaction_id: String) -> RepositoryResult<Transaction> {
    let statement = self.db.prepare(
      "
      SELECT
        t.id,
        type.name 'type',
        t.amount,
        (
          SELECT GROUP_CONCAT(tg.name)
          FROM tags as tg
          JOIN transactions_tags as ttg
          ON ttg.tag_id = tg.id
          WHERE ttg.transaction_id = t.id
        ) 'tags',
        t.created_at,
        t.updated_at
      FROM transactions as t
      JOIN transaction_types as type
      ON t.transaction_type_id = type.id
      WHERE t.id = ?1
      ",
    );

    let query = statement
      .bind(&[transaction_id.clone().into()])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let result = query
      .first::<D1Transaction>(None)
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?
      .ok_or(String::from("Transaction not found"))
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(result.into())
  }

  async fn list(
    &self,
    _params: TransactionQueryParams,
  ) -> RepositoryResult<PaginationResult<Transaction>> {
    let statement = self.db.prepare(
      "
      SELECT
        t.id,
        type.name 'type',
        t.amount,
        (
          SELECT GROUP_CONCAT(tg.name)
          FROM tags as tg
          JOIN transactions_tags as ttg
          ON ttg.tag_id = tg.id
          WHERE ttg.transaction_id = t.id
        ) 'tags',
        t.created_at,
        t.updated_at
      FROM transactions as t
      JOIN transaction_types as type
      ON t.transaction_type_id = type.id
      ",
    );

    let query = statement
      .bind(&[])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let d1_result = query
      .all()
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let result = d1_result
      .results::<D1Transaction>()
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(PaginationResult {
      total: result.len() as i64,
      items: result.into_iter().map(|v| v.into()).collect(),
    })
  }
}
