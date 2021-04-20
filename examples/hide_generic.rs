use rb_tree::RBQueue;

struct ByReference {
    queue: RBQueue<i32, &'static dyn Fn(&i32, &i32) -> std::cmp::Ordering>,
}

impl Default for ByReference {
    fn default() -> Self {
        Self {
            queue: RBQueue::new(&|l: &i32, r: &i32| l.cmp(r)),
        }
    }
}

struct Boxing {
    queue: RBQueue<i32, Box<dyn Fn(&i32, &i32) -> std::cmp::Ordering>>,
}

impl Default for Boxing {
    fn default() -> Self {
        Self {
            queue: RBQueue::new(Box::new(|l: &i32, r: &i32| l.cmp(r))),
        }
    }
}

fn main() {
    let mut h = ByReference::default();
    h.queue.insert(1i32);

    let mut b = Boxing::default();
    b.queue.insert(1i32);
}
