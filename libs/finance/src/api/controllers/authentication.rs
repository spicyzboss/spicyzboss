use axum::{
  body::Body,
  extract::Request,
  http::{Response, StatusCode},
  middleware::Next,
};
use worker::{Env, Result};

#[worker::send]
pub async fn authentication_guard(req: Request, env: Env, next: Next) -> Result<Response<Body>> {
  let auth_key_env = env.var("AUTH_KEY");

  let auth_key: String;

  match auth_key_env {
    Ok(key) => auth_key = key.to_string(),
    _ => {
      return Ok(
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Body::empty())
          .unwrap(),
      );
    }
  };

  let auth_header_key_param = req.headers().get("x-auth-key");

  match auth_header_key_param {
    Some(key) => {
      if auth_key.ne(key) {
        return Ok(
          Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap(),
        );
      }

      return Ok(next.run(req).await);
    }
    None => Ok(
      Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::empty())
        .unwrap(),
    ),
  }
}
