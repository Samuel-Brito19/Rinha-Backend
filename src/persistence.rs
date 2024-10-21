use sqlx::{postgres::PgRow, PgPool};
use uuid::Uuid;

use crate::{NewPerson, Person};

pub struct PostgresRepository {
    pool: PgPool,
}

pub enum PostgresError {
    NotFound,
}
impl PostgresRepository {
    pub async fn find_person(&self, id: Uuid) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as(
            "
            SELECT id, name, nickname, birth_date, stack
            FROM people
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_person(&self, new_person: NewPerson) -> Result<Person, sqlx::Error> {
        sqlx::query_as(
            "
            INSERT INTO people (id, name, nickname, birth_date, stack) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, nickname, birth_date, stack
            ",
        )
        .bind(Uuid::now_v7())
        .bind(new_person.name.0)
        .bind(new_person.nickname.0)
        .bind(new_person.birth_date)
        .bind(new_person.stack)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn search_people(&self, query: String) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as(
            "
            SELECT id, name, nickname, birth_date, stack
            FROM people
            WHERE to_tsquery('people', $1) @@ search
            LIMIT 50
            ",
        )
        .bind(query)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn count_people(&self) -> Result<i32, sqlx::Error> {
        sqlx::query("SELECT count(*) FROM people")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get(0))
    }
}
