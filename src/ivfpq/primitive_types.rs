use serde::Deserialize;
use core::slice::Iter;

use super::ivfpq::{SEGMENT_DIM, EMBEDDING_M_SEGMENTS, CODE_SIZE, CQ_K_CENTROIDS, CENTROIDS_PER_SUBSPACE_CLUSTER};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Segment([f64; SEGMENT_DIM]);

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Embedding([Segment; EMBEDDING_M_SEGMENTS]);

impl Embedding {
    pub fn into_segments<'a>(&'a self) -> Iter<'a, Segment> {
        self.0.iter()
    }
}


pub(super) type PqCode = [u32; CODE_SIZE];
pub(super) type Clusters = u8;
pub(super) type DBResult<T> = Result<T, String>; // may change this error type
pub(super) type DistanceTable = [[f32; EMBEDDING_M_SEGMENTS]; CENTROIDS_PER_SUBSPACE_CLUSTER];
pub(super) type Codebook = [Embedding; CQ_K_CENTROIDS];

impl<> Into<Embedding> for PqCode {
    fn into(self) -> Embedding {
        unimplemented!()
    }
}

// this is what gets stored by rocksdb (the value, index is the key also in rdb)
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IVListEntry {
    // vector_id: u32, acts as key for AVL
    pq_code: PqCode,
    cluster: u8
}

impl IVListEntry {
    pub fn get_code(&self) -> &PqCode {
        &self.pq_code
    }
}