mod node;

use node::Node;
use node::Node::{Internal, Leaf};
use std::fmt::{Debug, Display, Result, Formatter};

pub struct RBTree<T: PartialOrd> {
    root: Node<T>,
    contained: usize
}

fn ordered_insertion<'a, T: PartialOrd>(cur: &'a Node<T>, order: &mut Vec<&'a T>) {
    if cur.is_leaf() {
        return;
    }
    ordered_insertion(cur.get_left(), order);
    if let Some(v) = cur.value() {
        order.push(v);
    }
    ordered_insertion(cur.get_right(), order);
}

impl<T: PartialOrd + Debug> Display for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

fn write_to_level<T: PartialOrd + Debug>(
    cur: &Node<T>, 
    level: usize, 
    levels: &mut Vec<String>
) {
    if levels.len() <= level {
        match cur {
            Internal(n) => levels.push(format!("{}:{:?}", n.colour(), n.value())),
            Leaf => levels.push("___".to_string())
        }
    } else {
        match cur {
            Internal(n) => levels[level] += &format!(" {}:{:?}", n.colour(), n.value()),
            Leaf => levels[level] += " ___"
        }
    }
    if !cur.is_leaf() {
        write_to_level(cur.get_left(), level + 1, levels);
        write_to_level(cur.get_right(), level + 1, levels);
    }
}

impl<T: PartialOrd + Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.root, 0, &mut levels);
        let mut f_string = "".to_string();
        for i in 0..levels.len() {
            f_string += &levels[i];
            if i != levels.len() - 1 {
                f_string += "\n";
            }
        }
        write!(f, "{}", f_string)
    }
}

impl<T: PartialOrd> RBTree<T> {
    pub fn new() -> RBTree<T> {
        RBTree {root: Leaf, contained: 0}
    }
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::new();
        ordered_insertion(&self.root, &mut order);
        order
    }

    pub fn len(&self) -> usize {
        self.contained
    }

    pub fn insert(&mut self, val: T) {
        self.root.insert(val);
        self.contained += 1;
    }

    // pub fn contains(&self, val: &T) -> bool {
        
    // }

    // pub fn get(&self, val: &T) -> Option<&T> {

    // }

    // pub fn at(&self, index: usize) -> Option<&T> {

    // }

    // pub fn remove(&mut self, val: &T) -> Option<T> {

    // }

    // pub fn pop(&mut self) -> Option<T> {

    // }

    // pub fn peek(&self) -> Option<&T> {

    // }


}

#[cfg(test)]
mod tests {
    use crate::RBTree;
    use crate::node::Node;
    use crate::node::Colour::*;

    #[test]
    fn test_print() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.insert(1.2);
        assert_eq!(format!("{}", t), "[1.0, 1.2, 2.0, 3.0]");
        assert_eq!(t.len(), 4);
        assert_eq!(format!("{:?}", t), "B:2.0\nB:1.0 B:3.0\n___ R:1.2 ___ ___\n___ ___");
    }

    #[test]
    fn test_case1_left() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(0.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 0.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Black);
    }

    #[test]
    fn test_case1_right() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_left_mut().swap_colour(); // simulate the situation
        t.insert(4.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 3.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 4.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_left().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_left().get_left().colour(), Black);
    }

    #[test]
    fn test_case2_right() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_left_mut().swap_colour(); // simulate the situation
        t.insert(2.5);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 2.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 2.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_left().get_left().colour(), Black);
    }

    #[test]
    fn test_case2_left() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(1.5);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Black);
    }

    #[test]
    fn test_case3_at_root() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.insert(0.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 2.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_left().colour(), Black);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 0.0);
        assert_eq!(t.root.get_left().get_left().colour(), Red);
    }

    #[test]
    fn test_case3_not_root() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(1.5);
        t.insert(2.5);
        t.insert(4.0);
        t.insert(5.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Black);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().get_right().value().unwrap(), 4.0);
        assert_eq!(t.root.get_right().get_right().get_right().colour(), Black);
        assert_eq!(*t.root.get_right().get_right().get_right().get_right().value().unwrap(), 5.0);
        assert_eq!(t.root.get_right().get_right().get_right().get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().get_left().value().unwrap(), 2.5);
        assert_eq!(t.root.get_right().get_right().get_left().colour(), Black);
    }

    #[test]
    fn test_insertion_transfer_children() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        *t.root.get_left_mut().get_right_mut() = Node::new_black(1.5);
        t.insert(0.0);
        assert_eq!(*t.root.get_right().get_left().value().unwrap(), 1.5);

        // TODO: write one for the inner version as well
    }

    #[test]
    fn test_complex_insertion() {

    }
}