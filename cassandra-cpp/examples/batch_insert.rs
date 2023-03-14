// Example that performs batch inserts.

use cassandra_cpp::*;
use std::env;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


fn main() {
    let contact_points: String = env::var("CONTACT_POINTS").expect("CONTACT_POINTS must be set, comma separated for multiple: HOSTNAME[,HOSTNAME]");

    let create_keyspace = stmt!("create keyspace if not exists example");
    let create_table = stmt!("create table if not exists example.example_table (id int primary key, f1 text)");
    let insert_statement = "insert into example.example_table (id, f1) values (?, ?)";

    // batchtype: LOGGED, UNLOGGED, COUNTER
    let batch_type = BatchType::LOGGED;

    let total_number_rows = 1_000_000;
    let batch_size = 50;
    let insert_string_length = 200;
    let random_characters = |length: usize| -> String
    {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
    };

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points.as_str()).expect("Error setting contact points");
    let session = cluster.connect().expect("Error connecting to cluster");

    // creation of the keyspace and table if they don't exist already
    session.execute(&create_keyspace).wait().expect("Error creating keyspace");
    session.execute(&create_table).wait().expect("Error creating table");

    let prepared_statement = session.prepare(insert_statement).unwrap().wait().expect("Error creating prepared statement");
    let mut batch = Batch::new(batch_type);

    for counter in 1..=total_number_rows
    {
        let mut statement = prepared_statement.bind();
        statement.bind(0, counter).expect("Error setting example_table.id");
        statement.bind(1, random_characters(insert_string_length).as_str()).expect("Error setting example_table.f1");
        batch.add_statement(&statement).expect("Error adding statement to batch");
        if counter%batch_size == 0
        {
            session.execute_batch(&batch).wait().expect("Error executing batch");
            batch = Batch::new(batch_type);
        }
    }
}

