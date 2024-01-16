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
use futures::future;
use uuid::Uuid;
use worker::D1Database;

use async_trait::async_trait;

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

    let create_txn_statement = self.db.prepare(
      "
      INSERT INTO transactions (id, amount, transaction_type_id)
      VALUES (?1, ?2, ?3);
      ",
    );

    let txn_query = create_txn_statement
      .bind(&[
        id.clone().into(),
        new_transaction.amount.into(),
        new_transaction.transaction_type_id.into(),
      ])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let txn_query_result = txn_query.run().await;

    match txn_query_result {
      Ok(_) => {}
      Err(e) => return Err(D1RepositoryError::from(e).into_inner()),
    }

    // TODO: This is a bad way to do this, should batch these statements
    if let Some(tags) = new_transaction.tags {
      let txn_tag_queries = tags
        .iter()
        .map(|tag| {
          let create_txn_tag_statement = self.db.prepare(
            "
            INSERT INTO transactions_tags (transaction_id, tag_id)
            VALUES (?1, ?2);
            ",
          );

          create_txn_tag_statement
            .bind(&[id.clone().into(), tag.clone().into()])
            .map_err(|v| D1RepositoryError::from(v).into_inner())
            .unwrap()
        })
        .map(|query| async move { query.run().await });

      future::join_all(txn_tag_queries).await;
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
        t.transaction_type_id,
        t.amount,
        t.created_at,
        t.updated_at,
        tg.name 'tag',
        (
          SELECT tt.name
          FROM transaction_types as tt
          WHERE tt.id = t.transaction_type_id
        ) 'type'
      FROM transactions as t
      LEFT JOIN transactions_tags as tot
      ON t.id = tot.transaction_id
      LEFT JOIN tags as tg
      ON tg.id = tot.tag_id
      WHERE t.id = ?1;
      ",
    );

    let query = statement
      .bind(&[transaction_id.into()])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let option_result_with_err = query
      .all()
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let result = option_result_with_err
      .results::<D1Transaction>()
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
        t.transaction_type_id,
        t.amount,
        t.created_at,
        t.updated_at,
        tg.name 'tag',
        (
          SELECT tt.name
          FROM transaction_types as tt
          WHERE tt.id = t.transaction_type_id
        ) 'type'
      FROM transactions as t
      LEFT JOIN transactions_tags as tot
      ON t.id = tot.transaction_id
      LEFT JOIN tags as tg
      ON tg.id = tot.tag_id
      ",
    );

    let query = statement.bind(&[]).expect("statement binding error");

    let option_result_with_err = query
      .all()
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let result = option_result_with_err
      .results::<D1Transaction>()
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(result.into())
  }
}
