// for local deployment
// no authentication
// no TLS
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::new;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::IntoRustByName;

const NODE_ADDRESS: &str = "192.168.66.80:9042";

fn main() {
    let node = NodeTcpConfigBuilder::new(NODE_ADDRESS, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session = new(&cluster_config, RoundRobin::new()).unwrap();

    let query= "SELECT keyspace_name FROM system_schema.keyspaces";
    let rows = session.query(query).unwrap()
        .get_body()
        .unwrap()
        .into_rows()
        .unwrap();
    for row in rows.iter() {
        let col: String = row.get_r_by_name("keyspace_name").unwrap();
        println!("ks name = {}", col);
    }
}