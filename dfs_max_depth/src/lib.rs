#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

pub struct Node {
    pub value: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }

    pub fn set_left(&mut self, node: Rc<RefCell<Node>>) {
        self.left = Some(node);
    }

    pub fn set_right(&mut self, node: Rc<RefCell<Node>>) {
        self.right = Some(node);
    }
}

fn max_depth(root: Rc<RefCell<Node>>, depth: i32) -> i32 {
    let left_depth = match root.borrow().left.clone() {
        Some(node) => max_depth(node, depth),
        None => 0,
    };

    let right_depth = match root.borrow().right.clone() {
        Some(node) => max_depth(node, depth),
        None => 0,
    };

    if right_depth > left_depth {
        right_depth + 1
    } else {
        left_depth + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_depth_single_node() {
        let root = Node::new(1);
        let max = max_depth(root, 0);
        assert_eq!(max, 1);
    }

    #[test]
    fn test_max_depth_two_levels() {
        let root = Node::new(1);
        root.borrow_mut().set_left(Node::new(2));
        root.borrow_mut().set_right(Node::new(3));

        let max = max_depth(root, 0);
        assert_eq!(max, 2);
    }

    #[test]
    fn test_max_depth_left_skewed() {
        let root = Node::new(1);
        let left = Node::new(2);
        left.borrow_mut().set_left(Node::new(3));
        root.borrow_mut().set_left(left);

        let max = max_depth(root, 0);
        assert_eq!(max, 3);
    }

    #[test]
    fn test_max_depth_right_skewed() {
        let root = Node::new(1);
        let right = Node::new(2);
        right.borrow_mut().set_right(Node::new(3));
        root.borrow_mut().set_right(right);

        let max = max_depth(root, 0);
        assert_eq!(max, 3);
    }

    #[test]
    fn test_max_depth_unbalanced_tree() {
        let root = Node::new(1);

        // Left subtree: depth 2
        let left = Node::new(2);
        left.borrow_mut().set_left(Node::new(4));
        root.borrow_mut().set_left(left);

        // Right subtree: depth 4
        let right = Node::new(3);
        let right_left = Node::new(5);
        let right_left_left = Node::new(6);
        right_left_left.borrow_mut().set_left(Node::new(7));
        right_left.borrow_mut().set_left(right_left_left);
        right.borrow_mut().set_left(right_left);
        root.borrow_mut().set_right(right);

        let max = max_depth(root, 0);
        assert_eq!(max, 5);
    }

    #[test]
    fn test_max_depth_complete_binary_tree() {
        let root = Node::new(1);

        // Level 2
        let left = Node::new(2);
        let right = Node::new(3);

        // Level 3
        left.borrow_mut().set_left(Node::new(4));
        left.borrow_mut().set_right(Node::new(5));
        right.borrow_mut().set_left(Node::new(6));
        right.borrow_mut().set_right(Node::new(7));

        root.borrow_mut().set_left(left);
        root.borrow_mut().set_right(right);

        let max = max_depth(root, 0);
        assert_eq!(max, 3);
    }

    #[test]
    fn test_max_depth_only_left_child() {
        let root = Node::new(1);
        root.borrow_mut().set_left(Node::new(2));

        let max = max_depth(root, 0);
        assert_eq!(max, 2);
    }

    #[test]
    fn test_max_depth_only_right_child() {
        let root = Node::new(1);
        root.borrow_mut().set_right(Node::new(2));

        let max = max_depth(root, 0);
        assert_eq!(max, 2);
    }

    #[test]
    fn test_max_depth_deep_tree() {
        let root = Node::new(1);
        let n2 = Node::new(2);
        let n3 = Node::new(3);
        let n4 = Node::new(4);
        let n5 = Node::new(5);
        let n6 = Node::new(6);

        n5.borrow_mut().set_left(n6);
        n4.borrow_mut().set_right(n5);
        n3.borrow_mut().set_left(n4);
        n2.borrow_mut().set_right(n3);
        root.borrow_mut().set_left(n2);

        let max = max_depth(root, 0);
        assert_eq!(max, 6);
    }
}
