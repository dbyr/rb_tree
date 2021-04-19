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
#[derive(Clone)]
pub struct RBMap<K: PartialOrd, V> {
    map: RBTree<Mapper<K, V>>
}

/// A red black tree that can be used to store
/// elements sorted by their PartialOrd provided
/// ordering.
#[derive(Clone)]
pub struct RBTree<T: PartialOrd> {
    root: Node<T>,
    contained: usize
}

/// A priority queue implemented using a red black
/// tree. The ordering supplied must satisfy the assymetry
/// and transitivity rules as outlined by  the dorumentation
/// of std::cmp::PartialOrd.
#[derive(Clone)]
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

/// Allows the creation of a queue using C-like
/// comparison values. That is to say, `cmp`
/// should return a value less than, equal to,
/// or greater than 0 when `l` should be placed
/// before, is equal to, or be placed after `r`
/// respectively.
/// 
/// `cmp` should be a function that takes two values
/// from the queue and returns an integer (i8)
/// providing the information as above.
/// 
/// # Example:
/// ```
/// # #[macro_use(new_c_queue)]
/// # extern crate rb_tree;
/// # use rb_tree::RBQueue;
/// # fn main() {
/// let mut q = new_c_queue!(|l: &i64, r| (r - l));
/// q.insert(1);
/// q.insert(2);
/// q.insert(3);
/// assert_eq!(q.ordered(), [&3, &2, &1]);
/// # }
/// ```
/// 
/// # Example:
/// ```
/// # #[macro_use(new_c_queue)]
/// # extern crate rb_tree;
/// # use rb_tree::RBQueue;
/// # fn main() {
/// let q = new_c_queue!(|l: &i64, r| (r - l); 1, 2, 3);
/// assert_eq!(q.ordered(), [&3, &2, &1]);
/// # }
/// ```
#[macro_export]
macro_rules! new_c_queue {
    ($cmp:expr) => {
        RBQueue::new(move |l, r| {
            let comp = Box::new($cmp);
            match comp(l, r) as i8 {
                -128i8 ..= -1 => std::cmp::Ordering::Less,
                0 => std::cmp::Ordering::Equal,
                1 ..= 127i8 => std::cmp::Ordering::Greater
            }
        })
    };

    ($cmp:expr; $($v:expr),*) => {{
        let mut q = RBQueue::new(move |l, r| {
            let comp = Box::new($cmp);
            match comp(l, r) as i8 {
                -128i8 ..= -1 => std::cmp::Ordering::Less,
                0 => std::cmp::Ordering::Equal,
                1 ..= 127i8 => std::cmp::Ordering::Greater
            }
        });
        $(
            q.insert($v);
        )*
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
