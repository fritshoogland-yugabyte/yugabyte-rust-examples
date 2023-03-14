// for local deployment
// no authentication
// no TLS
// This example is inspired and largely taken from: https://github.com/Metaswitch/cassandra-rs/blob/main/examples/simple.rs

use cassandra_cpp::*;
use std::env;

fn main() {
    // contact points set the ip addresses as a comma separated list, and is mandatory.
    // see https://docs.rs/cassandra-cpp/latest/cassandra_cpp/struct.Cluster.html#method.set_contact_points
    let contact_points: String = env::var("CONTACT_POINTS").expect("CONTACT_POINTS must be set, comma separated for multiple: HOSTNAME[,HOSTNAME]");
    // by default, port 9042 is used, which can be changed using cassandra_cpp::Cluster::set_port().

    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces");

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points.as_str()).expect("Error setting contact points");
    let session = cluster.connect().expect("Error connecting to cluster");

    let result = session.execute(&query).wait().expect("Error executing query");
    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name").expect("Error getting row for column using get_by_name()");
        println!("keyspace_name: {}", col);
    }
}

