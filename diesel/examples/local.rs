// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/
// https://diesel.rs/guides/getting-started

use diesel::{pg::PgConnection, prelude::*, sql_types::*, sql_query};
use std::env;


#[derive(Debug, QueryableByName)]
pub struct Tables {
    #[diesel(sql_type = Text)] 
    pub table_catalog: String,
    #[diesel(sql_type = Text)] 
    pub table_schema: String,
    #[diesel(sql_type = Text)] 
    pub table_name: String,
    #[diesel(sql_type = Text)] 
    pub table_type: String,
}

fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    // The port is set to 5433 (the default YugabyteDB port), set to 5432 for the postgres default.
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    // the username is set to yugabyte (the default YugabyteDB username), set to postgres for the postgres default.
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());

    // create connection
    let mut connection = PgConnection::establish(&format!("host={} port={} user={}", hostname, port, username)).expect("Error connecting to database");

    // execute query and fetch result
    let rows: Vec<Tables> = sql_query("select table_catalog, table_schema, table_name, table_type from information_schema.tables where table_type = 'BASE TABLE'")
               .load(&mut connection)
               .expect("Error execution query");

    for row in rows.iter().map(|row| row.table_name.clone()).collect::<Vec<String>>()
    {
        println!("{}", row);
    }
}
