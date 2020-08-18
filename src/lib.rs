mod node;
pub mod rbtree;
pub mod rbmap;
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
