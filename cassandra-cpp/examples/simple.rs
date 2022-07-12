// for local deployment
// no authentication
// no TLS
use cassandra_cpp::*;
use std::env;

// fill out contact points

fn main() {
    let contact_points: String = env::var("CONTACT_POINTS").expect("CONTACT_POINTS must be set");

    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces");

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points.as_str()).unwrap();
    let session = cluster.connect().unwrap();

    let result = session.execute(&query).wait().unwrap();
    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name").unwrap();
        println!("ks name: {}", col);
    }
}

