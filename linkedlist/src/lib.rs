#![allow(dead_code)]

use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node<T: Debug> {
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub value: T,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct LinkedList<T>
where
    T: Debug,
{
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Rc<RefCell<Node<T>>>>,
    pub size: usize,
}

impl<T> LinkedList<T>
where
    T: Debug,
{
    pub fn new() -> LinkedList<T> {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        self.size += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            let tail = self.tail.as_ref().unwrap();
            tail.borrow_mut().next = Some(node.clone());
            self.tail = Some(node);
        }
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        if self.size == 0 {
            let node = self.head.take()?;
            self.tail = None;
            let value = Rc::try_unwrap(node).ok()?.into_inner().value;
            return Some(value);
        }

        let mut current = self.head.clone();
        for _ in 0..self.size - 1 {
            let next = current.as_ref()?.borrow().next.clone();
            current = next;
        }

        if let Some(node) = current {
            let tail_node = node.borrow_mut().next.take()?;
            self.tail = Some(node.clone());
            let value = Rc::try_unwrap(tail_node).ok()?.into_inner().value;
            return Some(value);
        }

        None
    }

    pub fn traversal(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            println!("{:?}", node.borrow().value);
            let next = node.borrow().next.clone();
            cur = next;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut list = LinkedList::new();

        list.push(1);
        list.push(2);

        list.traversal();

        println!("{:?}", list.pop());
        println!("{:?}", list.pop());
        println!("{:?}", list.pop());
    }
}
