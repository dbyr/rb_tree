use crate::RBTree;

use crate::node::Colour::Black;
use crate::node::Node::Leaf;
use std::fmt::{Debug, Display, Result, Formatter};
use crate::helpers::{write_to_level, ordered_insertion};
use std::iter::{ExactSizeIterator, FusedIterator, FromIterator};

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

    // fn ordered_mut(&mut self) -> Vec<&mut T> {
    //     let mut order = Vec::new();
    //     ordered_insertion_mut(&mut self.root, &mut order);
    //     order
    // }

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

    /// Returns an iterator over the elements
    /// contained in this RBTree.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t = RBTree::new();
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

    // pub (crate) fn iter_mut(&mut self) -> IterMut<T> {
    //     IterMut {
    //         pos: 0,
    //         ordered: self.ordered_mut()
    //     }
    // }

    /// Returns an iterator representing the
    /// difference between the items in this RBTree
    /// and those in another RBTree, i.e. the values
    /// in `self` but not in `other`.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..5).for_each(|v| {t2.insert(v);});
    /// assert_eq!(
    ///     t1.difference(&t2).collect::<Vec<&usize>>(),
    ///     vec!(&0, &1)
    /// );
    /// assert_eq!(
    ///     t2.difference(&t1).collect::<Vec<&usize>>(),
    ///     vec!(&3, &4)
    /// );
    /// ```
    pub fn difference<'a>(
        &'a self,
        other: &'a RBTree<T>
    ) -> Difference<'a, T> {
        let mut iterl = self.iter();
        let mut iterr = other.iter();
        Difference {
            nextl: iterl.next(),
            nextr: iterr.next(),
            left: iterl,
            right: iterr
        }
    }

    /// Returns an iterator representing the
    /// symmetric difference between the items
    /// in this RBTree and those in another, i.e.
    /// the values in `self` or `other` but not in both.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..5).for_each(|v| {t2.insert(v);});
    /// assert_eq!(
    ///     t1.symmetric_difference(&t2).collect::<Vec<&usize>>(),
    ///     vec!(&0, &1, &3, &4)
    /// );
    /// assert_eq!(
    ///     t2.symmetric_difference(&t1).collect::<Vec<&usize>>(),
    ///     vec!(&0, &1, &3, &4)
    /// );
    /// ```
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a RBTree<T>
    ) -> SymmetricDifference<'a, T> {
        let mut iterl = self.iter();
        let mut iterr = other.iter();
        SymmetricDifference {
            nextl: iterl.next(),
            nextr: iterr.next(),
            left: iterl,
            right: iterr
        }
    }

    /// Returns an iterator representing the intersection
    /// of this RBTree and another, i.e. the values that
    /// appear in both `self` and `other`.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..5).for_each(|v| {t2.insert(v);});
    /// assert_eq!(
    ///     t1.intersection(&t2).collect::<Vec<&usize>>(),
    ///     vec!(&2)
    /// );
    /// ```
    pub fn intersection<'a>(
        &'a self,
        other: &'a RBTree<T>
    ) -> Intersection<'a, T> {
        let mut iterl = self.iter();
        let mut iterr = other.iter();
        Intersection {
            nextl: iterl.next(),
            nextr: iterr.next(),
            left: iterl,
            right: iterr
        }
    }

    /// Returns an iterator representing the union
    /// of this RBTree and another, i.e. the values
    /// that appear in at least one of the RBTrees.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..5).for_each(|v| {t2.insert(v);});
    /// assert_eq!(
    ///     t1.union(&t2).collect::<Vec<&usize>>(),
    ///     vec!(&0, &1, &2, &3, &4)
    /// );
    /// ```
    pub fn union<'a>(
        &'a self,
        other: &'a RBTree<T>
    ) -> Union<'a, T> {
        let mut iterl = self.iter();
        let mut iterr = other.iter();
        Union {
            nextl: iterl.next(),
            nextr: iterr.next(),
            left: iterl,
            right: iterr
        }
    }

    /// Returns true if this RBTree and another are disjoint,
    /// i.e. there are no values in `self` that appear in `other`
    /// and vice versa, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..5).for_each(|v| {t2.insert(v);});
    /// assert!(!t1.is_disjoint(&t2));
    /// t2.pop(); // remove '2' from t2
    /// assert!(t1.is_disjoint(&t2));
    /// ```
    pub fn is_disjoint(&self, other: &RBTree<T>) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns true if this RBTree is a subset of another,
    /// i.e. at least all values in `self` also appear in
    /// `other`, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// let mut t3 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..10).for_each(|v| {t2.insert(v);});
    /// (3..7).for_each(|v| {t3.insert(v);});
    /// assert!(!t1.is_subset(&t2));
    /// assert!(t3.is_subset(&t2));
    /// ```
    pub fn is_subset(&self, other: &RBTree<T>) -> bool {
        self.intersection(other).collect::<Vec<&T>>().len() == self.len()
    }

    /// Returns true if this RBTree is a superset of another,
    /// i.e. at least all values in `other` also appear in
    /// `self`, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t1 = RBTree::new();
    /// let mut t2 = RBTree::new();
    /// let mut t3 = RBTree::new();
    /// (0..3).for_each(|v| {t1.insert(v);});
    /// (2..10).for_each(|v| {t2.insert(v);});
    /// (3..7).for_each(|v| {t3.insert(v);});
    /// assert!(!t2.is_superset(&t1));
    /// assert!(t2.is_superset(&t3));
    /// ```
    pub fn is_superset(&self, other: &RBTree<T>) -> bool {
        other.intersection(self).collect::<Vec<&T>>().len() == other.len()
    }

    /// Retains in this RBTree only those values for which 
    /// the passed closure returns true.
    /// # Example:
    /// ```
    /// use rb_tree::RBTree;
    /// 
    /// let mut t: RBTree<usize> = (0..10).collect();
    /// t.retain(|v| v % 2 == 0);
    /// assert_eq!(t.iter().collect::<Vec<&usize>>(), vec!(&0, &2, &4, &6, &8));
    /// ```
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut rep = RBTree::new();
        while let Some(v) = self.pop() {
            if f(&v) {
                rep.insert(v);
            }
        }
        std::mem::swap(&mut rep, self);
    }
}

