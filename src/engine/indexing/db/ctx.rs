

use std::{env, rc::Rc};

use futures::Future;
use queues::{IsQueue, Queue};
use rdb::{ColumnFamily, Error, DB, DBAccess};

use crate::{engine::{utils::{pq::train::Codebook, types::{ClusterId, VecId}}, indexing::ivf_controller::ActionFuture}, tokenizer::tokenize::Embedding};

use super::{inverted_list::InvertedList, DBInterface, vector_storage::{EmbeddingHolder, SegmentHolder}, dbaccess::DbAccess};
const GENERIC_PATH: String = env::var("DB_STORAGE").unwrap();
pub struct Context {

    db_path: String,
    // the environment where context will hold its data will be the same in all of the fields that require 
    // db interaction
    inverted_list: InvertedList,
    codebook: Codebook,
    accessor: DbAccess,

    pending_actions_queue: Vec<Queue<ActionType>>,
    // actions_processing will be awaken whenever queued_actions > 0
    queued_actions: i32,
    
    //coarse_quantizer:
    // if multiple searches are made, its unnecesary to keep loading embeddings while they're being used
    // multiple queries can access same cluster embeddings (consider this in a threaded scenario)

    // therefore makes no sense to keep the environment's database for the embeddings open if its being held here
    // whenever you cache the embeddings from an environment you must remove the database from memory, otherwise it is nonsense
    // For this being efficient, context has to be shared among all different 'utilities' (different users making different queries)
    // Perhaps use TTL from rdb
    
    pub distance_function: Box<dyn DFUtility>, // Box<dyn Trait> since you want to own the unmutable value, and have dynamic dispatch 
    // because it won't be changing in runtime

    pub dbs_names: Vec<String>
    
}



#[derive(Clone)]
pub enum ActionType
{
    Load{
        embeddings: Vec<VecId>,
        cluster: ClusterId
    },
    Dump{
        embeddings: Vec<VecId>,
        cluster: ClusterId
    }
}

// pub trait Load {}
pub trait Dump {}



///	_Function for running the actions processing logic_
///
///	# _Arguments_
///
/// * `queues` - __
pub fn actions_processing(queues: &[Queue<ActionType>], processing_time: i32) -> usize {
    todo!()
}

impl Context {

    pub fn new() -> Context {
        Context { inverted_list: (), codebook: (), loaded_clusters: (), distance_function: () }
    }

    
    
    pub fn get_ready(&mut self, act_type: &ActionType) -> ActionFuture {
        // implement a future such that:
        // if the segments to be loaded are cached then it is ready
        // otherwise wait to be awaken and deque action

        // checked in dbaccess whether act_type segments are cached
        // if true return a cb somehow, specific to return Poll::Ready() on future (without queing)
        // else return a different cb, specific to return Poll::Pending and wait (queing (dequeing is done by actions_processing thread))


        // queue action somewhere but not inside the callback
        // since mutex won't make sense then
        let action_future = ActionFuture::new();
        action_future.set_cb(|cx| {
            match self.accessor.get_cached(act_type) {
                Some(resp) => {
                    Some(resp)
                },
                None => {
                    // specify how to wake the thread
                    cx.waker();
                }
            }
        })

    }

    fn queue_action(&mut self, act_type: &ActionType) -> () {
        todo!()
    }

    pub fn load_embeddings(&self, act_type: ActionType) -> Result<Rc<EmbeddingHolder>, Error> {
        let token = self.add_action(&act_type)?;
        self.response(token)?
    }
    pub fn dump_embeddings()

    pub fn get_centroid(&self, cluster_id: ClusterId) -> &Embedding {
        self.codebook.get_embedding(cluster_id)
    }

    pub fn load_cluster(&mut self, cluster: &ClusterId) -> Result<Rc<EmbeddingHolder>, Error> {
        
        match self.loaded_clusters[*cluster] {
            Some(eh) => {
                // return cached (in use or not-expired TTL) eh
                // when dropping last reference to any EH, start a TTL

                Ok(Rc::clone(&eh))
                
            },
            None => {
                // cache new loaded embedding holder
                let db = DB::open(&self.db_opts, &self.dbs_names[*cluster])?;

                let new_eh = EmbeddingHolder::new(&db);
                new_eh.load(&[])?;
                self.loaded_clusters[*cluster] = Some(Rc::new(new_eh));
                Ok(Rc::clone(&self.loaded_clusters[*cluster].unwrap()))
            }
        }
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

    let env: ColumnFamily;
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

