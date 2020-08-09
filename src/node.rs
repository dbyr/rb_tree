use std::boxed::Box;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::mem::swap as m_swap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Colour {
    Red,
    Black,
    DBlack
}

enum Insertion<T> {
    InvalidLeft,
    InvalidRight,
    Recoloured,
    Inserted,
    Replaced(T),
    Success
}

enum Removal<T> {
    Removed(T),
    Doubled(T),
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
    Leaf(Colour)
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
            Leaf(_) => false
        }
    }
}

impl<T: PartialOrd> PartialOrd<T> for Node<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        match self {
            Internal(n) => n.value.partial_cmp(other),
            Leaf(_) => None
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
            _ => false
        }
    }
    pub fn is_red(&self) -> bool {
        match self.colour {
            Red => true,
            _ => false
        }
    }

    pub fn is_double_black(&self) -> bool {
        match self.colour {
            DBlack => true,
            _ => false
        }
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
            DBlack => Black
        }
    }
}

impl<T: PartialOrd> Node<T> {

    pub fn new(val: T) -> Node<T> {
        Internal(
            Innards{
                value: val,
                colour: Red, // all newly inserted values are red
                r_child: Box::new(Leaf(Black)),
                l_child: Box::new(Leaf(Black))
            }
        )
    }

    pub fn new_black(val: T) -> Node<T> {
        Internal(
            Innards{
                value: val,
                colour: Black, // all newly inserted values are red
                r_child: Box::new(Leaf(Black)),
                l_child: Box::new(Leaf(Black))
            }
        )
    }

