use uuid::Uuid;

use crate::{NewPerson, Person};

struct PostgresRepository {

}

impl PostgresRepository {
    pub fn find_person(&self,id: Uuid) -> Option<Person> {
        todo!()
    }

    pub fn create_person(&self,new_person: NewPerson) -> Option<Person> {
        todo!()
    }

    pub fn search_people(&self, query: String) -> Vec<Person> {
        
    }
}