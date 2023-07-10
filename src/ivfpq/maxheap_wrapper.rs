use heapless::binary_heap::{BinaryHeap, Max};
use derivative::{self, Derivative};
use ordered_float::NotNan;
use super::ivfpq::PqCode;

pub const RETRIEVE_KNN: usize = 10;

#[derive(Derivative)]
#[derivative(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct HeapNode<'a> {
    distance: NotNan<f32>,
    #[derivative(Ord="ignore")]
    #[derivative(PartialOrd="ignore")]
    #[derivative(PartialEq="ignore")]
    code: &'a PqCode
}


pub(crate) struct BinaryHeapWrapper<'a>(BinaryHeap<HeapNode<'a>, Max, RETRIEVE_KNN>);

impl<'a> BinaryHeapWrapper<'a> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, item: HeapNode<'a>) -> Result<(), HeapNode> {
        if self.0.len() == self.0.capacity() {
            let peek = self.0.peek().unwrap();
            if peek > &item {
                self.0.pop();
                return unsafe {self.0.push(item)};
            } else {
                Ok(())
            }
        } else {
            return unsafe {self.0.push(item)};
        }
    } 
    pub fn sorted(self) -> Vec<HeapNode<'a>> {
        let mut vec = self.0.into_vec();
        vec.sort();
        let mut heap_vec: Vec<HeapNode<'_>> = Vec::new();
        vec.iter().for_each(|node| heap_vec.push(*node.clone()));
        heap_vec
    }
}