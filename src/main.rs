extern crate rocksdb as rdb;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate futures;

#[macro_use]
extern crate queues;


mod engine;
mod tokenizer;

use dotenv::dotenv;

fn main() {

    dotenv().ok();
}