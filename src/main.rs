use axum::{extract::{Path, State}, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post}, 
    Router,
    Json
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use time::Date;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
#[derive(Clone,Serialize)]
pub struct Person {
    id: Uuid,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "apelido")]
    nickname: String,
    #[serde(rename = "nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<String>>
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "apelido")]
    nickname: String,
    #[serde(rename = "nascimento", with = "date_format")]
    birth_date: Date,
    stack: Option<Vec<String>>
}
type AppState = Arc<Mutex<HashMap<Uuid, Person>>>;
#[tokio::main]
async fn main() {
    let people: HashMap<Uuid, Person> = HashMap::new();

    let app_state = Arc::new(Mutex::new(people));
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
    let my_people = people.lock().await;
    match my_people.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }

}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let person = Person {
        id, 
        name: new_person.name,
        nickname: new_person.nickname,
        birth_date: new_person.birth_date,
        stack: new_person.stack
    };

    people.lock().await.insert(id, person.clone());
    (StatusCode::OK, Json(person))
}
async fn count_person(people: State<AppState>) -> impl IntoResponse {
    let count = people.lock().await.len();
    (StatusCode::OK, Json(count))
}