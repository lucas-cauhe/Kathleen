use crate::engine::utils::types::{VecId, ClusterId};

// This structure will hold the env where anonymous db entries are clusterIds 
// and every value is an array of entryIds that refer to a primary key in the named db
// where a vectorId and a PQCode / PQCodeId are stored


pub struct InvertedList {

    // env: 
}

pub struct InvertedListInnerValue {
    vec_id: VecId,
    pq_code: Vec<ClusterId> // ClusterId must be a valid type for an entry in the Codebook
}

pub struct InvertedListValue{
    entries_array: Vec<InvertedListInnerValue> // each position in the array indicates the entry in the codebook table where the centroid vector is located
}

impl InvertedList {


    pub fn add_clusters(&self, ids: &[ClusterId]) -> Result<(), Error> {

        // check if id is contained in the Codebook and isn't duplicate

        // this db can be anonymous since it will be maximum size of k (k-means) and will also
        // contain the name for the named dbs
        todo!()

    }


    pub fn add_vector(&self, vec_id: VecId, cluster_id: ClusterId) -> Result<(), Error> {
        // check the vector exists in PQCodes and take its PQCode / PQCodeId
        // let pq_code: Vec<ClusterId> = todo!();
        
        
        // let db = lmdb::Database::open(
        //     &self.env, None, &lmdb::DatabaseOptions::defaults())?;

        // // get the cluster

        // let mut cluster_entry: &InvertedListValue;

        // {
        //     let txn = lmdb::ReadTransaction::new(&self.env)?;
        //     let mut access = txn.access();

        //     cluster_entry = access.get(&db, &cluster_id)?;
        //     cluster_entry.entries_array.push(InvertedListInnerValue {
        //         vec_id,
        //         pq_code
        //     });

        // }

        // {
        //     let txn = lmdb::WriteTransaction::new(&self.env)?;

        //     {
        //         let mut access = txn.access();
        //         access.put(&db, &cluster_id, &cluster_entry, lmdb::put::Flags::empty())?;
        //     }

        //     txn.commit()?;
        // }

        // Ok(())

    }

    pub fn batch_add_vector(&self, vec_ids: &[VecId], cluster_id: ClusterId) -> Result<(), Error> {
        todo!()
    }

    pub fn delete_vector(&self, vec_id: VecId, cluster_id: ClusterId) -> Result<VecId, Error> {
        todo!()
    }

    pub fn batch_delete_vector(&self, vec_ids: &[VecId], cluster_id: ClusterId) -> Result<Vec<VecId>, Error> {
        todo!()
    }

    pub fn transfer_vector(&self, vec_id: VecId, source_cluster_id: ClusterId, target_cluster_id: ClusterId) -> Result<(), Error> {
        todo!()
    }




}


// superior fns -> creation of InvertedList instance: generic function for initialization of any db implementing certain trait that handles
// the specifics of the creation for that db

// fn create_db<T>(db: T) where T: RegisterDB, -> T {}

// same as creation but for deletion (used in migrations)


