use tokio_postgres::{NoTls, Error};
use std::env;
use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;

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

    let connection_manager = PostgresConnectionManager::new_from_stringlike(&format!("host={hostname} port={port} user={username} password={password}"), NoTls)?;
    let pool = Pool::builder().min_idle(Some(10)).build(connection_manager).await?;
    let connection = pool.get().await.unwrap();

    let prepared_statement = connection.prepare("select now()").await?;
    let _ = connection.query(&prepared_statement, &[]).await?;

    Ok(())
}
