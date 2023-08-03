use avl::map::AvlTreeMap;
use linfa::{traits::Fit, DatasetBase};
use ndarray::{Array2, Array1};
use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};
use ordered_float::NotNan;
use std::marker::PhantomData;

use super::{
    maxheap_wrapper::{BinaryHeapWrapper, HeapNode},
    primitive_types::{Embedding, Clusters, IVListEntry, DistanceTable, Codebook}
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
pub const EMBEDDINGS_PER_CLUSTER: usize = 3;

/// holds tuple (cluster_no, embedding)
pub struct Centroid<'a> ((Clusters, &'a Embedding));

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvlWrapper(AvlTreeMap<u32, Box<IVListEntry>>);

impl AvlWrapper {
    pub fn new() -> Self {
        Self(AvlTreeMap::new())
    }

    pub fn from(src: AvlTreeMap<u32, Box<IVListEntry>>) -> Self {
        Self(src)
    }

    // get all the elements in-order
    pub fn get_all(&self) -> Vec<&Box<IVListEntry>> {
        self.0.iter()
            .map(|(k, v)| v)
            .collect::<Vec<&Box<IVListEntry>>>() 
    }

    pub fn add_embedding(&mut self, emb: &Embedding, cluster: Clusters, vec_id: u32, cb: &Codebook) {
        let code = emb.encode(cb);
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
        let mut ividx = Vec::with_capacity(CQ_K_CENTROIDS);
        for _ in 0..CQ_K_CENTROIDS {
            let wrapper = AvlWrapper::new();
            ividx.push(wrapper);
        }
        Self(ividx)
    }

    pub fn push(&mut self, value: AvlWrapper) {
        self.0.push(value)
    }
    
    pub fn get_cluster(&self, clust_no: Clusters) ->  &AvlWrapper {
          &self.0[clust_no as usize] 
    }
    pub fn get_cluster_mut(&mut self, clust_no: Clusters) ->  &mut AvlWrapper {
          &mut self.0[clust_no as usize] 
    }

    /// Table containing distance to each list entry segment for every centroid in Codebook from every query vector segment
    /// K x N table
    /// take from distance that is the lowest the formed codes what will give
    pub fn compute_distance_table(query_vector: &Embedding, codebook: &Codebook) -> DistanceTable {
        // compute distances
        let mut distance_table = vec![];

        for centroid in codebook {

            let c_segments = centroid.into_segments();
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
    pub fn get_nearest_centroid<'a>(&self, model: &Model, query_vector: &Embedding, codebook: &'a Codebook) -> Result<Centroid<'a>, String> {
       let predicted_cluster = model.predict(query_vector)?;
       Ok(Centroid((predicted_cluster.clone(), &codebook[predicted_cluster as usize])))
    }

    fn compute_residual(&self, v1: &Embedding, v2: &Embedding) -> Embedding {
       let ndarray_emb1 =  Array1::from(v1.to_vec());
       let ndarray_emb2 =  Array1::from(v2.to_vec());
       Embedding::from_base(
           ndarray_emb1 - ndarray_emb2
       )
    }

    pub fn add_embedding_to_cluster(&mut self, cluster: Clusters, emb: &Embedding, cb: &Codebook) {
        let avl: &mut AvlWrapper = self.0.get_mut(cluster as usize).unwrap();
        avl.add_embedding(emb, cluster, next_id(), cb)
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

use linfa_clustering::KMeans;

pub struct Model {
  pub model: Option<KMeans<f64, L2Dist>>
}

impl Model {
    pub fn new() -> Self {Self{model: None}}
    pub fn predict(&self, qv: &Embedding) -> Result<Clusters, String> {
       match &self.model {
           Some(m) => {
               let obs = DatasetBase::from(Array1::from(qv.to_vec()));
               Ok(m.predict(&obs) as Clusters)
           },
           None => Err("model not trained".to_string())
       }
    }
    pub fn k_means(&mut self, ividx: &mut InvertedIndex, embs: &[Embedding]) -> Codebook {
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
        
        let model = 
            match &self.model {
                Some(m) => m,
                None => { self.model = Some(KMeans::params_with_rng(CQ_K_CENTROIDS, rng)
                    .fit(&obs)
                    .expect("KMeans fitted"));
                self.model.as_ref().unwrap()}
        };

        // create codebook
        let codebook = model.centroids();
        let codebook: Codebook = {
            let mut tmp: Codebook = [Embedding::default(); CQ_K_CENTROIDS];
            codebook.rows().into_iter().enumerate().for_each(|(ind, emb)| tmp[ind] = Embedding::from_base(emb.to_owned()));
            tmp
        };
        // save it in the ividx
        // for each embedding
        for emb in embs {
            // predict the cluster it belongs to
            let new_obs = DatasetBase::from(Array1::from(emb.to_vec()));
            let pred_cluster: Clusters = model.predict(&new_obs) as Clusters;
            // add it to the predicted ividx entry
            ividx.add_embedding_to_cluster(pred_cluster, emb, &codebook)
        }
        // save the model somehow (static or return it)
        codebook
    }

} 

fn next_id() -> u32 {
    static mut ID: u32 = 1;
    unsafe {
        ID += 1;
        ID
    }
    
}

pub fn search<'a>(ividx: &'a InvertedIndex, query_vectors: &[Embedding], codebook: &Codebook, model: &Model) -> Result<Vec<Vec<HeapNode<'a>>>, String> {
    
