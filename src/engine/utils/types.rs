use crate::tokenizer::tokenize::Embedding;



pub type VecId = usize;
pub type SegmentId = usize;
pub type ClusterId = usize;
pub type ContainerId = usize;
pub type PQCodeId = usize;

pub struct KNN {
    k: usize,
    embeddings: Vec<Embedding>
}