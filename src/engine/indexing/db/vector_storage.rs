



// Divide each vector into k sub spaces
// Find nearest centroid

use lmdb_zero::{Environment, Error};

use crate::{tokenizer::tokenize::Embedding, engine::utils::types::{VecId, SegmentId}};



pub struct EmbeddingContainer {
    embedding_id: VecId,
    subspaces: Vec<SegmentId>
}

pub struct SegmentContainer {
    
    segment: Embedding,
    segment_id: SegmentId,
    referenced: i32 // number of embeddings that rely on this segment
}

pub struct NewContainer {
    env: Environment,
    embeddings: Vec<EmbeddingContainer>
}

impl NewContainer {

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
    // the entire segment from the db

    pub fn del_embedding(&self, embedding: &usize) -> Result<EmbeddingContainer, Error> {
        todo!()
    }

    pub fn del_embeddings(&self, embeddings: &[usize]) -> Result<Vec<EmbeddingContainer>, Error> {
        todo!()
    }

}

impl SegmentContainer {

    // won't add it if it is a duplicate nor it is very similar to one in the database
    pub fn add_segment(&self, embedding_id: &VecId) -> Result<(), Error> {
        todo!()
    }

    // For that similarity search maybe you could place each repo object field segments together so that comparing got easier and 
    // you could identify when two segments refer to the same value for a given field (e.g. both refer to the same programming language)
    

}

