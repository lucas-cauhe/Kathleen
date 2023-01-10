
use lmdb::{self, Error, Environment};
use std::env;

use crate::engine::{indexing::mem::mem::Move, utils::pq::train::Codebook};

use super::{inverted_list::InvertedList, DBInterface};
const GENERIC_PATH: String = env::var("DB_STORAGE").unwrap();
pub struct Context {
    inverted_list: InvertedList,
    codebook: Codebook,
    //coarse_quantizer: 
}

// Function ran during initialization

pub fn load_context() -> Result<Context, Error> {
    
    // use default env to open its dbs and load each Context field

    let env: Environment;
    unsafe {
        let builder = lmdb::EnvBuilder::new().unwrap();
        builder.set_maxdbs(2); // may be 3
        env = builder.open(GENERIC_PATH.push_str("default.db"),
        lmdb::open::Flags::empty(),
        0o600)?;
    }

    Ok(Context {
        inverted_list: load_db_instance::<InvertedList>(&env, "inverted_list")?,
        codebook: load_db_instance::<Codebook>(&env, "codebook")?
    })
}


pub fn load_db_instance<DBInstance>(env: &Environment, db_name: &str) -> Result<DBInstance, Error> 
where
    DBInstance: DBInterface
{
    let db = lmdb::Database::open(&env, Some(db_name), &lmdb::DatabaseOptions::new(lmdb::db::CREATE))?;
    let instance_init: DBInstance;
    {
        let txn = lmdb::ReadTransaction::new(env)?;
        let access = txn.access();

        instance_init = access.get(&db, "default")?;
    }
    Ok(instance_init)
}


// MIGRATIONS??
/* 
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
*/