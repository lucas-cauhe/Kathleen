
use avl::map::AvlTreeMap;
use std::{ops::Deref, slice::Iter, vec};
use ordered_float::NotNan;
use super::{
    maxheap_wrapper::{BinaryHeapWrapper, HeapNode},
    primitive_types::{Embedding, Clusters, IVListEntry, DBResult, DistanceTable}
};

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

pub struct AvlWrapper<T>(AvlTreeMap<u32, Box<T>>);

impl<T: > AvlWrapper<T> {
    pub fn new() -> Self {
        Self(AvlTreeMap::new())
    }

    /// centroids should have the lowest ids in the tree
    pub fn get_centroids<'a>(&'a self) -> Vec<Centroid<'a>> {
        // perform CENTROIDS_PER_SUBSPACE_CLUSTER iterations in-order through self 
        self.0.iter().take(CENTROIDS_PER_SUBSPACE_CLUSTER).map(|(key, dtype)| dtype.into_centroid()).collect::<Centroid<'a>>()
    }

    // get all the elements in-order
    pub fn get_all<'a>(&'a self) -> Iter<'a, Box<T>> {
        unimplemented!()
    }
}

pub struct InvertedIndex<T>([AvlWrapper<T>; CENTROIDS_PER_SUBSPACE_CLUSTER]);

impl<T> InvertedIndex<T> {
    pub fn empty() -> Self {
        Self([AvlWrapper::new(); CQ_K_CENTROIDS])
    }

    /// loads cluster <clust_no> into self and returns a reference to it
    pub fn load_cluster(&mut self, clust_no: Clusters, db: &DatabaseWrapper) -> DBResult<&AvlWrapper<T>> {
        // load cluster <clust_no>
        // start iterator over vectors in cluster <clust_no> (filter those that contain <clust_no> in cluster field)
        let loaded_ds: [u8] = db.get_cf_bla_bla();
        self.0[clust_no as usize] = loaded_ds.deserialize_from_bytes();
    }

    /// pushes <entry> to cluster <clust_no>, eventually it pushes it to the db
    pub fn push_to_cluster(&mut self, clust_no: Clusters, db: &DatabaseWrapper, entry: IVListEntry) -> DBResult<()> {
        unimplemented!()
    }

    /// reloads itself to apply latest changes in the coarse quantizer
    /// such as cluster redistribution or embeddings switching clusters
    /// all data InMemory must be flushed and reorganized before calling this function
    pub fn reload() -> Result<(), String> {
        unimplemented!()
    }

    /// Table containing distance to each list entry segment for every centroid in Codebook from every query vector segment
    /// K x N table
    /// take from distance that is the lowest the formed codes what will give
    pub fn compute_distance_table(&self, query_vector: &Embedding, nearest_centroid: Clusters) -> DistanceTable {
        // get embeddings from rdb for nearest_centroid enty
        let embs: &AvlWrapper<IVListEntry> = self.load_cluster(nearest_centroid, db).expect("Error loading embeddings in cluster");
        // compute distances
        let distance_table = vec![];
        let centroids = embs.get_centroids();

        for centroid in centroids {

            let c_segments = centroid.0.1.into_segments();
            let qv_segments = query_vector.into_segments();

            let distances = c_segments
                .zip(qv_segments)
                .map(|(c_j, qv)| /* search for pre-built euclidean function */ euclidean(c_j, qv))
                .collect::<[f32; EMBEDDING_M_SEGMENTS]>();

            distance_table.push(distances);
        }
        distance_table
    }

    /// retrieves the nearest neighbor for the requested query vector
    pub fn get_nearest_centroid(&self, query_vector: &Embedding) -> Centroid {
        unimplemented!()
    }

    fn compute_residual(&self, v1: &Embedding, v2: &Embedding) -> Embedding {
        unimplemented!()
    }

}

impl<T> Deref for InvertedIndex<T> {
    type Target = [AvlWrapper<T>; CENTROIDS_PER_SUBSPACE_CLUSTER];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


// build coarse quantizer as struct interchangeable for IVlist
// build coarse quantizer from IVList and viceversa
// only difference is vector representation
// switching between pqcodes and raw vectors will be runtime cost
// each struct has its own thing


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




pub fn k_means(ividx: &mut InvertedIndex<IVListEntry>, embs: &[Embedding]) {
    // search for a built k_means
} 


pub fn search(ividx: InvertedIndex<IVListEntry>, query_vectors: &[Embedding] ) -> Result<(Vec<Vec<HeapNode>>, InvertedIndex<IVListEntry>), String> {
    
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

    let distance_results = Vec::new();
    for (ind, resid) in residuals.iter().enumerate() {
        let dt = ividx.compute_distance_table(resid, cq_nearest_centroids[ind].0.0);
        let max_heap: BinaryHeapWrapper<HeapNode<'_>, {RETRIEVE_KNN}> = BinaryHeapWrapper::new();
        let embs = ividx.load_cluster(cq_nearest_centroids[ind].0.0, db).expect("Error loading cluster");
        embs.get_all()
            .for_each(|entry| {
                let emb_dist = entry.get_code().iter()
                    .enumerate()
                    .map(|(subq, code)| dt[subq][*code as usize] )
                    .sum::<f32>();
                if let Ok(distance) = NotNan::new(emb_dist) {
                    max_heap.push(HeapNode {
                        distance,
                        code: entry.get_code()
                    }).expect("Error while pushing distance to maxheap");
                }
            });
        distance_results.push(max_heap.sorted());
    }
    Ok((distance_results, ividx))
}



// DATABASE

// represent database wrapper to make common calls (put, write...)
pub struct DatabaseWrapper{}

impl DatabaseWrapper {
    pub fn add(embs: &[IVListEntry]) -> DBResult<()> {
        // add embs to the database
        unimplemented!()
    }
}
