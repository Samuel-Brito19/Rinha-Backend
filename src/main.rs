use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};
use time:: Date;
mod persistence;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
#[derive(Clone,Serialize, sqlx::FromRow)]
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
pub struct PersonName(String);

pub enum PersonNameError {
    PersonNameTooLong
}

impl TryFrom<String> for PersonName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 32 {
            Ok(PersonName(value))
        } else {
            Err("name is too big")
        }
    }
}

impl PersonName {
    pub fn parse_string(name: String) -> Result<PersonName, PersonNameError> {
        if name.len() > 32 {
            Ok(PersonName(name))
        } else {
            Err(PersonNameError::PersonNameTooLong)
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Nickaname(String);

pub enum NickanameError {
    NickanameTooLong
}

impl TryFrom<String> for Nickaname {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 100 {
            Ok(Nickaname(value))
        } else {
            Err("name is too big")
        }
    }
}

impl Nickaname {
    pub fn parse_string(name: String) -> Result<Nickaname, NickanameError> {
        if name.len() > 32 {
            Ok(Nickaname(name))
        } else {
            Err(NickanameError::NickanameTooLong)
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Tech(String);

impl TryFrom<String> for Tech {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 100 {
            Ok(Self(value))
        } else {
            Err("tech is too big")
        }
    }
}

impl From<Tech> for String {
    fn from(value: Tech) -> Self {
        value.0
    }
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    #[serde(rename = "nome")]
    name: PersonName,
    #[serde(rename = "apelido")]
    nickname: Nickaname,
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
    if new_person.name.0.len() > 100 || new_person.nickname.0.len() > 32 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY)
    }

    match &new_person.stack {
        Some(stack) => {
            if stack.iter().any(|tech| tech.len() > 32) {
                return Err(StatusCode::UNPROCESSABLE_ENTITY)
            }
        }
        _ => {}
    }
     
    let id = Uuid::now_v7();
    let person = Person {
        id, 
        name: new_person.name.0,
        nickname: new_person.nickname.0,
        birth_date: new_person.birth_date,
        stack: new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect())
    };

    people.lock().await.insert(id, person.clone());
    Ok((StatusCode::OK, Json(person)))
}
async fn count_person(State(people): State<AppState>) -> impl IntoResponse {
    Json(people.lock().await.len());
}