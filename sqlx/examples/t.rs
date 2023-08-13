use sqlx::{Row, postgres::PgPoolOptions};
use std::env;
use chrono::{DateTime, Utc};
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());
    let database: String = env::var("PGDATABASE").unwrap_or("yugabyte".to_string());

    let pool = PgPoolOptions::new().min_connections(5).max_connections(5).connect(&format!("postgres://{}@{}:{}/{}", username, hostname, port, database)).await.expect("Error connecting to database");

    let mut futures = vec![];
    let future = sqlx::query("select now()").fetch_all(&pool);
    futures.push(future);
    let future = sqlx::query("select now()").fetch_all(&pool);
    futures.push(future);

    let results = join_all(futures).await;

    for result in results 
    {
        for out in result.unwrap().iter().map(|row| format!("{}", row.get::<DateTime<Utc>, _>("now"))).collect::<Vec<String>>()
        {
            println!("{}", out);
        }
    }
}
