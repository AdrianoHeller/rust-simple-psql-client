use std::error::Error;
use chrono::{DateTime, Utc};
use postgres::{Client};

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_logged: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn check_table(client: &mut Client) -> Result<(),Box<dyn Error>> {
        let init_query: &str = "\
        CREATE TABLE IF NOT EXISTS users (\
        id SERIAL PRIMARY KEY,\
        name TEXT NOT NULL,\
        email TEXT NOT NULL,\
        is_logged BOOL,\
        created_at TIMESTAMP WITH TIME ZONE)";
        client.batch_execute(init_query).unwrap();
        Ok(())
    }
    pub fn new(client: &mut Client,id: String,name: String, email: String) -> Result<(),Box<dyn Error>> {
        let new_user: User = User {
            id,
            name,
            email,
            is_logged: false,
            created_at: Utc::now(),
        };
        let insert_query: &str = "INSERT INTO users (name,email,is_logged,created_at) VALUES ($1,$2,$3,$4)";
        client.query(insert_query,&[
            &new_user.name,
            &new_user.email,
            &new_user.is_logged,
            &new_user.created_at]).unwrap();
        Ok(())
    }
    fn get_all(client: &mut Client, query: &str) -> Result<(),Box<dyn Error>> {
        let recover_query: &str = query;
        let request = client.query(recover_query,&[])?;
        for row in request {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            let email: &str = row.get(2);
            let is_logged: bool = row.get(3);
            let created_at: DateTime<Utc> = row.get(4);
            println!("id:{},\nname:{},\nemail:{:?},\nis_logged:{},\ncreated_at:{}\n",id,name,email,is_logged,created_at);
        }
        Ok(())
    }
}