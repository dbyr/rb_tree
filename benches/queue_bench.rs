extern crate rand;
extern crate rand_chacha;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use fnv::FnvHashSet;
use rand::{Rng, SeedableRng};
use rb_tree::RBQueue;

const SIZE: usize = 5000;

/// Bench test adding 'random' numbers (same sequence every time)
/// and then popping them all.
#[cfg(feature = "queue")]
#[cfg(test)]
fn queue_random(c: &mut Criterion) {
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

    c.bench_function("queue_random", |b| {
        b.iter({
            || {
                let mut q = RBQueue::new(|l: &usize, r| l.cmp(r));
                for v in values.iter() {
                    q.insert(*v);
                }
                while !q.is_empty() {
                    let _ = q.pop();
                }
            }
        })
    });
}

/// Bench test adding numbers in sorted order and then popping them all.
#[cfg(feature = "queue")]
#[cfg(test)]
fn queue_in_order(c: &mut Criterion) {
    c.bench_function("queue_in_order", |b| {
        b.iter({
            || {
                let mut q = RBQueue::new(|l: &usize, r| l.cmp(r));
                for v in 0..=SIZE {
                    q.insert(v);
                }
                while !q.is_empty() {
                    let _ = q.pop();
                }
            }
        })
    });
}

#[cfg(feature = "queue")]
criterion_group!(queue_benches, queue_in_order, queue_random);
#[cfg(feature = "queue")]
criterion_main!(queue_benches);
