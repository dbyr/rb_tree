mod node;

use node::Node;
use node::Colour::Black;
use node::Node::{Internal, Leaf};
use std::fmt::{Debug, Display, Result, Formatter};

pub struct RBTree<T: Debug + PartialOrd> {
    root: Node<T>,
    contained: usize
}

fn ordered_insertion<'a, T: Debug + PartialOrd>(cur: &'a Node<T>, order: &mut Vec<&'a T>) {
    if cur.is_leaf() {
        return;
    }
    ordered_insertion(cur.get_left(), order);
    if let Some(v) = cur.value() {
        order.push(v);
    }
    ordered_insertion(cur.get_right(), order);
}

impl<T: PartialOrd + Debug> Display for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

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

impl<T: PartialOrd + Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.root, "".to_string(), 0, &mut levels);
        let mut f_string = "".to_string();
        for i in 0..levels.len() {
            f_string += &levels[i];
            if i != levels.len() - 1 {
                f_string += "\n";
            }
        }
        write!(f, "{}", f_string)
    }
}

impl<T: Debug + PartialOrd> RBTree<T> {
    pub fn new() -> RBTree<T> {
        RBTree {root: Leaf(Black), contained: 0}
    }
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::new();
        ordered_insertion(&self.root, &mut order);
        order
    }

    pub fn len(&self) -> usize {
        self.contained
    }

    pub fn insert(&mut self, val: T) -> Option<T> {
        match self.root.insert(val) {
            Some(v) => Some(v),
            None => {
                self.contained += 1;
                None
            }
        }
    }

    // pub fn contains(&self, val: &T) -> bool {
        
    // }

    // pub fn get(&self, val: &T) -> Option<&T> {

    // }

    // pub fn at(&self, index: usize) -> Option<&T> {

    // }

    pub fn remove(&mut self, val: &T) -> Option<T> {
        match self.root.remove(val) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            },
            None => None
        }
    }

    // pub fn pop(&mut self) -> Option<T> {

    // }

    // pub fn peek(&self) -> Option<&T> {

    // }


}

#[cfg(test)]
mod tests {
    use crate::RBTree;
    use crate::node::Node;
    use crate::node::Colour::*;

