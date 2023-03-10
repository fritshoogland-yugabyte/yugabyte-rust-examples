# YugabyteDB Cassandra Query Language Examples

These examples demonstrate how to use YugabyteDB Cassandra Query Language (YCQL) in Rust. 
The examples use the `cassandra-cpp` crate, which reuses the libcassandra.so driver from either the Yugabyte C++ based driver, or the DataStax C++ driver.

The samples show how to use the cassandra-cpp driver to establish a connection with a YugabyteDB cluster and execute a simple query. In particular. you'll learn how to:
1. Creating a database session
2. Execute a simple query such as the one that returns the keystore names (`SELECT keyspace_name FROM system_schema.keyspaces`)


## Local/On-Premises YugabyteDB Deployment

The `local` example is for local early-stage testing and development with YugabyteDB. With those deployments, you run YugabyteDB on your local laptop or on-prem environemnt and don't need to set up SSL and authentication.

1. Deploy a local YugabyteDB instance:
   - [Local deployment on Mac, Linux, Docker or Kubernetes](https://docs.yugabyte.com/preview/quick-start/)

2. Pass a list of YugabyteDB connection endpoints in the `CONTACT_POINTS` environment variable and start the example:
    ```shell
    CONTACT_POINTS="YB_NODE1_IP,YB_NODE2_IP" cargo run --example local
    ```
    
3. Confirm the application executed successfully
    ```shell
        Finished dev [unoptimized + debuginfo] target(s) in 0.14s
         Running `target/debug/examples/local`
    keyspace_name: system_auth
    keyspace_name: system_schema
    keyspace_name: system
    ```

## YugabyteDB Managed Deployment

The `cloud` example demonstrates how to connect to the YCQL interface of a database running in YugabyteDB Managed.

1. [Deploy](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-quickstart/) a YugabyteDB Managed instance.
   a. You have to add your internet ip address to the ip allow list to allow YCQL access (step 2, network access). In most cases the [add current ip address] button should be able to do that.
   b. You have to set a username/password combination (step 3, db credentials). In most cases it should be appropriate to use the 'admin' user and its password, please download the credential file. This is the only time the password is shown.

2. Add your machine's ip address or local network ip address to the [IP Allow list](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-secure-clusters/add-connections/) (network access>ip allow list)
 
3. Obtain the CA (certificate authority) certificate file: clusters>click cluster>connect (upper left corner)>connect your application>click [download CA Cert] with 1. The filename generally is `root.crt`.
 
4. Obtain the hostname: clusters>click cluster>connect (upper left corner)>connect your application>click [YCQL] with 2.>select hostname with 'host'.

3. Start the example by providing the `CONTACT_POINTS`, `CA_CERTIFICATE`, `USER_NAME` and `PASSWORD` environment variables
    ```shell
    CONTACT_POINTS="(obtained with 4.)" CA_CERTIFICATE="(obtained with 3.)" USER_NAME="(set during 1. default 'admin')" PASSWORD="()" cargo run --example cloud
    ```
4. Confirm the application executed sucesfully
    ```shell
        Finished dev [unoptimized + debuginfo] target(s) in 0.07s
         Running `target/debug/examples/cloud`
    keyspace_name: system_auth
    keyspace_name: system_schema
    keyspace_name: system
    ```
