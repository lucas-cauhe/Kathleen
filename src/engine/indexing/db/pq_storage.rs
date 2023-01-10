

use crate::engine::utils::pq::train::Codebook;

use lmdb::Error;
use lmdb_zero::Environment;


struct DBCodebook<'a> {
    env: Environment,
    codebook: &'a Codebook,
}

impl DBCodebook<'_> {

    // adds just one new centroid appended to the previous list
    pub fn add_centroid_to_db(&self, centroid_id: &usize) -> Result<(), Error> {
        todo!()
    }

    // adds all of the Codebook's centroids to the database, this means deleting all the previous ones and pushing these new ones
    pub fn swap_centroids_to_db(&self) -> Result<(), Error> {
        todo!()
    }


    // appends all the centroids in Codebook to the previous list avoiding duplicates
    pub fn append_centroids_to_db(&self) -> Result<(), Error> {
        todo!()
    }

}


