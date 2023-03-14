// for yugabyte cloud
// authentication
// TLS

// This doesn't work, because there is no SSL/TLS in the current public version.

use cdrs_tokio::authenticators::StaticPasswordAuthenticator;
use cdrs_tokio::cluster::session::Session;
use cdrs_tokio::cluster::NodeRustlsConfigBuilder;
use cdrs_tokio::query::*;
use cdrs_tokio::types::IntoRustByName;
use std::env;
use rustls::*;

#[tokio::main]
async fn main() {
    // reading the variables from the environment variables
    let contact_points: Vec<SocketAddr> = vec![env::var("CONTACT_POINT").expect("CONTACT_POINT must be set").into()];
    let ca_certificate: String = env::var("CA_CERTIFICATE").expect("CA_CERTIFICATE must be set");
    let username: String = env::var("USER_NAME").expect("USER_NAME must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let authenticator = StaticPasswordAuthenticator::new(username, password);

    let mut root_store = RootCertStore::new().add_pem_file(&ca_certificate).expect("Error loading CA certificate");

    let mut ssl_build = SslConnector::builder(SslMethod::tls()).expect("Error building ssl connector");
    ssl_build.set_ca_file(ca_certificate).expect("Error reading CA certificate");
    let ssl_connector = ssl_build.build();

    let node = NodeRustlsConfigBuilder::new(node_address.as_str(), authenticator, ssl_connector).build();
    let cluster_config = ClusterSslConfig(vec![node]);
    let session = new_tls(&cluster_config, RoundRobin::new()).unwrap();

    let query= "SELECT keyspace_name FROM system_schema.keyspaces";
    let rows = session.query(query).expect("Error executing query")
        .get_body().unwrap()
        .into_rows().unwrap();
    for row in rows.iter() {
        let col: String = row.get_r_by_name("keyspace_name").expect("Error getting row for column using get_r_by_name()");
        println!("keyspace_name = {}", col);
    }
}
