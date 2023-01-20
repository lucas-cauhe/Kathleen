use crate::{tokenizer::tokenize::Embedding, engine::{utils::types::ClusterId, indexing::db::vector_storage::EmbeddingContainer}};



type VecId = usize;
pub struct Codebook {
    size: usize,
    embeddings: Vec<EmbeddingContainer>
}

impl Codebook {
    pub fn get_embedding(&self, id: ClusterId) -> &EmbeddingContainer {
        self.embeddings.get(id).unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.size.clone()
    }
}

pub struct Config {}
