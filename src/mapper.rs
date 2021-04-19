use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct Mapper<K: PartialOrd, V> {
    key: K,
    val: Option<V>
}

impl<K: PartialOrd, V> Mapper<K, V> {
    pub fn new(key: K, val: Option<V>) -> Mapper<K, V> {
        Mapper {
            key,
            val
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn is_some(&self) -> bool {
        self.val.is_some()
    }

    pub fn as_ref(&self) -> &V {
        self.val.as_ref().unwrap()
    }

    pub fn as_mut(&mut self) -> &mut V {
        self.val.as_mut().unwrap()
    }

    pub fn consume(self) -> (K, V) {
        (self.key, self.val.unwrap())
    }

    pub fn pair(&self) -> (&K, &V) {
        (&self.key, self.val.as_ref().unwrap())
    }

    pub fn mut_pair(&mut self) -> (&K, &mut V) {
        (&self.key, self.val.as_mut().unwrap())
    }
}

impl<K: PartialOrd + Debug, V: Debug> Debug for Mapper<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{:?}: {:?}]", self.key, self.val)
    }
}

impl<K: PartialOrd, V> PartialEq for Mapper<K, V> {
    fn eq(&self, other: &Mapper<K, V>) -> bool {
        self.key == other.key
    }
}

impl<K: PartialOrd, V> PartialOrd for Mapper<K, V> {
    fn partial_cmp(&self, other: &Mapper<K, V>) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: PartialOrd, V> PartialEq<Mapper<K, V>> for Mapper<&K, V> {
    fn eq(&self, other: &Mapper<K, V>) -> bool {
        *self.key == other.key
    }
}

impl<K: PartialOrd, V> PartialOrd<Mapper<K, V>> for Mapper<&K, V> {
    fn partial_cmp(&self, other: &Mapper<K, V>) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
