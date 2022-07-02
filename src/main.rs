use std::process;
use postgres::{Client, NoTls};

fn main() {

    println!("Starting connection with postgres...");

    let conn_str: &str = "postgresql://po5tgr3s:po5tgr35.5ql@localhost:5432/test";

    let mut client = Client::connect(conn_str,NoTls).unwrap_or_else(|err| {
        eprintln!("Error: {}",err);
        process::exit(1);
    });

    let creation_query: &str = "
    CREATE TABLE person IF NOT EXISTS (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
    ";

    client.batch_execute(creation_query);

    let name: &str = "Dalbo 09";
    let data = None::<&[u8]>;

    let insertion_query: &str = "INSERT INTO person (name, data) VALUES ($1, $2)";

    client.execute(insertion_query,&[&name,&data],);

    println!("Query executed");

    let recover_query: &str = "SELECT id,name,data FROM person";

    for row in client.query(recover_query,&[]).unwrap_or_else(|err| {
            eprintln!("Error: {}",err);
            process::exit(1);
    }) {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);
        println!("id:{},\nname:{},\ndata:{:?}\n",id,name,data);
    }
}
