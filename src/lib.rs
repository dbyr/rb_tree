mod node;

use node::Node;
use std::fmt::{Debug, Display, Result, Formatter};

pub struct RBTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
    contained: usize
}

fn ordered_insertion<'a, T: PartialOrd>(cur: &'a Node<T>, order: &mut Vec<&'a T>) {
    if let Some(n) = cur.get_left() {
        ordered_insertion(n, order);
    }
    order.push(cur.value());
    if let Some(n) = cur.get_right() {
        ordered_insertion(n, order);
    }
}

impl<T: PartialOrd + Debug> Display for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

fn write_to_level<T: PartialOrd + Debug>(
    cur: Option<&Box<Node<T>>>, 
    level: usize, 
    levels: &mut Vec<String>
) {
    if levels.len() <= level {
        match cur {
            Some(n) => levels.push(format!("{}:{:?}", n.colour(), n.value())),
            None => levels.push("___".to_string())
        }
    } else {
        match cur {
            Some(n) => levels[level] += &format!(" {}:{:?}", n.colour(), n.value()),
            None => levels[level] += " ___"
        }
    }
    match cur {
        Some(n) => {
            write_to_level(n.get_left(), level + 1, levels);
            write_to_level(n.get_right(), level + 1, levels);
        },
        None => ()
    }
}

impl<T: PartialOrd + Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(self.root.as_ref(), 0, &mut levels);
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
        RBTree {root: None, contained: 0}
    }
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::new();
        if let Some(n) = self.root.as_ref() {
            ordered_insertion(n, &mut order);
        }
        order
    }

    pub fn len(&self) -> usize {
        self.contained
    }

    pub fn insert(&mut self, val: T) {
        match self.root.as_mut() {
            Some(v) => v.insert(val),
            None => self.root = Some(Box::new(Node::new_black(val)))
        }
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

    #[test]
    fn test_simple() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.insert(1.2);
        assert_eq!(format!("{}", t), "[1.0, 1.2, 2.0, 3.0]");
        assert_eq!(t.len(), 4);
        assert_eq!(format!("{:?}", t), "B:1.2\nR:1.0 R:2.0\n___ ___ ___ B:3.0\n___ ___");
    }
}