use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Router};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let people: HashMap<uuid, > = HashMap::new();
    let app = Router::new()
    .route("/pessoas", get(search_people))
    .route("/pessoas/:id", get(find_person))
    .route("/pessoa", post(create_person))
    .route("/contagem-pessoas", get(count_person));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Busca de pessoas")
}

async fn find_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "find")
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "create")
}
async fn count_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "count")
}