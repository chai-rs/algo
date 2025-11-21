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

pub fn insert(node: Rc<RefCell<Node>>, value: i32) {
    let mut borrowed = node.borrow_mut();
    if borrowed.value >= value {
        match borrowed.left.clone() {
            Some(node) => insert(node, value),
            None => borrowed.left = Some(Node::new(value)),
        }
    } else {
        match borrowed.right.clone() {
            Some(node) => insert(node, value),
            None => borrowed.right = Some(Node::new(value)),
        }
    }
}

pub fn search(node: Option<Rc<RefCell<Node>>>, target: i32) -> bool {
    let node = if let Some(node) = node {
        if node.borrow().value == target {
            return true;
        }
        node
    } else {
        return false;
    };

    let borrowed = node.borrow();
    if borrowed.value >= target {
        search(borrowed.left.clone(), target)
    } else {
        search(borrowed.right.clone(), target)
    }
}

#[cfg(test)]
mod test_insert {
    use super::*;

    #[test]
    fn inserts_left_and_right_children() {
        let root = Node::new(10);

        insert(root.clone(), 5);
        insert(root.clone(), 15);

        let left_child = {
            let borrowed = root.borrow();
            borrowed.left.clone()
        }
        .expect("left child should exist");
        let right_child = {
            let borrowed = root.borrow();
            borrowed.right.clone()
        }
        .expect("right child should exist");

        assert_eq!(left_child.borrow().value, 5);
        assert_eq!(right_child.borrow().value, 15);
    }

    #[test]
    fn inserts_duplicates_to_left_subtree() {
        let root = Node::new(20);

        insert(root.clone(), 20);
        insert(root.clone(), 20);

        let left = {
            let borrowed = root.borrow();
            borrowed.left.clone()
        }
        .expect("first duplicate should be on the left");
        let left_left = {
            let borrowed = left.borrow();
            borrowed.left.clone()
        }
        .expect("second duplicate should go further left");

        assert_eq!(left.borrow().value, 20);
        assert_eq!(left_left.borrow().value, 20);
    }

    #[test]
    fn inserts_recursively_into_subtrees() {
        let root = Node::new(8);

        insert(root.clone(), 3);
        insert(root.clone(), 1);
        insert(root.clone(), 6);
        insert(root.clone(), 4);

        let left = root.borrow().left.clone().expect("left child missing");
        let right_of_left = {
            let borrowed = left.borrow();
            borrowed.right.clone()
        }
        .expect("right child of left subtree missing");

        assert_eq!(left.borrow().value, 3);
        assert_eq!(right_of_left.borrow().value, 6);
        assert_eq!(
            right_of_left
                .borrow()
                .left
                .clone()
                .expect("left child of node value 6 missing")
                .borrow()
                .value,
            4
        );
    }
}

#[cfg(test)]
mod test_search {
    use super::*;

    fn build_tree(values: &[i32]) -> Rc<RefCell<Node>> {
        let root = Node::new(values[0]);
        for &value in &values[1..] {
            insert(root.clone(), value);
        }
        root
    }

    #[test]
    fn finds_existing_values() {
        let root = build_tree(&[10, 5, 15, 3, 7, 12, 18]);

        assert!(search(Some(root.clone()), 10));
        assert!(search(Some(root.clone()), 3));
        assert!(search(Some(root), 18));
    }

    #[test]
    fn returns_false_for_missing_value() {
        let root = build_tree(&[8, 4, 12, 2, 6, 10, 14]);

        assert!(!search(Some(root.clone()), 5));
        assert!(!search(Some(root.clone()), 13));
        assert!(!search(Some(root), 1));
    }

    #[test]
    fn returns_false_for_empty_tree() {
        assert!(!search(None, 42));
    }
}
