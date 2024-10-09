use axum::{extract::{Path, State}, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post}, 
    Router,
    Json
};
use serde::Serialize;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use time::{macros::date, Date};

#[derive(Clone,Serialize)]
struct Person {
    id: Uuid,
    name: String,
    nickname: String,
    birth_date: Date,
    stack: Vec<String>
}

type AppState = Arc<HashMap<Uuid, Person>>;
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

    println!("{}", person.id);

    people.insert(person.id, person);
    let app_state = Arc::new(people);
    let app = Router::new()
    .route("/people", get(search_people))
    .route("/people/:id", get(find_person))
    .route("/people", post(create_person))
    .route("/count-people", get(count_person))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Busca de pessoas")
}

async fn find_person(State(people): State<AppState>, 
    Path(person_id): Path<Uuid>) -> impl IntoResponse {
    //let State(people) = state;
    match people.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }

}

async fn create_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "create")
}
async fn count_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "count")
}