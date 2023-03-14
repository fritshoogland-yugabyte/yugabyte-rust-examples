// for local deployment
// no authentication
// no TLS
// This example is inspired and largely taken from: https://github.com/scylladb/scylla-rust-driver/blob/main/examples/basic.rs

use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let known_nodes: String = env::var("KNOWN_NODES").expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();

    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .build()
        .await?;
    if let Some(rows) = session.query("SELECT keyspace_name FROM system_schema.keyspaces", &[]).await?.rows {
        for row in rows.into_typed::<(Option<String>,)>() {
            let read_row: (Option<String>,) = row?;
            println!("keyspace_name = {}", read_row.0.unwrap());
        }
    }
    Ok(())
}
