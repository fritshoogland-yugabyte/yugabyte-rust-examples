use std::env;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tokio_postgres::Error;
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

    let mut ssl_connection_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_connection_builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(ssl_connection_builder.build());

    let connection_manager = PostgresConnectionManager::new_from_stringlike(&format!("host={hostname} port={port} user={username} password={password} sslmode=require"), connector)?;
    let pool = Pool::builder().min_idle(Some(10)).build(connection_manager).await?;
    let connection = pool.get().await.unwrap();

    let _ = connection.query("select now()", &[]).await?;

    Ok(())
}
