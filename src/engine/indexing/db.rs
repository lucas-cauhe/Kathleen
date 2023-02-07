pub mod ctx;
pub mod inverted_list;
pub mod pq_storage;
pub mod dbaccess;
pub mod search;

use rdb::{Error, Options};

use crate::engine::utils::types::{KNN, SegmentId};

use self::{ctx::Context};

// Mayor functions when interacting with the database
// depending on the usage the implementor is seeking for, it should be better to split this trait 
// into different ones such as: Load, Dump, Search...
pub trait DBInterface {
    
}

// instead of implementing Load, it would be more rational that from_caller implemented some trait to bind the Rc references
/* pub trait Load {
    fn load(&mut self, segments: &[SegmentId]) -> Result<(), Error>;
}

pub trait Dump {
    fn dump(&self) -> Result<(), Error>;
} */


pub struct DBConfig {
    path: String,
    db_opts: Options
}

