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

pub fn find_total_nodes(root: Option<Rc<RefCell<Node>>>) -> usize {
    let root = match root {
        Some(root) => root,
        None => return 0,
    };

    let borrowed = root.borrow();
    find_total_nodes(borrowed.left.clone()) + find_total_nodes(borrowed.right.clone()) + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let result = find_total_nodes(None);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_node() {
        let root = Node::new(1);
        let result = find_total_nodes(Some(root));
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_nodes_left() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_two_nodes_right() {
        let root = Node::new(1);
        root.borrow_mut().right = Some(Node::new(2));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_three_nodes() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_complete_binary_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().left = Some(Node::new(6));
        right.borrow_mut().right = Some(Node::new(7));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 7);
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(3));

        let left_left = left.borrow().left.clone().unwrap();
        left_left.borrow_mut().left = Some(Node::new(4));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().right = Some(Node::new(2));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().right = Some(Node::new(3));

        let right_right = right.borrow().right.clone().unwrap();
        right_right.borrow_mut().right = Some(Node::new(4));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_unbalanced_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        let left_right = left.borrow().right.clone().unwrap();
        left_right.borrow_mut().left = Some(Node::new(6));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_large_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().left = Some(Node::new(6));
        right.borrow_mut().right = Some(Node::new(7));

        let left_left = left.borrow().left.clone().unwrap();
        left_left.borrow_mut().left = Some(Node::new(8));
        left_left.borrow_mut().right = Some(Node::new(9));

        let left_right = left.borrow().right.clone().unwrap();
        left_right.borrow_mut().left = Some(Node::new(10));

        let result = find_total_nodes(Some(root));
        assert_eq!(result, 10);
    }
}
