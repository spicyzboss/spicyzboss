use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
  pub id: String,
  pub name: String,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub deleted_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreateParams {
  pub name: String,
}
