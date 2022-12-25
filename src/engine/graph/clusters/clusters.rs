

// CLUSTERS FORMATION IN MEMORY 


// EACH CLUSTER MUST DEFINE:
    // - BOUNDARIES
    // - EMBEDDINGS IN IT
    // - ID
    // - CONNECTIONS TO OTHER CLUSTERS (PATH TYPES)
    // - ADDITIONAL INFO TO COMPUTE SIMILARITY BETWEEN CLUSTERS


pub struct Cluster {
    pub id: u32,
    //boundaries: 
    pub embeddings: Vec<Embedding>,
    pub connections: Vec<Rc<dyn Path>>,
    // additionals
    
}


