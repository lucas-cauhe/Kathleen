use super::ivf_controller::{Load, Dump};



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