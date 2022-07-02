use std::error::Error;
use chrono::{DateTime, Utc};
use postgres::{Client, Row};

pub struct User {
    id: String,
    name: String,
    email: String,
    is_logged: bool,
    created_at: DateTime<Utc>,
}

impl User {
    fn check_table(client: &mut Client) -> Result<(),Box<dyn Error>> {
        let init_query: &str = "\
        CREATE TABLE IF NOT EXISTS users (\
        id SERIAL PRIMARY KEY,\
        name TEXT NOT NULL,\
        email TEXT NOT NULL,\
        is_logged BOOL,\
        created_at TIMETZ)";
        client.batch_execute(init_query).unwrap();
        Ok(())
    }
    fn new(client: &mut Client,id: String,name: String, email: String) -> Result<(),Box<dyn Error>> {
        let new_user: User = User {
            id,
            name,
            email,
            is_logged: false,
            created_at: Utc::now(),
        };
        let insert_query: &str = "INSERT INTO users (name,email,is_logged,created_at) VALUES ({$1},{$2},{$3},{$4})";
        client.query(insert_query,&[&new_user.name,&new_user.email,&new_user.is_logged,&new_user.created_at]).unwrap();
        Ok(())
    }
}