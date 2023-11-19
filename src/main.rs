mod context;
mod error;
mod log;
mod model;
mod web;

use crate::error::{Error, Result};
use crate::log::log_request;
use crate::model::ModelController;
use axum::http::{Method, Uri};
use axum::Json;
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router, Server,
};
use context::Context;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let router_hello = Router::new()
        .merge(router_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_context_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(router_static());

    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("->> LISTENING om {address}\n");

    Server::bind(&address)
        .serve(router_hello.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    context: Option<Context>,
    request_method: Method,
    uri: Uri,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid: Uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string()
            }
        });

        println!("->> client_error_body {client_error_body}");

        (*status, Json(client_error_body)).into_response()
    });

    let client_error = client_status_error.unzip().1;
    log_request(
        uuid,
        request_method,
        uri,
        context,
        service_error,
        client_error,
    )
    .await;
    println!();
    error_response.unwrap_or(res)
}

fn router_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

fn router_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello_path/:name", get(handle_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello {name}</h1>"))
}

async fn handle_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {name:?}", "HANDLER");

    Html(format!("Hello path ->> {name}"))
}
