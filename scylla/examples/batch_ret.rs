// Example that performs batch inserts.

use scylla::{Session, SessionBuilder};
use scylla::batch::Batch;
use std::env;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use scylla::frame::response::result::Row;

#[tokio::main]
async fn main() {
    let known_nodes: String = env::var("KNOWN_NODES").expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();

    let create_keyspace = "create keyspace if not exists example";
    let create_table = "create table if not exists example.example_table (id int primary key, f1 text)";
    //let insert_statement = "insert into example.example_table (id, f1) values (?, ?) returns status as row";
    //let insert_statement = "insert into example.example_table (id, f1) values (?, ?) returns status as row";
    let insert_statement = "update example.example_table set f1 = 'a' where id = 1 if f1 = 'b' returns status as row";

    let total_number_rows = 1;
    let batch_size = 1;
    let insert_string_length = 200;
    let random_characters = |length: usize| -> String
        {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        };

    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .build()
        .await
        .expect("Error creating session");

    session.query(create_keyspace, &[]).await.expect("Error executing create keyspace");
    session.query(create_table, &[]).await.expect("Error executing create table");

    let mut batch: Batch = Default::default();
    //let mut batch_values = Vec::new();
    let prepared_statement = session.prepare(insert_statement).await.expect("Error creating prepared statement");

    // For batching using regular (non-prepared) statements: use batch.append_statement(insert_statement);
    // for batching using a prepared statement: use batch.append_statement(prepated_statement.clone());
    // I tried regular batching and: session.prepare_batch(&batch).await.expect("Error executing prepare_batch");
    // But that did not result in lower time (?)
    for counter in 1..=total_number_rows
    {
        //batch.append_statement(insert_statement);
        batch.append_statement(prepared_statement.clone());
        // The scylla driver wants the values in a separate variable
        // The batch_values variable must be a slice `&[]` or a tuple `()` which matches the number of placeholders `?` for the statement.
        // The slice or tuple for a row must be in a super slice or tuple that matches the number of rows.
        // The documentation (https://rust-driver.docs.scylladb.com/stable/queries/values.html) mentions a maximum of 16 elements.
        // Or use a custom struct which derives from `ValueList`
        //batch_values.push((counter, random_characters(insert_string_length)));

        if counter%batch_size == 0
        {
            batch.set_tracing(true);
            //let batch_result = session.batch(&batch, &batch_values).await.expect("Error executing batch");
            let batch_result = session.batch(&batch, ((),)).await.expect("Error executing batch");
            //for row in batch_result.rows() { println!("{:#?}", row[0].columns[0])}
            //for row in batch_result.rows() { println!("{:#?}", row[0].columns[0])}
            //println!("{:#?}", batch_result.rows().unwrap_or("a").column[0].unwrap_or("b"));
            //println!("{:#?}", batch_result.rows().unwrap_or(Vec::new())[0].columns[0]);
            //println!("{:#?}", batch_result.rows().Some(applied));
            println!("{:?}", batch_result.rows().unwrap_or( { Vec::<Row>::new() } ).);
            //println!("{:?}", batch_result.rows() );
            batch = Default::default();
            //batch_values = Vec::new();
        }
    }
}
