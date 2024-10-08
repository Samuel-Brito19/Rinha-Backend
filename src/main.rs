use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Router};
use uuid::Uuid;
use std::collections::HashMap;
use time::{macros::date, Date};

struct Person {
    id: Uuid,
    name: String,
    nickname: String,
    birth_date: Date,
    stack: Vec<String>
}
#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: "Samuel".to_string(),
        nickname: "Major".to_string(),
        birth_date: date!(2001-10-29),
        stack: vec!["Typescript".to_string(), "Javascript".to_string()]
    };

    people.insert(person.id, person);
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