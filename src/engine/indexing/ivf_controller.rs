use std::{cell::RefCell, sync::{Arc, Mutex}, future::Future, task::Waker};

use rdb::Error;
use futures::{executor::block_on};


use crate::engine::utils::types::{ClusterId, VecId};

use super::db::{ctx::{Context, ActionType}, dbaccess::EmbeddingHolder};


pub struct IVFController {
    ctx: Arc<Context>
}

pub enum ResponseResult {
    Load(Arc<EmbeddingHolder>),
    Dump()
}

#[derive(Clone)]
pub struct ActionWaker {
    pub action: ActionType,
    pub waker: Option<Waker>,
    pub response: Option<ResponseResult>
}

impl ActionWaker {
    pub fn new(act: ActionType, resp: Option<ResponseResult>) -> Self {
        ActionWaker { 
            action: act,
            waker: None,
            response: resp
        }
    }
}

pub struct ActionFuture {
    shared_state: Arc<Mutex<ActionWaker>>
}

impl ActionFuture {
    pub fn new(act_waker: Arc<Mutex<ActionWaker>>) -> Self {
        ActionFuture { 
            shared_state: act_waker,
        }
    }
}

impl Future for ActionFuture {
    type Output = Result<ResponseResult, Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if let Some(cached) = shared_state.response {
            std::task::Poll::Ready(Ok(cached))
        } else {
            shared_state.waker = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

trait InternalAction
{
    fn wait_for_action(&self, ctx: &Arc<Context>, act_type: &ActionType) -> Result<ResponseResult, Error>{
        
        // if there are any chances a thread could panic while having the lock
        // when taking it you must ensure it is not poisoned
        
        // queue_action cannot be a blocking function since it has the mutex
        // if it were to be awaitable then every other thread waiting for the mutex
        // wouldn't take it until the db opt was done. What would break the whole system designed
        let complete: ActionFuture = ctx.dispatch_future(act_type);
        let result = block_on(complete); // will call poll defined in the custom impl

        // IMPORTANT
        // CHECK THE SHARED STATE IS DROPPED BY THIS TIME
        // IF NOT DROP IT EXPLICITLY NOW
        // drop(complete);

        result
    }
}

pub struct Load {}
pub struct Dump {}

impl InternalAction for Load {}
impl InternalAction for Dump {}

impl Load {
    pub fn load(&self, ctx: &Arc<RefCell<Context>>, embeddings: &[VecId], cluster: &ClusterId) -> Result<Arc<EmbeddingHolder>, Error> {
        let response = self.wait_for_action(ctx, &ActionType::Load { embeddings, cluster });
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
        let response = self.wait_for_action(ctx, &ActionType::Dump { embeddings, cluster });
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