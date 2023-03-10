// for local deployment
// no authentication
// no TLS
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::new;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::IntoRustByName;
use std::env;

fn main() {
    let node_address: String = env::var("NODE_ADDRESS").expect("NODE_ADDRESS must be set");

    let node = NodeTcpConfigBuilder::new(node_address.as_str(), NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session = new(&cluster_config, RoundRobin::new()).expect("Error creating session");

    let query= "SELECT keyspace_name FROM system_schema.keyspaces";
    let rows = session.query(query).expect("Error executing query")
        .get_body()
        .unwrap()
        .into_rows()
        .unwrap();
    for row in rows.iter() {
        let col: String = row.get_r_by_name("keyspace_name").expect("Error getting row for column using get_r_by_name()");
        println!("keyspace_name = {}", col);
    }
}