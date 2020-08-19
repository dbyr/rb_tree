use rb_tree::RBTree;

// uses an rbtree to sort data
fn sort<T: PartialOrd>(to_order: Vec<T>) -> Vec<T> {
    let mut tree: RBTree<T> = to_order.into_iter().collect();
    let mut ordered = Vec::new();
    while let Some(v) = tree.pop() {
        ordered.push(v);
    }
    ordered
}

fn main() {
    let eg1 = vec!(3, 6, 1, 2, 0, 4, -1, 5, 10, 11, -13);
    assert_eq!(sort(eg1), vec!(-13, -1, 0, 1, 2, 3, 4, 5, 6, 10, 11));

    let eg2 = vec!("Is", "this", "the", "real", "life");
    assert_eq!(sort(eg2), vec!("Is", "life", "real", "the", "this"))
}