// for local deployment
// no authentication
// no TLS
// This example is inspired and largely taken from: https://github.com/scylladb/scylla-rust-driver/blob/main/examples/basic.rs

use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let known_nodes: String = env::var("KNOWN_NODES").expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();

    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .build()
        .await
        .expect("Error creating session");

    if let Some(rows) = session.query("SELECT keyspace_name FROM system_schema.keyspaces", &[])
        .await
        .expect("Error executing query")
        .rows
    {
        for row in rows.into_typed::<(Option<String>,)>()
        {
            let read_row: (Option<String>,) = row.expect("Error reading row");
            println!("keyspace_name = {}", read_row.0.expect("Error reading column 0"));
        }
    }
}
