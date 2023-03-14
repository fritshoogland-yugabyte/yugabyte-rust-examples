// for yugabyte cloud
// authentication
// TLS
// This example is inspired on: https://github.com/scylladb/scylla-rust-driver/blob/main/examples/tls.rs

use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let known_nodes: String = env::var("KNOWN_NODES")expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();
    let ca_certificate: String = env::var("CA_CERTIFICATE").expect("CA_CERTIFICATE must be set");
    let username: String = env::var("USER_NAME").expect("USER_NAME must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let mut ssl_context = SslContextBuilder::new(SslMethod::tls())?;
    ssl_context.set_ca_file(ca_certificate).unwrap();
    // SslVerifyMode::PEER for use with self-signed certificate, together with CA certificate.
    // SslVerifyMode::NONE for using SSL without client certificate.
    ssl_context.set_verify(SslVerifyMode::PEER);

    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .user(username, password)
        .ssl_context(Some(ssl_context.build()))
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
