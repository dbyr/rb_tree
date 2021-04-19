use crate::node::Node;
use crate::node::Node::{Internal, Leaf};
use std::fmt::Debug;

pub fn write_to_level<T: Debug>(
    cur: &Node<T>,
    from_str: String,
    level: usize,
    levels: &mut Vec<String>,
) {
    if levels.len() <= level {
        match cur {
            Internal(n) => levels.push(format!("{}{}:{:?}", from_str, n.colour(), n.value())),
            Leaf(_) => levels.push(format!("{}___", from_str)),
        }
    } else {
        match cur {
            Internal(n) => levels[level] += &format!(" {}{}:{:?}", from_str, n.colour(), n.value()),
            Leaf(_) => levels[level] += &format!(" {}___", from_str),
        }
    }
    if !cur.is_leaf() {
        write_to_level(
            cur.get_left(),
            format!("{:?}->", cur.value().unwrap()),
            level + 1,
            levels,
        );
        write_to_level(
            cur.get_right(),
            format!("{:?}->", cur.value().unwrap()),
            level + 1,
            levels,
        );
    }
}

pub fn ordered_insertion<'a, T>(cur: &'a Node<T>, order: &mut Vec<&'a T>) {
    if cur.is_leaf() {
        return;
    }
    ordered_insertion(cur.get_left(), order);
    if let Some(v) = cur.value() {
        order.push(v);
    }
    ordered_insertion(cur.get_right(), order);
}

// inserts into stack start and all left children
// of start down to the leaf
pub fn insert_left_down<'a, T>(start: &'a Node<T>, stack: &mut Vec<&'a Node<T>>) {
    let mut cur = start;
    while !cur.is_leaf() {
        stack.push(cur);
        cur = cur.get_left();
    }
}
