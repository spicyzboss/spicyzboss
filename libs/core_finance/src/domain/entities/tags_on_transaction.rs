use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagsOnTransaction {
  pub transaction_id: String,
  pub tag_id: i32,
  pub created_at: String,
  pub updated_at: String,
}
