// for yugabyte cloud
// authentication
// TLS
// This example is inspired on: https://github.com/scylladb/scylla-rust-driver/blob/main/examples/tls.rs

use scylla::{IntoTypedRows, Session, SessionBuilder};
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};
use std::env;

#[tokio::main]
async fn main() {
    let known_nodes: String = env::var("KNOWN_NODES").expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();

    let ca_certificate: String = env::var("CA_CERTIFICATE").expect("CA_CERTIFICATE must be set");
    let username: String = env::var("USER_NAME").expect("USER_NAME must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let mut ssl_context = SslContextBuilder::new(SslMethod::tls()).expect("Error creating ssl context");
    ssl_context.set_ca_file(ca_certificate).expect("Error loading CA certificate");
    // SslVerifyMode::PEER for use with self-signed certificate, together with CA certificate.
    // SslVerifyMode::NONE for using SSL without client certificate.
    ssl_context.set_verify(SslVerifyMode::PEER);

    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .user(username, password)
        .ssl_context(Some(ssl_context.build()))
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
    };
}
