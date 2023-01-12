
use ndarray::{self,Array, ArrayBase, OwnedRepr, Dim};

// 1. partition query vector
//  2. find nearest partition centroid
//  3. compute residual
//  4. compute distance table with codebook
//  5. extract nearest neighbors for nearest partition


// if multiple queries are made at once -> gss is computed using all embeddings inside a cluster so use that similarity computed
// but give the nearest neighbors computed more relevance with a kind of mean: sum from 0 to k res = (nn-i + previously-computed-similarity) / i


use crate::{tokenizer::tokenize::Embedding, engine::utils::types::{KNN, ResultEmbedding}};

use super::{DBInterface, ctx::{Context, DFUtility}};


pub fn search_one(query_vector: &Embedding, ctx: &Context, k: usize) -> Result<KNN, String> {
    
    query_vector.partition(ctx.k);

    todo!()

}

pub fn search_many<T>(query_vectors: &[Embedding], ctx: &Context, k: usize) -> Result<KNN, String> 
{
    
    let similarity_table: ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>> = Array::zeros((query_vectors.len(), ctx.get_total_clusters()));
    let search_results: Vec<Option<KNN>> = vec![None; query_vectors.len()];
    for (index, query) in query_vectors.iter().enumerate() {
        search_results[index] = Some(search_one(query, ctx, k)?);
        compute_similarity(&search_results[index], ctx, &similarity_table.row(index));
    }

    let columns_sum = similarity_table.sum_axis(0);
    let result_cluster: usize =  columns_sum.iter().reduce(|min, next| ctx.distance_function.nearest(min, next) ) as usize;

    // if any of the query_vectors were in result_cluster, return its search_result
    // else perform search_one with the input query the result_cluster's centroid
    let clusters_match = search_results.iter().filter(|r| r.unwrap().cluster == result_cluster ).collect::<Vec<&Option<KNN>>>();

    // return the embeddings that were most similar to their respective queries
    if let Some(neighbors) = most_similar_to_respectives(&clusters_match, &ctx.distance_function) {
        Ok(neighbors)
    }
    let result_cluster_centroid = ctx.get_centroid(result_cluster);
    Ok(search_one(result_cluster_centroid, ctx, k))

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

// Embedding has to include a field where to specify its similarity to a given query vector -> Design ResultEmbedding
pub fn most_similar_to_respectives(matches: &[Option<KNN>], df: &dyn DFUtility) -> Option<KNN> {

    if matches.is_empty() {
        None
    }

    let neighbors = KNN {
        k: matches[0].unwrap().k,
        embeddings: Vec::new(),
        cluster: matches[0].unwrap().cluster    
    };

    let mut min_distances: Vec<Option<ResultEmbedding>> = vec![None; neighbors.k];
    let mut moving_indices = vec![0; matches.len()];

    for ind in 0..matches.len() {
        let compare_array = matches.iter().enumerate().map(|(i, m)| m.unwrap().embeddings[moving_indices[i]].distance_to_query).collect();
        let min_ind = compare_array.iter().reduce(|min, next| df.nearest(min, next)).unwrap();
        moving_indices[min_ind] += 1;
        min_distances[ind] = Some(compare_array[min_ind]);
    }

    neighbors.embeddings = min_distances.iter().map(|re| re.unwrap()).collect();
    



}


