# cdrs_tokio
Github: https://github.com/krojew/cdrs-tokio  
Documentation: https://docs.rs/cdrs-tokio/latest/cdrs_tokio/

This is a YugabyteDB example of the usage of the cdrs_tokio crate.

Please mind this crate does show recent activity, but does not provide SSL support like the cdrs crate.
There is a feature for TLS, but this generate crate version issues.

## Local example

The `local` example is for local early-stage testing and development with YugabyteDB. With those deployments, you run YugabyteDB on your local laptop or on-premises environemnt and don't need to set up SSL and authentication.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

In order to run this example, setup a YugabyteDB database:
1. Deploy a local YugabyteDB instance:
    - [Local deployment on Mac, Linux, Docker or Kubernetes](https://docs.yugabyte.com/preview/quick-start/)
2. Pass a list of YugabyteDB connection endpoints in the `NODE_ADDRESS` environment variable and start the example:

```shell
NODE_ADDRESS="192.168.66.80:9042" cargo run --example local
```

Example output:
```shell
➜ NODE_ADDRESS="192.168.66.80:9042" cargo run --example local
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/local`
keyspace_name = system_auth
keyspace_name = system_schema
keyspace_name = system
```