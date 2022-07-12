# cassandra-cpp
This is a YugabyteDB example of the usage of the cassandra-cpp crate, using the datastax cpp driver.

The examples are deliberately kept to a minimum, and perform two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The first example uses no authentication and no SSL. 
The only thing required to run is to set the CONTACT_POINTS environment variable:
```shell
CONTACT_POINTS="192.168.66.80,192.168.66.81,192.168.66.82" cargo run --example simple
```

Example output:
```shell
fritshoogland@ip-192-168-1-116 cassandra-cpp % CONTACT_POINTS="192.168.66.80,192.168.66.81,192.168.66.82" cargo run --example simple
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/simple`
ks name: system_auth
ks name: system_schema
ks name: system
```

The second example uses authentication and SSL.
For this example, CONTACT_POINTS, CA_CERTIFICATE, USER_NAME and PASSWORD must be set.

This is also how Yugabyte Cloud can be used:
```shell
CONTACT_POINTS="xxxx.aws.ybdb.io" CA_CERTIFICATE="root.crt" USER_NAME="admin" PASSWORD="xxxx" cargo run --example cloud
```

Example output:
```shell
fritshoogland@ip-192-168-1-116 cassandra-cpp % CONTACT_POINTS="xxxx.aws.ybdb.io" CA_CERTIFICATE="root.crt" USER_NAME="admin" PASSWORD="xxxx" cargo run --example cloud
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/examples/cloud`
ks name: system_auth
ks name: system_schema
ks name: system
```