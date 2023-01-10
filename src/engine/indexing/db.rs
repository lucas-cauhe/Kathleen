pub mod ctx;
pub mod inverted_list;
pub mod pq_storage;
pub mod vector_storage;
pub mod search;

use lmdb::Error;


// Mayor functions when interacting with the database
pub trait DBInterface {
    fn dump<I>(&self, raw_type: &I) -> Result<(), Error>;

    fn load(&self) -> Result<dyn DBInterface, Error>;
}