use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

mod models;

mod error;
mod web;

pub use self::error::{Error, Result};


#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    println!("->> LISTENING on {}...\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap()
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:anything", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}</strong>"))
}
