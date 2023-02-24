

use std::{env, sync::{Mutex, Arc, mpsc::{Sender, channel}}, thread};

use queues::{IsQueue, Queue};

use crate::{engine::{utils::{pq::train::Codebook, types::{ClusterId, VecId}, concurrency::Context2Thread}, indexing::{ivf_controller::{ActionFuture, ActionWaker}, threads::{ActionsProcessingThread, CommonThread}}}};

use super::{inverted_list::InvertedList, dbaccess::DbAccess};
const GENERIC_PATH: String = env::var("DB_STORAGE").unwrap();
pub struct Context {

    db_path: String,
    // the environment where context will hold its data will be the same in all of the fields that require 
    // db interaction
    inverted_list: InvertedList,
    codebook: Codebook,
    accessor: DbAccess,

    pending_actions_queue: Vec<Mutex<Queue<Arc<Mutex<ActionWaker>>>>>,  // Vec<Queue<Rc<Mutex<ActionWaker>>>>,
    // actions_processing will be awaken whenever queued_actions > 0
    queued_actions:Arc<Mutex<(i32, i32)>>,
    senders: (Sender<i32>, Sender<i32>),
    
    //coarse_quantizer:
    // if multiple searches are made, its unnecesary to keep loading embeddings while they're being used
    // multiple queries can access same cluster embeddings (consider this in a threaded scenario)

    // therefore makes no sense to keep the environment's database for the embeddings open if its being held here
    // whenever you cache the embeddings from an environment you must remove the database from memory, otherwise it is nonsense
    // For this being efficient, context has to be shared among all different 'utilities' (different users making different queries)
    // Perhaps use TTL from rdb
    
    pub distance_function: Box<dyn DFUtility>, // Box<dyn Trait> since you want to own the unmutable value, and have dynamic dispatch 
    // because it won't be changing in runtime
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
impl Context {

    pub fn new() -> Context {
        let pending_actions_queue = Vec::new();
        let ch1 = channel::<i32>();
        let ch2 = channel::<i32>();
        let queued_actions = Arc::new(Mutex::new((0, 0)));
        let act_processing_thr = ActionsProcessingThread::new(
            &pending_actions_queue,
            Context2Thread::new(ch1.1, ch2.1),
            queued_actions.clone()
        );

        thread::spawn(act_processing_thr.task());

        Context { 
            db_path: String::from(GENERIC_PATH),
            inverted_list: InvertedList::new(), 
            codebook: Codebook::new(), 
            accessor: DbAccess::new(),
            pending_actions_queue,
            queued_actions,
            senders: (ch1.0, ch2.0),
            distance_function: CosineDistFn::new()
        }
    }

    
    
    pub fn dispatch_future(&mut self, act_type: &ActionType) -> ActionFuture {
        // implement a future such that:
        // if the segments to be loaded are cached then it is ready
        // otherwise wait to be awaken and deque action

        // checked in dbaccess whether act_type segments are cached
        // if true return a cb somehow, specific to return Poll::Ready() on future (without queing)
        // else return a different cb, specific to return Poll::Pending and wait (queing (dequeing is done by actions_processing thread))
        let mut action_waker: Arc<Mutex<ActionWaker>>; 
        match self.accessor.get_cached(act_type) {
            Some(resp) => {
                action_waker = Arc::new(Mutex::new(ActionWaker::new(act_type, Some(resp))));
            }
            None => {
                action_waker = Arc::new(Mutex::new(ActionWaker::new(act_type, None)));
                self.queue_action(action_waker.clone());
            }
        };

        // Theres no threat that the response might be dropped by the 
        // context while callback is being called since the counted reference
        // has already been called
        
        ActionFuture::new(action_waker)

    }

    fn queue_action(&mut self, act_type: Arc<Mutex<ActionWaker>>) -> () {
        let action = act_type.lock().unwrap();
        match *action.action {
            ActionType::Load {embeddings, cluster} | ActionType::Dump { embeddings, cluster } => {
                let queue = self.pending_actions_queue.get(cluster).unwrap();
                let mutex_queue = queue.lock().unwrap();
                drop(action);
                mutex_queue.add(act_type);
            }
        }
        self.notify_processor();
        // semaphores check (atomic)
            // if thread is out -> release trigger sem
            // if thread is in but still waiting -> if I query the x-th element release no_threads semaphore
            // if it's not waiting -> add action without any notification
       
    }

    fn notify_processor(&self) {
    
        let  (mut count, thres) = self.queued_actions.lock().unwrap();
        if *count == 0 {
            self.senders.0.send(1);
        }
        *count += 1;
        if *count == *thres {
            self.senders.1.send(*thres);
        }
    
    }

    // pub fn get_centroid(&self, cluster_id: ClusterId) -> &Embedding {
    //     self.codebook.get_embedding(cluster_id)
    // }
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
