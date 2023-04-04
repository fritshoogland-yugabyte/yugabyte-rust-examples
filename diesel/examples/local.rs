// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/
// https://diesel.rs/guides/getting-started

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
//use diesel::sql_query;

#[derive(Debug, QueryableByName)]
#[diesel(table_name = information_schema)]
pub struct InformationschemaTables {
        pub table_catalog: String, 
        pub table_schema: String,
        pub table_name: String,
        pub table_type: String,
        pub self_referencing_column_name: String,
        pub reference_generation: String,
        pub user_defined_type_catalog: String,
        pub user_defined_type_schema: String,
        pub user_defined_type_name: String,
        pub is_insertable_into: String,
        pub is_typed: String,
        pub commit_action: String,
}
table! {
    information_schema (table_schema, table_name) {
        table_catalog -> VarChar,
        table_schema -> VarChar,
        table_name -> VarChar,
        table_type -> VarChar,
        self_referencing_column_name -> VarChar,
        reference_generation -> VarChar,
        user_defined_type_catalog -> VarChar,
        user_defined_type_schema -> VarChar,
        user_defined_type_name -> VarChar,
        is_insertable_into -> VarChar,
        is_typed -> VarChar,
        commit_action -> VarChar,
    }
}


fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    // The port is set to 5433 (the default YugabyteDB port), set to 5432 for the postgres default.
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    // the username is set to yugabyte (the default YugabyteDB username), set to postgres for the postgres default.
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());

    // create connection
    let mut connection = PgConnection::establish(&format!("host={} port={} user={}", hostname, port, username)).expect("Error connecting to database");

    //println!("{:#?}", connection);
    /*
    // execute query and fetch result
    let rows = sql_query("select * from information_schema.tables where table_type = 'BASE TABLE'").load::<self::InformationschemaTables>(&mut connection).expect("Error execution query");

    println!("{:#?}", rows);
    */
    /*
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };
    */
}
