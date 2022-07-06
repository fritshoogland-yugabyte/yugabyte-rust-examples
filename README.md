# Rust examples for YugabyteDB

## YSQL

## YCQL

### cassandra-cpp with the datastax driver
The cassandra-cpp driver for Rust uses the cassandra c++ driver, and uses the function in it via the Rust FFI (foreign function interface).
The way the cassandra-cpp driver crate works is that it actually is a wrapper around the raw driver binding crate cassandra-cpp-sys.

#### driver installation
#### OSX
```shell
brew install cassandra-cpp-driver openssl
```
#### Linux
EL8 (validated with Alma 8):
```shell
yum install https://downloads.datastax.com/cpp-driver/centos/8/cassandra/v2.16.0/cassandra-cpp-driver-2.16.0-1.el8.x86_64.rpm
```
EL7 (validated with Centos 7):
```shell
yum install https://downloads.datastax.com/cpp-driver/centos/7/cassandra/v2.16.0/cassandra-cpp-driver-2.16.0-1.el7.x86_64.rpm
```
[Cargo project with no authentication](cassandra-cpp-datastax).  
[Cargo project with SSL and authentication, ready for Yugabyte Cloud](cassandra-cpp-datastax-cloud).

### CDRS
The cdrs driver for Rust is a 'pure rust' driver, meaning it has no non-rust dependencies.  
[Cargo project with no authentication](cdrs).
[Cargo project with SSL and authentication, ready for Yugabyte Cloud](cdrs-cloud).

### scylla
The scylla driver for Rust is a 'pure rust' driver, meaning it has no non-rust dependencies.  
The scylla driver is an asynchronous driver.  
The driver obviously is created for the Scylla database, but it can also be used for any Cassandra compatible database.  
[Cargo project with no authentication](scylla).  
[Cargo project with SSL and authentication, ready for Yugabyte Cloud](scylla-cloud).

