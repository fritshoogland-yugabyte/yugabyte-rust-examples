// for local deployment
// no authentication
// no TLS
use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;

const KNOWN_NODE: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let session: Session = SessionBuilder::new()
        .known_node(KNOWN_NODE)
        .build()
        .await?;
    if let Some(rows) = session.query("SELECT keyspace_name FROM system_schema.keyspaces", &[]).await?.rows {
        for row in rows.into_typed::<(Option<String>,)>() {
            let read_row: (Option<String>,) = row?;
            println!("ks name = {}", read_row.0.unwrap());
        }
    }
    Ok(())
}
