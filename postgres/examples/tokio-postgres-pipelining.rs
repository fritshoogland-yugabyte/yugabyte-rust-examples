use tokio_postgres::{NoTls, Error, connect};
use std::env;
use futures::future::join_all;

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

    // apply pipelining to the prepare/parse phase
    let mut prepare_futures = vec![];
    let future = client.prepare("select 'one'");
    prepare_futures.push(future);
    let future = client.prepare("select 'two'");
    prepare_futures.push(future);
    let future = client.prepare("select 'three'");
    prepare_futures.push(future);
    // the join_all sends all the prepare statements in one go
    let results = join_all(prepare_futures).await;

    
    // apply pipelining to the bind and execute phase
    // iterate over all the prepare results and create execute futures in the execute_futures vector.
    let mut execute_futures = vec![];
    results.iter().for_each(|prepare| {
        let future = client.query(prepare.as_ref().unwrap(), &[]);
        execute_futures.push(future); 
    });
    // tje join_all here sends all the bind and execute messages in one go.
    let results = join_all(execute_futures).await;

    // get the results
    results.iter().for_each(|result| println!("{}", result.as_ref().unwrap()[0].get::<_, &str>(0)));

    Ok(())
}
