# cdrs 
This is a YugabyteDB example of the usage of the cdrs crate.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The first example uses no authentication and no SSL.
The only thing required to run is to set the NODE_ADDRESS environment variable:
```shell
NODE_ADDRESS="192.168.66.80:9042" cargo run --example simple
```

Example output:
```shell
fritshoogland@ip-192-168-1-116 cdrs % NODE_ADDRESS="192.168.66.80:9042" cargo run --example simple
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/simple`
ks name = system_auth
ks name = system_schema
ks name = system
```

The second example uses authentication and SSL.
For this example, NODE_ADDRESS, CA_CERTIFICATE, USER_NAME and PASSWORD must be set.

This is also how Yugabyte Cloud can be used:
```shell
NODE_ADDRESS="xxxx.aws.ybdb.io:9042" CA_CERTIFICATE="root.crt" USER_NAME="admin" PASSWORD="xxxx" cargo run --example cloud
```

Example output:
```shell
fritshoogland@ip-192-168-1-116 cdrs % NODE_ADDRESS="xxxx.aws.ybdb.io:9042" CA_CERTIFICATE="root.crt" USER_NAME="admin" PASSWORD="xxxx" cargo run --example cloud
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/cloud`
ks name = system_auth
ks name = system_schema
ks name = system
```