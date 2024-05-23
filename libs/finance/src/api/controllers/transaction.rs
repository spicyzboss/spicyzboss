use std::{collections::HashMap, sync::Arc};

use worker::{Env, Request, Response, Result};

use crate::{
  api::TransactionCreateParamsDTO,
  domain::{errors::CommonError, repositories::TransactionQueryParams},
  infrastructure::TransactionD1Repository,
  usecases::TransactionUsecase,
};

#[worker::send]
pub async fn list_transactions(req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TransactionD1Repository::new(db.clone()));
  let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

  let query: HashMap<String, String> = req.url().unwrap().query_pairs().into_owned().collect();

  let tags = query
    .get("tags")
    .map(|v| v.split(",").map(|v| v.to_string()).collect());

  let cursor = query.get("cursor").map(|v| v.to_string());

  let size = query
    .get("size")
    .map(|v| v.parse::<usize>().unwrap_or_default());

  let show_deleted = query
    .get("show_deleted")
    .map(|v| v.to_string().to_lowercase().eq("true"));

  let result = usecase
    .repository
    .list(TransactionQueryParams::new(
      tags,
      size,
      cursor,
      show_deleted,
    ))
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
pub async fn get_transaction(req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TransactionD1Repository::new(db.clone()));
  let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

  let url = req.url()?;

  let transaction_id = url
    .path_segments()
    .unwrap()
    .nth(1)
    .unwrap()
    .parse::<String>()?;

  let result = usecase.repository.retrieve(transaction_id).await;

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
pub async fn create_transaction(mut req: Request, env: Env) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TransactionD1Repository::new(db.clone()));
  let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

  let plain_body = req.text().await?;
  let params = serde_json::from_str::<TransactionCreateParamsDTO>(&plain_body)?;
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
