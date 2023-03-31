// for YugabyteDB cloud deployment

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::Client;
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::env;

fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    // The port is set to 5433 (the default YugabyteDB port), set to 5432 for the postgres default.
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    // the username is set to yugabyte (the default YugabyteDB username), set to postgres for the postgres default.
    let username: String = env::var("PGUSER").unwrap_or("admin".to_string());
    let password: String = env::var("PGPASSWORD").expect("PGPASSWORD must be set");
    let ca_certificate_file: String = env::var("PGSSLROOTCERT").expect("PGSSLROOTCERT must be set");
    let database: String = env::var("PGDATABASE").unwrap_or("yugabyte".to_string());

    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).expect("Unable to create SSL builder");
    ssl_connector_builder.set_ca_file(&ca_certificate_file).expect(&format!("Unable to load CA file: {}", &ca_certificate_file));
    ssl_connector_builder.set_verify(SslVerifyMode::PEER);
    let ssl_connector = MakeTlsConnector::new(ssl_connector_builder.build());

    let mut connection = Client::connect(&format!("host={} port={} user={} password={} dbname={}", hostname, port, username, password, database), ssl_connector).expect("Error connecting cloud.yugabyte.com database");

    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };
}
