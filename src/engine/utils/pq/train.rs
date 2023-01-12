use crate::{tokenizer::tokenize::Embedding, engine::utils::types::ClusterId};



type VecId = usize;
pub struct Codebook {
    size: usize,
    embeddings: Vec<Embedding>
}

impl Codebook {
    pub fn get_embedding(&self, id: ClusterId) -> &Embedding {
        self.embeddings.get(id).unwrap()
    }
}

pub struct Config {}
