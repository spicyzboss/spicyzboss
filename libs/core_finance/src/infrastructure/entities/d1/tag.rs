use serde::{Deserialize, Serialize};

use crate::domain::entities::Tag;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct D1Tag {
  pub id: i32,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
}

impl From<D1Tag> for Tag {
  fn from(tag: D1Tag) -> Tag {
    Tag {
      id: tag.id,
      name: tag.name,
      created_at: tag.created_at,
      updated_at: tag.updated_at,
    }
  }
}
