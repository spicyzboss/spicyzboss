use std::sync::Arc;

use crate::{
  domain::{
    entities::{Tag, TagCreateParams},
    repositories::{
      PaginationMetadata, PaginationResult, QueryParams, RepositoryResult, TagQueryParams,
      TagRepository,
    },
  },
  infrastructure::{errors::D1RepositoryError, D1Tag},
};

use async_trait::async_trait;
use uuid::Uuid;
use worker::D1Database;

pub struct TagD1Repository {
  pub db: Arc<D1Database>,
}

impl TagD1Repository {
  pub fn new(db: Arc<D1Database>) -> Self {
    TagD1Repository { db }
  }
}

#[async_trait(?Send)]
impl TagRepository for TagD1Repository {
  async fn create(&self, params: TagCreateParams) -> RepositoryResult<Tag> {
    let id = Uuid::now_v7().to_string();

    let create_tag_stmt = self.db.prepare(
      "
      INSERT INTO tags (id, name)
      VALUES (?1, ?2);
      ",
    );

    let binded_create_tag_stmt = create_tag_stmt
      .bind(&[id.clone().into(), params.name.to_uppercase().into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let _create_tag_result = binded_create_tag_stmt
      .run()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let tag_result = self
      .retrieve(id)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(tag_result)
  }

  async fn retrieve(&self, tag_id: String) -> RepositoryResult<Tag> {
    let receive_tag_stmt = self.db.prepare(
      "
      SELECT
        id,
        name,
        created_at,
        updated_at,
        deleted_at
      FROM tags
      WHERE id = ?1 AND deleted_at IS NULL;
      ",
    );

    let binded_receive_tag_stmt = receive_tag_stmt
      .bind(&[tag_id.into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let receive_tag_result = binded_receive_tag_stmt
      .first::<D1Tag>(None)
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .ok_or(String::from("Tag not found"))
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(receive_tag_result.into())
  }

  async fn list(&self, params: TagQueryParams) -> RepositoryResult<PaginationResult<Tag>> {
    let size = params.size();
    let cursor = params.cursor();
    let show_deleted = params.show_deleted();

    let count_tags_stmt = self.db.prepare(format!(
      "
      SELECT COUNT(*) 'count'
      FROM tags
      {}
      ",
      if show_deleted {
        ""
      } else {
        "WHERE deleted_at IS NULL"
      }
    ));

    let list_tags_stmt = self.db.prepare(format!(
      "
      SELECT
        id,
        name,
        created_at,
        updated_at,
        deleted_at
      FROM tags
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

    let binded_count_tags_stmt = count_tags_stmt
      .bind(&[])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let count_tags_result = binded_count_tags_stmt
      .first::<usize>(Some("count"))
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .unwrap_or_default();

    let binded_list_tags_stmt = list_tags_stmt
      .bind(&[cursor.into(), size.into()])
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    let list_tags_result = binded_list_tags_stmt
      .all()
      .await
      .map_err(|e| D1RepositoryError::from(e).into_inner())?
      .results::<D1Tag>()
      .map_err(|e| D1RepositoryError::from(e).into_inner())?;

    Ok(PaginationResult::new(
      PaginationMetadata::new(
        count_tags_result,
        size,
        list_tags_result.last().map(|v| v.id.to_string()),
      ),
      list_tags_result.into_iter().map(|v| v.into()).collect(),
    ))
  }
}
