
// UNDERLAYING LAYER FROM ABSTRACTION LAYERS OF GRAPH DESIGN 
// IVF CALLS SHOULD BE PROCESSED THROUGH HERE SO THAT MEMORY ALLOCATION AND ACCESS WOULD BE EFFICIENT

/*
    THIS COULD REPRESENT A TINY CLUSTER WHICH IS EASY TO MOVE AROUND AND MANAGE
    HENCE, LARGER AMOUNTS OF MEMORY, IF NEEDED TO BE REALLOCATED, WILL ONLY COST SOME MEMORY INSTANCES TO WORK OUT, NOT THE WHOLE THING
    THIS WOULD REPRESENT GH DATA CRAWLED FROM A USER -> A CLUSTER WOULD HOLD MANY MemInstances WHICH, GIVEN A CASE, A SINGLE USER
    MEMORY COULD BE MOVED SOMEWHERE ELSE

    THEREFORE A WHOLE CLUSTER STRUCTURE WILL HAVE A BOXED REFERENCE TO EACH OF ITS MemInstances WHILE EACH MemInstance WILL OWN ALL OF
    ITS FIELDS, INCLUDING BLOBS.

*/

use crate::{tokenizer::tokenize::Embedding, engine::indexing::index::RepoFlat};



pub struct MemInstance {



    // Owned Blobs
    blobs: Vec<Blob>
}

// if I am holding all the info in the database the Blob object should own all of its fields
// if the user is keeping his vectorized crawled repos, BlobUser structure should be used

pub struct Blob {
    // Vector Representation of the repo_obj
    // (V. dim)*sizeof(i32)
    embedding: Embedding,

    repo_obj: RepoFlat,
    
}


// this means that any BlobUser structure would be accessible while a user script is being ran, i.e. the user is active
// when only one user is browsing, no other structures could be found in the db
    // NEEDS A FIX
pub struct BlobUser<'a> {

    // Vector Representation of the repo_obj
    // (V. dim)*sizeof(i32)
    embedding: Embedding,

    // it stores a flattened version of the original Repo object
    repo_obj: &'a RepoFlat,

    

}

impl Blob {
    pub fn get_embedding<'a>(&self) -> &'a Embedding {
        &self.embedding
    }

    pub fn get_repo_obj<'a>(&self) -> &'a RepoFlat {
        &self.repo_obj
    }
}


// Declare unsafe operations