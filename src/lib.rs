mod node;
pub mod rbtree;
pub mod rbmap;
pub mod rbqueue;
mod helpers;
mod mapper;
#[cfg(test)]
mod rbtree_tests;

use node::Node;
use mapper::Mapper;

/// A map implemented using a red black tree to
/// store key-value pairs.
pub struct RBMap<K: PartialOrd, V> {
    map: RBTree<Mapper<K, V>>
}

/// A red black tree that can be used to store
/// elements sorted by their PartialOrd provided
/// ordering.
pub struct RBTree<T: PartialOrd> {
    root: Node<T>,
    contained: usize
}

/// A priority queue implemented using a red black
/// tree. The ordering supplied must satisfy the assymetry
/// and transitivity rules as outlined by  the dorumentation
/// of std::cmp::PartialOrd.
pub struct RBQueue<T, P> 
where P: Fn(&T, &T) -> std::cmp::Ordering {
    root: Node<T>,
    contained: usize,
    cmp: P
}