// for YugabyteDB cloud deployment

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::Client;
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::env;

fn main()
{
    // The settings for connecting to cloud.yugabyte.com
    // Please mind this will work for other cloud connections to a PostgreSQL database too.
    // PGHOST must be set otherwise it will panic (expect)
    // PGPORT is set to 5433 if not set (unwrap_or)
    // PGUSER is set to yugabyte if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("postgres".to_string());

    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).expect("Unable to create SSL builder");
    ssl_connector_builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(ssl_connector_builder.build());

    let mut connection = Client::connect(&format!("host={hostname} port={port} user={username} sslmode=require"), connector).unwrap();

    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };
}
