# scylla-cloud
This is a YugabyteDB example of the usage of the scylla crate.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The file `src/main.rs` contains some constant definitions:
```rust
const KNOWN_NODE: &str = "";
const CA_CERTIFICATE: &str = "";
const USERNAME: &str = "";
const PASSWORD: &str = "";
```
Which all must be set in order to be able to logon.
Known node address must be set to a node address including colon portnumber.