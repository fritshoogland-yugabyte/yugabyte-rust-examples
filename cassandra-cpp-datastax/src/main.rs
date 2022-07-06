// for local deployment
// no authentication
// no TLS
use cassandra_cpp::*;

// fill out contact points
const CONTACT_POINTS: &str = "192.168.66.80,192.168.66.81,192.168.66.82";

fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces");

    let mut cluster = Cluster::default();
    cluster.set_contact_points(CONTACT_POINTS).unwrap();
    let session = cluster.connect().unwrap();

    let result = session.execute(&query).wait().unwrap();
    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name").unwrap();
        println!("ks name: {}", col);
    }
}

