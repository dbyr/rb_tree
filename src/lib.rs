mod node;
pub mod rbtree;
pub mod rbmap;
#[macro_use]
pub mod rbqueue;
mod helpers;
mod mapper;
#[cfg(test)]
mod rbtree_tests;
#[cfg(test)]
mod stress_test;

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
where P: Copy + Fn(&T, &T) -> std::cmp::Ordering {
    root: Node<T>,
    contained: usize,
    cmp: P
}

/// Returns an RBTree containing the items
/// given separated by commas.
/// # Example:
/// ```
/// use rb_tree::{RBTree, new_set};
/// 
/// let t1 = new_set!('b', 'a', 'd', 'c');
/// let t2 = new_set!('d', 'f', 'e', 'c');
///
/// let mut in_both = t1.intersection(&t2);
/// assert_eq!(in_both.next().unwrap(), &'c');
/// assert_eq!(in_both.next().unwrap(), &'d');
/// assert_eq!(in_both.next(), None);
/// ```
#[macro_export]
macro_rules! new_set {
    ( $($v:expr),* ) => {{
        let mut t = RBTree::new();
        $(
            t.insert($v);
        )*
        t
    }};
}

/// Returns an RBQueue that prioritises on given
/// closure and contains the comma-separated
/// elements following it.
/// # Example:
/// use rb_tree::{RBQueue, new_queue};
/// 
/// let mut q = new_queue!(|l, r| {
/// match l - r {
///     i32::MIN..=-1_i32 => Greater,
///     0 => Equal,
///     1_i32..=i32::MAX => Less
/// }
/// }; 1, 2, 3, 4);
/// assert_eq!(q.pop().unwrap(), 4);
/// assert_eq!(q.pop().unwrap(), 3);
/// assert_eq!(q.pop().unwrap(), 2);
/// assert_eq!(q.pop().unwrap(), 1);
/// assert_eq!(q.pop(), None);
/// ```
#[macro_export]
macro_rules! new_queue {
    ($comp:expr; $($v:expr),*) => {{
        let mut q = RBQueue::new($comp);
        $(q.insert($v);)*
        q
    }};
}

/// Returns an RBMap containing the (key, value)
/// pairs separated by commas.
/// # Example:
/// ```
/// use rb_tree::{RBMap, new_map};
/// 
/// let m = new_map!((1, 'a'), (2, 'b'), (3, 'c'));
/// assert_eq!(m.get(&1).unwrap(), &'a');
/// assert_eq!(m.get(&2).unwrap(), &'b');
/// assert_eq!(m.get(&3).unwrap(), &'c');
/// ```
#[macro_export]
macro_rules! new_map {
    ( $(($k:expr, $v:expr)),* ) => {{
        let mut m = RBMap::new();
        $(
            m.insert($k, $v);
        )*
        m
    }};
}