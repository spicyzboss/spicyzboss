use serde::{Deserialize, Serialize};

use crate::domain::entities::CategoryCreateParams;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreateParamsDTO {
  pub name: String,
}

impl From<CategoryCreateParamsDTO> for CategoryCreateParams {
  fn from(params: CategoryCreateParamsDTO) -> Self {
    CategoryCreateParams { name: params.name }
  }
}

impl From<CategoryCreateParams> for CategoryCreateParamsDTO {
  fn from(params: CategoryCreateParams) -> Self {
    CategoryCreateParamsDTO { name: params.name }
  }
}
