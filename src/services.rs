use std::error::Error;
use postgres::{Client, Row};

pub fn update_table(client: &mut Client, table_name: &str, field: &str,field_old_type: &str, new_field_type: &str ) -> Result<(),Box<dyn Error>> {
  let init_query: String = format!("ALTER TABLE {} ALTER COLUMN {} TYPE {} {}",table_name,field,field_old_type,new_field_type);
  client.batch_execute(init_query.as_str()).unwrap();
  Ok(())
}

pub fn get_all_data_from_table(client: &mut Client, table: &str) -> Result<Vec<Row>,Box<dyn Error>> {
  let injected_query: String = format!("SELECT * FROM {}",table);
  let vector_data:Vec<Row> = client.query(injected_query.as_str(),&[]).unwrap();
  Ok(vector_data)
}

