use crate::RBTree;

use crate::node::Colour::Black;
use crate::node::Node::Leaf;
use std::fmt::{Debug, Display, Result, Formatter};
use crate::helpers::{write_to_level, ordered_insertion};
use std::iter::{ExactSizeIterator, FusedIterator};

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

impl<T: PartialOrd + Debug> Display for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
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
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// ```
    pub fn new() -> RBTree<T> {
        RBTree {root: Leaf(Black), contained: 0}
    }

    /// Clears all entries from the tree.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut tree = RBTree::new();
    /// tree.insert(2);
    /// tree.insert(5);
    /// tree.clear();
    /// assert_eq!(tree.len(), 0);
    /// assert!(!tree.contains(&2));
    /// ```
    pub fn clear(&mut self) {
        self.root = Leaf(Black);
        self.contained = 0;
    }

    /// Clears the tree and returns all values
    /// as an iterator in their PartialOrd order.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut tree = RBTree::new();
    /// tree.insert(2);
    /// tree.insert(5);
    /// assert_eq!(tree.len(), 2);
    /// let mut drain = tree.drain();
    /// assert_eq!(drain.next().unwrap(), 2);
    /// assert_eq!(drain.next().unwrap(), 5);
    /// assert!(drain.next().is_none());
    /// assert_eq!(tree.len(), 0);
    /// ```
    pub fn drain(&mut self) -> Drain<T> {
        let mut rep = RBTree::new();
        std::mem::swap(&mut rep, self);
        Drain {tree: rep}
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
    /// Returns true if this item was not already
    /// in the tree, and false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// assert_eq!(t.insert("Hello".to_string()), true);
    /// assert_eq!(t.insert("Hello".to_string()), false);
    /// ```
    pub fn insert(&mut self, val: T) -> bool {
        match self.root.insert(val) {
            Some(_) => false,
            None => {
                self.contained += 1;
                true
            }
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
    /// assert_eq!(t.replace("Hello".to_string()), None);
    /// assert_eq!(t.replace("Hello".to_string()), Some("Hello".to_string()));
    /// ```
    pub fn replace(&mut self, val: T) -> Option<T> {
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
    pub fn get<K: PartialOrd<T>>(&self, val: &K) -> Option<&T> {
        self.root.get(val)
    }

    pub(crate) fn get_mut<K: PartialOrd<T>>(&mut self, val: &K) -> Option<&mut T> {
        self.root.get_mut(val)
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
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.take(&2), None);
    /// ```
    pub fn take<K: PartialOrd<T>>(&mut self, val: &K) -> Option<T> {
        match self.root.remove(val) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            },
            None => None
        }
    }

    /// Removes an item the tree. Returns true
    /// if it was contained in the tree, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.remove(&2), true);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.remove(&2), false);
    /// ```
    pub fn remove<K: PartialOrd<T>>(&mut self, val: &K) -> bool {
        match self.root.remove(val) {
            Some(_) => {
                self.contained -= 1;
                true
            },
            None => false
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

pub struct Drain<T: PartialOrd> {
    tree: RBTree<T>
}

impl<T: PartialOrd> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.tree.pop()
    }
}

impl<T: PartialOrd> ExactSizeIterator for Drain<T> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<T: PartialOrd> FusedIterator for Drain<T> {}