use tokio_postgres::{NoTls, Error};
use std::env;
use deadpool_postgres::{Config, Runtime, PoolConfig, Timeouts};

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

    let mut deadpool_config = Config::new();
    deadpool_config.host = Some(hostname);
    deadpool_config.port = Some(port.parse::<u16>().unwrap());
    deadpool_config.user = Some(username.to_string());
    deadpool_config.dbname = Some(username);
    deadpool_config.password = Some(password);
    deadpool_config.pool = Some(PoolConfig { max_size: 10, timeouts: Timeouts::new() });
    let pool = deadpool_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let connection = pool.get().await.unwrap();

    let prepared_statement = connection.prepare("select now()").await?;
    let _ = connection.query(&prepared_statement, &[]).await?;

    Ok(())
}
