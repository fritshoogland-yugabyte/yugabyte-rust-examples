// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::{NoTls, Row, error::SqlState};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::{Pool, PooledConnection};
use std::{env, io::stdin, time::Duration, thread::sleep, thread, time::Instant};
use rand::{thread_rng, Rng};
use log::*;
//use chrono::DateTime;

static TRANSACTIONAL: bool = true;
static NUM_ROWS: u32 = 1000000;
static NUM_THREADS: u32 = 5;
static NUM_RETRIES: u32 = 20;

fn main()
{
    env_logger::init();
    // The settings for creating a connectionpool for a local YugabyteDB / PostgreSQL database without authentication
    // PGHOST must be set, otherwise it will panic (expect)
    // PGPORT is set to 5433 if not set (unwrap_or)
    // PGUSER is set to yugabyte if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());
    let password: String = env::var("PGPASS").unwrap_or("yugabyte".to_string());
    let pool_size = env::var("POOLSIZE").unwrap_or("10".to_string());

    // setup connectionpool
    // the pool::builder().max_size() function sets the maximum number of connections
    // the pool::builder().min_idle() function sets the number of idle connections, None (default) sets max_size.
    let config: postgres::Config = format!("host={hostname} port={port} user={username} password={password}").parse().unwrap();
    //let pool_manager = PostgresConnectionManager::new(format!("host={hostname} host=192.168.66.81 port={port} user={username} password={password}").parse().unwrap(), NoTls);
    let pool_manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_size(pool_size.parse().unwrap()).build(pool_manager).expect("Unable to create connectionpool");


    // take a connection from the pool
    //let mut connection = pool.get().expect("Unable to get connection from connection pool");
    //let mut connection = get_connection(pool);
    loop {
        println!("{hostname}:{port}");
        println!("Rows: {NUM_ROWS}, threads: {NUM_THREADS}, transactional: {TRANSACTIONAL}");

        let mut input = String::new();
        println!("press enter to continue");
        stdin().read_line(&mut input).expect("Error reading from stdin");
        let timer = Instant::now();

        let mut join_handles = vec![];
        for thread_number in 0..NUM_THREADS
        {
            let pool = pool.clone();
            let handle = thread::Builder::new().name(format!("runner {thread_number}")).spawn(move || {
                let mut connection = pool.get().unwrap();
                connection.simple_query(format!("set application_name='runner {thread_number}'").as_str()).unwrap();
                if TRANSACTIONAL
                {
                    update_looper_transaction(&mut connection);
                }
                else
                {
                    update_looper(&mut connection);
                };
            });
            join_handles.push(handle);
        }
        // wait for threads to finish
        for handle in join_handles {
            handle.unwrap().join().unwrap();
        }


        println!("Done, time: {:?}", timer.elapsed());
        //let result = connection.query_one("select now()", &[]).expect("Error executing query");
        /*
        for loop_nr in 1..1000000
        {
            let _result = query_with_retry(&mut connection, format!("update t set f1={} where id=1", loop_nr).as_str()).expect("Error executing query");
        }

         */
        //println!("Result: {:?}", result.get::<usize, SystemTime>(0));
    }
    /*
    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        let table_name: &str = row.get(0);
        println!("{}", table_name);
    };

     */
}

fn update_looper(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>)
{
    for loop_nr in 1..NUM_ROWS
    {
        let _result = query_with_retry(connection, format!("update t set f1={} where id=1", loop_nr).as_str()).expect("Error executing query");
        //println!("{}", loop_nr);
    }
    println!("done!");
}

fn update_looper_transaction(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>)
{
    for loop_nr in 1..NUM_ROWS
    {
        let _result = query_with_retry_tx(connection, format!("update t set f1={} where id=1", loop_nr).as_str()).expect("Error executing query");
    }
    println!("done!");
}

/*
fn get_connection(pool: Pool<PostgresConnectionManager<NoTls>>) -> PooledConnection<PostgresConnectionManager<NoTls>>
{
    loop
    {
        match pool.get()
        {
            Err(error) => println!("Error: {:?}", error),
            connection => {
                break connection.unwrap()
            },
        };
    }
}

 */

fn query_with_retry_tx(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>, query: &str) -> Result<Vec<Row>, String>
{
    let mut backoff_ms = thread_rng().gen_range(2..20);
    for retry_nr in 0..=NUM_RETRIES
    {
        let mut transaction = connection.transaction().unwrap();
        let result = transaction.query(query, &[]);
        let result_commit = transaction.commit();

        match result
        {
            Err(error) => {
                if is_retryable_error(&error)
                {
                    sleep(Duration::from_millis(backoff_ms));
                    backoff_ms *= 2;
                    info!("query retryable error: {:?}, backoff time: {}, try: {}", error.code().unwrap(), backoff_ms, retry_nr);

                    continue;
                }
                else
                {
                    return Err(error.to_string())
                }
            },
            Ok(row) => {
                match result_commit
                {
                    Err(error) => {
                        if is_retryable_error(&error)
                        {
                            sleep(Duration::from_millis(backoff_ms));
                            backoff_ms *= 2;
                            info!("commit retryable error: {:?}, backoff time: {}, try: {}", error.code().unwrap(), backoff_ms, retry_nr);

                            continue;
                        }
                    },
                    Ok(()) => {
                        return Ok(row)
                    }
                }
            },
        }
    }
    // the number of loops was exhausted, which means it never did return okay, so it's error.
    Err("Too many retries tried.".into())
}

fn query_with_retry(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>, query: &str) -> Result<Vec<Row>, String>
{
    // get a random waiting time
    let mut backoff_ms = thread_rng().gen_range(1..10);
    // 10 retries
    for retry_nr in 0..=NUM_RETRIES
    {
        let result = connection.query(query, &[]);

        match result
        {
            Err(error) => {
                // the query resulted in error
                // is_retryable_error returns true if it's retryable
                if is_retryable_error(&error)
                {
                    // But to make sure the retry makes sense, wait some time to have the other transaction finish.
                    sleep(Duration::from_millis(backoff_ms));
                    // increase time exponentially by multiplying it by 2.
                    backoff_ms *= 2;
                    info!("retryable error: {:?}, backoff time: {}, try: {}", error.code().unwrap(), backoff_ms, retry_nr);

                    continue;
                }
                else
                {
                    // if the error is not a retryable error, just return error
                    return Err(error.to_string())
                }
            },
            Ok(rows) => {
                return Ok(rows)
            },
        }
    }
    Err("Too many retries tried.".into())
}

fn is_retryable_error(sql_error: &postgres::Error) -> bool
{
    // return okay if the below sqlstates are matched, otherwise return false.
    sql_error.code()
        .map(|error| {
            *error == SqlState::T_R_SERIALIZATION_FAILURE ||    // E40001
            *error == SqlState::T_R_DEADLOCK_DETECTED           // E40P01
        })
        .unwrap_or(false)
}
