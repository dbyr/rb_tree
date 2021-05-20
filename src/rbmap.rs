use crate::helpers::write_to_level;
use crate::mapper::Mapper;
use crate::rbtree;
use crate::{RBMap, RBTree};

use std::fmt::{Debug, Display, Formatter, Result};
use std::iter::{ExactSizeIterator, FromIterator, FusedIterator};

impl<K: PartialOrd + Debug, V: Debug> Debug for RBMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.map.root, "".to_string(), 0, &mut levels);
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

impl<K: PartialOrd + Debug, V: Debug> Display for RBMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

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
        RBMap { map: RBTree::new() }
    }

    /// Creates an RBTree set of the keys
    /// contained in this map.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMap, RBTree};
    ///
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// map.insert("Foo", "Bar");
    /// let kset = map.keyset();
    /// assert!(kset.contains(&&"Hello"));
    /// assert!(kset.contains(&&"Foo"));
    /// assert!(!kset.contains(&&"Bar"));
    /// ```
    pub fn keyset(&self) -> RBTree<&K> {
        let mut keys = RBTree::new();
        for key in self.keys() {
            keys.insert(key);
        }
        keys
    }

    /// Creates a set from the keys in this
    /// map.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMap, RBTree};
    ///
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// map.insert("Foo", "Bar");
    /// let kset = map.into_keyset();
    /// assert!(kset.contains(&"Hello"));
    /// assert!(kset.contains(&"Foo"));
    /// assert!(!kset.contains(&"Bar"));
    /// ```
    pub fn into_keyset(self) -> RBTree<K> {
        let mut kset = RBTree::new();
        for (key, _) in self.into_iter() {
            kset.insert(key);
        }
        kset
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
        match self.map.get(&Mapper::new(key, None)) {
            None => false,
            Some(v) => v.is_some(),
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
        Drain { tree: rep }
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
        self.map.get(&Mapper::new(key, None)).map(|v| v.as_ref())
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
    /// assert_eq!(map.get_pair(&"Hello").unwrap(), (&"Hello", &"world"));
    /// ```
    pub fn get_pair(&self, key: &K) -> Option<(&K, &V)> {
        self.map
            .get(&Mapper::new(key, None))
            .map(|v| (v.key(), v.as_ref()))
    }

    /// Returns an option containing a reference
    /// to the key-value pair associated with this
    /// key of which the value is mutable.
    /// Returns none if this key does not have an
    /// associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert!(map.get(&"Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get_pair(&"Hello").unwrap(), (&"Hello", &"world"));
    /// ```
    pub fn get_pair_mut(&mut self, key: &K) -> Option<(&K, &mut V)> {
        self.map
            .get_mut(&Mapper::new(key, None))
            .map(|v| v.mut_pair())
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
        self.map
            .get_mut(&Mapper::new(key, None))
            .map(|v| v.as_mut())
    }

    /// Returns an option containing a reference to the
    /// value associated with the key that has the smallest
    /// `PartialOrd` value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.peek(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.peek().unwrap(), &"World");
    /// ```
    pub fn peek(&self) -> Option<&V> {
        self.map.peek().map(|v| v.as_ref())
    }

    /// Returns an option containing a reference to the
    /// value associated with the key that has the largest
    /// `PartialOrd` value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.peek_back(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.peek_back().unwrap(), &"Foo");
    /// ```
    pub fn peek_back(&self) -> Option<&V> {
        self.map.peek_back().map(|v| v.as_ref())
    }

    /// Returns an option containing a pair with a reference to the
    /// key with the smallest `PartialOrd` value and a reference
    /// to its associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.peek_pair(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.peek_pair().unwrap(), (&2, &"World"));
    /// ```
    pub fn peek_pair(&self) -> Option<(&K, &V)> {
        self.map.peek().map(|v| v.pair())
    }

    /// Returns an option containing a pair with a reference to the
    /// key with the largest `PartialOrd` value and a reference
    /// to its associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.peek_pair_back(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.peek_pair_back().unwrap(), (&7, &"Foo"));
    /// ```
    pub fn peek_pair_back(&self) -> Option<(&K, &V)> {
        self.map.peek_back().map(|v| v.pair())
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
        self.map
            .replace(Mapper::new(key, Some(val)))
            .map(|v| v.consume())
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
        self.map
            .take(&Mapper::new(key, None))
            .map(|v| v.consume().1)
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
        self.map.take(&Mapper::new(key, None)).map(|v| v.consume())
    }

    /// Removes the pair associated with the key that has the smallest
    /// `PartialOrd` value and returns the associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.pop(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.pop().unwrap(), "World");
    /// assert_eq!(map.pop().unwrap(), "Hello");
    /// ```
    pub fn pop(&mut self) -> Option<V> {
        self.map.pop().map(|v| v.consume().1)
    }

    /// Removes the pair associated with the key that has the largest
    /// `PartialOrd` value and returns the associated value.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.pop(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.pop_back().unwrap(), "Foo");
    /// assert_eq!(map.pop_back().unwrap(), "Bar");
    /// ```
    pub fn pop_back(&mut self) -> Option<V> {
        self.map.pop_back().map(|v| v.consume().1)
    }

    /// Removes the pair associated with the key that has the smallest
    /// `PartialOrd` value and returns it.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.pop_pair(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.pop_pair().unwrap(), (2, "World"));
    /// assert_eq!(map.pop_pair().unwrap(), (5, "Hello"));
    /// ```
    pub fn pop_pair(&mut self) -> Option<(K, V)> {
        self.map.pop().map(|v| v.consume())
    }

    /// Removes the pair associated with the key that has the smallest
    /// `PartialOrd` value and returns it.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// assert_eq!(map.pop_pair_back(), None);
    ///
    /// map.insert(5, "Hello");
    /// map.insert(2, "World");
    /// map.insert(7, "Foo");
    /// map.insert(6, "Bar");
    ///
    /// assert_eq!(map.pop_pair_back().unwrap(), (7, "Foo"));
    /// assert_eq!(map.pop_pair_back().unwrap(), (6, "Bar"));
    /// ```
    pub fn pop_pair_back(&mut self) -> Option<(K, V)> {
        self.map.pop_back().map(|v| v.consume())
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
    /// map.retain(|_, v| *v % 2 == 0);
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

    /// An iterator that visits all key-value
    /// pairs in their key's partialord order.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// let mut pairs = map.iter();
    /// assert_eq!(pairs.next().unwrap(), (&1, &1));
    /// assert_eq!(pairs.next().unwrap(), (&2, &4));
    /// assert_eq!(pairs.next().unwrap(), (&3, &9));
    /// assert_eq!(pairs.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            pos: 0,
            ordered: self.ordered(),
        }
    }

    /// An iterator that visits all key-value
    /// pairs in their key's partialord order
    /// and presents the value only as mutable.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// map.iter_mut().for_each(|(_, v)| *v *= 2);
    ///
    /// let mut pairs = map.iter();
    /// assert_eq!(pairs.next().unwrap(), (&1, &2));
    /// assert_eq!(pairs.next().unwrap(), (&2, &8));
    /// assert_eq!(pairs.next().unwrap(), (&3, &18));
    /// assert_eq!(pairs.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            iter: self.map.iter(),
        }
    }

    /// An iterator that visits all values
    /// in their key's partialord order.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// let mut vals = map.values();
    /// assert_eq!(*vals.next().unwrap(), 1);
    /// assert_eq!(*vals.next().unwrap(), 4);
    /// assert_eq!(*vals.next().unwrap(), 9);
    /// assert_eq!(vals.next(), None);
    /// ```
    pub fn values(&self) -> Values<K, V> {
        Values {
            pos: 0,
            ordered: self.ordered(),
        }
    }

    /// An iterator that visits all values
    /// in their key's partialord order
    /// and presents them as mutable.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// map.values_mut().for_each(|v| *v *= 2);
    ///
    /// let mut vals = map.values();
    /// assert_eq!(*vals.next().unwrap(), 2);
    /// assert_eq!(*vals.next().unwrap(), 8);
    /// assert_eq!(*vals.next().unwrap(), 18);
    /// assert_eq!(vals.next(), None);
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            iter: self.iter_mut(),
        }
    }

    /// An iterator that visits all keys
    /// in their partialord order.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// let mut keys = map.keys();
    /// assert_eq!(*keys.next().unwrap(), 1);
    /// assert_eq!(*keys.next().unwrap(), 2);
    /// assert_eq!(*keys.next().unwrap(), 3);
    /// assert_eq!(keys.next(), None);
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        Keys {
            pos: 0,
            ordered: self.ordered(),
        }
    }

    /// Provides an interface for ensuring values
    /// are allocated to the given key.
    /// # Example:
    /// ```
    /// use rb_tree::RBMap;
    ///
    /// let mut map = RBMap::new();
    ///
    /// let val = map.entry(1).or_insert(2);
    /// *val = 3;
    /// assert_eq!(*map.get(&1).unwrap(), 3);
    /// ```
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        Entry { map: self, key }
    }

    // internal helper methods
    fn ordered(&self) -> Vec<(&K, &V)> {
        self.map.iter().map(|m| (m.key(), m.as_ref())).collect()
    }
}

