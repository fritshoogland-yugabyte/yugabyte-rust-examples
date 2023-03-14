// this is not a perfect or reasonable example.
// it does work, based on the documentation at: https://github.com/AlexPikalov/cdrs/blob/master/examples/prepare_batch_execute.rs
// what this does is create a batch of one and execute it.
//
// but it doesn't really allow to create a dynamic batch, or at least I cannot get this to be done.

#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

//type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

use std::env;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Row {
    id: i32,
    f1: String,
}

impl Row {
    fn into_query_values(self) -> QueryValues {
        query_values!(self.id, self.f1)
    }
}


fn main() {
    let node_address: String = env::var("NODE_ADDRESS").expect("NODE_ADDRESS must be set");

    let create_keyspace = "create keyspace if not exists example";
    let create_table = "create table if not exists example.example_table (id int primary key, f1 text)";
    let insert_statement = "insert into example.example_table (id, f1) values (?, ?)";

    let total_number_rows = 1_000_000;
    //let batch_size = 50;
    /*
    let insert_string_length = 200;
    let random_characters = |length: usize| -> String
        {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length.try_into().unwrap())
                .map(char::from)
                .collect()
        };

     */
    //println!("{}", random_characters(10));

    let node = NodeTcpConfigBuilder::new(node_address.as_str(), NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session = new_session(&cluster_config, RoundRobin::new()).expect("Error creating session");

    session.query(create_keyspace).expect("Error creating keyspace");
    session.query(create_table).expect("Error creating table");

    let prepared_statement = session.prepare(&insert_statement).expect("Error creating prepared statement");

    let mut batch = BatchQueryBuilder::new();

    for counter in 1..=total_number_rows
    {
        //let row = Row { id: counter, f1: random_characters(insert_string_length) };
        let row = Row { id: counter as i32, f1: "aaa".to_string() };
        /*
        let batch = BatchQueryBuilder::new()
            .add_query_prepared(prepared_statement.clone(), row.into_query_values())
            .finalize()
            .expect("aa");
        session.batch_with_params(batch).expect("error executing");
         */
        &batch.add_query_prepared(prepared_statement.clone(), row.into_query_values());

        //let batch = BatchQueryBuilder::new().add_query_prepared(prepared_statement.clone(), row.into_query_values()).build().expect("aa");

        //session.batch_with_params(batch).expect("Error executing batch");
        //let t = session.batch_with_params_tw(batch, true, true).expect("Error executing batch").unwrap();
        //println!("{:#?}", t);
        //if counter%batch_size == 0
        //{
            //println!("{:#?}", &batch);
            //batch.finalize().expect("Error finalizing batch");
            //session.batch_with_params(batch).expect("Error executing batch");
            //session.batch(batch).expect("Error executing batch");
            //batch = BatchQueryBuilder::new();
        //}
    }
}
