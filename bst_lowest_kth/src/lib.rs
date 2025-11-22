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

pub fn find_lowest_kth(root: Option<Rc<RefCell<Node>>>, k: usize) -> Option<i32> {
    let mut result = None;
    let mut count = 0;
    lowest_kth(root, k, &mut result, &mut count);
    result
}

fn lowest_kth(
    node: Option<Rc<RefCell<Node>>>,
    k: usize,
    result: &mut Option<i32>,
    count: &mut usize,
) {
    match node {
        None => return,
        Some(node) => {
            let borrowed = node.borrow();

            lowest_kth(borrowed.left.clone(), k, result, count);

            *count += 1;
            if *count == k {
                *result = Some(borrowed.value);
                return;
            }

            lowest_kth(borrowed.right.clone(), k, result, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a BST from values
    fn insert(root: &mut Option<Rc<RefCell<Node>>>, value: i32) {
        match root {
            None => *root = Some(Node::new(value)),
            Some(node) => {
                let mut borrowed = node.borrow_mut();
                if value < borrowed.value {
                    insert(&mut borrowed.left, value);
                } else {
                    insert(&mut borrowed.right, value);
                }
            }
        }
    }

    fn build_bst(values: &[i32]) -> Option<Rc<RefCell<Node>>> {
        let mut root = None;
        for &v in values {
            insert(&mut root, v);
        }
        root
    }

    #[test]
    fn test_find_1st_smallest() {
        let root = build_bst(&[5, 3, 7, 2, 4, 6, 8]);
        assert_eq!(find_lowest_kth(root, 1), Some(2));
    }

    #[test]
    fn test_find_3rd_smallest() {
        let root = build_bst(&[5, 3, 7, 2, 4, 6, 8]);
        assert_eq!(find_lowest_kth(root, 3), Some(4));
    }

    #[test]
    fn test_find_last_element() {
        let root = build_bst(&[5, 3, 7, 2, 4, 6, 8]);
        assert_eq!(find_lowest_kth(root, 7), Some(8));
    }

    #[test]
    fn test_k_exceeds_size() {
        let root = build_bst(&[5, 3, 7]);
        assert_eq!(find_lowest_kth(root, 10), None);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(find_lowest_kth(None, 1), None);
    }

    #[test]
    fn test_single_node() {
        let root = build_bst(&[42]);
        assert_eq!(find_lowest_kth(root, 1), Some(42));
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = build_bst(&[5, 4, 3, 2, 1]);
        assert_eq!(find_lowest_kth(root, 3), Some(3));
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = build_bst(&[1, 2, 3, 4, 5]);
        assert_eq!(find_lowest_kth(root, 4), Some(4));
    }

    #[test]
    fn test_k_zero() {
        let root = build_bst(&[5, 3, 7]);
        assert_eq!(find_lowest_kth(root, 0), None);
    }

    #[test]
    fn test_negative_values() {
        let root = build_bst(&[0, -5, 5, -10, -3, 3, 10]);
        assert_eq!(find_lowest_kth(root, 1), Some(-10));
        let root2 = build_bst(&[0, -5, 5, -10, -3, 3, 10]);
        assert_eq!(find_lowest_kth(root2, 4), Some(0));
    }
}
