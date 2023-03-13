# cdrs 
Github: https://github.com/AlexPikalov/cdrs  
Documentation: https://docs.rs/cdrs/latest/cdrs/

This is a YugabyteDB example of the usage of the cdrs crate.

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

## YugabyteDB Managed example
The `cloud` example demonstrates how to connect to the YCQL interface of a database running in YugabyteDB Managed.

1. [Deploy](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-quickstart/) a YugabyteDB Managed instance.
    1. You have to add your internet ip address to the ip allow list to allow YCQL access (step 2, network access). In most cases the [add current ip address] button should be able to do that.
    2. You have to set a username/password combination (step 3, db credentials). In most cases it should be appropriate to use the 'admin' user and its password, please download the credential file. This is the only time the password is shown.

2. Add your machine's ip address or local network ip address to the [IP Allow list](https://docs.yugabyte.com/preview/yugabyte-cloud/cloud-secure-clusters/add-connections/) (network access>ip allow list)

3. Obtain the CA (certificate authority) certificate file: clusters>click cluster>connect (upper left corner)>connect your application>click [download CA Cert] with 1. The filename generally is `root.crt`.

4. Obtain the hostname: clusters>click cluster>connect (upper left corner)>connect your application>click [YCQL] with 2.>select hostname with 'host'.

5. Start the example by providing the `NODE_ADDRESS`, `CA_CERTIFICATE`, `USER_NAME` and `PASSWORD` environment variables

```shell
NODE_ADDRESS="(obtained with 4.)" CA_CERTIFICATE="(obtained with 3.)" USER_NAME="(set during 1. default 'admin')" PASSWORD="(see credentials file)" cargo run --example cloud
```

Example output:
```shell
➜ NODE_ADDRESS="xxx.ybdb.io:9042" CA_CERTIFICATE="root.crt" USER_NAME="admin" PASSWORD="xxxx" cargo run --example cloud
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/cloud`
keyspace_name = system_auth
keyspace_name = system_schema
keyspace_name = system
```
