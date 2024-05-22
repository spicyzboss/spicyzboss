use finance::api::*;
use worker::{event, Context, Env, Request, Response, Result, Router};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let router = Router::new();

  router
    .get_async("/transactions", list_transactions)
    .get_async("/transactions/:id", retrieve_transaction)
    .post_async("/transactions", create_transaction)
    .get_async("/tags", list_tags)
    .get_async("/tags/:id", retrieve_tag)
    .post_async("/tags", create_tag)
    .get_async("/categories", list_categories)
    .get_async("/categories/:id", retrieve_category)
    .post_async("/categories", create_category)
    .run(req, env)
    .await
}
