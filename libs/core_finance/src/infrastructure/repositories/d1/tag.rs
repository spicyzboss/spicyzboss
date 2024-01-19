use std::sync::Arc;

use crate::{
  domain::{
    entities::{Tag, TagCreateParams},
    repositories::{PaginationResult, RepositoryResult, TagQueryParams, TagRepository},
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
  async fn create(&self, new_tag: TagCreateParams) -> RepositoryResult<Tag> {
    let id = Uuid::new_v4().to_string();

    let statement = self.db.prepare(
      "
      INSERT INTO tags (name)
      VALUES (?1);
      ",
    );

    let query = statement
      .bind(&[new_tag.name.into()])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let query_result = query
      .run()
      .await;

    match query_result {
      Ok(_) => {}
      Err(e) => return Err(D1RepositoryError::from(e).into_inner()),
    }

    let result = self
      .retrieve(id)
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(result)
  }

  async fn retrieve(&self, tag_id: String) -> RepositoryResult<Tag> {
    let statement = self.db.prepare(
      "
      SELECT id, name, created_at, updated_at
      FROM tags
      WHERE id = ?1;
      ",
    );

    let query = statement
      .bind(&[tag_id.into()])
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    let result = query
      .first::<D1Tag>(None)
      .await
      .map_err(|v| D1RepositoryError::from(v).into_inner())?
      .ok_or(String::from("Tag not found"))
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(result.into())
  }

  async fn list(&self, _params: TagQueryParams) -> RepositoryResult<PaginationResult<Tag>> {
    let statement = self.db.prepare(
      "
      SELECT id, name
      FROM tags
      ORDER BY id ASC
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
      .results::<D1Tag>()
      .map_err(|v| D1RepositoryError::from(v).into_inner())?;

    Ok(PaginationResult {
      total: result.len() as i64,
      items: result.into_iter().map(|v| v.into()).collect(),
    })
  }
}
