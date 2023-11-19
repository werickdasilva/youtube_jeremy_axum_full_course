use crate::context::Context;
use crate::error::{Error, Result};
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_context_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_context_resolver", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|t| t.value().to_string());

    let result_context = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _expp, _sign)) => Ok(Context::new(user_id)),
        Err(e) => Err(e),
    };

    if result_context.is_err() && !matches!(result_context, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_context);
    Ok(next.run(req).await)
}

pub async fn mw_require_auth<B>(
    ctx: Result<Context>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Context", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(Error::AuthFailContextNotInRequestExt)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_who, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    println!("Data {user_id}, {exp}, {sign}");
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
