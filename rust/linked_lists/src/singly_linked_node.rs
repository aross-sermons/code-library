// src/singly_linked_node.rs

use core::fmt;

pub struct Node<T: fmt::Display + PartialEq> {
    data: Option<T>,
    next: Option<Box<Node<T>>>,
}

// Implement Display for Node<T> for Node printing
impl<T: fmt::Display + PartialEq> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => write!(f, "({})", data),
            None => write!(f, "None"),
        }
    }
}

// Implement PartialEq for Node<T> for Node comparison
impl<T: fmt::Display + PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: fmt::Display + PartialEq> Node<T> {
    pub fn new(data: Option<T>) -> Self {
        Node {
            data,
            next: None,
        }
    }

    pub fn set_next(&mut self, next_node: Option<Box<Node<T>>>) {
        self.next = next_node;
    }

    pub fn get_tail(&self) -> &Node<T> {
        let mut current_node = self;
        while let Some(ref next_node) = current_node.next {
            current_node = next_node;
        }
        current_node
    }

    fn get_tail_mut(&mut self) -> &mut Node<T> {
        let mut current_node = self;
        while let Some(ref mut next_node) = current_node.next {
            current_node = next_node;
        }
        current_node
    }

    pub fn append(&mut self, new_node: Node<T>) {
        let boxed_node = Some(Box::new(new_node));
        let tail = self.get_tail_mut();
        tail.next = boxed_node;
    }

    pub fn print_forward(&self) {
        let mut current_node = self;
        while let Some(ref next_node) = current_node.next {
            println!("{}", current_node);
            current_node = next_node;
        }
        println!("{}", current_node);
    }
}