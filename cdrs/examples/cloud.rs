// for yugabyte cloud
// authentication
// TLS
use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::new_ssl;
use cdrs::cluster::{ClusterSslConfig, NodeSslConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::IntoRustByName;
use openssl::ssl::{SslConnector, SslMethod};
use std::env;

fn main() {
    let node_address: String = env::var("NODE_ADDRESS").expect("NODE_ADDRESS must be set");
    let ca_certificate: String = env::var("CA_CERTIFICATE").expect("CA_CERTIFICATE must be set");
    let username: String = env::var("USER_NAME").expect("USER_NAME must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let authenticator = StaticPasswordAuthenticator::new(username, password);
    let mut ssl_build = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_build.set_ca_file(ca_certificate).unwrap();
    let ssl_connector = ssl_build.build();

    let node = NodeSslConfigBuilder::new(node_address.as_str(), authenticator, ssl_connector).build();
    let cluster_config = ClusterSslConfig(vec![node]);
    let session = new_ssl(&cluster_config, RoundRobin::new()).unwrap();

    let query= "SELECT keyspace_name FROM system_schema.keyspaces";
    let rows = session.query(query).unwrap()
        .get_body().unwrap()
        .into_rows().unwrap();
    for row in rows.iter() {
        let col: String = row.get_r_by_name("keyspace_name").unwrap();
        println!("ks name = {}", col);
    }
}
