// for local deployment
// no authentication
// no TLS
// This example is inspired and largely taken from: https://github.com/scylladb/scylla-rust-driver/blob/main/examples/basic.rs

use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let known_node: String = env::var("KNOWN_NODE").expect("KNOWN_NODE must be set");

    let session: Session = SessionBuilder::new()
        .known_node(known_node)
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
