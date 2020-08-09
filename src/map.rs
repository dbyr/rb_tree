#[macro_export]
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
/// with RBTree in order to 
macro_rules! make_map {
    ($key:ty, $val:ty) => {
        make_map_named!(Mapper{$key, $val});
    };
}