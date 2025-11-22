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

pub fn lca_safe(node: Option<Rc<RefCell<Node>>>, p: i32, q: i32) -> Option<Rc<RefCell<Node>>> {
    if !search(node.clone(), p) || !search(node.clone(), q) {
        return None;
    }
    lca(node, p, q)
}

pub fn lca(node: Option<Rc<RefCell<Node>>>, p: i32, q: i32) -> Option<Rc<RefCell<Node>>> {
    match node {
        Some(node) => {
            let value = node.borrow().value;
            if p < value && q < value {
                return lca(node.borrow().left.clone(), p, q);
            }
            if p > value && q > value {
                return lca(node.borrow().right.clone(), p, q);
            }
            Some(node)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a BST:
    //        6
    //       / \
    //      2   8
    //     / \ / \
    //    0  4 7  9
    //      / \
    //     3   5
    fn build_test_tree() -> Rc<RefCell<Node>> {
        let root = Node::new(6);
        let n2 = Node::new(2);
        let n8 = Node::new(8);
        let n0 = Node::new(0);
        let n4 = Node::new(4);
        let n7 = Node::new(7);
        let n9 = Node::new(9);
        let n3 = Node::new(3);
        let n5 = Node::new(5);

        n4.borrow_mut().left = Some(n3);
        n4.borrow_mut().right = Some(n5);
        n2.borrow_mut().left = Some(n0);
        n2.borrow_mut().right = Some(n4);
        n8.borrow_mut().left = Some(n7);
        n8.borrow_mut().right = Some(n9);
        root.borrow_mut().left = Some(n2);
        root.borrow_mut().right = Some(n8);

        root
    }

    #[test]
    fn test_lca_different_subtrees() {
        let root = build_test_tree();
        let result = lca(Some(root), 2, 8);
        assert_eq!(result.unwrap().borrow().value, 6);
    }

    #[test]
    fn test_lca_same_subtree_left() {
        let root = build_test_tree();
        let result = lca(Some(root), 0, 4);
        assert_eq!(result.unwrap().borrow().value, 2);
    }

    #[test]
    fn test_lca_same_subtree_right() {
        let root = build_test_tree();
        let result = lca(Some(root), 7, 9);
        assert_eq!(result.unwrap().borrow().value, 8);
    }

    #[test]
    fn test_lca_one_is_ancestor() {
        let root = build_test_tree();
        let result = lca(Some(root), 2, 4);
        assert_eq!(result.unwrap().borrow().value, 2);
    }

    #[test]
    fn test_lca_deep_nodes() {
        let root = build_test_tree();
        let result = lca(Some(root), 3, 5);
        assert_eq!(result.unwrap().borrow().value, 4);
    }

    #[test]
    fn test_lca_with_root() {
        let root = build_test_tree();
        let result = lca(Some(root), 0, 9);
        assert_eq!(result.unwrap().borrow().value, 6);
    }

    #[test]
    fn test_lca_empty_tree() {
        let result = lca(None, 1, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_lca_reversed_order() {
        let root = build_test_tree();
        let result = lca(Some(root), 8, 2);
        assert_eq!(result.unwrap().borrow().value, 6);
    }
}
