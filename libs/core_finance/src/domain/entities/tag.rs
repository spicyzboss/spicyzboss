use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Tag {
  pub id: i32,
  pub name: String,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TagCreateParams {
  pub name: String,
}
