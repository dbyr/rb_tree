use crate::RBQueue;

use crate::node::Colour::Black;
use crate::node::Node::Leaf;
use std::fmt::{Debug, Display, Result, Formatter};
use crate::helpers::{write_to_level, ordered_insertion};
use std::iter::{ExactSizeIterator, FusedIterator};

impl<T: Debug, P> Debug for RBQueue<T, P> 
where P: Fn(&T, &T) -> std::cmp::Ordering {
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

impl<T: Debug, P> Display for RBQueue<T, P>
where P: Fn(&T, &T) -> std::cmp::Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

impl<T, P> RBQueue<T, P>
where P: Fn(&T, &T) -> std::cmp::Ordering {

    /// Creates and returns a new RBQueue.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(3);
    /// t.insert(2);
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// ```
    pub fn new(cmp: P) -> RBQueue<T, P> {
        RBQueue {
            root: Leaf(Black),
            contained: 0,
            cmp: cmp
        }
    }

    /// Clears all entries from the tree.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut tree = RBQueue::new();
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
    /// use rb_tree::RBQueue;
    /// 
    /// let mut tree = RBQueue::new();
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
        let mut vec = Vec::with_capacity(self.len());
        while let Some(v) = self.pop_back() {
            vec.push(v);
        }
        Drain {
            ordered: vec
        }
    }

    /// Returns a vector presenting the contained
    /// elements of the RBQueue in the order by which
    /// they are prioritised (that is, in the in-order
    /// tree traversal order).
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(2);
    /// let order = t.ordered();
    /// assert_eq!(*order[1], 2);
    /// ```
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::with_capacity(self.len());
        ordered_insertion(&self.root, &mut order);
        order
    }

    /// Returns the number of elements contained
    /// in the tree.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
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
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
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

    /// Inserts a new element into the RBQueue.
    /// Returns true if this item was not already
    /// in the tree, and false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// assert_eq!(t.insert("Hello".to_string()), true);
    /// assert_eq!(t.insert("Hello".to_string()), false);
    /// ```
    pub fn insert(&mut self, val: T) -> bool {
        match self.root.insert(val, &self.cmp) {
            Some(_) => false,
            None => {
                self.contained += 1;
                true
            }
        }
    }

    /// Inserts a new element into the RBQueue.
    /// Returns None if this item was not already
    /// in the tree, and the previously contained
    /// item otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// assert_eq!(t.replace("Hello".to_string()), None);
    /// assert_eq!(t.replace("Hello".to_string()), Some("Hello".to_string()));
    /// ```
    pub fn replace(&mut self, val: T) -> Option<T> {
        match self.root.insert(val, &self.cmp) {
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
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
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
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(1);
    /// assert_eq!(*t.get(&1).unwrap(), 1);
    /// assert_eq!(t.get(&2), None);
    /// ```
    pub fn get(&self, val: &T) -> Option<&T> {
        self.root.get(val, &self.cmp)
    }

    // pub fn at(&self, index: usize) -> Option<&T> {

    // }

    /// Removes an item the tree. Returns the matching item
    /// if it was contained in the tree, None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.take(&2), None);
    /// ```
    pub fn take(&mut self, val: &T) -> Option<T> {
        match self.root.remove(val, &self.cmp) {
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
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.remove(&2), true);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.remove(&2), false);
    /// ```
    pub fn remove(&mut self, val: &T) -> bool {
        match self.root.remove(val, &self.cmp) {
            Some(_) => {
                self.contained -= 1;
                true
            },
            None => false
        }
    }

    /// Removes the item at the front of the priority
    /// queue that the RBQueue represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
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
    /// queue that the RBQueue represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(*t.peek().unwrap(), 1);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.root.peek(false)
    }

    /// Removes the item at the back of the priority
    /// queue that the RBQueue represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
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
    /// queue that the RBQueue represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(*t.peek_back().unwrap(), 3);
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        self.root.peek(true)
    }

    /// Returns an iterator over the elements
    /// contained in this RBQueue.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t = RBQueue::new();
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(5);
    /// assert_eq!(t.iter().collect::<Vec<&usize>>(), vec!(&1, &3, &5));
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            pos: 0,
            ordered: self.ordered()
        }
    }

    /// Retains in this RBQueue only those values for which 
    /// the passed closure returns true.
    /// # Example:
    /// ```
    /// use rb_tree::RBQueue;
    /// 
    /// let mut t: RBQueue<usize> = (0..10).collect();
    /// t.retain(|v| v % 2 == 0);
    /// assert_eq!(t.iter().collect::<Vec<&usize>>(), vec!(&0, &2, &4, &6, &8));
    /// ```
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut tmp = Vec::new();
        while let Some(v) = self.pop() {
            if f(&v) {
                tmp.push(v);
            }
        }
        for v in tmp {
            self.insert(v);
        }
    }
}

pub struct IntoIter<T> {
    order: Vec<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.order.pop()
    }
}

impl<T, P> IntoIterator for RBQueue<T, P>
where P: Fn(&T, &T) -> std::cmp::Ordering {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> IntoIter<T> {
        let mut vec = Vec::with_capacity(self.len());
        while let Some(v) = self.pop_back() {
            vec.push(v);
        }
        IntoIter {
            order: vec
        }
    }
}

pub struct Drain<T> {
    ordered: Vec<T>
}

impl<T> Iterator for Drain<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.ordered.pop()
    }
}

impl<T> ExactSizeIterator for Drain<T> {
    fn len(&self) -> usize {
        self.ordered.len()
    }
}

impl<T> FusedIterator for Drain<T> {}

pub struct Iter<'a, T> {
    pos: usize,
    ordered: Vec<&'a T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let ret = self.ordered.get(self.pos);
        match ret {
            Some(v) => {
                self.pos += 1;
                Some(*v)
            },
            None => None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}