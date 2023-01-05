



use lmdb_zero::{Environment, Error};


// This structure will hold the env where anonymous db entries are clusterIds 
// and every value is an array of entryIds that refer to a primary key in the named db
// where a vectorId and a PQCode / PQCodeId are stored

type VectorId = usize;
type ClusterId = usize;
type PQCodeId = usize;

pub struct InvertedList {

    env: Environment
}

pub struct InvertedListInnerValue {
    vec_id: VectorId,
    pq_code_id: PQCodeId
    //pq_code: Vec<usize>
}

pub struct InvertedListValue{
    entries_array: Vec<usize>
}

impl InvertedList {


    pub fn add_cluster(&self, id: ClusterId) -> Result<(), Error> {

        // check if id is contained in the Codebook and isn't duplicate

        // this db can be anonymous since it will be maximum size of k (k-means) and will also
        // contain the name for the named dbs
        let db = lmdb::Database::open(
            &self.env, None, &lmdb::DatabaseOptions::defaults())?;
        
        {
            let txn = lmdb::WriteTransaction::new(&self.env)?;

            {
                let mut access = txn.access();
                access.put(&db, &id, &InvertedListValue {entries_array: Vec::new()}, lmdb::put::Flags::empty())?;
            }

            txn.commit()?;
        }


        Ok(())

    }


    pub fn add_vector(&self, vec_id: VectorId, cluster_id: ClusterId) -> Result<(), Error> {
        // check the vector exists in PQCodes and take its PQCode / PQCodeId
        let pq_code_id: PQCodeId = todo!();
        
        
        let db = lmdb::Database::open(
            &self.env, None, &lmdb::DatabaseOptions::defaults())?;

        // get the cluster

        let mut cluster_entry: &InvertedListValue;

        {
            let txn = lmdb::ReadTransaction::new(&self.env)?;
            let mut access = txn.access();

            cluster_entry = access.get(&db, &cluster_id)?;
            cluster_entry.entries_array.push(InvertedListInnerValue {
                vec_id,
                pq_code_id
            });

        }

        {
            let txn = lmdb::WriteTransaction::new(&self.env)?;

            {
                let mut access = txn.access();
                access.put(&db, &cluster_id, &cluster_entry, lmdb::put::Flags::empty())?;
            }

            txn.commit()?;
        }

        Ok(())

    }

    pub fn batch_add_vector(&self, vec_ids: &[VectorId], cluster_id: ClusterId) -> Result<(), Error> {
        todo!()
    }

    pub fn delete_vector(&self, vec_id: VectorId, cluster_id: ClusterId) -> Result<VectorId, Error> {
        todo!()
    }

    pub fn batch_delete_vector(&self, vec_ids: &[VectorId], cluster_id: ClusterId) -> Result<Vec<VectorId>, Error> {
        todo!()
    }

    pub fn transfer_vector(&self, vec_id: VectorId, source_cluster_id: ClusterId, target_cluster_id: ClusterId) -> Result<(), Error> {
        todo!()
    }




}


// superior fns -> creation of InvertedList instance: generic function for initialization of any db implementing certain trait that handles
// the specifics of the creation for that db

// fn create_db<T>(db: T) where T: RegisterDB, -> T {}

// same as creation but for deletion (used in migrations)


