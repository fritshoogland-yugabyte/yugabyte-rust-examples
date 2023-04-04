// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use std::env;

fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    // The port is set to 5433 (the default YugabyteDB port), set to 5432 for the postgres default.
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    // the username is set to yugabyte (the default YugabyteDB username), set to postgres for the postgres default.
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());

    let pool_manager = PostgresConnectionManager::new(format!("host={} port={} user={}", hostname, port, username).parse().unwrap(), NoTls);
    let pool = Pool::builder().max_size(5).build(pool_manager).expect("Unable to create connectionpool");

    // take a connection from the pool
    let mut connection = pool.get().expect("Unable to get connection from connection pool");
    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };
}
