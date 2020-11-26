#[macro_use(rbqueue_c_new)]
extern crate rb_tree;

use rb_tree::RBQueue;

fn main() {
    
    // use the default comarator
    let mut q1 = RBQueue::new(|l: &i64, r| {
        l.cmp(r)
    });

    // compare in the reverse order
    let mut q2 = rbqueue_c_new!(|l: &i64, r| (r - l));

    q1.insert(1);
    q1.insert(2);
    q1.insert(3);
    q2.insert(1);
    q2.insert(2);
    q2.insert(3);

    assert_eq!(q1.ordered(), [&1, &2, &3]);
    assert_eq!(q2.ordered(), [&3, &2, &1]);
}