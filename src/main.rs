extern crate lmdb_zero as lmdb;
extern crate dotenv;

mod engine;
mod tokenizer;

use dotenv::dotenv;

fn main() {

    dotenv().ok();
}