pub struct IntoIter<T: PartialOrd> {
    tree: RBTree<T>
}

impl<T: PartialOrd> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.tree.pop()
    }
}

impl<T: PartialOrd> IntoIterator for RBTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            tree: self
        }
    }
}

impl<T: PartialOrd> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut tree = RBTree::new();
        for i in iter {
            tree.insert(i);
        }
        tree
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

pub struct Iter<'a, T: PartialOrd> {
    pos: usize,
    ordered: Vec<&'a T>
}

impl<'a, T: PartialOrd> Iterator for Iter<'a, T> {
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

impl<'a, T: PartialOrd> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, T: PartialOrd> FusedIterator for Iter<'a, T> {}

// only for use with rbmap
#[allow(dead_code)]
pub (crate) struct IterMut<'a, T: PartialOrd> {
    pos: usize,
    ordered: Vec<&'a mut T>
}

impl<'a, 'b: 'a, T: PartialOrd> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.pos >= self.ordered.len() {
            None
        } else {
            let ret = self.ordered.pop();
            self.pos += 1;
            ret
            // Some(self.ordered[self.pos - 1])
        }
    }
}

impl<'a, T: PartialOrd> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, T: PartialOrd> FusedIterator for IterMut<'a, T> {}

pub struct Difference<'a, T: PartialOrd> {
    nextl: Option<&'a T>,
    nextr: Option<&'a T>,
    left: Iter<'a, T>,
    right: Iter<'a, T>
}

impl<'a, T: PartialOrd> Iterator for Difference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        // select and store the next next
        let mut res = None;
        'left: while let Some(vl) = self.nextl {
            self.nextl = self.left.next();
            'right: while let Some(vr) = self.nextr {
                if vl < vr {
                    res = Some(vl);
                    break 'left;
                } else if vl == vr {
                    self.nextr = self.right.next();
                    continue 'left;
                } else {
                    self.nextr = self.right.next();
                }
            }
            res = Some(vl);
            break; // don't want to skip values
        }

        // return the current next value
        res
    }
}

impl<'a, T: PartialOrd> FusedIterator for Difference<'a, T> {}

pub struct SymmetricDifference<'a, T: PartialOrd> {
    nextl: Option<&'a T>,
    nextr: Option<&'a T>,
    left: Iter<'a, T>,
    right: Iter<'a, T>
}

impl<'a, T: PartialOrd> Iterator for SymmetricDifference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        // select and store the next next
        let mut res = None;
        'left: while let Some(vl) = self.nextl {
            'right: while let Some(vr) = self.nextr {
                if vl < vr {
                    self.nextl = self.left.next();
                    res = Some(vl);
                    break 'left;
                } else if vl == vr {
                    self.nextl = self.left.next();
                    self.nextr = self.right.next();
                    continue 'left;
                } else {
                    self.nextr = self.right.next();
                    res = Some(vr);
                    break 'left;
                }
            }

            // don't want to skip values
            self.nextl = self.left.next();
            res = Some(vl);
            break;
        }
        if res.is_none() {
            res = self.nextr;
            self.nextr = self.right.next();
        }

        // return the current next value
        res
    }
}

impl<'a, T: PartialOrd> FusedIterator for SymmetricDifference<'a, T> {}

pub struct Intersection<'a, T: PartialOrd> {
    nextl: Option<&'a T>,
    nextr: Option<&'a T>,
    left: Iter<'a, T>,
    right: Iter<'a, T>
}

impl<'a, T: PartialOrd> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        // select and store the next next
        let mut res = None;
        'left: while let Some(vl) = self.nextl {
            'right: while let Some(vr) = self.nextr {
                if vl < vr {
                    self.nextl = self.left.next();
                    continue 'left;
                } else if vl == vr {
                    self.nextr = self.right.next();
                    self.nextl = self.left.next();
                    res = Some(vl);
                    break 'left;
                } else {
                    self.nextr = self.right.next();
                }
            }
            break; // don't bother iterating the remaining lefts
        }

        // return the current next value
        res
    }
}

impl<'a, T: PartialOrd> FusedIterator for Intersection<'a, T> {}

pub struct Union<'a, T: PartialOrd> {
    nextl: Option<&'a T>,
    nextr: Option<&'a T>,
    left: Iter<'a, T>,
    right: Iter<'a, T>
}

impl<'a, T: PartialOrd> Iterator for Union<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        // select and store the next next
        let mut res = None;
        'left: while let Some(vl) = self.nextl {
            'right: while let Some(vr) = self.nextr {
                if vl < vr {
                    self.nextl = self.left.next();
                    res = Some(vl);
                    break 'left;
                } else if vl == vr {
                    self.nextr = self.right.next();
                    self.nextl = self.left.next();
                    res = Some(vl);
                    break 'left;
                } else {
                    self.nextr = self.right.next();
                    res = Some(vr);
                    break 'left;
                }
            }
            self.nextl = self.left.next();
            res = Some(vl);
            break; // don't skip values
        }
        if res.is_none() {
            res = self.nextr;
            self.nextr = self.right.next();
        }

        // return the current next value
        res
    }
}

impl<'a, T: PartialOrd> FusedIterator for Union<'a, T> {}