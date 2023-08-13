use scylla::{Session, SessionBuilder};
use scylla::ValueList;
use std::env;
use std::collections::HashMap;
use scylla::frame::response::result::CqlValue;

#[tokio::main]
async fn main() {
    let known_nodes: String = env::var("KNOWN_NODES").expect("KNOWN_NODES must be set: comma separated for multiple; HOSTNAME:PORT[,HOSTNAME:PORT]");
    let known_nodes: Vec<&str> = known_nodes.split(',').collect();

    let create_keyspace = "create keyspace if not exists example";
    let drop_table = "drop table if exists example.example_table";
    let create_table = "create table example.example_table (id int primary key, f1 text) with transactions  = { 'enabled' : true }";

    #[derive(ValueList)]
    struct Example {
       id: i32,
       f1: String,
    }
    let list = vec![Example{id: 1, f1: "aaaaa".to_string()}, Example{id: 2, f1: "bbbbb".to_string()}];

    let mut insert_statements = String::new();
    for row in &list
    {
       insert_statements += format!("insert into example.example_table( id, f1 ) values ( {}, '{}' ); ", row.id, row.f1).as_str();
    }
    
    println!("{}", insert_statements);
   
    let transaction = format!("begin transaction {insert_statements} end transaction;");

    //println!("{:#?}", transaction);
    //println!("{:#?}", argument_values);
    let session: Session = SessionBuilder::new()
        .known_nodes(&known_nodes)
        .build()
        .await
        .expect("Error creating session");

    session.query(create_keyspace, &[]).await.expect("Error executing create keyspace");
    session.query(drop_table, &[]).await.expect("Error executing drop table");
    session.query(create_table, &[]).await.expect("Error executing create table");

    session.query(transaction, &[]).await.expect("Error executing query");
}