impl<K: PartialOrd, V: PartialOrd> RBMap<K, V> {
    /// Creates an RBTree set of the values
    /// contained in this map.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMap, RBTree};
    ///
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// map.insert("Foo", "Bar");
    /// let vset = map.valueset();
    /// assert!(vset.contains(&&"World"));
    /// assert!(vset.contains(&&"Bar"));
    /// assert!(!vset.contains(&&"Foo"));
    /// ```
    pub fn valueset(&self) -> RBTree<&V> {
        let mut values = RBTree::new();
        for value in self.values() {
            values.insert(value);
        }
        values
    }

    /// Creates a set of keys and a set of values
    /// from the given map.
    ///
    /// Note: any mapping information is lost
    /// when this operation is performed.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMap, RBTree};
    ///
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// map.insert("Foo", "Bar");
    /// let (kset, vset) = map.into_sets();
    /// assert!(kset.contains(&"Hello"));
    /// assert!(kset.contains(&"Foo"));
    /// assert!(!kset.contains(&"Bar"));
    /// assert!(vset.contains(&"World"));
    /// assert!(vset.contains(&"Bar"));
    /// assert!(!vset.contains(&"Foo"));
    /// ```
    pub fn into_sets(self) -> (RBTree<K>, RBTree<V>) {
        let mut kset = RBTree::new();
        let mut vset = RBTree::new();
        for (key, value) in self.into_iter() {
            kset.insert(key);
            vset.insert(value);
        }
        (kset, vset)
    }

