use crate::{RBMap, RBTree};

use crate::rbtree;
use crate::mapper::Mapper;
use std::iter::{ExactSizeIterator, FusedIterator};

impl<K: PartialOrd, V> RBMap<K, V> {

    /// Creates and returns a new, empty RBMap
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// assert_eq!(map.remove(&"Hello").unwrap(), "World");
    /// ```
    pub fn new() -> RBMap<K, V> {
        RBMap {
            map: RBTree::new()
        }
    }
    
    /// Clears all entries from the RBMap
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "world");
    /// map.insert("Foo", "bar");
    /// assert_eq!(map.len(), 2);
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    /// assert!(map.remove(&"Hello").is_none());
    /// ```
    pub fn clear(&mut self) {
        self.map = RBTree::new();
    }

    /// Returns true if the map contains an entry
    /// for key, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(!map.contains_key(&"Hello"));
    /// map.insert("Hello", "world");
    /// assert!(map.contains_key(&"Hello"));
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        match self.map.get(
            &Mapper::new(key, None)
        ) {
            None => false,
            Some(v) => v.is_some()
        }
    }

    /// Clears the map and returns an iterator
    /// over all key-value pairs that were contained
    /// in the order of their keys' PartialOrd order.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "world");
    /// map.insert("Foo", "bar");
    /// let mut drain = map.drain();
    /// assert_eq!(drain.next().unwrap(), ("Foo", "bar"));
    /// assert_eq!(drain.next().unwrap(), ("Hello", "world"));
    /// assert!(drain.next().is_none());
    /// ```
    pub fn drain(&mut self) -> Drain<K, V> {
        let mut rep = RBTree::new();
        std::mem::swap(&mut self.map, &mut rep);
        Drain {tree: rep}
    }

    /// Returns an option containing a reference
    /// to the value associated with this key,
    /// or none if this key does not have an associated
    /// value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.get(&"Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get(&"Hello").unwrap(), &"world");
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.map.get(
            &Mapper::new(key, None)
        ) {
            Some(v) => Some(v.as_ref()),
            None => None
        }
    }

    /// Returns an option containing a reference
    /// to the key-value pair associated with this
    /// key, or none if this key does not have an
    /// associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.get(&"Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get_key_value(&"Hello").unwrap(), (&"Hello", &"world"));
    /// ```
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match self.map.get(
            &Mapper::new(key, None)
        ) {
            Some(v) => Some((v.key(), v.as_ref())),
            None => None
        }
    }

    /// Returns an option containing a mutable
    /// reference to the value associated with this
    /// key, or none if this key does not have an associated
    /// value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.get(&"Hello").is_none());
    /// map.insert("Hello", "world");
    /// *map.get_mut(&"Hello").unwrap() = "world!";
    /// assert_eq!(map.get(&"Hello").unwrap(), &"world!");
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.map.get_mut(
            &Mapper::new(key, None)
        ) {
            Some(v) => Some(v.as_mut()),
            None => None
        }
    }

    /// Inserts a value to associate with the given key
    /// into the map, returning the previously-stored key-value
    /// pair if one existed, None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "world");
    /// map.insert("Foo", "bar");
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)> {
        match self.map.replace(
            Mapper::new(key, Some(val))
        ) {
            Some(v) => Some(v.consume()),
            None => None
        }
    }

    /// Returns true if there are no key-value pairs
    /// stored in this RBMap, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.is_empty());
    /// map.insert(1, 2);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.len() == 0
    }
    
    /// Returns the number of key-value pairs stored
    /// in this RBMap.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(1, 1);
    /// assert_eq!(map.len(), 1);
    /// map.insert(2, 4);
    /// assert_eq!(map.len(), 2);
    /// map.remove(&2);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Removes the key-value pair associated with key,
    /// if one exists, and returns the associated value,
    /// or None if the pair did not exist.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.remove(&2).is_none());
    /// map.insert(2, 4);
    /// assert_eq!(map.remove(&2).unwrap(), 4);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.map.take(
            &Mapper::new(key, None)
        ) {
            Some(v) => Some(v.consume().1),
            None => None
        }
    }

    /// Removes the key-value pair associated with key,
    /// if one exists, and returns it, or None if the pair
    /// did not exist.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// assert!(map.remove_entry(&2).is_none());
    /// map.insert(2, 4);
    /// assert_eq!(map.remove_entry(&2).unwrap(), (2, 4));
    /// ```
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        match self.map.take(
            &Mapper::new(key, None)
        ) {
            Some(v) => Some(v.consume()),
            None => None
        }
    }

    /// Removes all key-value pairs that do not return true for the
    /// provided method.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    /// 
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    /// map.retain(|_, v| v % 2 == 0);
    /// 
    /// let mut pairs = map.drain();
    /// assert_eq!(pairs.next().unwrap(), (2, 4));
    /// assert_eq!(pairs.next(), None);
    /// ```
    pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, mut logic: F) {
        let mut rep = RBMap::new();
        for (key, mut val) in self.drain() {
            if logic(&key, &mut val) {
                rep.insert(key, val);
            }
        }
        std::mem::swap(self, &mut rep);
    }

    
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            pos: 0,
            ordered: self.ordered()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            iter: self.map.iter()
        }
    }

    // internal helper methods
    fn ordered(&self) -> Vec<(&K, &V)> {
        self.map.iter().map(|m| (m.key(), m.as_ref())).collect()
    }
}

// this should be fine to do since only one
// borrow can occur when mutable
pub struct Iter<'a, K: PartialOrd, V> {
    pos: usize,
    ordered: Vec<(&'a K, &'a V)>
}

impl<'a, K: PartialOrd, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(*v)
            },
            None => None
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for Iter<'a, K, V> {}

pub struct Keys<'a, K: PartialOrd, V> {
    pos: usize,
    ordered: Vec<(&'a K, &'a V)>
}

impl<'a, K: PartialOrd, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(v.0)
            },
            None => None
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for Keys<'a, K, V> {}

pub struct Values<'a, K: PartialOrd, V> {
    pos: usize,
    ordered: Vec<(&'a K, &'a V)>
}

impl<'a, K: PartialOrd, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(v.1)
            },
            None => None
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for Values<'a, K, V> {}

pub struct IterMut<'a, K: PartialOrd, V> {
    iter: rbtree::Iter<'a, Mapper<K, V>>
}

impl<'a, K: PartialOrd, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        let next = self.iter.next();
        match next {
            Some(iv) => {
                let v = unsafe {
                    let ptr = iv as *const Mapper<K, V>;
                    &mut *(ptr as *mut Mapper<K, V>)
                };
                Some(v.mut_pair())
            },
            None => None
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for IterMut<'a, K, V> {}

pub struct Drain<K: PartialOrd, V> {
    tree: RBTree<Mapper<K, V>>
}

impl<K: PartialOrd, V> Iterator for Drain<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        let next = self.tree.pop();
        match next {
            Some(v) => Some(v.consume()),
            None => None
        }
    }
}

impl<K: PartialOrd, V> ExactSizeIterator for Drain<K, V> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<K: PartialOrd, V> FusedIterator for Drain<K, V> {}