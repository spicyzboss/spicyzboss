use axum::{
  body::Body,
  extract::Request,
  http::Response,
  middleware::{from_fn, Next},
  routing::{get, post},
  Extension, Router,
};
use finance::api::*;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

macro_rules! handler (
  ($name:path) => {
    |Extension(env): Extension<Env>, req: Request| async {
      let resp = $name(req.try_into().expect("convert request"), env).await.expect("handler result");
      Into::<Response<Body>>::into(resp)
    }
  }
);

macro_rules! layer {
  ($name:path) => {
    |Extension(env): Extension<Env>, req: Request, next: Next| async {
      $name(req, env, next).await.expect("handler result")
    }
  };
}

fn api_v1_services() -> Router {
  Router::new()
    .route("/transactions", get(handler!(list_transactions)))
    .route("/transactions/:id", get(handler!(get_transaction)))
    .route("/transactions", post(handler!(create_transaction)))
    .route("/tags", get(handler!(list_tags)))
    .route("/tags/:id", get(handler!(get_tag)))
    .route("/tags", post(handler!(create_tag)))
    .route("/categories", get(handler!(list_categories)))
    .route("/categories/:id", get(handler!(get_category)))
    .route("/categories", post(handler!(create_category)))
}

#[event(fetch)]
async fn main(req: HttpRequest, env: Env, _ctx: Context) -> Result<Response<Body>> {
  console_error_panic_hook::set_once();

  let router = Router::new()
    .nest("/v1", api_v1_services())
    .layer(from_fn(layer!(authentication_guard)))
    .layer(Extension(env))
    .call(req)
    .await?;

  Ok(router)
}
