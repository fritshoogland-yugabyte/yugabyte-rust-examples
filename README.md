# Rust examples for YugabyteDB

## YCQL
### cassandra-cpp with the Yugabyte C++ driver
The cassandra-cpp driver for Rust uses the Cassandra C++ driver, and uses the functions in it via the Rust FFI (foreign functions interface).
The way the cassandra-cpp driver crate works is that it actually is a wrapper around the raw driver binding crate cassandra-cpp-sys, which provides the actual bindings.

#### driver installation
##### OSX
```shell
brew install autoconf automake cmake libtool libuv openssl zlib git
git clone https://github.com/yugabyte/cassandra-cpp-driver.git
cd cassandra-cpp-driver
mkdir build
pushd build
cmake ..
make
make install
popd
```
The `make install` command installs the driver (libcassandra.dylib) in `/usr/local/lib`.
##### Linux (EL)
EL7 (validated with Centos 7)
```shell
sudo yum install automake cmake gcc-c++ git libtool openssl-devel zlib-devel
sudo yum install https://downloads.datastax.com/cpp-driver/centos/7/dependencies/libuv/v1.35.0/libuv-1.35.0-1.el7.x86_64.rpm
sudo yum install https://downloads.datastax.com/cpp-driver/centos/7/dependencies/libuv/v1.35.0/libuv-devel-1.35.0-1.el7.x86_64.rpm
git clone https://github.com/yugabyte/cassandra-cpp-driver.git
cd cassandra-cpp-driver
mkdir build
pushd build
cmake ..
make
sudo make install
popd
```
The `make install` command install the driver (libcassandra.so) in `/usr/local/lib64`.
In order to make the dynamic loader find the library add the directory to the LD_LIBRARY_PATH environment variable:
```shell
export LD_LIBRARY_PATH=/usr/local/lib64
```
The error that is shown when the libcassandra.so driver is not in the path is:
```
target/debug/examples/local: error while loading shared libraries: libcassandra.so.2: cannot open shared object file: No such file or directory
```
##### Linux (EL)
EL8 (validated with Alma 8)
```shell
sudo yum install automake cmake gcc-c++ git libtool openssl-devel zlib-devel
sudo yum install https://downloads.datastax.com/cpp-driver/centos/8/dependencies/libuv/v1.35.0/libuv-1.35.0-1.el8.x86_64.rpm
sudo yum install https://downloads.datastax.com/cpp-driver/centos/8/dependencies/libuv/v1.35.0/libuv-devel-1.35.0-1.el8.x86_64.rpm
git clone https://github.com/yugabyte/cassandra-cpp-driver.git
cd cassandra-cpp-driver
mkdir build
pushd build
cmake ..
make
sudo make install
popd
```
The `make install` command installs the driver (libcassandra.so) in `/usr/local/lib64`, which is not in the default library path.
In order to make the dynamic loader find the library add the directory to the LD_LIBRARY_PATH environment variable:
```shell
export LD_LIBRARY_PATH=/usr/local/lib64
```
The error that is shown when the libcassandra.so driver is not in the path is:
```
target/debug/examples/local: error while loading shared libraries: libcassandra.so.2: cannot open shared object file: No such file or directory
```
[Cargo project with examples](cassandra-cpp).

----
### cassandra-cpp with the DataStax driver
The cassandra-cpp driver for Rust uses the Cassandra C++ driver, and uses the functions in it via the Rust FFI (foreign function interface).
The way the cassandra-cpp driver crate works is that it actually is a wrapper around the raw driver binding crate cassandra-cpp-sys, which provides the actual bindings.

#### driver installation
##### OSX
```shell
brew install cassandra-cpp-driver openssl
```
##### Linux (EL)
EL8 (validated with Alma 8):
```shell
yum install https://downloads.datastax.com/cpp-driver/centos/8/cassandra/v2.16.0/cassandra-cpp-driver-2.16.0-1.el8.x86_64.rpm
```
EL7 (validated with Centos 7):
```shell
yum install https://downloads.datastax.com/cpp-driver/centos/7/cassandra/v2.16.0/cassandra-cpp-driver-2.16.0-1.el7.x86_64.rpm
```
[Cargo project with examples](cassandra-cpp).  

----
### scylla
The scylla driver for Rust is a 'pure rust' driver, meaning it has no non-rust dependencies.  
The scylla driver is an asynchronous driver.  
The driver obviously is created for the Scylla database, but it can also be used for any Cassandra compatible database.  
[Cargo project with examples](scylla).

---
# WARNING
Below are crates that are available, but are not active or do lack reasonable SSL support.

### cdrs
The cdrs driver for Rust is a 'pure rust' driver, meaning it has no non-rust dependencies.  
[Cargo project with examples](cdrs).  
Warning: the cdrs driver project asks for a new maintainer, and looks inactive.

### cdrs-tokio
The cdrs-tokio driver for Rust is a 'pure rust' driver, meaning it has no non-rust dependencies.  
[Cargo project with examples](cdrs_tokio)
Warning: the cdrs-tokio driver project looks active, but seems to lack SSL/TLS support.  
(tls support is a feature, which is not documented and throws crate compatiblity issues)

### cassandra
The cassandra crate is the original crate with bindings to the cassandra c++ driver, but has been abandoned.