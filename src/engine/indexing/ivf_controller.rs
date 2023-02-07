use std::{cell::RefCell, sync::{Arc, Mutex}, future::Future};

use rdb::Error;
use futures::{executor::block_on, FutureExt};


use crate::engine::utils::types::{ClusterId, VecId};

use super::db::{ctx::{Context, ActionType}, dbaccess::EmbeddingHolder, self};


pub struct IVFController {
    ctx: Arc<Mutex<Context>>
}

pub enum ResponseResult {
    Load(Arc<EmbeddingHolder>),
    Dump()
}

pub struct ActionFuture {
    cb: dyn Fn(&mut std::task::Context) -> Option<ResponseResult>
}

impl ActionFuture {
    pub fn new() -> Self {
        ActionFuture { cb: () }
    }

    pub fn set_cb<F>(&self, f: F) -> () 
    where
        F: Fn(&mut std::task::Context) -> Option<ResponseResult>
    {
        self.cb = f;
    }
}

impl Future for ActionFuture {
    type Output = Result<ResponseResult, Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(res) = self.cb(cx) {
            std::task::Poll::Ready(Ok(res))
        } else {
            // the wake method has been passed to the cb
            std::task::Poll::Pending
        }
    }
}

trait InternalAction
{
    fn add_action(&self, ctx: Arc<Mutex<Context>>, act_type: &ActionType) -> Result<ResponseResult, Error>{
        
        // if there are any chances a thread could panic while having the lock
        // when taking it you must ensure it is not poisoned
        
        // queue_action cannot be a blocking function since it has the mutex
        // if it were to be awaitable then every other thread waiting for the mutex
        // wouldn't take it until the db opt was done. What would break the whole system designed
        let complete: ActionFuture =
        {
            let local_ctx = ctx.lock().unwrap();
            // action.add(*local_ctx.queue_action(act_type)?);
            *local_ctx.get_ready(act_type)
        };
        block_on(complete) // will call poll defined in the custom impl
    }
    //fn response(&self, token: (usize, usize)) -> Result<ResponseResult, Error>;
}

pub struct Load {}
pub struct Dump {}

impl InternalAction for Load {}
impl InternalAction for Dump {}

// impl InternalAction for Load {
//     fn response(&self, token: (usize, usize)) -> Result<ResponseResult, Error> {
//         // wait for the action with token <token> be completed
//         //ResponseResult::Load(res);
//     }
// }

// impl InternalAction for Dump {
//     fn response(&self, token: (usize, usize)) -> Result<ResponseResult, Error> {
//         // wait for the action with token <token> be completed
//         todo!()
//     }
// }

impl Load {
    pub fn load(&self, ctx: Arc<RefCell<Context>>, embeddings: &[VecId], cluster: &ClusterId) -> Result<Arc<EmbeddingHolder>, Error> {
        let response = block_on(self.add_action(ctx, &ActionType::Load { embeddings, cluster }));
        match response {
            Ok(res) => {
                match res {
                    ResponseResult::Load(holder) => Ok(holder),
                    _ => Error { message: "Unexpected ResponseResult returned in load".to_string() }
                }
            },
            Err(e) => Error {message: format!("Error waiting the response for load: {:?}", e)}
        }
    }
}

impl Dump {
    pub fn dump(&self, ctx: Arc<RefCell<Context>>, embeddings: &[VecId], cluster: &ClusterId) -> Result<(), Error> {
        let response = block_on(self.add_action(ctx, &ActionType::Dump { embeddings, cluster }));
        match response {
            Ok(res) => {
                match res {
                    ResponseResult::Dump() => Ok(()),
                    _ => Error { message: "Unexpected ResponseResult returned in dump".to_string() }
                }
            },
            Err(e) => Error { message: format!("Error waiting the response for dump: {:?}", e) }
        }
    }
}