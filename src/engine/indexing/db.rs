pub mod ctx;
pub mod inverted_list;
pub mod pq_storage;
pub mod vector_storage;
pub mod search;

use lmdb::Error;

use crate::engine::utils::types::{KNN, SegmentId};

use self::{ctx::Context};


// Mayor functions when interacting with the database
// depending on the usage the implementor is seeking for, it should be better to split this trait 
// into different ones such as: Load, Dump, Search...
pub trait DBInterface {
    fn dump<I>(&self, raw_type: &I) -> Result<(), Error>;

    fn load<T>(&self) -> Result<T, Error>
    where
        T: DBInterface;
        
    fn search(&self, ctx: &Context) -> Result<KNN, Error>;
}

// instead of implementing Load, it would be more rational that from_caller implemented some trait to bind the Rc references
pub trait Load {
    fn load(&mut self, segments: &[SegmentId], from_caller: &impl Load) -> Result<(), Error>;
}

pub trait Dump {
    fn dump(&self) -> Result<(), Error>;
}