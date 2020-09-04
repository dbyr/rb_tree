pub struct Mapper<K: PartialOrd, V> {
    key: K,
    val: Option<V>
}

#[allow(dead_code)]
impl<K: PartialOrd, V> Mapper<K, V> {
    pub fn new(key: K, val: Option<V>) -> Mapper<K, V> {
        Mapper {
            key: key,
            val: val
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn is_none(&self) -> bool {
        self.val.is_none()
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