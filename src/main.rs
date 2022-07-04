use std::{env, process};
use postgres::{Client, NoTls};
use api_test::{models, services, utils};
use api_test::services::get_all_data_from_table;

fn main() {
    println!("Starting connection with postgres...");

    let postgres_var: String= env::var("POSTGRES_CONN_STR").unwrap_or_else(|err|{
        eprintln!("Error: {}",err);
        process::exit(1);
    });

    let mut connection = Client::connect(postgres_var.as_str(),NoTls).unwrap_or_else(|err|{
        eprintln!("Error: {}",err);
        process::exit(1);
    });

    let _cursor = get_all_data_from_table(&mut connection,"users").unwrap_or_else(|err|{
        eprintln!("Error: {}",err);
        process::exit(1);
    });



    // let conn_str: &str = "postgresql://po5tgr3s:po5tgr35.5ql@localhost:5432/test";

    // let mut client = Client::connect(conn_str,NoTls).unwrap_or_else(|err| {
    //     eprintln!("Error: {}",err);
    //     process::exit(1);
    // });

    // if let Err(e) = models::User::check_table(&mut client) {
    //     eprintln!("Error found: {}",e);
    //     process::exit(1);
    // };

    let random_hash = utils::create_random_id(70);

    if let Err(e) = models::User::new(
        &mut connection,
        String::from(&random_hash),
        String::from("Don Henley"),
        String::from("don@eagles.com"))
    {
        eprintln!("Error found: {}",e);
        process::exit(1);
    };

    let full_data = services::get_all_data_from_table(&mut connection,"users").unwrap_or_else(|err|{
        eprintln!("Error found: {}",err);
        process::exit(1);
    });

    println!("{:?}",full_data);

    println!("Query executed");
    //
    // let mut _get_user_by_id = |table: &str,id: &str| -> Result<(),Box<dyn Error>>{
    //     let recover_query: String = format!("SELECT * FROM {} WHERE id={}",table,id);
    //     let req: Vec<Row>= client.query(recover_query.as_str(),&[]).unwrap_or_else(|err| {
    //         eprintln!("Error found: {}",err);
    //         process::exit(1);
    //     });
    //     for row in req {
    //         let id: i32 = row.get(0);
    //         let name: &str = row.get(1);
    //         let data: Option<&[u8]> = row.get(2);
    //         println!("{},{},{:?}",id,name,data);
    //     }
    //     Ok(())
    // };
}





