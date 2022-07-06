# cdrs-cloud
This is a YugabyteDB example of the usage of the cdrs crate.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The file `src/main.rs` contains some constant definitions:
```rust
const NODE_ADDRESS: &str = "";
const USERNAME: &str = "";
const PASSWORD: &str = "";
const CA_CERTIFICATE: &str = "";
```
Which all must be set in order to be able to logon.
Node address must be set to a node address including colon portnumber.