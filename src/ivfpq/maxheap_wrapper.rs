use heapless::binary_heap::{BinaryHeap, Max};
use derivative::{self, Derivative};
use ordered_float::NotNan;
use super::primitive_types::PqCode;

#[derive(Derivative, Clone)]
#[derivative(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct HeapNode<'a> {
    distance: NotNan<f64>,
    #[derivative(Ord="ignore")]
    #[derivative(PartialOrd="ignore")]
    #[derivative(PartialEq="ignore")]
    code: &'a PqCode
}

impl<'a> HeapNode<'a> {
    pub fn new(d: NotNan<f64>, c: &'a PqCode) -> Self {
        Self { distance: d, code: c }
    }
}


pub(crate) struct BinaryHeapWrapper<T, const N:usize>(BinaryHeap<T, Max, N>);

impl<T: Ord + PartialOrd + Clone, const N:usize> BinaryHeapWrapper<T, N> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.0.len() == self.0.capacity() {
            let peek = self.0.peek().unwrap();
            if peek > &item {
                self.0.pop();
                return self.0.push(item);
            } else {
                Ok(())
            }
        } else {
            return self.0.push(item);
        }
    } 
    pub fn sorted(self) -> Vec<T> {
        let mut vec = self.0.into_vec();
        vec.sort();
        let mut heap_vec: Vec<T> = Vec::new();
        vec.iter().for_each(|node| heap_vec.push(node.clone()));
        heap_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::ivfpq::ivfpq::EMBEDDING_M_SEGMENTS;

    use super::*;

    #[test]
    fn default_behaviour_works() {
        let mut heap: BinaryHeapWrapper<usize, 4> = BinaryHeapWrapper::new();

        // We can use peek to look at the next item in the heap. In this case,
        // there's no items in there yet so we get None.
        assert_eq!(heap.0.peek(), None);

        // Let's add some scores...
        heap.push(1).unwrap();
        heap.push(5).unwrap();
        heap.push(2).unwrap();

        // Now peek shows the most important item in the heap.
        assert_eq!(heap.0.peek(), Some(&5));

        // We can check the length of a heap.
        assert_eq!(heap.0.len(), 3);

        // We can iterate over the items in the heap, although they are returned in
        // a random order.
        for x in &heap.0 {
            println!("{}", x);
        }

        // If we instead pop these scores, they should come back in order.
        assert_eq!(heap.0.pop(), Some(5));
        assert_eq!(heap.0.pop(), Some(2));
        assert_eq!(heap.0.pop(), Some(1));
        assert_eq!(heap.0.pop(), None);

        // We can clear the heap of any remaining items.
        heap.0.clear();

        // The heap should now be empty.
        assert!(heap.0.is_empty())
    }

    #[test]
    fn expected_behaviour_works() {
        let mut heap: BinaryHeapWrapper<usize, 4> = BinaryHeapWrapper::new();

        // We can use peek to look at the next item in the heap. In this case,
        // there's no items in there yet so we get None.
        assert_eq!(heap.0.peek(), None);

        // Let's add some scores...
        heap.push(1).unwrap();
        heap.push(5).unwrap();
        heap.push(2).unwrap();
        heap.push(3).unwrap();
        heap.push(4).unwrap();

        // Now peek shows the most important item in the heap.
        assert_eq!(heap.0.peek(), Some(&4));

        // We can check the length of a heap.
        assert_eq!(heap.0.len(), 4);

        // We can iterate over the items in the heap, although they are returned in
        // a random order.
        for x in &heap.0 {
            println!("{}", x);
        }

        // If we instead pop these scores, they should come back in order.
        assert_eq!(heap.0.pop(), Some(4));
        assert_eq!(heap.0.pop(), Some(3));
        assert_eq!(heap.0.pop(), Some(2));
        assert_eq!(heap.0.pop(), Some(1));
        assert_eq!(heap.0.pop(), None);

        // We can clear the heap of any remaining items.
        heap.0.clear();

        // The heap should now be empty.
        assert!(heap.0.is_empty())

    }

    #[test]
    fn expected_behaviour_works_with_heap_nodes() {
        let mut heap: BinaryHeapWrapper<HeapNode<'_>, 4> = BinaryHeapWrapper::new();

        // We can use peek to look at the next item in the heap. In this case,
        // there's no items in there yet so we get None.
        assert_eq!(heap.0.peek(), None);

        // Let's add some scores...
        heap.push(HeapNode{
            distance: NotNan::new(25.333).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }).unwrap();
        heap.push(HeapNode{
            distance: NotNan::new(12.4).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }).unwrap();
        heap.push(HeapNode{
            distance: NotNan::new(1.6).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }).unwrap();
        heap.push(HeapNode{
            distance: NotNan::new(13.16).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }).unwrap();
        heap.push(HeapNode{
            distance: NotNan::new(22.43).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }).unwrap();

        // Now peek shows the most important item in the heap.
        assert_eq!(heap.0.peek(), Some(&HeapNode{
            distance: NotNan::new(22.43).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }));

        // We can check the length of a heap.
        assert_eq!(heap.0.len(), 4);

        // We can iterate over the items in the heap, although they are returned in
        // a random order.
        for x in &heap.0 {
            println!("{:?}", x);
        }

        // If we instead pop these scores, they should come back in order.
        assert_eq!(heap.0.pop(), Some(HeapNode{
            distance: NotNan::new(22.43).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }));
        assert_eq!(heap.0.pop(), Some(HeapNode{
            distance: NotNan::new(13.16).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }));
        assert_eq!(heap.0.pop(), Some(HeapNode{
            distance: NotNan::new(12.4).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }));
        assert_eq!(heap.0.pop(), Some(HeapNode{
            distance: NotNan::new(1.6).unwrap(),
            code: &[1; EMBEDDING_M_SEGMENTS]
        }));
        assert_eq!(heap.0.pop(), None);

        // We can clear the heap of any remaining items.
        heap.0.clear();

        // The heap should now be empty.
        assert!(heap.0.is_empty())

    }
}