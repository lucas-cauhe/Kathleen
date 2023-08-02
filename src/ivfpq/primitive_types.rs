use ndarray::Array1;
use serde::{Deserialize, Serialize};
use core::{slice::Iter, f64};
use std::str::FromStr;

use crate::ivfpq::ivfpq::{SEGMENT_DIM, EMBEDDING_M_SEGMENTS, CQ_K_CENTROIDS, CENTROIDS_PER_SUBSPACE_CLUSTER, InvertedIndex};


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Segment([f64; SEGMENT_DIM]);

impl Segment {
    pub fn new(src: [f64; SEGMENT_DIM]) -> Self {
        Self(src)
    }

    pub fn to_vec(&self) -> Vec<f64> {
        Vec::from(self.0)
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

    pub fn from_base(src: Array1<f64>) -> Self {
        let mut emb = [Segment([0.0; SEGMENT_DIM]); EMBEDDING_M_SEGMENTS];
        let mut last = 0;
        for segment in 0..EMBEDDING_M_SEGMENTS {
            let mut seg = [0.0; SEGMENT_DIM];
            for element in last..last+SEGMENT_DIM {
                seg[element-last] = src[element];
            }
            last += SEGMENT_DIM;
            emb[segment] = Segment(seg);
        }
        Embedding(emb)
    }

    pub fn read_from_str(src: &str) -> Self {
        let mut string_src = src.to_string();
        string_src = string_src.replace('[', "");
        string_src = string_src.replace(']', "");
        let mut src_iter = string_src.split(',').into_iter();
        let mut embedding = [Segment::default(); EMBEDDING_M_SEGMENTS];
        for seg in 0..EMBEDDING_M_SEGMENTS {
            let mut segment = [0.0; SEGMENT_DIM];
            for f in 0..SEGMENT_DIM {
                let nxt = src_iter.next().unwrap().replace(' ', "");
                println!("{nxt}");
                segment[f] = f64::from_str(&nxt).unwrap();
            }
            embedding[seg] = Segment::new(segment);
        }
        Embedding::new(embedding)
    }

    pub fn to_vec(&self) -> Vec<f64> {
        let mut arr = Vec::new();
        for segment in self.0 {
            arr.append(&mut segment.0.to_vec())
        }
        arr
    }

    pub fn encode(&self, cb: &Codebook) -> PqCode {
        let dt = InvertedIndex::compute_distance_table(&self, cb);
        let mut mins_array: Vec<(Clusters, f64)> /* (clust_no, min_dist) */= vec![(0, std::f64::MAX); EMBEDDING_M_SEGMENTS];
        dt.iter()
            .enumerate()
            .for_each(|(clust_no, next_cluster)| next_cluster.iter().enumerate().for_each(|(seg_no, next_seg)| {
                if &mins_array[seg_no].1 > next_seg  {
                    mins_array[seg_no] = (clust_no as u8, next_seg.clone());
                }
            }));
        let mut code = [0; EMBEDDING_M_SEGMENTS];
        mins_array.into_iter().enumerate().for_each(|(ind, (clust, _))| code[ind] = clust);
        code
    }
}

impl Default for Embedding {
    fn default() -> Self {
        Self([Default::default(); EMBEDDING_M_SEGMENTS])
    }
}

pub(super) type PqCode = [Clusters; EMBEDDING_M_SEGMENTS];
pub(super) type Clusters = u8;
pub(super) type DBResult<T> = Result<T, rocksdb::Error>; // may change this error type
pub(super) type DistanceTable = [[f64; EMBEDDING_M_SEGMENTS]; CENTROIDS_PER_SUBSPACE_CLUSTER];
pub(super) type Codebook = [Embedding; CQ_K_CENTROIDS];

fn code_from_src(source: &str) -> PqCode {
    let mut no_spaces = source.replace(' ', "");
    no_spaces.remove(0);
    no_spaces.remove(no_spaces.len()-1);
    let mut code: PqCode = [0; EMBEDDING_M_SEGMENTS];
    no_spaces.split(',').enumerate().for_each(|(ind, c)| code[ind] = c.parse::<u32>().unwrap() as Clusters );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ivfpq::ivfpq::InvertedIndex;
   #[test]
   fn encoding_works() {
        let embs_per_cluster = 3;
        let mut cb: Codebook = [Embedding::default(); CQ_K_CENTROIDS];
        let codebook_embs_file = std::fs::read_to_string("tests/codebook_test_embeddings").unwrap();
        let mut codebook_embs_file = codebook_embs_file.split('\n').into_iter();
        for element in 0..CQ_K_CENTROIDS {
            cb[element] = Embedding::read_from_str(codebook_embs_file.next().unwrap());
        }
       let test_embs = std::fs::read_to_string("tests/test_embeddings").unwrap();
       let test_embs = test_embs.split('\n').into_iter();
       let embs_to_encode  = test_embs.take(embs_per_cluster).map(|emb| Embedding::read_from_str(emb));
       let encoded_embs = embs_to_encode
           .map(|emb| emb.encode(&cb).clone())
           .collect::<Vec<PqCode>>();
       
        assert_eq!(
            vec![[1_u8, 3_u8, 3_u8, 3_u8], [0_u8, 3_u8, 3_u8, 3_u8], [1_u8, 3_u8, 0_u8, 3_u8]], encoded_embs
            );

   }
}

