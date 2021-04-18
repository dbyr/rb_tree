#[macro_use(new_c_queue)]
extern crate rb_tree;

use rb_tree::RBQueue;

fn from_origin(p: &(f64, f64)) -> f64 {
    ((p.0 * p.0) + (p.1 * p.1)).sqrt()
}

fn main() {
    
    // orders values by their distance from the origin
    // or absolute size of x value on equal distance
    let mut q = new_c_queue!(|l: &(f64, f64), r| {
        let l_dist = from_origin(l);
        let r_dist = from_origin(r);
        if l_dist == r_dist {
            l.0 - r.0
        } else {
            l_dist - r_dist
        }
    });

    q.insert((0.0, 0.0));
    q.insert((-5.0, 0.0));
    q.insert((2.0, 3.0));
    q.insert((3.0, 2.0));

    assert_eq!(q.ordered(), [&(0.0, 0.0), &(2.0, 3.0), &(3.0, 2.0), &(-5.0, 0.0)]);
}