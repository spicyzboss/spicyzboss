use std::sync::Arc;

use core_finance::{
  api::TransactionCreateParamsDTO, domain::errors::CommonError,
  infrastructure::TransactionD1Repository, usecases::TransactionUsecase,
};
use hyper::{header, StatusCode};
use worker::{event, Context, Env, Method, Request, Response, Result};

fn is_post_request(req: &Request) -> bool {
  return matches!(req.method(), Method::Post);
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let db = Arc::new(env.d1("DB").expect("no d1 binding"));
  let repo = Arc::new(TransactionD1Repository::new(db.clone()));
  let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

  if !is_post_request(&req) {
    let mut response = Response::error(
      StatusCode::METHOD_NOT_ALLOWED.canonical_reason().unwrap(),
      StatusCode::METHOD_NOT_ALLOWED.as_u16(),
    )?;

    response
      .headers_mut()
      .append(header::ALLOW.as_str(), Method::Post.as_ref())?;

    return Ok(response);
  }

  let plain_body = req.clone_mut().unwrap().text().await?;

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
