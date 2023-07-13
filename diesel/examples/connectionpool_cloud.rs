//use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use diesel::sql_query;
use diesel::sql_types::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;


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
    let password: String = env::var("PGPASSWORD").expect("PGPASSWORD must be set");
    let database: String = env::var("PGDATABASE").unwrap_or("yugabyte".to_string());
    let ca_certificate_file: String = env::var("PGSSLROOTCERT").expect("PGSSLROOTCERT must be set");

    // create connectionpool
    let connection_manager = ConnectionManager::<PgConnection>::new(&format!("host={hostname} port={port} user={username} password={password} dbname={database} sslmode=verify-full sslrootcert={ca_certificate_file}"));
    let connection_pool = Pool::builder().test_on_check_out(true).build(connection_manager).expect("Error creating connectionpool");

    // get connection from pool
    let connection_pool = connection_pool.clone();
    let mut connection = connection_pool.get().unwrap();

    // execute query and fetch result
    let rows: Vec<Tables> = sql_query("select table_catalog, table_schema, table_name, table_type from information_schema.tables where table_type = 'BASE TABLE'")
               .load(&mut connection)
               .expect("Error execution query");

    for row in rows.iter().map(|row| row.table_name.clone()).collect::<Vec<String>>()
    {
        println!("{}", row);
    }
}
