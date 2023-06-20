// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use std::env;

fn main()
{
    // The settings for connecting to cloud.yugabyte.com
    // Please mind this will work for other cloud connections to a PostgreSQL database too.
    // PGHOST must be set otherwise it will panic (expect)
    // PGPASSWORD must be set otherwise it will panic (expect)
    // PGSSLROOTCERT must be set otherwise it will panic (expect)
    // PGPORT is set to 5433 if not set (unwrap_or)
    // PGUSER is set to yugabyte if not set (unwrap_or)
    // PGDATABASE is set to yugabyte if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let password: String = env::var("PGPASSWORD").expect("PGPASSWORD must be set");
    let ca_certificate_file: String = env::var("PGSSLROOTCERT").expect("PGSSLROOTCERT must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("admin".to_string());
    let database: String = env::var("PGDATABASE").unwrap_or("yugabyte".to_string());

    // This is building the SSL connector (identical to cloud.rs)
    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).expect("Unable to create SSL builder");
    ssl_connector_builder.set_ca_file(&ca_certificate_file).expect(&format!("Unable to load CA file: {}", &ca_certificate_file));
    ssl_connector_builder.set_verify(SslVerifyMode::PEER);
    let ssl_connector = MakeTlsConnector::new(ssl_connector_builder.build());

    // setup connectionpool using the SSL connector
    // the pool::builder().max_size() function sets the maximum number of connections
    // the pool::builder().min_idle() function sets the number of idle connections, None (default) sets max_size.
    let pool_manager = PostgresConnectionManager::new(format!("host={} port={} user={} password={} dbname={}", hostname, port, username, password, database).parse().unwrap(), ssl_connector);
    let pool = Pool::builder().max_size(5).build(pool_manager).expect("Unable to create connectionpool");

    // take a connection from the pool
    let mut connection = pool.get().expect("Unable to get connection from connection pool");
    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };
}
