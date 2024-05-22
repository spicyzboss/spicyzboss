use std::sync::Arc;

use crate::{
  domain::{
    entities::{Category, CategoryCreateParams},
    repositories::{
      CategoryQueryParams, CategoryRepository, PaginationMetadata, PaginationResult, QueryParams,
      RepositoryResult,
    },
  },
  infrastructure::{errors::D1RepositoryError, D1Category},
};

use async_trait::async_trait;
use uuid::Uuid;
use worker::D1Database;

pub struct CategoryD1Repository {
  pub db: Arc<D1Database>,
}

impl CategoryD1Repository {
  pub fn new(db: Arc<D1Database>) -> Self {
    CategoryD1Repository { db }
  }
}

#[async_trait(?Send)]
impl CategoryRepository for CategoryD1Repository {
  async fn create(&self, params: CategoryCreateParams) -> RepositoryResult<Category> {
    let id = Uuid::now_v7().to_string();

    let create_ctg_stmt = self.db.prepare(
      "
      INSERT INTO categories (id, name)
      VALUES (?1, ?2);
      ",
    );

    let binded_create_ctg_stmt = create_ctg_stmt
      .bind(&[id.clone().into(), params.name.to_uppercase().into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let _create_ctg_result = binded_create_ctg_stmt
      .run()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let ctg_result = self
      .retrieve(id)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(ctg_result)
  }

  async fn retrieve(&self, tag_id: String) -> RepositoryResult<Category> {
    let receive_ctg_stmt = self.db.prepare(
      "
      SELECT
        id,
        name,
        created_at,
        updated_at,
        deleted_at
      FROM categories
      WHERE id = ?1 AND deleted_at IS NULL;
      ",
    );

    let binded_receive_ctg_stmt = receive_ctg_stmt
      .bind(&[tag_id.into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let receive_ctg_result = binded_receive_ctg_stmt
      .first::<D1Category>(None)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .ok_or(String::from("Category not found"))
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(receive_ctg_result.into())
  }

  async fn list(
    &self,
    params: CategoryQueryParams,
  ) -> RepositoryResult<PaginationResult<Category>> {
    let size = params.size();
    let cursor = params.cursor();
    let show_deleted = params.show_deleted();

    let count_ctgs_stmt = self.db.prepare(format!(
      "
      SELECT COUNT(*) 'count'
      FROM categories
      {}
      ",
      if show_deleted {
        ""
      } else {
        "WHERE deleted_at IS NULL"
      }
    ));

    let list_ctgs_stmt = self.db.prepare(format!(
      "
      SELECT
        id,
        name,
        created_at,
        updated_at,
        deleted_at
      FROM categories
      WHERE
        id > ?1
        {}
      ORDER BY id ASC
      LIMIT ?2
      ",
      if show_deleted {
        ""
      } else {
        "AND deleted_at IS NULL"
      }
    ));

    let binded_count_ctgs_stmt = count_ctgs_stmt
      .bind(&[])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let count_ctgs_result = binded_count_ctgs_stmt
      .first::<usize>(Some("count"))
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .unwrap_or_default();

    let binded_list_ctgs_stmt = list_ctgs_stmt
      .bind(&[cursor.into(), size.into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let list_ctgs_result = binded_list_ctgs_stmt
      .all()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .results::<D1Category>()
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(PaginationResult::new(
      PaginationMetadata::new(
        count_ctgs_result,
        size,
        list_ctgs_result.last().map(|v| v.id.to_string()),
      ),
      list_ctgs_result.into_iter().map(|v| v.into()).collect(),
    ))
  }
}
