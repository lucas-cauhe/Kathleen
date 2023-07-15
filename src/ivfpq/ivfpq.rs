
use avl::map::AvlTreeMap;
use ndarray::{Array2, Array1};
use serde::{Serialize, Deserialize};
use std::{ops::{Deref, DerefMut}, slice::Iter};
use ordered_float::NotNan;

use super::{
    maxheap_wrapper::{BinaryHeapWrapper, HeapNode},
    primitive_types::{Embedding, Clusters, IVListEntry, DistanceTable}
};
use linfa_clustering;
use linfa::{self, prelude::Predict};
use linfa_nn::distance::{L2Dist, Distance};
// k's between subspaces k-means and coarse quantizer may differ, take it into account

pub const EMBEDDING_DIM: usize = 12;
pub const EMBEDDING_M_SEGMENTS: usize = 4;
/// EMBEDDING_DIM % EMBEDDING_M_SEGMENTS == 0
pub const SEGMENT_DIM: usize = EMBEDDING_DIM / EMBEDDING_M_SEGMENTS;
pub const CENTROIDS_PER_SUBSPACE_CLUSTER: usize = 8;
pub const K_MAX_CENTROIDS: usize = EMBEDDING_M_SEGMENTS * CENTROIDS_PER_SUBSPACE_CLUSTER;
pub const RETRIEVE_KNN: usize = 10;
pub const CQ_K_CENTROIDS: usize = 8;
pub const CODE_SIZE: usize = 1;

/* 
impl IntoCentroid for IVListEntry {
    fn into_centroid<'a>(&'a self) -> Centroid<'a> {
        Centroid((
            self.cluster,
            &self.pq_code.into()
        ))
    }
} */

/// holds tuple (cluster_no, embedding)
pub struct Centroid<'a> ((Clusters, &'a Embedding));

pub trait IntoCentroid {
    fn into_centroid<'a>(&'a self) -> Centroid<'a>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvlWrapper(AvlTreeMap<u32, Box<IVListEntry>>);

impl AvlWrapper {
    pub fn new() -> Self {
        Self(AvlTreeMap::new())
    }

    pub fn from(src: AvlTreeMap<u32, Box<IVListEntry>>) -> Self {
        Self(src)
    }

    /// centroids should have the lowest ids in the tree
    pub fn get_centroids<'a>(&'a self) -> Vec<Centroid<'a>> {
        // perform CENTROIDS_PER_SUBSPACE_CLUSTER iterations in-order through self 
        //self.0.iter().take(CENTROIDS_PER_SUBSPACE_CLUSTER).map(|(key, dtype)| dtype.into_centroid()).collect::<Centroid<'a>>()
        unimplemented!()
    }

    // get all the elements in-order
    pub fn get_all<'a>(&'a self) -> Iter<'a, Box<IVListEntry>> {
        unimplemented!()
    }

    pub fn add_embedding(&mut self, emb: &Embedding, cluster: Clusters, vec_id: u32, dt: DistanceTable) {
        let code = emb.encode(dt);
        let entry = IVListEntry::new(
            code,
            cluster
        );
        self.0.insert(vec_id, Box::new(entry));
    }
}

impl Deref for AvlWrapper {
    type Target = AvlTreeMap<u32, Box<IVListEntry>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AvlWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct InvertedIndex(Vec<AvlWrapper>);

impl InvertedIndex {
    pub fn empty() -> Self {
        Self(Vec::with_capacity(CQ_K_CENTROIDS))
    }

    pub fn push(&mut self, value: AvlWrapper) {
        self.0.push(value)
    }
    
    pub fn get_cluster(&self, clust_no: Clusters) -> &AvlWrapper {
        &self.0[clust_no as usize]
    }

    /// Table containing distance to each list entry segment for every centroid in Codebook from every query vector segment
    /// K x N table
    /// take from distance that is the lowest the formed codes what will give
    pub fn compute_distance_table(&self, query_vector: &Embedding, nearest_centroid: Clusters) -> DistanceTable {
        // get embeddings from rdb for nearest_centroid enty
        let embs= self.get_cluster(nearest_centroid);
        // compute distances
        let mut distance_table = vec![];
        let centroids = embs.get_centroids();

        for centroid in centroids {

            let c_segments = centroid.0.1.into_segments();
            let qv_segments = query_vector.into_segments();

            let distances = c_segments
                .zip(qv_segments)
                .map(|(c_j, qv)| L2Dist::distance(&L2Dist, Array1::from(c_j.to_vec()).view(), Array1::from(qv.to_vec()).view()));
            let distances = {
                let mut tmp = [0.0; EMBEDDING_M_SEGMENTS];
                distances.enumerate().for_each(|(ind, d)| tmp[ind] = d);
                tmp
            };
            distance_table.push(distances);
        }
        let distance_table = {
            let mut tmp = [[0.0; EMBEDDING_M_SEGMENTS]; CENTROIDS_PER_SUBSPACE_CLUSTER];
            distance_table.into_iter().enumerate().for_each(|(ind, dists)| tmp[ind] = dists);
            tmp
        };
        distance_table
    }

