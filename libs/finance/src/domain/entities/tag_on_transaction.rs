use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagOnTransaction {
  pub transaction_id: String,
  pub tag_id: String,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}
