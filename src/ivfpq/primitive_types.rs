use serde::{Deserialize, Serialize};
use core::slice::Iter;

use crate::ivfpq::ivfpq::{SEGMENT_DIM, EMBEDDING_M_SEGMENTS, CODE_SIZE, CQ_K_CENTROIDS, CENTROIDS_PER_SUBSPACE_CLUSTER};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Segment([f64; SEGMENT_DIM]);

impl Segment {
    pub fn new(src: [f64; SEGMENT_DIM]) -> Self {
        Self(src)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Embedding([Segment; EMBEDDING_M_SEGMENTS]);

impl Embedding {
    pub fn into_segments<'a>(&'a self) -> Iter<'a, Segment> {
        self.0.iter()
    }
    pub fn new(src: [Segment; EMBEDDING_M_SEGMENTS]) -> Self {
        Self(src)
    }
}

impl Default for Embedding {
    fn default() -> Self {
        Self([Default::default(); EMBEDDING_M_SEGMENTS])
    }
}

pub(super) type PqCode = [u32; CODE_SIZE];
pub(super) type Clusters = u8;
pub(super) type DBResult<T> = Result<T, rocksdb::Error>; // may change this error type
pub(super) type DistanceTable = [[f32; EMBEDDING_M_SEGMENTS]; CENTROIDS_PER_SUBSPACE_CLUSTER];
pub(super) type Codebook = [Embedding; CQ_K_CENTROIDS];

impl<> Into<Embedding> for PqCode {
    fn into(self) -> Embedding {
        unimplemented!()
    }
}

fn code_from_src(source: &str) -> PqCode {
    let mut no_spaces = source.replace(' ', "");
    no_spaces.remove(0);
    no_spaces.remove(no_spaces.len()-1);
    let mut code: PqCode = [0; CODE_SIZE];
    no_spaces.split(',').enumerate().for_each(|(ind, c)| code[ind] = c.parse::<u32>().unwrap() );
    code
}

// this is what gets stored by rocksdb (the value, index is the key also in rdb)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct IVListEntry {
    // vector_id: u32, acts as key for AVL
    pq_code: PqCode,
    cluster: Clusters
}

impl IVListEntry {
    pub fn new(pq_code: PqCode, cluster: Clusters) -> Self {
        Self {
            pq_code,
            cluster
        }
    }

    pub fn from_str(source: &str) -> Self {
        let splitted = source.split(';').collect::<Vec<&str>>();
        let code = splitted[0];
        let cluster = splitted[1];
        Self { 
            pq_code: code_from_src(code), 
            cluster: cluster.parse::<Clusters>().unwrap()
        }
    }

    pub fn get_code(&self) -> &PqCode {
        &self.pq_code
    }
}

impl ToString for IVListEntry {
    fn to_string(&self) -> String {
        format!("{:?};{}", self.pq_code,self.cluster)
    }
}