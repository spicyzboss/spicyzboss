use serde::{Deserialize, Serialize};

use crate::domain::entities::TagCreateParams;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagCreateParamsDTO {
  pub name: String,
}

impl From<TagCreateParamsDTO> for TagCreateParams {
  fn from(params: TagCreateParamsDTO) -> Self {
    TagCreateParams { name: params.name }
  }
}

impl From<TagCreateParams> for TagCreateParamsDTO {
  fn from(params: TagCreateParams) -> Self {
    TagCreateParamsDTO { name: params.name }
  }
}
