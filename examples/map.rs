extern crate rb_tree;

use rb_tree::RBMap;

fn main() {
    let mut t = RBMap::new();
    t.insert("Hello", 0);
    t.insert("World", 1);
    assert_eq!(t.remove(&"World").unwrap(), 1);

    t.insert("Blah", 2);
    
    for (_, v) in t.iter_mut() {
        println!("{:?}", v);
        *v = 6;
    }
    for (_, v) in t.iter_mut() {
        println!("{:?}", v);
    }
}