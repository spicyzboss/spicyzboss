use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Tags {
  pub id: i32,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
}
