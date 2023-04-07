// for local deployment
// no authentication
// no TLS

// example influenced by https://docs.rs/postgres/latest/postgres/

use postgres::{Client, NoTls, SimpleQueryMessage};
use std::env;

fn main()
{
    // The settings for connecting to a local YugabyteDB / PostgreSQL database without authentication
    // PGHOST must be set, otherwise it will panic (expect)
    // PGPORT is set to 5433 if not set (unwrap_or)
    // PGUSER is set to yugabyte if not set (unwrap_or)
    let hostname: String = env::var("PGHOST").expect("PGHOST must be set");
    let port: String = env::var("PGPORT").unwrap_or("5433".to_string());
    let username: String = env::var("PGUSER").unwrap_or("yugabyte".to_string());

    // create connection
    // if no database is specified, the username is taken as the database name
    let mut connection = Client::connect(&format!("host={} port={} user={}", hostname, port, username), NoTls).expect("Error connecting to database");

    println!("simple_query");
    // simple_query(query: &str) -> Result<Vec<SimpleQueryMessage>, Error>
    // uses simple query protocol
    // returns vector of SimpleQueryMessage
    for message in connection.simple_query("select table_name from information_schema.tables where table_type = 'BASE TABLE'").expect("Error executing simple_query")
    {
        if let SimpleQueryMessage::Row(row) = message
        {
            // get(0) is the first column of the resultset
            println!("{}", row.get(0).unwrap());
        }
    };

    println!("execute");
    // execute(query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    // Uses extended query protocol
    // returns number of rows that are result of the SQL
    let result = connection.execute("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing execute");
    println!("{}", result);

    println!("query");
    // query(query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    // uses extended query protocol
    // returns vector of Row
    for row in connection.query("select table_name from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query")
    {
        println!("{}", row.get::<usize, &str>(0));
    };

    println!("query_one");
    // query_one(query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    // uses extended query protocol
    // returns a single Row (and errors if it doesn't)
    let row = connection.query_one("select count(*) from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query_one");
    println!("{}", row.get::<usize, i64>(0));

    println!("query_opt");
    // query_opt(query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, Error>
    // uses extended query protocol
    // returns zero or a single Row (and errors if it doesn't)
    let row = connection.query_opt("select count(*) from information_schema.tables where table_type = 'BASE TABLE'", &[]).expect("Error executing query_opt");
    match row
    {
        Some(row) => println!("{}", row.get::<usize, i64>(0)),
        None => println!("No rows"),
    }

    println!("batch_execute");
    // batch_execute(query: &str) -> Result<(), Error>
    // Uses simple query protocol
    // returns error if any of the queries fail
    connection.batch_execute("select count(*) from information_schema.tables where table_type = 'BASE TABLE';select now();select now();select now()").expect("Error executing batch_execute");

    println!("prepare");
    // prepare(query: &str) -> Result<Statement, Error>
    // returns prepared statement
    let statement = connection.prepare("select table_name from information_schema.tables where table_name = $1").expect("Error creating prepared statement");
    // run any query type statement or execute
    // uses extended query protocol
    // prepared statement are not compatible with simple_query()
    for row in connection.query(&statement, &[&"pg_class".to_string()]).expect("Error executing query with a prepared statement")
    {
        println!("{}", row.get::<usize, &str>(0));
    };

    println!("transaction");
    // begins a database transaction, that later can be committed or rolled back.
    // a transaction with a different isolation level is possible with build_transaction()
    // first create a table to perform a transaction on
    connection.batch_execute("create table test_table( id int primary key, f1 text )").expect("Error creating test_table");
    // begin the transaction
    let mut transaction = connection.transaction().expect("Error creating transaction");
    // prepare
    let statement = transaction.prepare("insert into test_table (id, f1) values ($1, $2)").expect("Error building prepared statement");
    // build a data vector
    let mut data = Vec::new();
    for counter in 0..=10
    {
        data.push((counter, "AAAAAAAAAA".to_string()));
    }
    // perform the inserts inside the transaction
    for counter in 0..=10
    {
        // please mind that this is not most efficient: a multi-values insert statement is:
        // insert into table (field1, field2) values (data1, data2), (data1, data2), (data1, data2)
        // (so the values repeated, separated with a comma)
        transaction.query(&statement, &[&data[counter].0, &data[counter].1]).expect("Error executing query");
    }
    // a transaction must be committed or rolled back before it frees the connection
    transaction.commit().expect("Error executing commit");
    // drop the test_table
    connection.batch_execute("drop table test_table").expect("Error dropping test_table");

}
