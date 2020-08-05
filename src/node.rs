use std::boxed::Box;

#[derive(Clone, Copy)]
pub enum Colour {
    Red,
    Black
}

// represents a node in the rb_tree
pub struct Node<T: PartialOrd> {
    value: T,
    colour: Colour,
    r_child: Option<Box<Node<T>>>,
    l_child: Option<Box<Node<T>>>
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

    pub fn set_left(&mut self, new_l: T) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(Node::new(new_l)));
        std::mem::swap(&mut rep, &mut self.l_child);
        rep
    }

    pub fn set_right(&mut self, new_r: T) -> Option<Box<Node<T>>> {
        let mut rep = Some(Box::new(Node::new(new_r)));
        std::mem::swap(&mut rep, &mut self.r_child);
        rep
    }
}