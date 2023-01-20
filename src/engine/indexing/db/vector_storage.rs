



// Divide each vector into k sub spaces
// Find nearest centroid

use std::{rc::Rc, collections::HashMap};

use lmdb_zero::{Environment, Error};

use crate::{tokenizer::tokenize::Embedding, engine::utils::types::{VecId, SegmentId}};
use super::{DBInterface, Load, Dump};



// SegmentContainers will initially be inside each cluster environment but should be in a global scope 
// so that many embeddings' subspaces match many segments
pub struct SegmentContainer {
    
    segment: Embedding,
    segment_id: SegmentId,
}

pub struct SegmentHolder {
    env: Environment,
    // in the given environment, all segments in segments are currently being reference by some embedding
    // the access to each segment container must be instant or else it would be easier to load at every time a new segment -> HashMap

    // this should not be a RefCell since EC trying to access this SH mustn't mutate the SC itself, but perhaps the state of the hashmap
    // via SH's interface
    segments: HashMap<SegmentId,Rc<SegmentContainer>> 
}


pub struct EmbeddingContainer {
    embedding_id: VecId,
    refered_segments: Vec<Rc<SegmentContainer>>
}
// Why should the embedding holder keep already loaded embeddings while being in use
// 0. Keep track of embeddings being used
// 1. Act as cache
// 2. If there's a search being done while 
pub struct EmbeddingHolder {
    env: Environment,
    embeddings: Option<Vec<EmbeddingContainer>>
}

impl EmbeddingHolder {

    // Botleneck: since the database is designed to have as little segments as possible, each segment from an
    // embedding will be added one at a time in order to check for duplicates (and not really add it) or consider some merging
    // in the case one given db segment is almost identical to another trying to get added

    

    pub fn add_embedding(&self, embedding: &usize) -> Result<(), Error> {
        todo!()
    }

    pub fn add_all_embeddings(&self) -> Result<(), Error> {
        todo!()
    }

    // Process of deletion: load requested segment container, decrement referenced field by 1, if referenced is 0 go ahead and delete
    // the entire segment from the db, Rc procedure

    pub fn del_embedding(&self, embedding: &usize) -> Result<EmbeddingContainer, Error> {
        todo!()
    }

    pub fn del_embeddings(&self, embeddings: &[usize]) -> Result<Vec<EmbeddingContainer>, Error> {
        todo!()
    }

    pub fn get_cached_embeddings(&self) -> Option<Vec<EmbeddingContainer>> {
        self.embeddings
    }

}

// the db context is telling you to load the required embeddings to perform some operations
impl Load for EmbeddingHolder {
    // loads the embeddings from the embeddingHolder's env into embeddings if they haven't been already loaded
    fn load(&mut self) -> Result<Self, Error> {
        
        // fetch embeddings from db

            // if the embeddingHolder embeddings field is not None => return those

            //  for every embedding, load it into a new embeddingContainer
            //  (SegmentHolder Load implementation)  for every segment required by an embeddingContainer, load into the segmentHolder the required segment in the form of a segmentContainer
            //      if it doesn't already exists in which case you'll increase the reference by 1
            // Wrap all this into an EmbeddingHolder

            // unload database memory if any was cached

        // if the embeddingHolder embeddings field was None => update it
        // return an updated version of yourself since the embeddingHolder for an env is cached in the context

    }
}

// whenever an embeddingHolder is no longer wanted to be cached in the context, it must be dumped
// to the db since it might have changed over the period of its usage
impl Dump for EmbeddingHolder {
    fn dump(&self) -> Result<(), Error> {
        todo!()   
    }
}


impl Drop for EmbeddingHolder {
    fn drop(&mut self) {
        // special way to drop the dependents
    }
}


impl SegmentHolder {



    pub fn not_cached(&self, segments: &[SegmentId]) -> Option<Vec<SegmentId>> {
        let not_cached_segments: Vec<SegmentId> = segments.iter().filter(|s_id| !self.segments.contains_key(*s_id)).collect();
        match not_cached_segments.len() {
            0 => None,
            _ => Some(not_cached_segments)
        }
    }

    pub fn update_cache(&mut self, container: SegmentContainer) -> () {
        self.segments.insert(container.segment_id, Rc::new(container));
    }

    pub fn cached_segments(&self, segments: &[SegmentId]) -> Vec<Rc<SegmentContainer>> {
        let result = vec![];
        for segment_id in segments {
            let ref_segment = self.segments.get(&segment_id).unwrap();
            result.push(Rc::clone(ref_segment))
        }
        result
    }

}


impl Load for SegmentHolder {
    fn load(&mut self, segments: &[SegmentId], from_caller: &impl Load) -> Result<(), Error> {
        
        // fetch segments from db
            
            // if the SH already contains the required segments => increase reference by 1
        
            // otherwise take them from the db and create new entries in the db with their ids.
        if let Some(uncached) = self.not_cached(segments) {
            let db = lmdb::Database::open(
                &self.env, Some("segments"), &lmdb::DatabaseOptions::new(lmdb::db::CREATE))?;
            
            {
                let txn = lmdb::WriteTransaction::new(&self.env)?;
                let access = txn.access();

                for new_segment_id in uncached {
                    let loaded_segment: Embedding = access.get(&db, &new_segment_id)?;
                    let segment = SegmentContainer::new(&loaded_segment, &new_segment_id);
                    self.update_cache(segment);
                }

                
            }
            // is memory cleaned when db is dropped??
            
        }
        
        // bound references to the load caller
        from_caller.bind(self.cached_segments(segments));

        Ok(())
    }
}



impl SegmentContainer {

    pub fn new(segment: Embedding, segment_id: &SegmentId) -> SegmentContainer {
        SegmentContainer { segment, segment_id }
    }

    // won't add it if it is a duplicate nor it is very similar to one in the database
    pub fn add_segment(&self, embedding_id: &VecId) -> Result<(), Error> {
        todo!()
    }

    // For that similarity search maybe you could place each repo object field segments together so that comparing got easier and 
    // you could identify when two segments refer to the same value for a given field (e.g. both refer to the same programming language)
    

}

