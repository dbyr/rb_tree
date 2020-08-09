# RBTree
Implementation of the Red Black tree data structure in Rust. This implementation can be used as a set, or a priority queue and has methods to support both use cases There is also a supporting pair of macros that can be used to allow the tree to be used as a map (see `make_map` and `make_map_named` in `src/map.rs`). 

Currently all comparisons are done via the `PartialOrd` trait, meaning types must implement this trait in order to be used with the RBTree. I have tentative plans to allow one to opt out of this requirement in favour of a user-provided comparison method.