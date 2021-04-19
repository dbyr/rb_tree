# rb_tree

This crate contains an implementation of the red-black tree data structure and several data structures that are built on top of this implementation. The data structures currently include RBTree, RBMap, and RBQueue.

## Data Structures

### RBTree

This data structure can be used as a set and has methods to support its use as a set. Methods specific to this data structure include set operations such as union, difference etc. Values are stored in their `PartialOrd` ordering.

### RBMap

This data structure provides an interface for using the RBTree as a map. Values in the map are ordered by their keys' `PartialOrd` ordering.

### RBQueue

This data structure allows the use of the underlying red-black tree as a priority queue. A comparison function is provided on instantiation (either with `RBQueue::new(Fn(&T, &T) -> std::cmp::Ordering)` or `rbqueue_c_new!(Fn(&T, &T) -> i8)`) which is used to order the entries.

## Examples

```rust
use rb_tree::RBTree;

// uses an rbtree to sort data
fn sort<T: PartialOrd>(to_order: Vec<T>) -> Vec<T> {
    let mut tree = RBTree::new();
    let mut ordered = Vec::new();
    for v in to_order {
        tree.insert(v);
    }
    while let Some(v) = tree.pop() {
        ordered.push(v);
    }
    ordered
}

fn main() {
    let eg1 = vec!(3, 6, 1, 2, 0, 4, -1, 5, 10, 11, -13);
    assert_eq!(sort(eg1), vec!(-13, -1, 0, 1, 2, 3, 4, 5, 6, 10, 11));

    let eg2 = vec!("Is", "this", "the", "real", "life");
    assert_eq!(sort(eg2), vec!("Is", "life", "real", "the", "this"))
}
```

```rust
use rb_tree::RBMap;

fn main() {
    let mut squares = RBMap::new();
    for i in 0..10 {
        squares.insert(i, i);
    }

    squares.values_mut().for_each(|v| *v = (*v as f64).powi(2) as u32);

    for i in 0..10 {
        assert_eq!(*squares.get(&i).unwrap(), (i as f64).powi(2) as u32);
    }
}
```

```rust
#[macro_use(rbqueue_c_new)]
extern crate rb_tree;

use rb_tree::RBQueue;

fn main() {
    
    // use the default comarator
    let mut q1 = RBQueue::new(|l: &i64, r| {
        l.cmp(r)
    });

    // compare in the reverse order
    let mut q2 = rbqueue_c_new!(|l: &i64, r| (r - l));

    q1.insert(1);
    q1.insert(2);
    q1.insert(3);
    q2.insert(1);
    q2.insert(2);
    q2.insert(3);

    assert_eq!(q1.ordered(), [&1, &2, &3]);
    assert_eq!(q2.ordered(), [&3, &2, &1]);
}
```
