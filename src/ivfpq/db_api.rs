use std::{path::Path, marker::PhantomData};
use super::{primitive_types::{DBResult, Codebook}, 
            ivfpq::{InvertedIndex, CQ_K_CENTROIDS}
};
use rocksdb::{DB, Options};
use serde_cbor;

// DATABASE
// will store: Inverted Index File with all its entries
//             Codebook as for subquantizers which will build up to the CQ Codebook
//             CQ and each subquantizers' states somehow??

// the ivf will be working in-memory, although it will get eventually flushed to disk



struct Closed {}
struct Open {}

// represent database wrapper to make common calls (put, write...)
pub struct DatabaseWrapper<T>{
    database: DB,
    _open: PhantomData<T>
}

fn db_options() -> Options {
    let mut options = Options::default();
    options.create_if_missing(true);
    options
}

impl<> DatabaseWrapper<Closed> {

    pub fn open(path: &Path) -> DBResult<DatabaseWrapper<Open>> {
        let db = DB::open(&db_options(), path)?;
        Ok(DatabaseWrapper::<Open> {
            database: db,
            _open: PhantomData
        })
    }
}

impl<> DatabaseWrapper<Open> {

    pub fn persist_codebook(&self, codeb: Codebook) -> DBResult<()> {
        // add embs to the database
        // serialize them into byte arrays
        // search how to udpate or set some values in rdb
        let key = b"codebook";
        match self.database.get(key)? {
            Some(codebook) /* Deserialize codebook & add embedding */ => {
                // deserialize
                let deserialized_cb: Codebook = serde_cbor::from_slice(&codebook).expect("Deserialization failed: ");

                // add embedding if changed
                if codeb != deserialized_cb {
                    // remove current codebook
                    self.database.delete(key)?;
                    // serialize codebook
                    let serialized_cb = serde_cbor::to_vec(&codeb).expect("Serialization failed");
                    self.database.put(key, serialized_cb)?;
                }
            },
            None /* Create Codebook (OnDisk, create it without looking for changes) */ => {
                self.database.put(key, serde_cbor::to_vec(&codeb).expect("Serialization failed"))?;
            }
        }
        Ok(())
    }

    /// this function will be called when first loading the codebook
    pub fn load_codebook(&self) -> DBResult<Codebook> {
        let key = b"codebook";
        match self.database.get(key)? {
            Some(codebook) /* Deserialize Codebook */ => {
                Ok(serde_cbor::from_slice(&codebook).expect("Failed Deserializing:"))
            },
            None /* Create Codebook (InMemory) */ => {
                Ok([Default::default(); CQ_K_CENTROIDS])
            }
        }
        
    }

    pub fn persist_ivf(&self, ivf: InvertedIndex) -> DBResult<()> {
        // same as persist_codebook
        let key = b"ivf";
        match self.database.get(key)? {
            Some(db_ivf) /* Deserialize IVF & add embedding */ => {
                // deserialize
                let deserialized_ivf: InvertedIndex = serde_cbor::from_slice(&db_ivf).expect("Deserialization failed: ");

                // add entry if changed
                if ivf != deserialized_ivf {
                    // remove current ivf
                    self.database.delete(key)?;
                    // serialize ivf
                    let serialized_cb = serde_cbor::to_vec(&ivf).expect("Serialization failed");
                    self.database.put(key, serialized_cb)?;
                } 
            },
            None /* Create IVF (OnDisk, create it without looking for changes) */ => {
                self.database.put(key, serde_cbor::to_vec(&ivf).expect("Serialization failed"))?;
            }
        }
        Ok(())
    }

    pub fn load_ivf(&self) -> DBResult<InvertedIndex> {
        // same as load_codebook
        let key = b"ivf";
        match self.database.get(key)? {
            Some(ivf) /* Deserialize IVF */ => {
                Ok(serde_cbor::from_slice(&ivf).expect("Error Deserializing: "))
            },
            None /* Create IVF (InMemory) */ => {
                Ok(InvertedIndex::empty())
            }
        }
    }


}

/// must be ran with -- --test-threads=1 or else db lock will only be acquired by one test
#[cfg(test)]
mod tests {
    use crate::ivfpq::{primitive_types::{Embedding, Segment, IVListEntry}, ivfpq::{SEGMENT_DIM, EMBEDDING_M_SEGMENTS, AvlWrapper, CODE_SIZE}};

    use super::*;
    #[test]
    fn work_with_codebook() {
        let db = DatabaseWrapper::open(Path::new("./dbre")).expect("Opening failed: ");
        let mut codebook = db.load_codebook().unwrap();
        let segment = Segment::new([-1.; SEGMENT_DIM]);
        codebook[0] = Embedding::new([segment; EMBEDDING_M_SEGMENTS]);
        codebook[1] = Embedding::new([segment; EMBEDDING_M_SEGMENTS]);
        let cb_clone = codebook.clone();
        db.persist_codebook(codebook).unwrap();
        assert_eq!(cb_clone, db.load_codebook().unwrap())
    }

    #[test]
    fn work_with_inverted_index() {
        let db = DatabaseWrapper::open(Path::new("./dbre")).expect("Opening failed: ");
        let mut ivf = db.load_ivf().unwrap();
        let mut avl = AvlWrapper::new();
        avl.insert(123, Box::new(IVListEntry::new([1; EMBEDDING_M_SEGMENTS], 0)));
        avl.insert(124, Box::new(IVListEntry::new([1; EMBEDDING_M_SEGMENTS], 1)));
        ivf.push(avl);
        let ivf_clone = ivf.clone();
        db.persist_ivf(ivf).unwrap();
        let reloaded_ivf = db.load_ivf().unwrap();
        // only visible with -- --nocapture
        println!("{:?}", ivf_clone);
        println!("{:?}", reloaded_ivf);
        assert_eq!(ivf_clone, reloaded_ivf);
    }
}