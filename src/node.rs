use std::boxed::Box;
use std::ops::DerefMut;
use std::mem::swap as m_swap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Colour {
    Red,
    Black
}

enum Insertion {
    InvalidLeft,
    InvalidRight,
    Recoloured,
    Inserted,
    Success
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Red => write!(f, "R"),
            Colour::Black => write!(f, "B")
        }
    }
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

    // panicing operators used internally very carefully
    // they are essentially used for convenience and to make
    // code look nicer while working with certain guarantees
    // (i.e., their use should never actually cause a panic)
    fn innards(&mut self) -> &mut Innards<T> {
        match self {
            Internal(n) => n,
            Leaf => panic!("Attempted to extract details of leaf node")
        }
    }

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
            self.swap_colour();
            self.child(false).swap_colour();
            self.child(true).swap_colour();
            Recoloured
        } else if inner {
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
                        } else if !right {
                            InvalidLeft
                        } else {
                            InvalidRight
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
}