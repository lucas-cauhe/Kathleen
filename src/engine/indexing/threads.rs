use std::sync::{Arc, Mutex};

use futures::executor::block_on;
use queues::Queue;

use crate::engine::utils::concurrency::Context2Thread;

use super::ivf_controller::{Load, Dump, ActionWaker, ResponseResult};



pub struct SearchThread {
    // search_params,
    loader: Load,
    dumper: Dump
}
pub struct InsertThread {
    // insert values
    // ...
}
//pub struct UpdateThread {}

pub struct ActionsProcessingThread<'a> {
    next: usize,
    queues: &'a [Mutex<Queue<Arc<Mutex<ActionWaker>>>>],
    receivers: Context2Thread,
    set_count: Arc<Mutex<(i32, i32)>>
}

impl ActionsProcessingThread<'_> {
    pub fn new(q: &[Mutex<Queue<Arc<Mutex<ActionWaker>>>>], 
        c: Context2Thread,
        sc: Arc<Mutex<(i32, i32)>> ) -> Self {
        ActionsProcessingThread { 
            next: 0,
            queues: q,
            receivers: c,
            set_count: sc,
        }
    }
}

pub trait CommonThread {
    
    fn task(&self);
}

impl CommonThread for SearchThread {
    fn task(&self) {
        todo!()
    }
}

impl CommonThread for SearchThread {
    fn task(&self) {
        todo!()
    }
}

impl CommonThread for ActionsProcessingThread<'_> {
    fn task(&self) {

        loop {
           
            // wait until some queue has a Future to be resolved
            //wait on semaphore if there are no actions queued
            // notify you're in (atomic)
            let queues_are_empty = {
                let (count, _) = self.set_count.lock().unwrap();
                *count == 0
            };
            if queues_are_empty{
                self.receivers.rx_ch1.recv();
            }

            // check how many threads are working by the time it is awaken
            // (x)
            let no_threads; 
            {
                let  (_, mut thres) = self.set_count.lock().unwrap();
                *thres = no_threads;
            }

            // wait until there are x elements in the queues or threads are done
            // querying
            // another semaphore
            // notify you've waited for all
            while self.receivers.rx_ch2.try_recv() < Ok(no_threads) {
                self.receivers.rx_ch2.recv();
            }
            

            // perform next-max strategy
            // loop through every queue, compare to the max, 
            // if max == next => update next
            let max_arr = self.queues.iter().map(|qu| {
                let q = qu.lock().unwrap();
                q.size();
            });
            let max = max_arr.position(|e| e==max_arr.max() ).unwrap();
            if max == self.next {
                self.next += 1;
            }

            let mut actions_to_process = self.queues.get(max).unwrap().lock().unwrap();

            // perform actions && unblock waiting threads
            for action in actions_to_process {
                
                let mut action_type = action.lock().unwrap();
                // access the db via dbaccessor from ctx to perform action
                let resp: ResponseResult;
                action_type.response = Some(resp);
                
                if let Some(waker) = action.waker.take() {
            
                    waker.wake();

                    // drop own Arc
                    // will this do it?
                    *actions_to_process.remove();
                }
            }
            let (mut count, _) = self.set_count.lock().unwrap();
            *count = *count-actions_to_process.len();
            // notify you're out if there are no queries left (atomic)
        }

        
    }
}