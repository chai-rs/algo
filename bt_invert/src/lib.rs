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

pub fn invert(node: Option<Rc<RefCell<Node>>>) {
    let node = match node {
        Some(node) => node,
        None => return,
    };

    // borrow & post-order
    let mut borrowed = node.borrow_mut();
    invert(borrowed.left.clone());
    invert(borrowed.right.clone());

    // invert
    let tmp = borrowed.left.clone();
    borrowed.left = borrowed.right.clone();
    borrowed.right = tmp;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_tree() {
        invert(None);
        // Should not panic
    }

    #[test]
    fn test_single_node() {
        let root = Node::new(1);
        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert!(borrowed.left.is_none());
        assert!(borrowed.right.is_none());
    }

    #[test]
    fn test_two_nodes_left_only() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));

        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert!(borrowed.left.is_none());
        assert!(borrowed.right.is_some());
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, 2);
    }

    #[test]
    fn test_two_nodes_right_only() {
        let root = Node::new(1);
        root.borrow_mut().right = Some(Node::new(3));

        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert!(borrowed.right.is_none());
        assert!(borrowed.left.is_some());
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, 3);
    }

    #[test]
    fn test_three_nodes() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, 3);
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, 2);
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

        invert(Some(root.clone()));

        // Check root
        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, 3);
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, 2);

        // Check left subtree (was right)
        let new_left = borrowed.left.clone().unwrap();
        let new_left_borrowed = new_left.borrow();
        assert_eq!(new_left_borrowed.left.as_ref().unwrap().borrow().value, 7);
        assert_eq!(new_left_borrowed.right.as_ref().unwrap().borrow().value, 6);

        // Check right subtree (was left)
        let new_right = borrowed.right.clone().unwrap();
        let new_right_borrowed = new_right.borrow();
        assert_eq!(new_right_borrowed.left.as_ref().unwrap().borrow().value, 5);
        assert_eq!(new_right_borrowed.right.as_ref().unwrap().borrow().value, 4);
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(3));

        let left_left = left.borrow().left.clone().unwrap();
        left_left.borrow_mut().left = Some(Node::new(4));

        invert(Some(root.clone()));

        // After inversion, should be right-skewed
        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert!(borrowed.left.is_none());
        assert!(borrowed.right.is_some());

        let right = borrowed.right.clone().unwrap();
        let right_borrowed = right.borrow();
        assert_eq!(right_borrowed.value, 2);
        assert!(right_borrowed.left.is_none());
        assert!(right_borrowed.right.is_some());

        let right_right = right_borrowed.right.clone().unwrap();
        let right_right_borrowed = right_right.borrow();
        assert_eq!(right_right_borrowed.value, 3);
        assert!(right_right_borrowed.left.is_none());
        assert!(right_right_borrowed.right.is_some());
        assert_eq!(
            right_right_borrowed.right.as_ref().unwrap().borrow().value,
            4
        );
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().right = Some(Node::new(2));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().right = Some(Node::new(3));

        invert(Some(root.clone()));

        // After inversion, should be left-skewed
        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert!(borrowed.right.is_none());
        assert!(borrowed.left.is_some());

        let left = borrowed.left.clone().unwrap();
        let left_borrowed = left.borrow();
        assert_eq!(left_borrowed.value, 2);
        assert!(left_borrowed.right.is_none());
        assert!(left_borrowed.left.is_some());
        assert_eq!(left_borrowed.left.as_ref().unwrap().borrow().value, 3);
    }

    #[test]
    fn test_unbalanced_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().right = Some(Node::new(6));

        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, 3);
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, 2);

        // Check new left (was right with only right child)
        let new_left = borrowed.left.clone().unwrap();
        let new_left_borrowed = new_left.borrow();
        assert!(new_left_borrowed.right.is_none());
        assert_eq!(new_left_borrowed.left.as_ref().unwrap().borrow().value, 6);

        // Check new right (was left with both children)
        let new_right = borrowed.right.clone().unwrap();
        let new_right_borrowed = new_right.borrow();
        assert_eq!(new_right_borrowed.left.as_ref().unwrap().borrow().value, 5);
        assert_eq!(new_right_borrowed.right.as_ref().unwrap().borrow().value, 4);
    }

    #[test]
    fn test_double_invert() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        // Invert twice should return to original
        invert(Some(root.clone()));
        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, 1);
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, 2);
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, 3);

        let left_node = borrowed.left.clone().unwrap();
        let left_borrowed = left_node.borrow();
        assert_eq!(left_borrowed.left.as_ref().unwrap().borrow().value, 4);
        assert_eq!(left_borrowed.right.as_ref().unwrap().borrow().value, 5);
    }

    #[test]
    fn test_negative_values() {
        let root = Node::new(-1);
        root.borrow_mut().left = Some(Node::new(-2));
        root.borrow_mut().right = Some(Node::new(-3));

        invert(Some(root.clone()));

        let borrowed = root.borrow();
        assert_eq!(borrowed.value, -1);
        assert_eq!(borrowed.left.as_ref().unwrap().borrow().value, -3);
        assert_eq!(borrowed.right.as_ref().unwrap().borrow().value, -2);
    }
}
