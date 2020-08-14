mod node;
#[macro_use]
mod map;
#[cfg(test)]
mod tests;

use node::Node;
use node::Colour::Black;
use node::Node::{Internal, Leaf};
use std::fmt::{Debug, Display, Result, Formatter};
use std::cmp::Ordering;

pub struct RBTree<T: PartialOrd> {
    root: Node<T>,
    contained: usize,
    order: Option<Box<Fn(T, T) -> Ordering>>
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
    from_str: String,
    level: usize, 
    levels: &mut Vec<String>
) {
    if levels.len() <= level {
        match cur {
            Internal(n) => levels.push(format!(
                "{}{}:{:?}", from_str, n.colour(), n.value()
            )),
            Leaf(_) => levels.push(format!("{}___", from_str))
        }
    } else {
        match cur {
            Internal(n) => levels[level] += &format!(
                " {}{}:{:?}", from_str, n.colour(), n.value()
            ),
            Leaf(_) => levels[level] += &format!(" {}___", from_str)
        }
    }
    if !cur.is_leaf() {
        write_to_level(
            cur.get_left(), 
            format!("{:?}->", cur.value().unwrap()), 
            level + 1, 
            levels
        );
        write_to_level(
            cur.get_right(), 
            format!("{:?}->", cur.value().unwrap()), 
            level + 1, 
            levels
        );
    }
}

impl<T: PartialOrd + Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.root, "".to_string(), 0, &mut levels);
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

    /// Creates and returns a new RBTree.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(3);
    /// t.insert(2);
    /// assert_eq!(t.remove(&2).unwrap(), 2);
    /// ```
    pub fn new() -> RBTree<T> {
        RBTree {root: Leaf(Black), contained: 0, order: None}
    }

    /// Returns a vector presenting the contained
    /// elements of the RBTree in the order by which
    /// they are prioritised (that is, in the in-order
    /// tree traversal order).
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(2);
    /// let order = t.ordered();
    /// assert_eq!(*order[1], 2);
    /// ```
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::new();
        ordered_insertion(&self.root, &mut order);
        order
    }

    /// Returns the number of elements contained
    /// in the tree.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(2);
    /// assert_eq!(t.len(), 3);
    /// t.remove(&2);
    /// assert_eq!(t.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.contained
    }

    /// Returns true if there are no items
    /// present in the tree, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// assert!(t.is_empty());
    /// t.insert(3);
    /// assert!(!t.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            true
        } else {
            false
        }
    }

    /// Inserts a new element into the RBTree.
    /// Returns None if this item was not already
    /// in the tree, and the previously contained
    /// item otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// assert_eq!(t.insert("Hello".to_string()), None);
    /// assert_eq!(t.insert("Hello".to_string()), Some("Hello".to_string()));
    /// ```
    pub fn insert(&mut self, val: T) -> Option<T> {
        match self.root.insert(val) {
            Some(v) => Some(v),
            None => {
                self.contained += 1;
                None
            }
        }
    }

    /// Returns true if the tree contains the
    /// specified item, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(2);
    /// assert!(!t.contains(&3));
    /// assert!(t.contains(&2));
    /// ```
    pub fn contains(&self, val: &T) -> bool {
        !self.get(val).is_none()
    }

    /// Returns the item specified if contained,
    /// None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(1);
    /// assert_eq!(*t.get(&1).unwrap(), 1);
    /// assert_eq!(t.get(&2), None);
    /// ```
    /// This method can be useful especially in conjunction
    /// with the "make_map" macro(s).
    /// # Example:
    /// ```
    /// # #[macro_use(make_map_named, make_map)]
    /// # extern crate rb_tree;
    /// 
    /// # fn main () {
    /// use rb_tree::RBTree;
    /// 
    /// make_map_named!(MyMap{String, usize});
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(MyMap::new("Hello".to_string(), 0));
    /// t.insert(MyMap::new("World".to_string(), 1));
    /// assert_eq!(t.get(&"World".to_string()).unwrap().val, 1);
    /// # }
    /// 
    /// ```
    pub fn get<K: PartialOrd<T>>(&self, val: &K) -> Option<&T> {
        self.root.get(val)
    }

    // pub fn at(&self, index: usize) -> Option<&T> {

    // }

    /// Removes an item the tree. Returns the matching item
    /// if it was contained in the tree, None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.remove(&2).unwrap(), 2);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.remove(&2), None);
    /// ```
    pub fn remove<K: PartialOrd<T>>(&mut self, val: &K) -> Option<T> {
        match self.root.remove(val) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            },
            None => None
        }
    }

    /// Removes the item at the front of the priority
    /// queue that the RBTree represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(t.pop().unwrap(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.root.pop(false) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            },
            None => None
        }
    }

    /// Peeks the item at the front of the priority
    /// queue that the RBTree represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(*t.peek().unwrap(), 1);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.root.peek(false)
    }

    /// Removes the item at the back of the priority
    /// queue that the RBTree represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(t.pop_back().unwrap(), 3);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        match self.root.pop(true) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            },
            None => None
        }
    }

    /// Peeks the item at the back of the priority
    /// queue that the RBTree represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(*t.peek_back().unwrap(), 3);
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        self.root.peek(true)
    }


}
