use std::boxed::Box;

#[derive(Clone, Copy)]
pub enum Colour {
    Red,
    Black
}

enum Insertion<T> {
    NotInsertedLeft(T),
    NotInertedRight(T),
    Recoloured,
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

// represents a node in the rb_tree
pub struct Node<T: PartialOrd> {
    value: T,
    colour: Colour,
    r_child: Option<Box<Node<T>>>,
    l_child: Option<Box<Node<T>>>
}

// convenience implementations for insertion and moving things around
impl<T: PartialOrd> PartialEq<T> for Node<T> {
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Node<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<T: PartialOrd> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.value == other.value
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[allow(dead_code)]
impl<T: PartialOrd> Node<T> {

    pub fn new(val: T) -> Node<T> {
        Node{
            value: val,
            colour: Colour::Red, // all newly inserted values are red
            r_child: None,
            l_child: None
        }
    }

    pub fn new_black(val: T) -> Node<T> {
        Node{
            value: val,
            colour: Colour::Black,
            r_child: None,
            l_child: None
        }
    }

    // convenience functions so matches don't appear everywhere
    pub fn is_black(&self) -> bool {
        match &self.colour {
            Colour::Red => false,
            Colour::Black => true
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
        self.colour = match &self.colour {
            Colour::Red => Colour::Black,
            Colour::Black => Colour::Red
        }
    }

    pub fn get_left(&self) -> Option<&Box<Node<T>>> {
        self.l_child.as_ref()
    }

    pub fn get_right(&self) -> Option<&Box<Node<T>>> {
        self.r_child.as_ref()
    }

    pub fn get_left_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.l_child.as_mut()
    }

    pub fn get_right_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.r_child.as_mut()
    }

    pub fn has_left(&self) -> bool {
        match self.l_child {
            Some(_) => true,
            None => false
        }
    }

    pub fn has_right(&self) -> bool {
        match self.r_child {
            Some(_) => true,
            None => false
        }
    }

    pub fn remove_left(&mut self) -> Option<Box<Node<T>>> {
        let mut rep = None;
        std::mem::swap(&mut rep, &mut self.l_child);
        rep
    }

    pub fn remove_right(&mut self) -> Option<Box<Node<T>>> {
        let mut rep = None;
        std::mem::swap(&mut rep, &mut self.r_child);
        rep
    }

    pub fn insert_left(&mut self, new_l: T) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(Node::new(new_l)));
        std::mem::swap(&mut rep, &mut self.l_child);
        rep
    }

    pub fn insert_right(&mut self, new_r: T) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(Node::new(new_r)));
        std::mem::swap(&mut rep, &mut self.r_child);
        rep
    }

    pub fn append_left(&mut self, node: Node<T>) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(node));
        std::mem::swap(&mut rep, &mut self.l_child);
        rep
    }

    pub fn append_right(&mut self, node: Node<T>) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(node));
        std::mem::swap(&mut rep, &mut self.r_child);
        rep
    }

    // returns the value if the value was not inserted
    fn insert_op(&mut self, new_v: T) -> Insertion<T> {
        if *self >= new_v {
            match self.l_child.as_mut() {

                // do the rotation if required
                Some(v) => {
                    let new_v = v.insert_op(new_v);
                    match new_v {
                        Insertion::NotInsertedLeft(v) => {
                            if
                        }
                    }
                    Insertion::Success
                },
                None => {
                    if self.is_red() {
                        Insertion::NotInsertedLeft(new_v)
                    } else {
                        self.l_child = Some(Box::new(Node::new(new_v)));
                        Insertion::Success
                    }
                }
            }
        } else {
            match self.r_child.as_mut() {
                Some(v) => v.insert_op(new_v),
                None => {
                    self.r_child = Some(Box::new(Node::new(new_v)));
                    false
                }
            }
        }
    }

    // only to be called on the root
    pub fn insert(&mut self, new_v: T) {
        // self.insert_op(new_v);
        if *self >= new_v {
            match self.l_child.as_mut() {
                Some(v) => v.insert(new_v),
                None => self.l_child = Some(Box::new(Node::new(new_v)))
            }
        } else {
            match self.r_child.as_mut() {
                Some(v) => v.insert(new_v),
                None => self.r_child = Some(Box::new(Node::new(new_v)))
            }
        }
    }
}