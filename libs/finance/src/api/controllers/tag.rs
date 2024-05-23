use std::{collections::HashMap, sync::Arc};

use worker::{Env, Request, Response, Result};

use crate::{
  api::TagCreateParamsDTO,
  domain::{errors::CommonError, repositories::TagQueryParams},
  infrastructure::TagD1Repository,
  usecases::TagUsecase,
};

#[worker::send]
pub async fn list_tags(req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TagD1Repository::new(db.clone()));
  let usecase = Arc::new(TagUsecase::new(repo.clone()));

  let query: HashMap<String, String> = req.url().unwrap().query_pairs().into_owned().collect();

  let cursor = query.get("cursor").map(|v| v.to_string());

  let size = query
    .get("size")
    .map(|v| v.parse::<usize>().unwrap_or_default());

  let show_deleted = query
    .get("show_deleted")
    .map(|v| v.to_string().to_lowercase().eq("true"));

  let result = usecase
    .repository
    .list(TagQueryParams::new(size, cursor, show_deleted))
    .await;

  match result {
    Ok(v) => Response::from_json(&v),
    Err(v) => {
      let error: CommonError = v.into();
      let response = Response::from_json(&error)?;

      Ok(response.with_status(error.code))
    }
  }
}

#[worker::send]
pub async fn get_tag(req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TagD1Repository::new(db.clone()));
  let usecase = Arc::new(TagUsecase::new(repo.clone()));

  let url = req.url()?;

  let tag_id = url
    .path_segments()
    .unwrap()
    .nth(1)
    .unwrap()
    .parse::<String>()?;
  let result = usecase.repository.retrieve(tag_id).await;

  match result {
    Ok(v) => Response::from_json(&v),
    Err(v) => {
      let error: CommonError = v.into();
      let response = Response::from_json(&error)?;
      Ok(response.with_status(error.code))
    }
  }
}

#[worker::send]
pub async fn create_tag(mut req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TagD1Repository::new(db.clone()));
  let usecase = Arc::new(TagUsecase::new(repo.clone()));

  let plain_body = req.text().await?;
  let params = serde_json::from_str::<TagCreateParamsDTO>(&plain_body)?;
  let result = usecase.repository.create(params.into()).await;

  match result {
    Ok(v) => Response::from_json(&v),
    Err(v) => {
      let error: CommonError = v.into();
      let response = Response::from_json(&error)?;

      Ok(response.with_status(error.code))
    }
  }
}
