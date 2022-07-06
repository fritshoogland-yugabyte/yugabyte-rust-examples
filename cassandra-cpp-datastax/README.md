# cassandra-cpp-datastax
This is a YugabyteDB example of the usage of the cassandra-cpp crate, using the datastax cpp driver.

The example is deliberately kept to a minimum, and performs two things:
1. creating a session.
2. selecting the keystore names (SELECT keyspace_name FROM system_schema.keyspaces).

The file `src/main.rs` contains a constant definition:
```rust
const CONTACT_POINTS: &str = "";
```
Which must be set to reflect the local contact points.