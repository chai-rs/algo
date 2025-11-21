#![allow(dead_code)]

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

pub fn delete(root: Rc<RefCell<Node>>, target: i32) {
    let mut parent: Option<Rc<RefCell<Node>>> = None;
    let mut cur = root.clone();
    let mut is_left_child = false;

    loop {
        let value = cur.borrow().value; // Borrow ends at end of this statement

        if value == target {
            break;
        } else if value > target {
            parent = Some(cur.clone());
            is_left_child = true;
            let next = cur.borrow().left.clone();
            cur = match next {
                Some(n) => n,
                None => return,
            }
        } else {
            parent = Some(cur.clone());
            is_left_child = false;
            let next = cur.borrow().right.clone();
            cur = match next {
                Some(n) => n,
                None => return,
            }
        }
    }

    let mut borrowed = cur.borrow_mut();
    match (borrowed.left.clone(), borrowed.right.clone()) {
        (None, None) => {
            drop(borrowed);
            if let Some(parent) = parent {
                if is_left_child {
                    parent.borrow_mut().left = None;
                } else {
                    parent.borrow_mut().right = None;
                }
            }
        }
        (Some(left), None) => {
            if let Some(parent) = parent {
                if is_left_child {
                    parent.borrow_mut().left = Some(left.clone());
                } else {
                    parent.borrow_mut().right = Some(left.clone());
                }
                drop(borrowed)
            } else {
                let child = left.borrow();
                borrowed.left = child.left.clone();
                borrowed.right = child.right.clone();
                borrowed.value = child.value;
            }
        }
        (None, Some(right)) => {
            if let Some(parent) = parent {
                if is_left_child {
                    parent.borrow_mut().left = Some(right.clone());
                } else {
                    parent.borrow_mut().right = Some(right.clone());
                }
                drop(borrowed)
            } else {
                let child = right.borrow();
                borrowed.left = child.left.clone();
                borrowed.right = child.right.clone();
                borrowed.value = child.value;
            }
        }
        (Some(_), Some(right)) => {
            let mut successor_parent = cur.clone();
            let mut successor = right.clone();

            loop {
                let left_child = successor.borrow().left.clone();
                match left_child {
                    Some(node) => {
                        successor_parent = successor.clone();
                        successor = node
                    }
                    None => break,
                }
            }

            let successor_value = successor.borrow().value;
            let successor_right = successor.borrow().right.clone();

            if Rc::ptr_eq(&cur, &successor_parent) {
                borrowed.right = successor_right;
            } else {
                successor_parent.borrow_mut().left = successor_right;
            }

            borrowed.value = successor_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a simple BST
    fn build_tree(values: &[i32]) -> Rc<RefCell<Node>> {
        let root = Node::new(values[0]);
        for &val in &values[1..] {
            insert(root.clone(), val);
        }
        root
    }

    fn insert(root: Rc<RefCell<Node>>, value: i32) {
        let mut cur = root;
        loop {
            let cur_val = cur.borrow().value;
            if value < cur_val {
                let next = cur.borrow().left.clone();
                match next {
                    Some(n) => cur = n,
                    None => {
                        cur.borrow_mut().left = Some(Node::new(value));
                        return;
                    }
                }
            } else {
                let next = cur.borrow().right.clone();
                match next {
                    Some(n) => cur = n,
                    None => {
                        cur.borrow_mut().right = Some(Node::new(value));
                        return;
                    }
                }
            }
        }
    }

    #[test]
    fn test_delete_leaf_left_child() {
        // Tree: 10 -> 5
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));

        delete(root.clone(), 5);

        assert!(root.borrow().left.is_none());
    }

    #[test]
    fn test_delete_leaf_right_child() {
        // Tree: 10 -> 15
        let root = Node::new(10);
        root.borrow_mut().right = Some(Node::new(15));

        delete(root.clone(), 15);

        assert!(root.borrow().right.is_none());
    }

    #[test]
    fn test_delete_node_with_only_left_child() {
        // Tree: 10 -> 5 -> 3
        let root = Node::new(10);
        let five = Node::new(5);
        five.borrow_mut().left = Some(Node::new(3));
        root.borrow_mut().left = Some(five);

        delete(root.clone(), 5);

        let left = root.borrow().left.clone().unwrap();
        assert_eq!(left.borrow().value, 3);
    }

    #[test]
    fn test_delete_node_with_only_right_child() {
        // Tree: 10 -> 15 -> 20
        let root = Node::new(10);
        let fifteen = Node::new(15);
        fifteen.borrow_mut().right = Some(Node::new(20));
        root.borrow_mut().right = Some(fifteen);

        delete(root.clone(), 15);

        let right = root.borrow().right.clone().unwrap();
        assert_eq!(right.borrow().value, 20);
    }

    #[test]
    fn test_delete_root_leaf() {
        // Single node tree
        let root = Node::new(10);
        delete(root.clone(), 10);
        // Root still exists but is now empty (no change since no parent)
        assert_eq!(root.borrow().value, 10);
    }

    #[test]
    fn test_delete_root_with_only_left_child() {
        // Tree: 10 -> 5
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));

        delete(root.clone(), 10);

        // Root should now have value 5
        assert_eq!(root.borrow().value, 5);
        assert!(root.borrow().left.is_none());
    }

    #[test]
    fn test_delete_root_with_only_right_child() {
        // Tree: 10 -> 15
        let root = Node::new(10);
        root.borrow_mut().right = Some(Node::new(15));

        delete(root.clone(), 10);

        // Root should now have value 15
        assert_eq!(root.borrow().value, 15);
        assert!(root.borrow().right.is_none());
    }

    #[test]
    fn test_delete_nonexistent_value() {
        let root = Node::new(10);
        root.borrow_mut().left = Some(Node::new(5));

        delete(root.clone(), 100);

        // Tree should be unchanged
        assert_eq!(root.borrow().value, 10);
        assert!(root.borrow().left.is_some());
    }

    #[test]
    fn test_delete_node_with_two_children_successor_is_right_child() {
        //     10
        //    /  \
        //   5    15
        //       /  \
        //      12   20
        // Delete 15: successor is 20 (direct right child)
        let root = build_tree(&[10, 5, 15, 12, 20]);

        delete(root.clone(), 15);

        let right = root.borrow().right.clone().unwrap();
        assert_eq!(right.borrow().value, 20);
        assert_eq!(right.borrow().left.clone().unwrap().borrow().value, 12);
        assert!(right.borrow().right.is_none());
    }

    #[test]
    fn test_delete_node_with_two_children_successor_is_deeper() {
        //     10
        //    /  \
        //   5    20
        //       /  \
        //      15   25
        //     /
        //    12
        // Delete 20: successor is 25, but wait - let me think again
        // Actually successor of 20 is 25's leftmost, which is 25 itself
        // Let me create a better tree:
        //     10
        //    /  \
        //   5    20
        //       /  \
        //      15   25
        //        \
        //        17
        // Delete 20: successor is 25 (no left child)
        let root = build_tree(&[10, 5, 20, 15, 25, 17]);

        delete(root.clone(), 20);

        let right = root.borrow().right.clone().unwrap();
        assert_eq!(right.borrow().value, 25);
        assert_eq!(right.borrow().left.clone().unwrap().borrow().value, 15);
    }

    #[test]
    fn test_delete_root_with_two_children() {
        //     10
        //    /  \
        //   5    15
        // Delete 10: successor is 15
        let root = build_tree(&[10, 5, 15]);

        delete(root.clone(), 10);

        assert_eq!(root.borrow().value, 15);
        assert_eq!(root.borrow().left.clone().unwrap().borrow().value, 5);
        assert!(root.borrow().right.is_none());
    }

    #[test]
    fn test_delete_node_with_two_children_successor_has_right_subtree() {
        //       10
        //      /  \
        //     5    20
        //         /  \
        //        15   30
        //       /  \
        //      12   17
        // Delete 20: successor is 30, right subtree stays
        // Wait no - in-order successor is leftmost in right subtree
        // So delete 20: go right to 30, no left child, so successor is 30
        let root = build_tree(&[10, 5, 20, 15, 30, 12, 17]);

        delete(root.clone(), 20);

        let right = root.borrow().right.clone().unwrap();
        assert_eq!(right.borrow().value, 30);
        // 15 should still be left child of 30
        let fifteen = right.borrow().left.clone().unwrap();
        assert_eq!(fifteen.borrow().value, 15);
    }
}
