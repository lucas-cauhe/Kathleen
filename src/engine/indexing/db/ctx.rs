
use lmdb::{self, Error, Environment};
use std::{env, rc::Rc};

use crate::{engine::{indexing::mem::mem::Move, utils::{pq::train::Codebook, types::ClusterId}}, tokenizer::tokenize::Embedding};

use super::{inverted_list::InvertedList, DBInterface, vector_storage::EmbeddingHolder};
const GENERIC_PATH: String = env::var("DB_STORAGE").unwrap();
pub struct Context {

    // env: Environment,
    // the environment where context will hold its data will be the same in all of the fields that require 
    // db interaction
    inverted_list: InvertedList,
    codebook: Codebook,
    //coarse_quantizer:
    // if multiple searches are made, its unnecesary to keep loading embeddings while they're being used
    // multiple queries can access same cluster embeddings (consider this in a threaded scenario)

    // therefore makes no sense to keep the environment's database for the embeddings open if its being held here
    // whenever you cache the embeddings from an environment you must remove the database from memory, otherwise it is nonsense
    // For this being efficient, context has to be shared among all different 'utilities' (different users making different queries)
    loaded_clusters: Vec<Option<Rc<EmbeddingHolder>>>,
    pub distance_function: Box<dyn DFUtility> // Box<dyn Trait> since you want to own the unmutable value, and have dynamic dispatch 
    // because it won't be changing in runtime
    
}

impl Context {
    pub fn get_centroid(&self, cluster_id: ClusterId) -> &Embedding {
        self.codebook.get_embedding(cluster_id)
    }

}



pub enum DistanceFunctionSelection {
    Cosine,
    Euclidean
}

struct CosineDistFn;
struct DefaultDistFn;

// make it generic for a & b params (consider f32 Ord case)
pub trait DFUtility {
    fn nearest(&self, a: i32, b: i32) -> i32;
}

impl DFUtility for CosineDistFn {
    fn nearest(&self, a: i32, b: i32) -> i32 {
        i32::max(a, b)
    }
}

impl DFUtility for DefaultDistFn {
    fn nearest(&self, a: i32, b: i32) -> i32 {
        i32::min(a, b)
    }
}

// Function ran during initialization

pub fn load_context(df: &DistanceFunctionSelection) -> Result<Context, Error> {
    
    // use default env to open its dbs and load each Context field

    let env: Environment;
    unsafe {
        let builder = lmdb::EnvBuilder::new().unwrap();
        builder.set_maxdbs(2); // may be 3
        env = builder.open(GENERIC_PATH.push_str("default.db"),
        lmdb::open::Flags::empty(),
        0o600)?;
    }

    let inverted_list = load_db_instance::<InvertedList>(&env, "inverted_list")?;
    let codebook = load_db_instance::<Codebook>(&env, "codebook")?;

    Ok(Context {
        env,
        inverted_list,
        codebook,
        loaded_clusters: vec![None; codebook.get_size()],
        distance_function: match df {
            DistanceFunctionSelection::Cosine => CosineDistFn,
            _ => DefaultDistFn
        }
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