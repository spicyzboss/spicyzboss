use std::collections::HashMap;

use hyper::StatusCode;
use worker::{event, Context, Env, Fetch, Request, Response, Result};

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
  let query: HashMap<String, String> = req.url().unwrap().query_pairs().into_owned().collect();

  if query.get("auth").is_some() {
    Fetch::Request(req).send().await
  } else {
    Response::error("Unauthorized", StatusCode::UNAUTHORIZED.into())
  }
}
