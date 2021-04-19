extern crate rand;
extern crate rand_chacha;

use crate::RBMap;

use fnv::FnvHashSet;
use rand::{Rng, SeedableRng};

#[test]
fn test_complex_tree_use() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(38);
    let mut q = RBMap::<u32, u32>::new();
    let mut in_q = FnvHashSet::<u32>::default();
    let mut to_add = FnvHashSet::<u32>::default();
    let mut to_del = FnvHashSet::<u32>::default();
    let max_size = 14;
    let min_size = 7;
    for _ in 0..100000 {
        to_add.clear();
        to_del.clear();

        loop {
            let key = rng.gen::<u32>();
            // only add keys not in q
            if in_q.contains(&key) {
                continue;
            }
            to_add.insert(key);
            if to_add.len() >= 5 || to_add.len() + q.len() > max_size {
                break;
            }
        }
        loop {
            if q.len() - to_del.len() == 0 {
                break;
            }
            // only delete keys found in q
            let key = *in_q.iter().nth(rng.gen_range(0..in_q.len())).unwrap();
            to_del.insert(key);
            if to_del.len() >= 5 || q.len() - to_del.len() < min_size {
                break;
            }
        }

        for key in to_add.iter() {
            if let Some(_) = q.insert(*key, *key) {
                panic!();
            }
            in_q.insert(*key);
        }

        for key in to_del.iter() {
            if let Some(deleted) = q.remove_entry(key) {
                in_q.remove(key);
                if deleted.0 != *key || deleted.1 != *key {
                    panic!();
                }
            } else {
                panic!();
            }
        }

        for key in in_q.iter() {
            if !q.contains_key(key) {
                panic!();
            } else {
                let value = q.get(key).unwrap();
                if value != key {
                    panic!();
                }
            }
        }

        if in_q.len() != q.len() {
            panic!();
        }
    }
}
