use std::boxed::Box;
use std::ops::DerefMut;
use std::mem::swap as m_swap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Colour {
    Red,
    Black,
    DBlack
}

enum Insertion {
    InvalidLeft,
    InvalidRight,
    Recoloured,
    Inserted,
    Success
}

enum Removal<T> {
    Removed(T),
    Match,
    NotFound
}

// makes matches nicer
pub struct Innards<T: PartialOrd> {
    value: T,
    colour: Colour,
    r_child: Box<Node<T>>,
    l_child: Box<Node<T>>
}

// represents a node in the rb_tree
pub enum Node<T: PartialOrd> {
    Internal(Innards<T>),
    Leaf
}

use Node::*;
use Colour::*;
use Insertion::*;
use Removal::*;

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Red => write!(f, "R"),
            Black => write!(f, "B"),
            DBlack => write!(f, "D")
        }
    }
}

// convenience implementations for insertion and moving things around
impl<T: PartialOrd> PartialEq<T> for Node<T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            Internal(n) => n.value == *other,
            Leaf => false
        }
    }
}

impl<T: PartialOrd> PartialOrd<T> for Node<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        match self {
            Internal(n) => n.value.partial_cmp(other),
            Leaf => None
        }
    }
}

impl<T: PartialOrd> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        if let (Internal(n1), Internal(n2)) = (self, other) {
            n1.value == n2.value
        } else {
            false
        }
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<std::cmp::Ordering> {
        if let (Internal(n1), Internal(n2)) = (self, other) {
            n1.value.partial_cmp(&n2.value)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> Innards<T> {
    pub fn is_black(&self) -> bool {
        match self.colour {
            Black => true,
            Red => false
        }
    }
    pub fn is_red(&self) -> bool {
        !self.is_black()
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn swap_colour(&mut self) {
        self.colour = match self.colour {
            Red => Black,
            Black => Red,
        }
    }
}

#[allow(dead_code)]
impl<T: PartialOrd> Node<T> {

    pub fn new(val: T) -> Node<T> {
        Internal(
            Innards{
                value: val,
                colour: Red, // all newly inserted values are red
                r_child: Box::new(Leaf),
                l_child: Box::new(Leaf)
            }
        )
    }

    pub fn new_black(val: T) -> Node<T> {
        Internal(
            Innards{
                value: val,
                colour: Black, // all newly inserted values are red
                r_child: Box::new(Leaf),
                l_child: Box::new(Leaf)
            }
        )
    }

    // convenience functions so matches don't appear everywhere
    pub fn is_black(&self) -> bool {
        match self {
            Internal(n) => n.is_black(),
            Leaf => true
        }
    }
    pub fn is_red(&self) -> bool {
        !self.is_black()
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Leaf => true,
            _ => false
        }
    }

    pub fn colour(&self) -> Colour {
        match self {
            Internal(n) => n.colour,
            Leaf => Black
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Internal(n) => Some(&n.value),
            Leaf => None
        }
    }

    pub fn swap_colour(&mut self) {
        if let Internal(n) = self {
            n.swap_colour();
        } // leaves always black
    }

    pub fn get_left(&self) -> &Node<T> {
        match self {
            Internal(n) => &n.l_child,
            Leaf => self
        }
    }

    pub fn get_right(&self) -> &Node<T> {
        match self {
            Internal(n) => &n.r_child,
            Leaf => self
        }
    }

    pub fn get_left_mut(&mut self) -> &mut Node<T> {
        match self {
            Internal(n) => &mut n.l_child,
            Leaf => self
        }
    }

    pub fn get_right_mut(&mut self) -> &mut Node<T> {
        match self {
            Internal(n) => &mut n.r_child,
            Leaf => self
        }
    }

    pub fn has_left(&self) -> bool {
        if let Internal(n) = self {
            !n.l_child.is_leaf()
        } else {
            false
        }
    }

    pub fn has_right(&self) -> bool {
        if let Internal(n) = self {
            !n.r_child.is_leaf()
        } else {
            false
        }
    }

    pub fn remove_left(&mut self) -> Node<T> {
        match self {
            Internal(n) => {
                let mut rep = Leaf;
                m_swap(&mut rep, &mut n.l_child);
                rep
            },
            Leaf => Leaf
        }
    }

    pub fn remove_right(&mut self) -> Node<T> {
        match self {
            Internal(n) => {
                let mut rep = Leaf;
                m_swap(&mut rep, &mut n.r_child);
                rep
            },
            Leaf => Leaf
        }
    }

    pub fn insert_left(&mut self, new_l: T) -> Node<T> {
        let mut rep = Node::new(new_l);
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.l_child),
            Leaf => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn insert_right(&mut self, new_r: T) -> Node<T> {
        let mut rep = Node::new(new_r);
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.r_child),
            Leaf => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn append_left(&mut self, mut rep: Node<T>) -> Node<T> {
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.l_child),
            Leaf => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn append_right(&mut self, mut rep: Node<T>) -> Node<T> {
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.r_child),
            Leaf => m_swap(self, &mut rep)
        }
        rep
    }

    // panicing operators only used internally very carefully
    // they are essentially used for convenience and to make
    // code look nicer while working with certain guarantees
    // (i.e., their use should never actually cause a panic)
    fn innards(&mut self) -> &mut Innards<T> {
        match self {
            Internal(n) => n,
            Leaf => panic!("Attempted to extract details of leaf node")
        }
    }

    // true gets the right child, false left
    fn child(&mut self, right: bool) -> &mut Node<T> {
        match self {
            Internal(n) => if right {
                n.r_child.deref_mut()
            } else {
                n.l_child.deref_mut()
            },
            Leaf => panic!("Attempted to get child of leaf")
        }
    }

    // reorders nodes when required upon insertion
    fn insert_switcheroo(
        &mut self,
        right: bool,
        inner: bool,
        recolour: bool
    ) -> Insertion {
        if recolour {

            // doesn't move anything, simply recolours
            self.swap_colour();
            self.child(false).swap_colour();
            self.child(true).swap_colour();
            Recoloured
        } else if inner {

            // realligns the newly inserted value as the new local root
            let mut tmp = Leaf;
            let mut l_child_tmp = Leaf;
            let mut r_child_tmp = Leaf;
            let gchild = !right;
            m_swap(&mut tmp, self.child(right).child(gchild));
            m_swap(&mut l_child_tmp, tmp.child(false));
            m_swap(&mut r_child_tmp, tmp.child(true));
            m_swap(tmp.child(right), self.child(right));
            m_swap(tmp.child(!right), self);
            m_swap(&mut tmp, self);
            m_swap(self.child(false).child(true), &mut l_child_tmp);
            m_swap(self.child(true).child(false), &mut r_child_tmp);
            self.swap_colour();
            self.child(!right).swap_colour();
            Success
        } else {

            // realigns the parent of the newly inserted value as the new
            // local root
            let mut tmp = Leaf;
            let mut child_tmp = Leaf;
            let gchild = !right;
            m_swap(&mut tmp, self.child(right));
            m_swap(tmp.child(gchild), self);
            m_swap(self, &mut child_tmp);
            m_swap(&mut tmp, self);
            m_swap(self.child(!right).child(right), &mut child_tmp);
            self.swap_colour();
            self.child(!right).swap_colour();
            Success
        }
    }

    // returns the value if the value was not inserted
    fn insert_op(&mut self, new_v: T) -> Insertion {
        match self {
            Internal(n) => {
                let (res, right, recolour) = if n.value >= new_v {
                    (n.l_child.insert_op(new_v), false, n.r_child.is_red())
                } else {
                    (n.r_child.insert_op(new_v), true, n.l_child.is_red())
                };
                match res {
                    InvalidLeft => {
                        self.insert_switcheroo(right, right, recolour)
                    },
                    InvalidRight => {
                        self.insert_switcheroo(right, !right, recolour)
                    },
                    Recoloured => {
                        if self.is_red() && self.child(right).is_red() {
                            if right {
                                InvalidRight
                            } else {
                                InvalidLeft
                            }
                        } else {
                            Success
                        }
                    },
                    Inserted => {
                        if self.is_black() {
                            Success
                        } else if right {
                            InvalidRight
                        } else {
                            InvalidLeft
                        }
                    },
                    Success => Success
                }
            },
            Leaf => {
                *self = Node::new(new_v);
                Inserted
            }
        }
    }

    // only to be called on the root
    pub fn insert(&mut self, new_v: T) {
        self.insert_op(new_v);
        if self.is_red() {
            self.swap_colour();
        }
    }

    // fn find_item(&mut self, val: &T) -> &mut Node<T> {
    //     let mut cur = self;
    //     while !cur.is_leaf() {
    //         if *cur >= *val {
    //             cur = cur.child(false);
    //         } else {
    //             cur = cur.child(true);
    //         }
    //         if cur == val {break;}
    //     }
    //     cur
    // }

    // fn swap_left_most_right(&mut self, swap: &mut T) -> bool {
    //     let mut ret = Leaf;
    //     let mut node = if self.get_right_mut().is_leaf() {
    //         return false;
    //     } else {
    //         self.child(true)
    //     };
    //     while !node.child(false).is_leaf() {
    //         node = node.child(false);
    //     }
    //     m_swap(swap, &mut node.innards().value);
    //     true
    // }

    // fn swap_down(&mut self, )

    // fn remove_op(&mut self, val: &T) -> Removal<T> {
    //     match self {
    //         Internal(n) => {
    //             let (res, right) = if n.value == *val {
    //                 (Match, true)
    //             } else if n.value > *val {
    //                 (n.l_child.remove_op(val), false)
    //             } else {
    //                 (n.r_child.remove_op(val), true)
    //             };
    //             match res {
    //                 Match => {
    //                     if self.swap_left_most_right(&mut self.innards().value) {

    //                     }
    //                     NotFound
    //                 },
    //                 Removed(n) => {
    //                     NotFound
    //                 },
    //                 NotFound => NotFound
    //             }
    //         },
    //         Leaf => {
    //             NotFound
    //         }
    //     }
    // }

    // pub fn remove(&mut self, val: &T) -> Option<T> {
    //     let item = self.find_item(val);
    //     let rep = if item.is_leaf() {
    //         return None;
    //     } else {
    //         item.left_most_right()
    //     };
    //     let mut tmp = Leaf;
    //     m_swap
    //     m_swap(&mut node.innards().value, &mut self.innards().value);
    //     None
    // }
}