    // this are centroids from the original coarse quantizer trained with raw vectors
    // this is used just to know which cluster does each query_vector belongs to
    let cq_nearest_centroids = query_vectors
        .iter()
        .map(|emb| /* this should be a call to the coarse quantizer */ ividx.get_nearest_centroid(model, emb, codebook).expect("Error getting nearest centroid: "))
        .collect::<Vec<Centroid>>();

    
    let residuals = cq_nearest_centroids
        .iter()
        .zip(query_vectors)
        .map(|(cent, qv)| ividx.compute_residual(cent.0.1, qv))
        .collect::<Vec<Embedding>>();

    let mut distance_results = Vec::new();
    for (ind, resid) in residuals.iter().enumerate() {
        let dt = InvertedIndex::compute_distance_table(resid, codebook);
        let mut max_heap: BinaryHeapWrapper<HeapNode<'_>, {RETRIEVE_KNN}> = BinaryHeapWrapper::new();
        let embs = ividx.get_cluster(cq_nearest_centroids[ind].0.0);
        embs.get_all().iter()
            .for_each(|entry| {
                let emb_dist = entry.get_code().iter()
                    .enumerate()
                    .map(|(subq, code)| dt[*code as usize][subq] )
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
    use linfa_nn::distance::{L2Dist, Distance};
    use ndarray::Array1;

    use crate::ivfpq::{
        primitive_types::{DistanceTable, Codebook, Embedding, Segment, IVListEntry}, 
        ivfpq::{CQ_K_CENTROIDS, EMBEDDING_M_SEGMENTS, AvlWrapper, SEGMENT_DIM},
        db_api::DatabaseWrapper};

    use super::*;
    use std::path::Path;

    #[test]
    fn it_searches() {
       // in real world scenario, the way to create an IVF will be by calling the load_ivf method from the db_api
       let database = DatabaseWrapper::open(Path::new("./dbre")).expect("Opening failed: ");
       let mut ividx = database.load_ivf().unwrap();
       let mut model = Model::new();
       let mut embs_list = vec![];
       let test_embs_str = std::fs::read_to_string("tests/k_means_test_embs").unwrap();
        let mut test_embs = test_embs_str.split('\n').into_iter();
        for _ in 0..EMBEDDINGS_PER_CLUSTER*CENTROIDS_PER_SUBSPACE_CLUSTER {
            embs_list.push(Embedding::read_from_str(test_embs.next().unwrap()));
        }
       let codebook = model.k_means(&mut ividx, &embs_list);
       let query_vectors = std::fs::read_to_string("./tests/search_query_vectors").unwrap();
       let mut groomed_qv = query_vectors.split('\n').into_iter();
       let to_search_embs = [
           Embedding::read_from_str(groomed_qv.next().unwrap()),
           Embedding::read_from_str(groomed_qv.next().unwrap())
       ];

       let results = search(&ividx, &to_search_embs, &codebook, &model);
       println!("{:?}", results);

    }
    
    // weirdo but works
    #[test]
    fn k_means_works() {
       let mut ividx = InvertedIndex::empty();
       let mut model = Model::new();
       let mut embs_list = vec![];
       let test_embs_str = std::fs::read_to_string("tests/k_means_test_embs").unwrap();
        let mut test_embs = test_embs_str.split('\n').into_iter();
        for _ in 0..EMBEDDINGS_PER_CLUSTER*CENTROIDS_PER_SUBSPACE_CLUSTER {
            embs_list.push(Embedding::read_from_str(test_embs.next().unwrap()));
        }
       let codebook = model.k_means(&mut ividx, &embs_list);
       // check that all codebook embs are found in their respective ividx entry
       for (cluster_no, centroid) in codebook.iter().enumerate() {
           //let found_centroid = ividx
           //    .get_cluster(cluster_no as u8)
           //    .iter()
           //    .filter(|emb| emb.1.get_code().clone() == centroid.encode(&codebook))
           //    .collect::<Vec<_>>();
           //
           //assert_eq!(found_centroid.len(), 1);
           println!("------Cluster {cluster_no}-----");
           println!("Centroid code: {:?}", centroid.encode(&codebook));
           println!("Ividx embs: {:?}", ividx.get_cluster(cluster_no as u8));
       }
       // list all the embeddings and check there is no one left from the embs_list
    }

    mod inverted_index {

        use crate::ivfpq::ivfpq::next_id;
        use crate::ivfpq::primitive_types::PqCode;
        use super::*;

        #[test]
        fn adding_embeddings_to_cluster() {
            let embs_per_cluster = 3;

            // declare pre-trained codebook
            // 2 of which are taken from insertion embeddings
            let mut cb: Codebook = [Embedding::default(); CQ_K_CENTROIDS];
            let codebook_embs_file = std::fs::read_to_string("tests/codebook_test_embeddings").unwrap();
            let mut codebook_embs_file = codebook_embs_file.split('\n').into_iter();
            for element in 0..CQ_K_CENTROIDS {
                cb[element] = Embedding::read_from_str(codebook_embs_file.next().unwrap());
            }
            let mut ividx = InvertedIndex::empty();
            let wrap1 = ividx.get_cluster_mut(1);
            let test_embs = std::fs::read_to_string("tests/test_embeddings").unwrap();
            let test_embs = test_embs.split('\n').into_iter();
            let embs_wrap1  = test_embs.take(embs_per_cluster).map(|emb| Embedding::read_from_str(emb));
            embs_wrap1.for_each(|emb| wrap1.add_embedding(&emb, 1, next_id(), &cb));
            // assert embeddings in both clusters match the specified in txt file
            let embeddings_clust_1 = ividx.get_cluster(1).get_all()
                .iter().map(|v| v.get_code().clone()).collect::<Vec<PqCode>>();
            assert_eq!(
                vec![[1_u8, 3_u8, 3_u8, 3_u8], [0_u8, 3_u8, 3_u8, 3_u8], [1_u8, 3_u8, 0_u8, 3_u8]], embeddings_clust_1
                );

        }

        #[test]
        fn distance_table_gets_computed() {
            // create codebook
            let mut codebook: Codebook = [Embedding::default(); CQ_K_CENTROIDS];
            let codebook_embs_file = std::fs::read_to_string("tests/codebook_test_embeddings").unwrap();
            let mut codebook_embs_file = codebook_embs_file.split('\n').into_iter();
            for element in 0..CQ_K_CENTROIDS {
                codebook[element] = Embedding::read_from_str(codebook_embs_file.next().unwrap());
            }
            // create query_vector (in real scenarios should be the residual)
            let query_vector: Embedding = Embedding::read_from_str(
                std::fs::read_to_string("./tests/query_vectors").unwrap().split('\n').into_iter().next().unwrap()
            );
            let dt: DistanceTable = InvertedIndex::compute_distance_table(&query_vector, &codebook);
            let get_distance = |c_j: Segment, qv: Segment| L2Dist::distance(&L2Dist, Array1::from(c_j.to_vec()).view(), Array1::from(qv.to_vec()).view()) ;
            let expected_dt: DistanceTable = [
                [
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([1.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([2.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([3.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ],
                [
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM])),
                    get_distance(Segment::new([4.0; SEGMENT_DIM]), Segment::new([1.0; SEGMENT_DIM]))
                ]
            ];
            assert_eq!(dt, expected_dt);
        }
    }
    
}
