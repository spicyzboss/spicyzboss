use serde::{Deserialize, Serialize};

use crate::domain::entities::Category;

#[derive(Serialize, Deserialize, Clone)]
pub struct D1Category {
  pub id: String,
  pub name: String,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}

impl From<D1Category> for Category {
  fn from(category: D1Category) -> Category {
    Category {
      id: category.id,
      name: category.name,
      created_at: category.created_at,
      updated_at: category.updated_at,
      deleted_at: category.deleted_at,
    }
  }
}
