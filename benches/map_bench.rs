extern crate rand;
extern crate rand_chacha;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use fnv::FnvHashSet;
use rand::{Rng, SeedableRng};
use rb_tree::RBMap;

const SIZE: usize = 5000;

/// Bench test adding 'random' numbers (same sequence every time)
/// and then removing them in the reverse order of insertion.
#[cfg(feature = "map")]
#[cfg(test)]
fn map_random(c: &mut Criterion) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    let mut picked_values = FnvHashSet::<usize>::default();

    let mut values = Vec::<usize>::with_capacity(SIZE);

    while values.len() < SIZE {
        let value = rng.gen_range(0_..(SIZE * 2));
        if picked_values.contains(&value) {
            continue;
        } else {
            picked_values.insert(value);
            values.push(value);
        }
    }
    drop(picked_values);
    // wonder why to_owned() doesn't work here
    let values_reverse: Vec<usize> = values.iter().rev().map(|x| *x).collect();

    c.bench_function("map_random", |b| {
        b.iter({
            || {
                let mut q = RBMap::<usize, usize>::new();
                for v in values.iter() {
                    q.insert(*v, v + 1);
                }
                for v in values_reverse.iter() {
                    q.remove(v);
                }
            }
        })
    });
}

/// Bench test adding numbers in sorted order
/// and then removing them in the reverse order of insertion.
#[cfg(feature = "map")]
#[cfg(test)]
fn map_in_order(c: &mut Criterion) {
    c.bench_function("map_in_order", |b| {
        b.iter({
            || {
                let mut q = RBMap::<usize, usize>::new();
                for v in 0..=SIZE {
                    q.insert(v, v + 1);
                }
                for v in SIZE..0 {
                    let _ = q.remove(&v);
                }
            }
        })
    });
}

#[cfg(feature = "map")]
criterion_group!(map_benches, map_in_order, map_random);
#[cfg(feature = "map")]
criterion_main!(map_benches);
