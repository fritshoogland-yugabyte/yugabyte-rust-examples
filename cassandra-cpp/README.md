# YugabyteDB Cassandra Query Language Examples

These examples demonstrate how to use YugabyteDB Cassandra Query Language (YCQL) in Rust. The examples reuse the standrad `cassandra-cpp` crate based on the DataStax cpp driver.

The current samples show how to use the standard Cassandra driver to establish a connection with a YugabyteDB cluster and execute a simple query. In particular. you'll learn how to:
1. Creating a database session
2. Execute a simple query such as the one that returns the keystore names (`SELECT keyspace_name FROM system_schema.keyspaces`)


## Local/On-Prem YugabyteDB Deployment

The `local` example is for local early-stage testing and development with YugabyteDB. With those deployments, you run YugabyteDB on your local laptop or on-prem environemnt and don't need to set up SSL and authentication.

1. Deploy a local YugabyteDB instance:
   - [Local deployment on Mac, Linux, Docker or Kubernetes](https://docs.yugabyte.com/preview/quick-start/)

2. Pass a list of YugabyteDB connection endpoints in the `CONTACT_POINTS` environment variable and start the example
    ```shell
    CONTACT_POINTS="YB_NODE1_IP, YB_NODE2_IP" cargo run --example local
    ```
    
3. Confirm the application executed successfully
    ```shell
        Finished dev [unoptimized + debuginfo] target(s) in 0.14s
         Running `target/debug/examples/simple`
    ks name: system_auth
    ks name: system_schema
    ks name: system
    ```

## YugabyteDB Managed Deployment

The `cloud` example demosntrates how to connect and use YugabyteDB Managed.

1. [Deploy](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-quickstart/) a YugabyteDB Managed instance

2. Add your machine's IP address to the [IP Allow list](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-secure-clusters/add-connections/)

3. Start the example by providing the `CONTACT_POINTS`, `CA_CERTIFICATE`, `USER_NAME` and `PASSWORD` environment variables
    ```shell
    CONTACT_POINTS="YB_MANAGED_HOST_NAME" CA_CERTIFICATE="PATH_TO_YB_MANAGED_ROOT_CRT" USER_NAME="YB_MANAGED_USER" PASSWORD="YB_MANAGED_PASSWORD" cargo run --example cloud
    ```
4. Confirm the application executed sucesfully
    ```shell
        Finished dev [unoptimized + debuginfo] target(s) in 0.07s
         Running `target/debug/examples/cloud`
    ks name: system_auth
    ks name: system_schema
    ks name: system
    ```
