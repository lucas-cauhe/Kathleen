
use lmdb::{self, Error, Environment};
use std::env;

use crate::engine::indexing::mem::mem::Move;

pub static CTX: Vec<lmdb::Environment> = Vec::new();
const GENERIC_PATH: String = env::var("DB_STORAGE").unwrap();

// MIGRATIONS??

pub fn create_env() -> Result<&'static Environment, Error> {
    // create env

    let env = unsafe {
        let tmp_env = lmdb::EnvBuilder::new().unwrap();
        tmp_env.set_maxdbs( 2);
        tmp_env.
        open(
          GENERIC_PATH.as_str(), 
          lmdb::open::Flags::empty(), 
          0o600).unwrap()
    };

    // plug it into CTX
    CTX.push(env);

    // return reference to CTX's
    Ok(CTX.get(CTX.len()-1).unwrap())
}

pub fn read() -> () {
    todo!()
}


pub fn delete() -> () {
    todo!()
}

pub fn insert_named(containers: &[Box<dyn Move>], env: &Environment) -> Result<(), Error> {
    // 1 repo object links to 1 embedding
    let objs_db = lmdb::Database::open(env, Some(&"Objs".to_string()), &lmdb::DatabaseOptions::create_map()).unwrap();
    let embeddings_db = lmdb::Database::open(env, Some(&"Embeddings".to_string()), &lmdb::DatabaseOptions::create_map()).unwrap();

    {
        let txn = lmdb::WriteTransaction::new(&env).unwrap();

        {
            let mut access = txn.access();
            for container in containers {
                access.put(&objs_db, &container.get_key(), &container.get_object(), lmdb::put::Flags::empty()).unwrap();
                access.put(&objs_db, &container.get_key(), &container.get_embedding(), lmdb::put::Flags::empty()).unwrap();
            }
        }
        txn.commit().unwrap();
    }

    Ok(())
}