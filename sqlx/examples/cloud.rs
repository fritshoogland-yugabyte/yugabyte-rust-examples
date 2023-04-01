use sqlx::{Row, Connection, postgres::PgConnection};
use std::env;

#[tokio::main]
async fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    // The port is set to 5433 (the default YugabyteDB port), set to 5432 for the postgres default.
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    // the username is set to yugabyte (the default YugabyteDB username), set to postgres for the postgres default.
    let username: String = env::var("PGUSER").unwrap_or("admin".to_string());
    let password: String = env::var("PGPASSWORD").expect("PGPASSWORD must be set");
    let database: String = env::var("PGDATABASE").unwrap_or("yugabyte".to_string());
    let ca_certificate_file: String = env::var("PGSSLROOTCERT").expect("PGSSLROOTCERT must be set");

    // create connection
    let mut connection = PgConnection::connect(&format!("postgres://{}:{}@{}:{}/{}?ssl=true&sslmode=verify-full&sslrootcert={}", username, password, hostname, port, database, ca_certificate_file)).await.expect("Error connecting to database");

    let rows = sqlx::query("select table_name from information_schema.tables where table_type = 'BASE TABLE'").fetch_all(&mut connection).await.expect("Error executing query");
    for row in rows.iter().map(|row| format!("{}", row.get::<String, _>("table_name"))).collect::<Vec<String>>()
    {
        println!("{}", row);
    }
}
