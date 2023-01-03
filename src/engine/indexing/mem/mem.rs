
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


// WHEN I SAY MEMORY I'M TRULY MEANING THE SPACE OCCUPIED IN MEMORY BY THE DB FILE MAPPED IN MEMORY


use std::{collections::HashMap, io};
use std::fs;
use crate::engine::graph::clusters::clusters::Cluster;
use crate::{tokenizer::tokenize::Embedding, engine::indexing::index::RepoFlat};

use lmdb::{self, Environment, Error};
use crate::engine::indexing::db::ctx::{CTX, create_env, insert_named};

type ContainerId = i32;
type ClusterId = i32;

// MemInstance will be dropped when required: merging data to db, after performing KNN, (more when working with clusters)
pub struct MemInstance {



    // Owned Blobs
    containers: Vec<Box<dyn Move>>,

    // first and last containers contained in this MemInstance
    // used for searching containers
    first: ContainerId,
    last: ContainerId,

    // used when merging mem instance to db
    cluster_id: Option<ClusterId>

}

impl MemInstance {
    // frees n Bytes representing k containers and returns an iterator over them
    // returns error if i/o actions failed or you're trying to free memory outside your container
    // this function won't call free_container() to make deletion faster
    // input must be multiple of Blob type
    // should first and last be modified, are ContainerIds gonna be lineal??
    pub fn free(&self, n: i32) -> Result<Vec<Box<dyn Move>>, io::Error>{
        todo!()
    }

    // frees memory from original cluster and allocates it in the new one
    pub fn transfer_to_cluster() -> () {
        todo!()
    }

    pub fn compute_belonging_cluster() -> () {
        todo!()
    }

    pub fn merge(&self) -> Result<(), Error> {
        // load cluster_id env from ctx
        let env: &Environment = unsafe {
            if let Some(e) = CTX.get(self.cluster_id.unwrap()) {
                e.into()
            } else {
                create_env()?
            }
        };

        insert_named(&self.containers, env)?;
        Ok(())
    }

    pub fn merge_consume(self) -> Result<(), Error> {
        self.merge()
    }
}

// Consider atomicity of these functions
pub trait Move {
    // frees a container
    // now the memory is owned by caller to this function and no longer to its MemInstance
    // returns error if i/o actions failed
    
    fn free_container(self: Box<Self>) -> Result<Box<dyn Move>, io::Error>;

    // dest_mem must be alligned to Container size
    // if dest_mem is None, reallocate will select the best place for the given destiny node
    fn reallocate(self: Box<Self>, dest_mem: i32, dest_node: ContainerId) -> Result<Box<dyn Move>, io::Error>;
    // container Id
    fn get_key(self: &Box<Self>) -> String;
    // flatten object
    fn get_object(self: &Box<Self>) -> String;
    fn get_embedding(self: &Box<Self>) -> Embedding;

}


// if I am holding all the info in the database the Blob object should own all of its fields
// if the user is keeping his vectorized crawled repos, BlobUser structure should be used

pub struct Container {
    // Vector Representation of the repo_obj
    // (V. dim)*sizeof(i32)
    embedding: Embedding,

    repo_obj: RepoFlat,

    id: ContainerId // containers_count in the time of insertion
    
}

// It will contain every connection to each processed neighbor cluster-node, hence path module will be built upon calls refered
// to this container
pub struct ClusterInfoContainer {

    // every similarity-computed processed neighbor
    // Optional since their similarity mightn't be computed yet
    neighbors: Vec<Option<f32>>, // if cluster ids are going to be randomly generated (large numbers) turn to hashmap below
    // neighbors: HashMap<i32, Option<f32>>, // K: Neighbor cluster id, V: Some(computed similarity between clusters)
    // same for stations
    stations: Vec<i32>, // Numbered stations between two cluster nodes, each value represents the station id

    containers_count: i32, // number of containers within the cluster

    centroid:  ContainerId,  // container id 

    // may not be necessary
    boundaries: Vec<ContainerId> // each value is the Container that acts as boundary to the cluster (used for computing similarities or distances)
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

impl Container {
    pub fn get_embedding<'a>(&self) -> &'a Embedding {
        &self.embedding
    }

    pub fn get_repo_obj<'a>(&self) -> &'a RepoFlat {
        &self.repo_obj
    }
}

impl Move for Container {
    fn free_container(self: Box<Self>) -> Result<Box<dyn Move>, io::Error> {
        todo!()
    }

    fn reallocate(self: Box<Self>, dest_mem: i32, dest_node: ContainerId) -> Result<Box<dyn Move>, io::Error> {
        todo!()
    }
}

impl ClusterInfoContainer {
    // GETTERS
}

impl Move for ClusterInfoContainer {
    fn free_container(self: Box<Self>) -> Result<Box<dyn Move>, io::Error> {
        todo!()
    }

    fn reallocate(self: Box<Self>, dest_mem: i32, dest_node: ContainerId) -> Result<Box<dyn Move>, io::Error> {
        todo!()
    }
}


// Declare unsafe operations