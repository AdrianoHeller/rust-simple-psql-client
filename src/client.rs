use std::error::Error;
use postgres::{Client,NoTls};
use std::env;

pub struct Connection {
    client: Client,
}

impl Connection {
    pub fn create(&self) -> Result<Client,Box<dyn Error>> {
        let conn_String: String = env::var("POSTGRES_CONN_STR".to_string())?;
        let db: Client = Client::connect(conn_String.as_str(),&[]).expect("No connection string provided.");
        Ok(db)
    }
}