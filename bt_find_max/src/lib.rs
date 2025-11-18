use std::{cell::RefCell, rc::Rc};

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

pub fn find_max(node: Option<Rc<RefCell<Node>>>) -> i32 {
    let node = match node {
        Some(node) => node,
        None => return i32::MIN,
    };

    let borrowed = node.borrow();
    let left = find_max(borrowed.left.clone());
    let right = find_max(borrowed.right.clone());
    return borrowed.value.max(left.max(right));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_node() {
        let root = Node::new(5);
        let result = find_max(Some(root));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_single_node_negative() {
        let root = Node::new(-10);
        let result = find_max(Some(root));
        assert_eq!(result, -10);
    }

    #[test]
    fn test_root_is_max() {
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));
        root.borrow_mut().right = Some(Node::new(3));

        let result = find_max(Some(root));
        assert_eq!(result, 10);
    }

    #[test]
    fn test_left_child_is_max() {
        let root = Node::new(5);
        root.borrow_mut().left = Some(Node::new(15));
        root.borrow_mut().right = Some(Node::new(3));

        let result = find_max(Some(root));
        assert_eq!(result, 15);
    }

    #[test]
    fn test_right_child_is_max() {
        let root = Node::new(5);
        root.borrow_mut().left = Some(Node::new(3));
        root.borrow_mut().right = Some(Node::new(20));

        let result = find_max(Some(root));
        assert_eq!(result, 20);
    }

    #[test]
    fn test_max_in_deep_left() {
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(2));
        left.borrow_mut().right = Some(Node::new(50));

        let result = find_max(Some(root));
        assert_eq!(result, 50);
    }

    #[test]
    fn test_max_in_deep_right() {
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));
        root.borrow_mut().right = Some(Node::new(15));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().left = Some(Node::new(12));
        right.borrow_mut().right = Some(Node::new(100));

        let result = find_max(Some(root));
        assert_eq!(result, 100);
    }

    #[test]
    fn test_all_negative_values() {
        let root = Node::new(-5);
        root.borrow_mut().left = Some(Node::new(-10));
        root.borrow_mut().right = Some(Node::new(-3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(-20));
        left.borrow_mut().right = Some(Node::new(-15));

        let result = find_max(Some(root));
        assert_eq!(result, -3);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let root = Node::new(-10);
        root.borrow_mut().left = Some(Node::new(5));
        root.borrow_mut().right = Some(Node::new(-3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(-20));
        left.borrow_mut().right = Some(Node::new(15));

        let result = find_max(Some(root));
        assert_eq!(result, 15);
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(20));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(30));

        let left_left = left.borrow().left.clone().unwrap();
        left_left.borrow_mut().left = Some(Node::new(40));

        let result = find_max(Some(root));
        assert_eq!(result, 40);
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = Node::new(10);
        root.borrow_mut().right = Some(Node::new(20));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().right = Some(Node::new(5));

        let right_right = right.borrow().right.clone().unwrap();
        right_right.borrow_mut().right = Some(Node::new(50));

        let result = find_max(Some(root));
        assert_eq!(result, 50);
    }

    #[test]
    fn test_complete_binary_tree() {
        let root = Node::new(25);
        root.borrow_mut().left = Some(Node::new(15));
        root.borrow_mut().right = Some(Node::new(30));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(10));
        left.borrow_mut().right = Some(Node::new(20));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().left = Some(Node::new(28));
        right.borrow_mut().right = Some(Node::new(35));

        let result = find_max(Some(root));
        assert_eq!(result, 35);
    }

    #[test]
    fn test_with_zero_values() {
        let root = Node::new(0);
        root.borrow_mut().left = Some(Node::new(-5));
        root.borrow_mut().right = Some(Node::new(-10));

        let result = find_max(Some(root));
        assert_eq!(result, 0);
    }
}
