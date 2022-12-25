
pub mod stations;
pub mod clusters;

pub struct Graph {
    nodes: Vec<Box<dyn Node>> // could be clusters or stations
    // Node implements functions related to clusters usage: memory manage, similarity ratiing, etc...
    //adj:  ADJACENCY MATRIX
} 


// MUST IMPLEMENT A WAY TO SAVE TO MEMORY