    /// Creates an RBTree set from the values
    /// contained in this map.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMap, RBTree};
    ///
    /// let mut map = RBMap::new();
    /// map.insert("Hello", "World");
    /// map.insert("Foo", "Bar");
    /// let vset = map.into_valueset();
    /// assert!(vset.contains(&"World"));
    /// assert!(vset.contains(&"Bar"));
    /// assert!(!vset.contains(&"Foo"));
    /// ```
    pub fn into_valueset(self) -> RBTree<V> {
        let mut vset = RBTree::new();
        for (_, value) in self.into_iter() {
            vset.insert(value);
        }
        vset
    }
}

impl<K: PartialOrd, V> Default for RBMap<K, V> {
    fn default() -> Self {
        RBMap::new()
    }
}

pub struct IntoIter<K: PartialOrd, V> {
    tree: RBTree<Mapper<K, V>>,
}

impl<K: PartialOrd, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.tree.pop().map(|v| v.consume())
    }
}

/// Provides the trait ExactSizeIterator for IntoIter<K, V>
/// # Example:
/// ```
/// use rb_tree::RBMap;
/// use std::iter::FusedIterator;
///
/// let mut map = RBMap::new();
///
/// map.insert(5, "Is");
/// map.insert(2, "This");
/// map.insert(7, "The");
/// map.insert(6, "Real");
/// map.insert(6, "World");
///
/// let mut iterator = map.into_iter();
/// assert_eq!(iterator.len(), 4);
/// let _ = iterator.next();
/// assert_eq!(iterator.len(), 3);
/// ```
impl<K: PartialOrd, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<K: PartialOrd, V> FusedIterator for IntoIter<K, V> {}

