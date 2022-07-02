use std::error::Error;
use std::process;
use postgres::{Client, NoTls, Row};

use api_test::{models,services};

fn main() {

    println!("Starting connection with postgres...");

    let conn_str: &str = "postgresql://po5tgr3s:po5tgr35.5ql@localhost:5432/test";

    let mut client = Client::connect(conn_str,NoTls).unwrap_or_else(|err| {
        eprintln!("Error: {}",err);
        process::exit(1);
    });

    if let Err(e) = models::User::check_table(&mut client) {
        eprintln!("Error found: {}",e);
        process::exit(1);
    };

    if let Err(e) = models::User::new(
        &mut client,
        String::from("0h76g762s5f64bioyo3cy53"),
        String::from("Don Henley"),
        String::from("don@eagles.com"))
    {
        eprintln!("Error found: {}",e);
        process::exit(1);
    };

    let full_data = services::get_all_data_from_table(&mut client,"users").unwrap_or_else(|err|{
        eprintln!("Error found: {}",err);
        process::exit(1);
    });

    println!("{:?}",full_data);

    println!("Query executed");

    let mut _get_user_by_id = |table: &str,id: &str| -> Result<(),Box<dyn Error>>{
        let recover_query: String = format!("SELECT * FROM {} WHERE id={}",table,id);
        let req: Vec<Row>= client.query(recover_query.as_str(),&[]).unwrap_or_else(|err| {
            eprintln!("Error found: {}",err);
            process::exit(1);
        });
        for row in req {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            let data: Option<&[u8]> = row.get(2);
            println!("{},{},{:?}",id,name,data);
        }
        Ok(())
    };
}





