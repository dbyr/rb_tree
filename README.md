# RBTree
This crate contains an implementation of the Red Black tree data structure and several data structures that are built on top of this implementation. The data structures currently include RBTree and RBMap

### RBTree
This data structure can be used as a set, or a priority queue based on the values' `PartialOrd` ordering, and has methods to support both use cases 

### RBMap
This data structure provides an interface for using the RBTree as a map. Values in the map are ordered by their keys' `PartialOrd` ordering.

### Future Additions
Currently all comparisons are done via the `PartialOrd` trait, meaning types must implement this trait in order to be used with the RBTree. I have plans to allow one to opt out of this requirement in favour of a user-provided comparison method. This will likely be v3.0, and include a type `RBQueue` or `RBPQueue` to provide this ability.

Examples:
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

```
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