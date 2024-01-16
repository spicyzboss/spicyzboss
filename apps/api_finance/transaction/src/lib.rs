use std::{collections::HashMap, sync::Arc};

use core_finance::{
  api::TransactionCreateParamsDTO,
  domain::{errors::CommonError, repositories::TransactionQueryParams},
  infrastructure::TransactionD1Repository,
  usecases::TransactionUsecase,
};
use worker::{event, Context, Env, Request, Response, Result, Router};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let router = Router::new();

  router
    .get_async("/transactions", |req, ctx| async move {
      let db = Arc::new(ctx.env.d1("DB").expect("no d1 binding"));
      let repo = Arc::new(TransactionD1Repository::new(db.clone()));
      let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

      let query: HashMap<String, String> = req.url().unwrap().query_pairs().into_owned().collect();
      let result = usecase
        .repository
        .list(TransactionQueryParams {
          offset: query
            .get("offset")
            .map(|v| v.parse::<i64>().unwrap_or_default()),
          limit: query
            .get("limit")
            .map(|v| v.parse::<i64>().unwrap_or_default()),
          tags: query
            .get("tags")
            .map(|v| v.split(",").map(|v| v.to_string()).collect()),
        })
        .await;

      match result {
        Ok(v) => Response::from_json(&v),
        Err(v) => {
          let error: CommonError = v.into();
          let response = Response::from_json(&error)?;

          Ok(response.with_status(error.code))
        }
      }
    })
    .get_async("/transactions/:id", |_req, ctx| async move {
      let db = Arc::new(ctx.env.d1("DB").expect("no d1 binding"));
      let repo = Arc::new(TransactionD1Repository::new(db.clone()));
      let usecase = Arc::new(TransactionUsecase::new(repo.clone()));

      let transaction_id = ctx.param("id").unwrap().to_string();
      let result = usecase.repository.retrieve(transaction_id).await;

      match result {
        Ok(v) => Response::from_json(&v),
        Err(v) => {
          let error: CommonError = v.into();
          let response = Response::from_json(&error)?;
          Ok(response.with_status(error.code))
        }
      }
    })
    .post_async("/transactions", |mut req, ctx| async move {
      let db = Arc::new(ctx.env.d1("DB").expect("no d1 binding"));
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
    })
    .run(req, env)
    .await
}
