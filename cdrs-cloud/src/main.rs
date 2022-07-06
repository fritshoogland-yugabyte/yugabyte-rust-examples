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

const NODE_ADDRESS: &str = "";
const USERNAME: &str = "";
const PASSWORD: &str = "";
const CA_CERTIFICATE: &str = "";

fn main() {

    let authenticator = StaticPasswordAuthenticator::new(USERNAME, PASSWORD);
    let mut ssl_build = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_build.set_ca_file(CA_CERTIFICATE).unwrap();
    let ssl_connector = ssl_build.build();

    let node = NodeSslConfigBuilder::new(NODE_ADDRESS, authenticator, ssl_connector).build();
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
