
use crate::{tokenizer::tokenize::Embedding, engine::indexing::db::search::DistanceFunction};



pub type VecId = usize;
pub type SegmentId = usize;
pub type ClusterId = usize;
pub type ContainerId = usize;
pub type PQCodeId = usize;

pub struct KNN {
    pub k: usize,
    pub embeddings: Vec<ResultEmbedding>,
    pub cluster: ClusterId
}

pub struct ResultEmbedding {
    pub embedding: Embedding,
    pub distance_to_query: DistanceFunction
}

