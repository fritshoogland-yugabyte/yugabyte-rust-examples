// for yugabyte cloud
// authentication
// TLS
use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};

const KNOWN_NODE: &str = "";
const CA_CERTIFICATE: &str = "";
const USERNAME: &str = "";
const PASSWORD: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut ssl_context = SslContextBuilder::new(SslMethod::tls())?;
    ssl_context.set_ca_file(CA_CERTIFICATE).unwrap();
    ssl_context.set_verify(SslVerifyMode::PEER);

    let session: Session = SessionBuilder::new()
        .known_node(KNOWN_NODE)
        .user(USERNAME, PASSWORD)
        .ssl_context(Some(ssl_context.build()))
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
