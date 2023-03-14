// for yugabyte cloud
// authentication
// TLS
// This example is inspired on: https://github.com/Metaswitch/cassandra-rs/blob/main/examples/simple.rs

use cassandra_cpp::*;
use std::{fs, env};

fn main() {
    // reading the variables from the environment variables
    let contact_points: String = env::var("CONTACT_POINTS").expect("CONTACT_POINTS must be set, comma separated for multiple: HOSTNAME[,HOSTNAME]");
    let ca_certificate: String = env::var("CA_CERTIFICATE").expect("CA_CERTIFICATE must be set");
    let username: String = env::var("USER_NAME").expect("USER_NAME must be set");
    let password: String = env::var("PASSWORD").expect("PASSWORD must be set");

    let query = stmt!("SELECT keyspace_name FROM system_schema.keyspaces;");

    let ca_cert = fs::read_to_string(ca_certificate).expect("Error reading CA certificate");
    let mut ssl = cassandra_cpp::Ssl::default();
    cassandra_cpp::Ssl::add_trusted_cert(&mut ssl, &ca_cert).expect("Error adding CA certificate to SSL");
    let verify_level = vec![cassandra_cpp::SslVerifyFlag::PEER_IDENTITY_DNS];
    cassandra_cpp::Ssl::set_verify_flags(&mut ssl, &verify_level);

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_points.as_str()).expect("Error setting contact points");
    cluster.set_ssl(&mut ssl);
    cluster.set_credentials(username.as_str(), password.as_str()).unwrap();

    let session = cluster.connect().expect("Error connecting to cluster");

    let result = session.execute(&query).wait().expect("Error executing query");
    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name").expect("Error getting row for column using get_by_name()");
        println!("keyspace_name: {}", col);
    }
}
