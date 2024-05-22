use std::sync::Arc;

use crate::{
  domain::{
    entities::{Transaction, TransactionCreateParams},
    repositories::{
      PaginationMetadata, PaginationResult, QueryParams, RepositoryResult, TransactionQueryParams,
      TransactionRepository,
    },
  },
  infrastructure::{errors::D1RepositoryError, D1Transaction},
};

use async_trait::async_trait;
use uuid::Uuid;
use worker::{D1Database, D1Type};

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
  async fn create(&self, params: TransactionCreateParams) -> RepositoryResult<Transaction> {
    let id = Uuid::now_v7().to_string();

    let create_tx_stmt = self.db.prepare(
      "
      INSERT INTO transactions (id, amount, category_id)
      VALUES (?1, ?2, ?3);
      ",
    );

    let binded_create_txt_stmt = create_tx_stmt
      .bind(&[
        id.clone().into(),
        params.amount.into(),
        params.category_id.into(),
      ])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let _create_tx_result = binded_create_txt_stmt
      .run()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    if let Some(tags) = params.tags.filter(|tags| !tags.is_empty()) {
      const TX_TAG_STMT_ARGS_SIZE: usize = 2;
      let create_tx_tag_stmt = self.db.prepare(
        "
        INSERT INTO transactions_tags (transaction_id, tag_id)
        VALUES (?1, ?2);
        ",
      );

      let mut tags_args: Vec<D1Type> = Vec::new();

      for tag in tags.iter() {
        tags_args.push(D1Type::Text(&id));
        tags_args.push(D1Type::Text(tag));
      }

      let binded_tag_args: Vec<Vec<&D1Type>> = tags_args
        .chunks(TX_TAG_STMT_ARGS_SIZE)
        .map(|args| args.iter().collect())
        .collect();

      let binded_create_tx_tag_stmts = create_tx_tag_stmt
        .batch_bind(binded_tag_args)
        .map_err(|e| D1RepositoryError::from(e).into_inner())?;

      let _create_tx_tag_results = self
        .db
        .batch(binded_create_tx_tag_stmts)
        .await
        .map_err(|e| D1RepositoryError::from(e).into_inner())?;
    }

    let tx_result = self
      .retrieve(id)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(tx_result.into())
  }

  async fn retrieve(&self, transaction_id: String) -> RepositoryResult<Transaction> {
    let retrieve_tx_stmt = self.db.prepare(
      "
      SELECT
        t.id,
        c.name 'category',
        t.amount,
        (
          SELECT GROUP_CONCAT(tg.name)
          FROM tags as tg
          JOIN transactions_tags as ttg
          ON ttg.tag_id = tg.id
          WHERE ttg.transaction_id = t.id
        ) 'tags',
        t.created_at,
        t.updated_at,
        t.deleted_at
      FROM transactions as t
      JOIN categories as c
      ON t.category_id = c.id
      WHERE t.id = ?1 AND t.deleted_at IS NULL
      ",
    );

    let binded_retrieve_tx_stmt = retrieve_tx_stmt
      .bind(&[transaction_id.clone().into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let retrieve_tx_result = binded_retrieve_tx_stmt
      .first::<D1Transaction>(None)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .ok_or(String::from("Transaction not found"))
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(retrieve_tx_result.into())
  }

  async fn list(
    &self,
    params: TransactionQueryParams,
  ) -> RepositoryResult<PaginationResult<Transaction>> {
    let size = params.size();
    let cursor = params.cursor();
    let show_deleted = params.show_deleted();

    let count_txs_stmt = self.db.prepare(format!(
      "
      SELECT COUNT(*) 'count'
      FROM transactions
      {}
      ",
      if show_deleted {
        ""
      } else {
        "WHERE deleted_at IS NULL"
      }
    ));

    let list_txs_stmt = self.db.prepare(format!(
      "
      SELECT
        t.id,
        c.name 'category',
        t.amount,
        (
          SELECT GROUP_CONCAT(tg.name)
          FROM tags as tg
          JOIN transactions_tags as ttg
          ON ttg.tag_id = tg.id
          WHERE ttg.transaction_id = t.id
        ) 'tags',
        t.created_at,
        t.updated_at,
        t.deleted_at
      FROM transactions as t
      JOIN categories as c
      ON t.category_id = c.id
      WHERE
        t.id > ?1
        {}
      ORDER BY t.id ASC
      LIMIT ?2
      ",
      if show_deleted {
        ""
      } else {
        "AND t.deleted_at IS NULL"
      }
    ));

    let binded_count_txs_stmt = count_txs_stmt
      .bind(&[])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let count_txs_result = binded_count_txs_stmt
      .first::<usize>(Some("count"))
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .unwrap_or_default();

    let binded_list_txs_stmt = list_txs_stmt
      .bind(&[cursor.into(), size.into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let list_txs_result = binded_list_txs_stmt
      .all()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .results::<D1Transaction>()
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(PaginationResult::new(
      PaginationMetadata::new(
        count_txs_result,
        size,
        list_txs_result.last().map(|v| v.id.to_string()),
      ),
      list_txs_result.into_iter().map(|v| v.into()).collect(),
    ))
  }
}
