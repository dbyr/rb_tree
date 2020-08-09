#[macro_use(make_map, make_map_named)]
extern crate rb_tree;

use rb_tree::RBTree;
use std::string::ToString;

make_map!(String, usize);

fn main() {
    let mut t = RBTree::new();
    t.insert(Mapper::new("Hello".to_string(), 0));
    t.insert(Mapper::new("World".to_string(), 1));
    assert_eq!(t.remove(&"World".to_string()).unwrap(), Mapper::new("World".to_string(), 1));
}