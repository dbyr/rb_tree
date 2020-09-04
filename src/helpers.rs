use std::fmt::Debug;
use crate::node::Node;
use crate::node::Node::{Internal, Leaf};

pub fn write_to_level<T: PartialOrd + Debug>(
    cur: &Node<T>, 
    from_str: String,
    level: usize, 
    levels: &mut Vec<String>
) {
    if levels.len() <= level {
        match cur {
            Internal(n) => levels.push(format!(
                "{}{}:{:?}", from_str, n.colour(), n.value()
            )),
            Leaf(_) => levels.push(format!("{}___", from_str))
        }
    } else {
        match cur {
            Internal(n) => levels[level] += &format!(
                " {}{}:{:?}", from_str, n.colour(), n.value()
            ),
            Leaf(_) => levels[level] += &format!(" {}___", from_str)
        }
    }
    if !cur.is_leaf() {
        write_to_level(
            cur.get_left(), 
            format!("{:?}->", cur.value().unwrap()), 
            level + 1, 
            levels
        );
        write_to_level(
            cur.get_right(), 
            format!("{:?}->", cur.value().unwrap()), 
            level + 1, 
            levels
        );
    }
}

pub fn ordered_insertion<'a, T: PartialOrd>(cur: &'a Node<T>, order: &mut Vec<&'a T>) {
    if cur.is_leaf() {
        return;
    }
    ordered_insertion(cur.get_left(), order);
    if let Some(v) = cur.value() {
        order.push(v);
    }
    ordered_insertion(cur.get_right(), order);
}

// pub (crate) fn ordered_insertion_mut<'a, T: PartialOrd>(cur: &'a mut Node<T>, order: &mut Vec<&'a mut T>) {
//     if cur.is_leaf() {
//         return;
//     }
//     ordered_insertion_mut(cur.get_left_mut(), order);
//     if let Some(v) = cur.value_mut() {
//         order.push(v);
//     }
//     ordered_insertion_mut(cur.get_right_mut(), order);
// }