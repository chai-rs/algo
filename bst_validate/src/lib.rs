use std::{cell::RefCell, i32, rc::Rc};

pub struct Node {
    pub value: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        let node = Self {
            value,
            left: None,
            right: None,
        };

        Rc::new(RefCell::new(node))
    }
}

pub fn validate(node: Rc<RefCell<Node>>) -> bool {
    _validate(node, i32::MIN, i32::MAX)
}

fn _validate(node: Rc<RefCell<Node>>, min: i32, max: i32) -> bool {
    let borrowed = node.borrow();
    let value = borrowed.value;
    let valid = value <= max && value > min;

    let left = if let Some(left) = borrowed.left.clone() {
        _validate(left, min, value)
    } else {
        true
    };

    let right = if let Some(right) = borrowed.right.clone() {
        _validate(right, value, max)
    } else {
        true
    };

    valid && left && right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let root = Node::new(10);
        assert!(validate(root));
    }

    #[test]
    fn test_valid_bst() {
        //     10
        //    /  \
        //   5    15
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));
        root.borrow_mut().right = Some(Node::new(15));
        assert!(validate(root));
    }

    #[test]
    fn test_valid_bst_three_levels() {
        //       10
        //      /  \
        //     5    15
        //    / \   / \
        //   2   7 12  20
        let root = Node::new(10);
        let left = Node::new(5);
        let right = Node::new(15);

        left.borrow_mut().left = Some(Node::new(2));
        left.borrow_mut().right = Some(Node::new(7));
        right.borrow_mut().left = Some(Node::new(12));
        right.borrow_mut().right = Some(Node::new(20));

        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert!(validate(root));
    }

    #[test]
    fn test_invalid_left_child_greater_than_parent() {
        //     10
        //    /
        //   15  (invalid: left child > parent)
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(15));
        assert!(!validate(root));
    }

    #[test]
    fn test_invalid_right_child_less_than_parent() {
        //     10
        //       \
        //        5  (invalid: right child < parent)
        let root = Node::new(10);
        root.borrow_mut().right = Some(Node::new(5));
        assert!(!validate(root));
    }

    #[test]
    fn test_invalid_grandchild_violates_ancestor() {
        //       10
        //      /
        //     5
        //      \
        //       15  (invalid: 15 > 10, violates BST property)
        let root = Node::new(10);
        let left = Node::new(5);
        left.borrow_mut().right = Some(Node::new(15));
        root.borrow_mut().left = Some(left);
        assert!(!validate(root));
    }

    #[test]
    fn test_equal_values_left() {
        //     10
        //    /
        //   10  (valid: left can be equal)
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(10));
        assert!(validate(root));
    }

    #[test]
    fn test_equal_values_right() {
        //     10
        //       \
        //       10  (invalid: right cannot be equal)
        let root = Node::new(10);
        root.borrow_mut().right = Some(Node::new(10));
        assert!(!validate(root));
    }

    #[test]
    fn test_only_left_children() {
        //       10
        //      /
        //     5
        //    /
        //   2
        let root = Node::new(10);
        let mid = Node::new(5);
        mid.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().left = Some(mid);
        assert!(validate(root));
    }

    #[test]
    fn test_only_right_children() {
        //   10
        //     \
        //     15
        //       \
        //       20
        let root = Node::new(10);
        let mid = Node::new(15);
        mid.borrow_mut().right = Some(Node::new(20));
        root.borrow_mut().right = Some(mid);
        assert!(validate(root));
    }
}