    #[test]
    fn test_print() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.insert(1.2);
        println!("{:?}", t);
        assert_eq!(format!("{}", t), "[1.0, 1.2, 2.0, 3.0]");
        assert_eq!(t.len(), 4);
        assert_eq!(format!("{:?}", t), "B:2.0\n2.0->B:1.0 2.0->B:3.0\n1.0->___ 1.0->R:1.2 3.0->___ 3.0->___\n1.2->___ 1.2->___");
    }

    #[test]
    fn test_add_existing() {
        let mut t = RBTree::new();
        assert_eq!(t.insert(2), None);
        assert_eq!(t.insert(2), Some(2));
    }

    // "cases" refer to this document here:
    // https://www.usna.edu/Users/cs/crabbe/SI321/current/red-black/red-black.html
    #[test]
    fn test_case1_left() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(0.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 0.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Black);
    }

    #[test]
    fn test_case1_right() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_left_mut().swap_colour(); // simulate the situation
        t.insert(4.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 3.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 4.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_left().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_left().get_left().colour(), Black);
    }

    #[test]
    fn test_case2_right() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_left_mut().swap_colour(); // simulate the situation
        t.insert(2.5);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 2.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 2.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_left().get_left().colour(), Black);
    }

    #[test]
    fn test_case2_left() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(1.5);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Red);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Black);
    }

    #[test]
    fn test_case3_at_root() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.insert(0.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 2.0);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_left().colour(), Black);
        assert_eq!(*t.root.get_left().get_left().value().unwrap(), 0.0);
        assert_eq!(t.root.get_left().get_left().colour(), Red);
    }

    #[test]
    fn test_case3_not_root() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        t.insert(1.5);
        t.insert(2.5);
        t.insert(4.0);
        t.insert(5.0);
        println!("{:?}", t);
        assert_eq!(*t.root.value().unwrap(), 1.5);
        assert_eq!(t.root.colour(), Black);
        assert_eq!(*t.root.get_left().value().unwrap(), 1.0);
        assert_eq!(t.root.get_right().colour(), Black);
        assert_eq!(*t.root.get_right().value().unwrap(), 2.0);
        assert_eq!(t.root.get_left().colour(), Black);
        assert_eq!(*t.root.get_right().get_right().value().unwrap(), 3.0);
        assert_eq!(t.root.get_right().get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().get_right().value().unwrap(), 4.0);
        assert_eq!(t.root.get_right().get_right().get_right().colour(), Black);
        assert_eq!(*t.root.get_right().get_right().get_right().get_right().value().unwrap(), 5.0);
        assert_eq!(t.root.get_right().get_right().get_right().get_right().colour(), Red);
        assert_eq!(*t.root.get_right().get_right().get_left().value().unwrap(), 2.5);
        assert_eq!(t.root.get_right().get_right().get_left().colour(), Black);
    }

    #[test]
    fn test_insertion_transfer_children() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour(); // simulate the situation
        *t.root.get_left_mut().get_right_mut() = Node::new_black(1.5);
        t.insert(0.0);
        assert_eq!(*t.root.get_right().get_left().value().unwrap(), 1.5);

        // creates a valid rbtree to test the scenario
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(3.0);
        t.insert(1.0);
        t.root.get_right_mut().swap_colour();
        t.root.get_left_mut().swap_colour();
        t.insert(1.5);
        t.root.get_left_mut().get_right_mut().swap_colour();
        t.root.get_left_mut().swap_colour();
        t.insert(1.25);
        t.insert(1.75);
        println!("{:?}", t);

        // now insert the value that should cause the reform
        t.insert(1.125);
        assert_eq!(
            format!("{:?}", t),
            "B:1.5\n1.5->R:1.0 1.5->R:2.0\n\
            1.0->___ 1.0->B:1.25 2.0->B:1.75 2.0->B:3.0\n\
            1.25->R:1.125 1.25->___ 1.75->___ 1.75->___ 3.0->___ 3.0->___\n\
            1.125->___ 1.125->___"
        );
    }

    #[test]
    fn test_complex_insertion() {
        let mut t = RBTree::new();
        t.insert(8);
        t.insert(5);
        t.insert(18);
        assert_eq!(
            format!("{:?}", t),
            "B:8\n\
            8->R:5 8->R:18\n\
            5->___ 5->___ 18->___ 18->___"
        );
        t.insert(15);
        t.insert(17);
        assert_eq!(
            format!("{:?}", t),
            "B:8\n\
            8->B:5 8->B:17\n\
            5->___ 5->___ 17->R:15 17->R:18\n\
            15->___ 15->___ 18->___ 18->___"
        );
        t.insert(25);
        t.insert(40);
        assert_eq!(
            format!("{:?}", t),
            "B:8\n\
            8->B:5 8->R:17\n\
            5->___ 5->___ 17->B:15 17->B:25\n\
            15->___ 15->___ 25->R:18 25->R:40\n\
            18->___ 18->___ 40->___ 40->___"
        );
        t.insert(80);
        assert_eq!(
            format!("{:?}", t),
            "B:17\n\
            17->R:8 17->R:25\n\
            8->B:5 8->B:15 25->B:18 25->B:40\n\
            5->___ 5->___ 15->___ 15->___ 18->___ 18->___ 40->___ 40->R:80\n\
            80->___ 80->___"
        );

        let mut t = RBTree::new();
        t.insert(4);
        t.insert(7);
        t.insert(12);
        t.insert(15);
        t.insert(3);
        t.insert(5);
        t.insert(14);
        t.insert(18);
        t.insert(16);
        t.insert(17);
        assert_eq!(
            format!("{:?}", t),
            "B:14\n\
            14->R:7 14->R:16\n\
            7->B:4 7->B:12 16->B:15 16->B:18\n\
            4->R:3 4->R:5 12->___ 12->___ 15->___ 15->___ 18->R:17 18->___\n\
            3->___ 3->___ 5->___ 5->___ 17->___ 17->___"
        )
    }

    #[test]
    fn test_removal_empty() {
        let mut t = RBTree::new();
        assert!(t.remove(&3.0).is_none());
        assert_eq!(t.contained, 0);
    }

    #[test]
    fn test_removal_notfound() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(1.0);
        t.insert(3.0);
        t.insert(1.5);
        assert!(t.remove(&0.0).is_none());
        assert!(t.remove(&1.2).is_none());
        assert!(t.remove(&1.8).is_none());
        assert!(t.remove(&2.1).is_none());
        assert!(t.remove(&3.9).is_none());
        assert_eq!(t.contained, 4);
        assert_eq!(
            format!("{}", t),
            "[1.0, 1.5, 2.0, 3.0]"
        );
    }

    #[test]
    fn test_remove_only_value() {
        let mut t = RBTree::new();
        t.insert(1);
        assert_eq!(t.len(), 1);
        assert_eq!(t.remove(&1).unwrap(), 1);
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_remove_root() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(1.0);
        t.insert(3.0);
        assert_eq!(t.remove(&2.0).unwrap(), 2.0);
        assert_eq!(
            format!("{:?}", t),
            "B:3.0\n3.0->R:1.0 3.0->___\n1.0->___ 1.0->___"
        );
        assert_eq!(t.remove(&3.0).unwrap(), 3.0);
        assert_eq!(
            format!("{:?}", t),
            "B:1.0\n1.0->___ 1.0->___"
        );

        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(1.0);
        t.insert(3.0);
        t.insert(1.5);
        t.insert(4.0);
        assert_eq!(t.remove(&2.0).unwrap(), 2.0);
        assert_eq!(
            format!("{:?}", t),
            "B:3.0\n\
            3.0->B:1.0 3.0->B:4.0\n\
            1.0->___ 1.0->R:1.5 4.0->___ 4.0->___\n\
            1.5->___ 1.5->___"
        );
        t.insert(3.5);
        assert_eq!(t.remove(&3.0).unwrap(), 3.0);
        assert_eq!(
            format!("{:?}", t),
            "B:3.5\n\
            3.5->B:1.0 3.5->B:4.0\n\
            1.0->___ 1.0->R:1.5 4.0->___ 4.0->___\n\
            1.5->___ 1.5->___"
        );
    }

    #[test]
    fn test_removal_no_double_black() {
        let mut t = RBTree::new();
        t.insert(2.0);
        t.insert(1.0);
        t.root.get_left_mut().swap_colour(); // simulating again...
        t.insert(3.0);
        t.root.get_right_mut().swap_colour();
        t.insert(1.5);
        t.insert(2.5);
        println!("{:?}", t);
        
        assert_eq!(t.remove(&1.0).unwrap(), 1.0);
        println!("{:?}", t);
        assert_eq!(
            format!("{:?}", t),
            "B:2.0\n\
            2.0->B:1.5 2.0->B:3.0\n\
            1.5->___ 1.5->___ 3.0->R:2.5 3.0->___\n\
            2.5->___ 2.5->___"
        );
    }

    #[test]
    fn test_removal_simple_case() {
        let mut t = RBTree::new();
        t.insert(30);
        t.insert(20);
        t.insert(40);
        t.insert(10);
        assert_eq!(t.remove(&10).unwrap(), 10);
        assert_eq!(
            format!("{:?}", t),
            "B:30\n\
            30->B:20 30->B:40\n\
            20->___ 20->___ 40->___ 40->___"
        );
    }

    #[test]
    fn test_black_leaf_removal() {
        let mut t = RBTree::new();
        t.insert(65);
        t.insert(50);
        t.insert(80);
        t.insert(10);
        t.insert(60);
        t.insert(62);
        t.insert(70);
        t.insert(90);
        t.insert(92);
        t.remove(&92); // adding & removing causes colour change
        assert_eq!(t.remove(&90).unwrap(), 90);
        assert_eq!(
            format!("{:?}", t),
            "B:65\n\
            65->R:50 65->B:80\n\
            50->B:10 50->B:60 80->R:70 80->___\n\
            10->___ 10->___ 60->___ 60->R:62 70->___ 70->___\n\
            62->___ 62->___"
        );
    }

    // test case is deletion example 3 from this doc
    // https://www.csee.umbc.edu/courses/undergraduate/341/spring04/Lectures/RedBlack/redblack.pdf
    #[test]
    fn test_remove_accumulative_changes() {
        let mut t = RBTree::new();
        t.insert(65);
        t.insert(50);
        t.insert(80);
        t.insert(10);
        t.insert(60);
        t.insert(62);
        t.insert(70);
        t.insert(90);
        t.insert(92);
        t.remove(&92); // adding & removing causes colour change
        t.remove(&90);
        t.remove(&80);
        assert_eq!(t.remove(&70).unwrap(), 70);
        assert_eq!(
            format!("{:?}", t),
            "B:50\n\
            50->B:10 50->R:62\n\
            10->___ 10->___ 62->B:60 62->B:65\n\
            60->___ 60->___ 65->___ 65->___"
        );
    }

    #[test]
    fn test_removal_case2_inner() {
        let mut t = RBTree::new();
        t.insert(30);
        t.insert(20);
        t.insert(40);
        t.insert(35);
        t.insert(50);
        println!("{:?}", t);
        assert_eq!(t.remove(&20).unwrap(), 20);
        println!("{:?}", t);
        assert_eq!(
            format!("{:?}", t),
            "B:35\n\
            35->B:30 35->B:40\n\
            30->___ 30->___ 40->___ 40->R:50\n\
            50->___ 50->___"
        );

        let mut t = RBTree::new();
        t.insert(8);
        t.insert(5);
        t.insert(9);
        t.insert(2);
        t.insert(7);
        t.insert(6);
        assert_eq!(t.remove(&2).unwrap(), 2);
        assert_eq!(
            format!("{:?}", t),
            "B:8\n\
            8->R:6 8->B:9\n\
            6->B:5 6->B:7 9->___ 9->___\n\
            5->___ 5->___ 7->___ 7->___"
        )
    }

    #[test]
    fn test_removal_case2_outer() {
        let mut t = RBTree::new();
        t.insert(30);
        t.insert(20);
        t.insert(40);
        t.insert(50);
        println!("{:?}", t);
        assert_eq!(t.remove(&20).unwrap(), 20);
        println!("{:?}", t);
        assert_eq!(
            format!("{:?}", t),
            "B:40\n\
            40->B:30 40->B:50\n\
            30->___ 30->___ 50->___ 50->___"
        );

        let mut t = RBTree::new();
        t.insert(12);
        t.insert(5);
        t.insert(3);
        t.insert(4);
        t.insert(10);
        t.insert(7);
        t.insert(11);
        t.insert(6);
        t.insert(8);
        t.insert(15);
        t.insert(13);
        t.insert(17);
        t.insert(14);
        assert_eq!(t.remove(&5).unwrap(), 5);
        assert_eq!(
            format!("{:?}", t),
            "B:10\n\
            10->B:6 10->B:12\n\
            6->B:3 6->B:7 12->B:11 12->R:15\n\
            3->___ 3->R:4 7->___ 7->R:8 11->___ 11->___ 15->B:13 15->B:17\n\
            4->___ 4->___ 8->___ 8->___ 13->___ 13->R:14 17->___ 17->___\n\
            14->___ 14->___"
        );
        assert_eq!(t.remove(&6).unwrap(), 6);
        assert_eq!(t.remove(&7).unwrap(), 7);
        assert_eq!(
            format!("{:?}", t),
            "B:10\n\
            10->B:4 10->B:12\n\
            4->B:3 4->B:8 12->B:11 12->R:15\n\
            3->___ 3->___ 8->___ 8->___ 11->___ 11->___ 15->B:13 15->B:17\n\
            13->___ 13->R:14 17->___ 17->___\n\
            14->___ 14->___"
        );
        assert_eq!(t.remove(&4).unwrap(), 4);
        assert_eq!(
            format!("{:?}", t),
            "B:12\n\
            12->B:10 12->B:15\n\
            10->B:8 10->B:11 15->B:13 15->B:17\n\
            8->R:3 8->___ 11->___ 11->___ 13->___ 13->R:14 17->___ 17->___\n\
            3->___ 3->___ 14->___ 14->___"
        );
    }

    #[test]
    fn test_removal_case3_red_parent() {
        let mut t = RBTree::new();
        t.insert(1);
        t.insert(2);
        t.insert(3);
        t.insert(4);
        t.insert(5);
        t.insert(6);
        t.insert(7);
        t.remove(&5);
        t.remove(&7);
        assert_eq!(t.remove(&3).unwrap(), 3);
        assert_eq!(
            format!("{:?}", t),
            "B:2\n\
            2->B:1 2->B:4\n\
            1->___ 1->___ 4->___ 4->R:6\n\
            6->___ 6->___"
        )
    }

    #[test]
    fn test_removeal_case4() {
        let mut t = RBTree::new();
        t.insert(2);
        t.insert(1);
        t.insert(4);
        t.insert(3);
        t.insert(5);
        t.insert(6);
        assert_eq!(t.remove(&1).unwrap(), 1);
        assert_eq!(
            format!("{:?}", t),
            "B:4\n\
            4->B:2 4->B:5\n\
            2->___ 2->R:3 5->___ 5->R:6\n\
            3->___ 3->___ 6->___ 6->___"
        );
    }

    #[test]
    fn test_gets_correct_descendant() {
        let mut t = RBTree::new();
        t.insert(10.0);
        t.insert(1.0);
        t.insert(9.0);
        t.insert(8.0);
        t.insert(7.0);
        t.insert(6.0);
        t.insert(7.3);
        t.remove(&7.0);
        assert_eq!(
            format!("{:?}", t),
            "B:9.0\n\
            9.0->R:7.3 9.0->B:10.0\n\
            7.3->B:1.0 7.3->B:8.0 10.0->___ 10.0->___\n\
            1.0->___ 1.0->R:6.0 8.0->___ 8.0->___\n\
            6.0->___ 6.0->___"
        );

        t.remove(&8.0);
        t.remove(&7.3);
        t.insert(2.0);
        t.remove(&6.0);
        assert_eq!(
            format!("{:?}", t),
            "B:9.0\n\
            9.0->B:2.0 9.0->B:10.0\n\
            2.0->R:1.0 2.0->___ 10.0->___ 10.0->___\n\
            1.0->___ 1.0->___"
        );
    }
}