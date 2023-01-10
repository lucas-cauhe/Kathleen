use std::any::TypeId;

use ndarray::{self, Array2, Array, ArrayBase, OwnedRepr, Dim};

// 1. partition query vector
//  2. find nearest partition centroid
//  3. compute residual
//  4. compute distance table with codebook
//  5. extract nearest neighbors for nearest partition


// if multiple queries are made at once -> gss is computed using all embeddings inside a cluster so use that similarity computed
// but give the nearest neighbors computed more relevance with a kind of mean: sum from 0 to k res = (nn-i + previously-computed-similarity) / i


use crate::{tokenizer::tokenize::Embedding, engine::utils::types::KNN};

use super::{DBInterface, ctx::Context};


pub fn search_one(query_vector: &Embedding, ctx: &Context, k: usize) -> Result<KNN, String> {
    
    query_vector.partition(ctx.k);

    todo!()

}

pub fn search_many<T>(query_vectors: &[Embedding], ctx: &Context, k: usize, df: &DistanceFunction) -> Result<KNN, String> 
{
    
    let similarity_table: ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>> = Array::zeros((query_vectors.len(), ctx.get_total_clusters()));

    for (index, query) in query_vectors.iter().enumerate() {
        let search_result: KNN = search_one(query, ctx, k)?;
        compute_similarity(&search_result, ctx, &similarity_table.row(index));
    }

    let columns_sum = similarity_table.sum_axis(0);
    let result_cluster: usize =  df.nearest_cluster(columns_sum);

    // if any of the query_vectors were in result_cluster, return its search_result
    // else perform search_one with the input query the result_cluster's centroid

    Ok(())

}

fn compute_similarity<T>(neighbors: &KNN, ctx: &Context, similarity_table: &[T]) -> () {
    todo!()
}

pub enum DistanceFunctionEnum {
    Cosine,
    Euclidean,
}

pub struct DistanceFunction{
    function: DistanceFunctionEnum
}

impl DistanceFunction {
    pub fn nearest_cluster<T>(&self, columns_sum: &[T]) -> usize {
        match self.function {
            DistanceFunctionEnum::Cosine => {

            },
            DistanceFunctionEnum::Euclidean => {

            }
        }
    }
}