    /// retrieves the nearest neighbor for the requested query vector
    pub fn get_nearest_centroid(&self, query_vector: &Embedding) -> Centroid {
        unimplemented!()
    }

    fn compute_residual(&self, v1: &Embedding, v2: &Embedding) -> Embedding {
        unimplemented!()
    }

    pub fn add_embedding_to_cluster(&mut self, cluster: Clusters, emb: &Embedding, dt: DistanceTable) {
        let avl: &mut AvlWrapper = self.0.get_mut(cluster as usize).unwrap();
        avl.add_embedding(emb, cluster, next_id(), dt)
    }

}

impl Deref for InvertedIndex {
    type Target = Vec<AvlWrapper>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// For each subspace you have to train that k-means, where D (dimensionalty of raw vector) has to be divisible
// by m (number of subspaces (segments) inside that raw vector)
// next decide how many possible centroids k can there be (k % m == 0) (in every subspace cluster there'll be k/m centroids)
// notation of pq_codes is as follows => a uint representing the centroid ordinal number putting together every subspace
// now embeddings will be like: [uint; D/m]
// let D = 12, m = 4, k = 32 -> pq_codes: [uint; 3] where the uint will be in range 0..32
// where uint 10 is the centroid number 1 in subspace 1, 23 is the centroid number 23%8 in subspace 23/8 (starting from 0)

// the concatenation of the centroids from each subquantizer with same index, yields the codebook

// so the distance table holds the distance to every centroid for every segment
// next you go over all the pq_codes in the coarse quantizer's cluster to which the query vector associated centoid is closest
// there you can get the distance to every encoded vector and perform KNN





pub fn k_means(ividx: &mut InvertedIndex, embs: &[Embedding]) {
    use linfa_clustering::KMeans;
    use linfa::{traits::Fit, DatasetBase};
    use rand_xoshiro::Xoshiro256Plus;
    use rand_xoshiro::rand_core::SeedableRng;
    let seed = 42;
    let rng = Xoshiro256Plus::seed_from_u64(seed);
    let mut data = Array2::zeros((embs.len(), SEGMENT_DIM*EMBEDDING_M_SEGMENTS));
    for ind in 0..embs.len() {
        let emb = embs[ind].to_vec();
        for each in 0..SEGMENT_DIM*EMBEDDING_M_SEGMENTS {
            data[[ind, each]] = emb[each];
        }
    }
    let obs = DatasetBase::from(data);

    let model: KMeans<_, _> = KMeans::params_with_rng(CQ_K_CENTROIDS, rng)
        .fit(&obs)
        .expect("KMeans fitted");

    // save it in the ividx
    // for each embedding
    for emb in embs {
        // predict the cluster it belongs to
        let new_obs = DatasetBase::from(Array1::from(emb.to_vec()));
        let pred_cluster: Clusters = model.predict(&new_obs) as Clusters;
        let dt: DistanceTable = ividx.compute_distance_table(emb, pred_cluster.clone());
        // add it to the predicted ividx entry
        ividx.add_embedding_to_cluster(pred_cluster, emb, dt)
    }
        
        
    
    // save the model somehow (static or return it)

} 

fn next_id() -> u32 {
    static mut ID: u32 = 1;
    unsafe {
        ID += 1;
        ID
    }
    
}

pub fn search<'a>(ividx: &'a InvertedIndex, query_vectors: &[Embedding] ) -> Result<Vec<Vec<HeapNode<'a>>>, String> {
    
    // this are centroids from the original coarse quantizer trained with raw vectors
    // this is used just to know which cluster does each query_vector belongs to
    let cq_nearest_centroids = query_vectors
        .iter()
        .map(|emb| /* this should be a call to the coarse quantizer */ ividx.get_nearest_centroid(emb))
        .collect::<Vec<Centroid>>();

    
    let residuals = cq_nearest_centroids
        .iter()
        .zip(query_vectors)
        .map(|(cent, qv)| ividx.compute_residual(cent.0.1, qv))
        .collect::<Vec<Embedding>>();

    let mut distance_results = Vec::new();
    for (ind, resid) in residuals.iter().enumerate() {
        let dt = ividx.compute_distance_table(resid, cq_nearest_centroids[ind].0.0);
        let mut max_heap: BinaryHeapWrapper<HeapNode<'_>, {RETRIEVE_KNN}> = BinaryHeapWrapper::new();
        let embs = ividx.get_cluster(cq_nearest_centroids[ind].0.0);
        embs.get_all()
            .for_each(|entry| {
                let emb_dist = entry.get_code().iter()
                    .enumerate()
                    .map(|(subq, code)| dt[subq][*code as usize] )
                    .sum::<f64>();
                if let Ok(distance) = NotNan::new(emb_dist) {
                    max_heap
                        .push(HeapNode::new(distance, entry.get_code()))
                        .expect("Error while pushing distance to maxheap");
                }
            });
        distance_results.push(max_heap.sorted());
    }
    Ok(distance_results)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_searches() {
        // get raw embeddings

        // perform k_means

        // add them to the db

        // search
    }

    #[test]
    fn k_means_works() {

    }

    #[test]
    fn distance_table_gets_computed() {

    }
}