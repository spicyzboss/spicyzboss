use serde::{Deserialize, Serialize};

use crate::domain::entities::Tag;

#[derive(Serialize, Deserialize, Clone)]
pub struct D1Tag {
  pub id: String,
  pub name: String,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}

impl From<D1Tag> for Tag {
  fn from(tag: D1Tag) -> Tag {
    Tag {
      id: tag.id,
      name: tag.name,
      created_at: tag.created_at,
      updated_at: tag.updated_at,
      deleted_at: tag.deleted_at,
    }
  }
}
