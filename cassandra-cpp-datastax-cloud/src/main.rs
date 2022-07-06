// for yugabyte cloud
// authentication
// TLS
use cassandra_cpp::*;
use std::fs;

// fill out contact points
const CONTACT_POINTS: &str = "";
const USERNAME: &str = "";
const PASSWORD: &str = "";
const CA_CERTIFICATE: &str = "";

fn main() {
    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");

    let ca_cert = fs::read_to_string(CA_CERTIFICATE).unwrap();
    let mut ssl = cassandra_cpp::Ssl::default();
    cassandra_cpp::Ssl::add_trusted_cert(&mut ssl, &ca_cert).unwrap();
    let verify_level = vec![cassandra_cpp::SslVerifyFlag::PEER_IDENTITY_DNS];
    cassandra_cpp::Ssl::set_verify_flags(&mut ssl, &verify_level);

    let mut cluster = Cluster::default();
    cluster.set_contact_points(CONTACT_POINTS).unwrap();
    cluster.set_ssl(&mut ssl);
    cluster.set_credentials(USERNAME, PASSWORD).unwrap();

    let session = cluster.connect().unwrap();

    let result = session.execute(&query).wait().unwrap();
    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name").unwrap();
        println!("ks name: {}", col);
    }
}
