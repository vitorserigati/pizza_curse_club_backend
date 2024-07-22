use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

const BIND: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    let listener = tokio::net::TcpListener::bind(BIND).await.unwrap();
    println!("->> LISTENING on {BIND}\n");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug)]

struct HelloProps {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloProps>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("hello <strong>{name}</strong>"))
}
