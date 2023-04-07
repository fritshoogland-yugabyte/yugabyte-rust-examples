// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use std::env;

fn main()
{
    // The settings for creating a connectionpool for a local YugabyteDB / PostgreSQL database without authentication
    // PGHOST must be set, otherwise it will panic (expect)
    // PGPORT is set to 5433 if not set (unwrap_or)
    // PGUSER is set to yugabyte if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());

    // setup connectionpool
    // the pool::builder().max_size() function sets the maximum number of connections
    // the pool::builder().min_idle() function sets the number of idle connections, None (default) sets max_size.
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