impl<K: PartialOrd, V> IntoIterator for RBMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter { tree: self.map }
    }
}

impl<K: PartialOrd, V> FromIterator<(K, V)> for RBMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = RBMap::new();
        for (key, val) in iter {
            map.insert(key, val);
        }
        map
    }
}

impl<K: PartialOrd, V> Extend<(K, V)> for RBMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, val) in iter {
            self.insert(key, val);
        }
    }
}

impl<'a, K: PartialOrd + Copy + 'a, V: Copy + 'a> Extend<(&'a K, &'a V)> for RBMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        for (&key, &val) in iter {
            self.insert(key, val);
        }
    }
}

// this should be fine to do since only one
// borrow can occur when mutable
pub struct Iter<'a, K: PartialOrd, V> {
    pos: usize,
    ordered: Vec<(&'a K, &'a V)>,
}

impl<'a, K: PartialOrd, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(*v)
            }
            None => None,
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
    ordered: Vec<(&'a K, &'a V)>,
}

impl<'a, K: PartialOrd, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(v.0)
            }
            None => None,
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
    ordered: Vec<(&'a K, &'a V)>,
}

impl<'a, K: PartialOrd, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(v.1)
            }
            None => None,
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.ordered.len() - self.pos
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for Values<'a, K, V> {}

pub struct ValuesMut<'a, K: PartialOrd, V> {
    iter: IterMut<'a, K, V>,
}

impl<'a, K: PartialOrd, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        match self.iter.next() {
            Some(v) => Some(v.1),
            None => None,
        }
    }
}

impl<'a, K: PartialOrd, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: PartialOrd, V> FusedIterator for ValuesMut<'a, K, V> {}

pub struct IterMut<'a, K: PartialOrd, V> {
    iter: rbtree::Iter<'a, Mapper<K, V>>,
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
            }
            None => None,
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
    tree: RBTree<Mapper<K, V>>,
}

impl<K: PartialOrd, V> Iterator for Drain<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.tree.pop().map(|v| v.consume())
    }
}

impl<K: PartialOrd, V> ExactSizeIterator for Drain<K, V> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<K: PartialOrd, V> FusedIterator for Drain<K, V> {}

pub struct Entry<'a, K: PartialOrd, V> {
    map: &'a mut RBMap<K, V>,
    key: K,
}

/// Follows a similar implementation to std::collections::HashMap,
/// in terms of behaviour, only differs in types used.
/// For further detail about any given method, please refer
/// to the documentation of HashMap::Entry.
/// For the time being only copyable keys can utilise
/// these methods
impl<'a, K: PartialOrd + Copy, V> Entry<'a, K, V> {
    pub fn insert(self, val: V) -> (&'a K, &'a mut V) {
        match self.map.remove_entry(&self.key) {
            Some((k, _)) => {
                self.map.insert(k, val);
            }
            None => {
                self.map.insert(self.key, val);
            }
        }
        self.map.get_pair_mut(&self.key).unwrap()
    }

    pub fn and_modify<F>(self, f: F) -> Entry<'a, K, V>
    where
        F: FnOnce(&mut V),
    {
        if let Some(v) = self.map.get_mut(&self.key).as_mut() {
            f(*v);
        }
        self
    }

    pub fn or_insert(self, default: V) -> &'a mut V {
        if !self.map.contains_key(&self.key) {
            self.map.insert(self.key, default);
        }
        self.map.get_mut(&self.key).unwrap()
    }

    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        if !self.map.contains_key(&self.key) {
            self.map.insert(self.key, default());
        }
        self.map.get_mut(&self.key).unwrap()
    }
}

impl<'a, K: PartialOrd + Copy, V: Default> Entry<'a, K, V> {
    pub fn or_default<F>(self) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        if !self.map.contains_key(&self.key) {
            self.map.insert(self.key, V::default());
        }
        self.map.get_mut(&self.key).unwrap()
    }
}