    // convenience functions so matches don't appear everywhere
    pub fn is_black(&self) -> bool {
        match self {
            Internal(n) => n.is_black(),
            Leaf(c) => *c == Black
        }
    }
    pub fn is_red(&self) -> bool {
        !self.is_black()
    }
    pub fn is_double_black(&self) -> bool {
        match self {
            Internal(n) => n.is_double_black(),
            Leaf(c) => *c == DBlack
        }
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Leaf(_) => true,
            _ => false
        }
    }

    pub fn colour(&self) -> Colour {
        match self {
            Internal(n) => n.colour,
            Leaf(c) => *c
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Internal(n) => Some(&n.value),
            Leaf(_) => None
        }
    }

    pub fn swap_colour(&mut self) {
        if let Internal(n) = self {
            n.swap_colour();
        } // leaves always black
    }
    fn black(&mut self) {
        match self {
            Internal(n) => n.colour = Black,
            Leaf(c) => *c = Black
        }
    }
    fn red(&mut self) {
        match self {
            Internal(n) => n.colour = Red,
            Leaf(c) => *c = Red
        }
    }
    fn double_black(&mut self) {
        match self {
            Internal(n) => n.colour = DBlack,
            Leaf(c) => *c = DBlack
        }
    }

    pub fn get_left(&self) -> &Node<T> {
        match self {
            Internal(n) => &n.l_child,
            Leaf(_) => self
        }
    }

    pub fn get_right(&self) -> &Node<T> {
        match self {
            Internal(n) => &n.r_child,
            Leaf(_) => self
        }
    }

    pub fn get_left_mut(&mut self) -> &mut Node<T> {
        match self {
            Internal(n) => &mut n.l_child,
            Leaf(_) => self
        }
    }

    pub fn get_right_mut(&mut self) -> &mut Node<T> {
        match self {
            Internal(n) => &mut n.r_child,
            Leaf(_) => self
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
                let mut rep = Leaf(Black);
                m_swap(&mut rep, &mut n.l_child);
                rep
            },
            Leaf(_) => Leaf(Black)
        }
    }

    pub fn remove_right(&mut self) -> Node<T> {
        match self {
            Internal(n) => {
                let mut rep = Leaf(Black);
                m_swap(&mut rep, &mut n.r_child);
                rep
            },
            Leaf(_) => Leaf(Black)
        }
    }

    pub fn insert_left(&mut self, new_l: T) -> Node<T> {
        let mut rep = Node::new(new_l);
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.l_child),
            Leaf(_) => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn insert_right(&mut self, new_r: T) -> Node<T> {
        let mut rep = Node::new(new_r);
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.r_child),
            Leaf(_) => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn append_left(&mut self, mut rep: Node<T>) -> Node<T> {
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.l_child),
            Leaf(_) => m_swap(self, &mut rep)
        }
        rep
    }

    pub fn append_right(&mut self, mut rep: Node<T>) -> Node<T> {
        match self {
            Internal(n) => m_swap(&mut rep, &mut n.r_child),
            Leaf(_) => m_swap(self, &mut rep)
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
            Leaf(_) => panic!("Attempted to extract details of leaf node")
        }
    }
    fn gut(self) -> Innards<T> {
        match self {
            Internal(n) => n,
            Leaf(_) => panic!("Attempted to extract details of leaf node")
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
            Leaf(_) => panic!("Attempted to get child of leaf")
        }
    }

    fn inner_switcheroo(&mut self, right: bool) {
        let mut tmp = Leaf(Black);
        let mut l_child_tmp = Leaf(Black);
        let mut r_child_tmp = Leaf(Black);
        m_swap(&mut tmp, self.child(right).child(!right));
        m_swap(&mut l_child_tmp, tmp.child(false));
        m_swap(&mut r_child_tmp, tmp.child(true));
        m_swap(tmp.child(right), self.child(right));
        m_swap(tmp.child(!right), self);
        m_swap(&mut tmp, self);
        m_swap(self.child(false).child(true), &mut l_child_tmp);
        m_swap(self.child(true).child(false), &mut r_child_tmp);
    }

    fn outer_switcheroo(&mut self, right: bool) {
        let mut tmp = Leaf(Black);
        let mut child_tmp = Leaf(Black);
        m_swap(&mut tmp, self.child(right));
        m_swap(tmp.child(!right), self);
        m_swap(self, &mut child_tmp);
        m_swap(&mut tmp, self);
        m_swap(self.child(!right).child(right), &mut child_tmp);
    }

    // reorders nodes when required upon insertion
    fn insert_switcheroo(
        &mut self,
        right: bool,
        inner: bool,
        recolour: bool
    ) -> Insertion<T> {
        if recolour {

            // doesn't move anything, simply recolours
            self.swap_colour();
            self.child(false).swap_colour();
            self.child(true).swap_colour();
            Recoloured
        } else if inner {

            // realligns the newly inserted value as the new local root
            self.inner_switcheroo(right);
            self.swap_colour();
            self.child(!right).swap_colour();
            Success
        } else {

            // realigns the parent of the newly inserted value as the new
            // local root
            self.outer_switcheroo(right);
            self.swap_colour();
            self.child(!right).swap_colour();
            Success
        }
    }

    // returns the value if the value was not inserted
    fn insert_op(&mut self, mut new_v: T) -> Insertion<T> {
        match self {
            Internal(n) => {
                let (res, right, recolour) = if n.value == new_v {
                    m_swap(&mut n.value, &mut new_v); // useful if used like a map
                    (Replaced(new_v), true, true)
                } else if n.value > new_v {
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
                    Replaced(v) => Replaced(v),
                    Success => Success
                }
            },
            Leaf(_) => {
                *self = Node::new(new_v);
                Inserted
            }
        }
    }

    // only to be called on the root
    pub fn insert(&mut self, new_v: T) -> Option<T> {
        let res = self.insert_op(new_v);
        if self.is_red() {
            self.swap_colour();
        }
        match res {
            Replaced(v) => Some(v),
            _ => None
        }
    }

    // returns true if double black propogates
    fn deletion_switcheroo(&mut self) -> bool {
        let right = if self.get_right().is_double_black() {
            false
        } else {
            true
        };
        if self.child(right).is_black() {
            let self_col = self.colour();
            if self.child(right).child(!right).is_red() {
                self.inner_switcheroo(right);
            } else if self.child(right).child(right).is_red() {
                self.outer_switcheroo(right);
            } else {
                self.child(right).red();
                self.child(!right).black();
                if self.is_black() {
                    self.double_black();
                    return true;
                } else {
                    self.black();
                    return false;
                }
            }

            // recolour things appropriately
            match self_col {
                Black => self.black(),
                Red => self.red(),
                _ => panic!("Double black at local root")
            }
            self.child(right).black();
            self.child(!right).child(!right).black();
            self.child(!right).black();
        } else {
            self.outer_switcheroo(right);
            self.black();
            self.child(!right).red();
            if self.child(!right).deletion_switcheroo() {
                return self.deletion_switcheroo();
            }
        }
        false
    }

    fn swap_innermost_descendant(&mut self) -> (T, bool) {
        let mut ret = Leaf(Black);
        let mut doubled = false;
        let right;
        
        // decide which value to switch with, if any
        let mut node = if !self.get_right().is_leaf() {
            right = true;
            self.child(right)
        } else if !self.get_left().is_leaf() {
            right = false;
            self.child(right)
        } else {
            if self.is_black() {doubled = true; ret = Leaf(DBlack)}
            m_swap(self, &mut ret);
            return (ret.gut().value, doubled);
        };
        while !node.child(!right).is_leaf() {
            node = node.child(!right);
        }

        // ensure the swap's child remains attached
        // and fix the black depth
        m_swap(&mut ret, node.child(right));
        if node.is_black() {
            if ret.is_red() {
                ret.swap_colour();
            } else {
                doubled = true;
                ret.double_black();
            }
        }
        m_swap(&mut ret, node);
        let mut retval = ret.gut().value;
        m_swap(&mut self.innards().value, &mut retval);
        (retval, doubled)
    }

    fn swap_doubles_up(&mut self, right: bool) -> bool {
        if self.child(!right).is_double_black() {
            self.deletion_switcheroo()
        } else if self.child(!right).swap_doubles_up(right) {
            self.deletion_switcheroo()
        } else {
            false
        }
    }

    fn remove_result_step(&mut self, res: Removal<T>, right: bool) -> Removal<T> {
        match res {
            Match => {
                let (ret, doubled) = self.swap_innermost_descendant();
                if doubled {
                    Doubled(ret)
                } else {
                    Removed(ret)
                }
            },
            Doubled(n) => {
                let doubled =
                if self.child(right).is_double_black() {
                    self.deletion_switcheroo()
                } else if self.child(right).swap_doubles_up(right) {
                    self.deletion_switcheroo()
                } else {
                    false
                };
                if doubled {
                    Doubled(n)
                } else {
                    Removed(n)
                }
            },
            Removed(n) => Removed(n),
            NotFound => NotFound
        }
    }

    fn remove_op<K: PartialOrd<T>>(&mut self, val: &K) -> Removal<T> {
        match self {
            Internal(n) => {
                let (res, right) = if *val == n.value {
                    (Match, true)
                } else if *val < n.value {
                    (n.l_child.remove_op(val), false)
                } else {
                    (n.r_child.remove_op(val), true)
                };
                self.remove_result_step(res, right)
            },
            Leaf(_) => {
                NotFound
            }
        }
    }

    fn pop_op(&mut self, back: bool) -> Removal<T> {
        match self {
            Internal(n) => {
                if back {
                    if n.r_child.is_leaf() {
                        self.remove_result_step(Match, true)
                    } else {
                        n.r_child.pop_op(back)
                    }
                } else {
                    if n.l_child.is_leaf() {
                        self.remove_result_step(Match, true)
                    } else {
                        n.l_child.pop_op(back)
                    }
                }
            },
            Leaf(_) => {
                NotFound
            }
        }
    }

    pub fn pop(&mut self, back: bool) -> Option<T> {
        match self.pop_op(back) {
            NotFound => None,
            Removed(v) => {
                Some(v)
            },
            Doubled(v) => {
                self.swap_colour();
                Some(v)
            },
            // uhh, shouldn't ever happen if I've coded it right
            _ => panic!("Returned invalid option, tree structure damaged")
        }
    }

    // as with insertion, this should only be called on the root
    pub fn remove<K: PartialOrd<T>>(&mut self, val: &K) -> Option<T> {
        match self.remove_op(val) {
            NotFound => None,
            Removed(v) => {
                Some(v)
            },
            Doubled(v) => {
                self.swap_colour();
                Some(v)
            },
            // uhh, shouldn't ever happen if I've coded it right
            _ => panic!("Returned invalid option, tree structure damaged")
        }
    }

    pub fn get<K: PartialOrd<T>>(&self, val: &K) -> Option<&T> {
        let mut cur = self;
        while !cur.is_leaf() {
            let cur_val = cur.value();
            if val == cur_val.unwrap() {
                return cur_val;
            } else if val < cur_val.unwrap() {
                cur = cur.get_left();
            } else {
                cur = cur.get_right();
            }
        }
        match cur {
            Internal(n) => Some(&n.value),
            _ => None
        }
    }

    pub fn peek(&self, back: bool) -> Option<&T> {
        let mut cur = self;
        while !cur.is_leaf() {
            if back {
                if !cur.get_right().is_leaf() {
                    cur = cur.get_right();
                } else {
                    break;
                }
            } else {
                if !cur.get_left().is_leaf() {
                    cur = cur.get_left();
                } else {
                    break;
                }
            }
        }
        match cur {
            Internal(n) => Some(&n.value),
            _ => None
        }
    }
}