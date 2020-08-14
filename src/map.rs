#[macro_export]
/// Same as "make_map" but allows the user
/// to name the resultant type.
/// # Example:
/// ```
/// # #[macro_use(make_map_named)]
/// # extern crate rb_tree;
/// # fn main() {
/// use rb_tree::RBTree;
/// 
/// make_map_named!(MyMap{usize, usize});
/// let mut t = RBTree::new();
/// t.insert(MyMap::new(1, 1));
/// t.insert(MyMap::new(2, 4));
/// t.insert(MyMap::new(3, 9));
/// 
/// assert_eq!(t.get(&2).unwrap().val, 4usize);
/// assert_eq!(t.pop().unwrap().val, 1usize);
/// assert_eq!(t.remove(&3).unwrap().val, 9usize);
/// # }
/// ```
macro_rules! make_map_named {
    ($name:ident{$key:ty, $val:ty}) => {
        #[derive(Debug)]
        struct $name {
            pub key: $key,
            pub val: $val
        }
        
        impl $name {
            pub fn new(key: $key, val: $val) -> $name {
                $name {
                    key: key,
                    val: val
                }
            }
        }
        
        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.key == other.key
            }
        }
        
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<std::cmp::Ordering> {
                self.key.partial_cmp(&other.key)
            }
        }
        
        impl PartialEq<$key> for $name {
            fn eq(&self, other: &$key) -> bool {
                self.key == *other
            }
        }
        
        impl PartialOrd<$key> for $name {
            fn partial_cmp(&self, other: &$key) -> Option<std::cmp::Ordering> {
                self.key.partial_cmp(other)
            }
        }
        
        impl PartialEq<$name> for $key {
            fn eq(&self, other: &$name) -> bool {
                *self == other.key
            }
        }
        
        impl PartialOrd<$name> for $key {
            fn partial_cmp(&self, other: &$name) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&other.key)
            }
        }
    };
}

#[macro_export]
/// Makes a key-value pair type that can be used
/// with RBTree in order to use the RBTree as a map.
/// Produces a type named "Mapper" with a default 
/// implementation of method "new" for key value
/// pairs.
/// # Example:
/// ```
/// # #[macro_use(make_map, make_map_named)]
/// # extern crate rb_tree;
/// # fn main() {
/// use rb_tree::RBTree;
/// 
/// make_map!(usize, usize);
/// let mut t = RBTree::new();
/// t.insert(Mapper::new(1, 1));
/// t.insert(Mapper::new(2, 4));
/// t.insert(Mapper::new(3, 9));
/// 
/// assert_eq!(t.get(&2).unwrap().val, 4usize);
/// assert_eq!(t.pop().unwrap().val, 1usize);
/// assert_eq!(t.remove(&3).unwrap().val, 9usize);
/// # }
/// ```
macro_rules! make_map {
    ($key:ty, $val:ty) => {
        make_map_named!(Mapper{$key, $val});
    };
}