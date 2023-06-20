use tokio_postgres::{NoTls, Error, connect};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error>
{
    // The settings for connecting to a local YugabyteDB / PostgreSQL database without authentication
    // PGHOST must be set, otherwise it will panic (expect)
    // PGPORT is set to 5432 if not set (unwrap_or)
    // PGUSER is set to postgres if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5432".to_string());
    let username: String = env::var("PGUSER").unwrap_or("postgres".to_string());
    let password: String = env::var("PGPASS").unwrap_or("postgres".to_string());
    // create connection
    // if no database is specified, the username is taken as the database name
    let (client, connection) = connect(&format!("host={hostname} port={port} user={username} password={password}"), NoTls).await?;
    // create the connection
    tokio::spawn(async move {
        if let Err(error) = connection.await { eprintln!("Connection error: {}", error); }
    });
   // execute
    let _ = client.query("select now()", &[]).await?;
   // return Ok
    Ok(())
}
