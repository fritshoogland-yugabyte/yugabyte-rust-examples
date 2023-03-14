// for local deployment
// no authentication
// no TLS
use cdrs_tokio::cluster::session::{TcpSessionBuilder, SessionBuilder};
use cdrs_tokio::cluster::{NodeTcpConfigBuilder, NodeAddress};
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::types::IntoRustByName;
use std::env;

#[tokio::main]
async fn main() {
    let contact_points: Vec<NodeAddress> = vec![env::var("CONTACT_POINT").expect("CONTACT_POINT must be set").into()];

    let query= "SELECT keyspace_name FROM system_schema.keyspaces";

    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_points(contact_points)
        .build()
        .await
        .expect("Error creating cluster config");

    let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
        .build()
        .expect("Error creating session");

    let rows = session
        .query(query)
        .await
        .expect("Query execution error")
        .response_body()
        .expect("Error get body")
        .into_rows()
        .expect("Error into rows");

    for row in rows
    {
        let val: String = row.get_r_by_name("keyspace_name").expect("Error getting row for column using get_r_by_name()");
        println!("keyspace_name = {}", val);
    }
}