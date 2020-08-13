# RBTree
Implementation of the Red Black tree data structure in Rust. This implementation can be used as a set, or a priority queue and has methods to support both use cases There is also a supporting pair of macros that can be used to allow the tree to be used as a map (see `make_map` and `make_map_named` in `src/map.rs`, also an example of use in `examples/map.rs` in the source). 

Currently all comparisons are done via the `PartialOrd` trait, meaning types must implement this trait in order to be used with the RBTree. I have tentative plans to allow one to opt out of this requirement in favour of a user-provided comparison method.

Example:
```
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