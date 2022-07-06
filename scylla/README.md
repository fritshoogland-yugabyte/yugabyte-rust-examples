# scylla 
This is a YugabyteDB example of the usage of the asynchronous scylla crate.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The file `src/main.rs` contains a constant definition:
```rust
const KNOWN_NODE: &str = "";
```
Which must be set to reflect a known node address including colon portnumber.