use std::boxed::Box;
use std::ops::{DerefMut, Deref};
use std::mem::swap as m_swap;
use std::cmp::Ordering::{Equal, Greater, Less};

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
pub struct Innards<T> {
    value: T,
    colour: Colour,
    r_child: Box<Node<T>>,
    l_child: Box<Node<T>>
}

// represents a node in the rb_tree
pub enum Node<T> {
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

impl<T> Innards<T> {
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

impl<T> Node<T> {

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

    // method used for testing
    #[cfg(test)]
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
        match self {
            Internal(n) => n.is_red(),
            _ => false
        }
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

    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            Internal(n) => Some(&mut n.value),
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
    fn peek_child(&self, right: bool) -> &Node<T> {
        match self {
            Internal(n) => if right {
                n.r_child.deref()
            } else {
                n.l_child.deref()
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
    fn insert_op<P>(&mut self, mut new_v: T, cmp: &P) -> Insertion<T>
    where P: Fn(&T, &T) -> std::cmp::Ordering {
        match self {
            Internal(n) => {
                let order = cmp(&n.value, &new_v);
                let (res, right, recolour) = match order {
                    Equal => {
                        m_swap(&mut n.value, &mut new_v); // useful if used like a map
                        (Replaced(new_v), true, true)
                    },
                    Greater => {
                        (n.l_child.insert_op(new_v, cmp), false, n.r_child.is_red())
                    },
                    Less => {
                        (n.r_child.insert_op(new_v, cmp), true, n.l_child.is_red())
                    }
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
    pub fn insert<P>(&mut self, new_v: T, cmp: &P) -> Option<T>
    where P: Fn(&T, &T) -> std::cmp::Ordering {
        let res = self.insert_op(new_v, cmp);
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
        let mut right = !self.get_right().is_double_black();
        let mut cur = self;

        // get to the bottom
        while cur.child(right).is_red() {
            cur.outer_switcheroo(right);
            cur.black();
            cur.child(!right).red();
            cur = cur.child(!right);
            right = !cur.get_right().is_double_black();
        }

        // perform the deletion
        let self_col = cur.colour();
        if cur.child(right).child(!right).is_red() {
            cur.inner_switcheroo(right);
        } else if cur.child(right).child(right).is_red() {
            cur.outer_switcheroo(right);
        } else {
            cur.child(right).red();
            cur.child(!right).black();
            if cur.is_black() {
                cur.double_black();
                return true;
            } else {
                cur.black();
                return false;
            }
        }

        // recolour things appropriately
        match self_col {
            Black => cur.black(),
            Red => cur.red(),
            _ => panic!("Double black at local root")
        }
        cur.child(right).black();
        cur.child(!right).child(!right).black();
        cur.child(!right).black();
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

    fn remove_op<K, P>(&mut self, val: &K, cmp: &P) -> Removal<T>
    where P: Fn(&K, &T) -> std::cmp::Ordering {
        match self {
            Internal(n) => {
                let order = cmp(val, &n.value);
                let (res, right) = match order {
                    Equal => (Match, true),
                    Less => (n.l_child.remove_op(val, cmp), false),
                    Greater => (n.r_child.remove_op(val, cmp), true)
                };
                self.remove_result_step(res, right)
            },
            Leaf(_) => {
                NotFound
            }
        }
    }

    fn pop_op(&mut self, back: bool) -> Removal<T> {
        let mut cur = self;
        while !cur.is_leaf() {
            if cur.child(back).is_leaf() {
                return cur.remove_result_step(Match, true);
            } else {
                cur = cur.child(back);
            }
        }
        NotFound
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
    pub fn remove<K, P>(&mut self, val: &K, cmp: &P) -> Option<T>
    where P: Fn(&K, &T) -> std::cmp::Ordering {
        match self.remove_op(val, cmp) {
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

    pub fn get<K, P>(&self, val: &K, cmp: &P) -> Option<&T>
    where P: Fn(&K, &T) -> std::cmp::Ordering {
        let mut cur = self;
        while !cur.is_leaf() {
            let cur_val = cur.value();
            let order = cmp(val, cur_val.unwrap());
            match order {
                Equal => return cur_val,
                Less => cur = cur.get_left(),
                Greater => cur = cur.get_right()
            }
        }
        match cur {
            Internal(n) => Some(&n.value),
            _ => None
        }
    }

    pub fn get_mut<K, P>(&mut self, val: &K, cmp: &P) -> Option<&mut T>
    where P: Fn(&K, &T) -> std::cmp::Ordering {
        let mut cur = self;
        while !cur.is_leaf() {
            let cur_val = cur.value().unwrap();
            let order = cmp(val, cur_val);
            match order {
                Equal => return cur.value_mut(),
                Less => cur = cur.get_left_mut(),
                Greater => cur = cur.get_right_mut()
            }
        }
        match cur {
            Internal(n) => Some(&mut n.value),
            _ => None
        }
    }

    pub fn peek(&self, back: bool) -> Option<&T> {
        let mut cur = self;
        while !cur.is_leaf() {
            if !cur.peek_child(back).is_leaf() {
                cur = cur.peek_child(back);
            } else {
                break;
            }
        }
        match cur {
            Internal(n) => Some(&n.value),
            _ => None
        }
    }
}