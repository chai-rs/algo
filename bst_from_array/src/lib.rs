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

/// Converts a sorted array into a height-balanced BST
///
/// # Arguments
/// * `nums` - A sorted array in ascending order
///
/// # Returns
/// * Root of the height-balanced BST, or None if array is empty
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
    build(&nums, 0, nums.len())
}

// Helper function (optional)
fn build(nums: &[i32], left: usize, right: usize) -> Option<Rc<RefCell<Node>>> {
    if right <= left {
        return None;
    }

    let mid = (left + right) / 2;
    let left = build(nums, left, mid);
    let right = build(nums, mid + 1, right);

    let node = Node::new(nums[mid]);
    {
        let mut borrowed = node.borrow_mut();
        borrowed.left = left;
        borrowed.right = right;
    }

    Some(node)
}

#[cfg(test)]
mod test {
    use crate::{Node, sorted_array_to_bst};
    use std::{cell::RefCell, rc::Rc};

    fn inorder(node: Option<Rc<RefCell<Node>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let borrowed = n.borrow();
            inorder(borrowed.left.clone(), result);
            result.push(borrowed.value);
            inorder(borrowed.right.clone(), result);
        }
    }

    fn height(node: Option<Rc<RefCell<Node>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let borrowed = n.borrow();
                1 + height(borrowed.left.clone()).max(height(borrowed.right.clone()))
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let result = sorted_array_to_bst(vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_element() {
        let result = sorted_array_to_bst(vec![1]);
        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(node.borrow().value, 1);
        assert!(node.borrow().left.is_none());
        assert!(node.borrow().right.is_none());
    }

    #[test]
    fn test_two_elements() {
        let result = sorted_array_to_bst(vec![1, 2]);
        assert!(result.is_some());
        let mut values = Vec::new();
        inorder(result, &mut values);
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_preserves_sorted_order() {
        let nums = vec![1, 2, 4, 5, 7, 8, 10];
        let result = sorted_array_to_bst(nums.clone());
        let mut values = Vec::new();
        inorder(result, &mut values);
        assert_eq!(values, nums);
    }

    #[test]
    fn test_balanced_tree() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let result = sorted_array_to_bst(nums);
        assert!(result.is_some());
        let root = result.unwrap();
        let borrowed = root.borrow();
        let left_height = height(borrowed.left.clone());
        let right_height = height(borrowed.right.clone());
        assert!((left_height - right_height).abs() <= 1);
    }

    #[test]
    fn test_root_is_middle() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = sorted_array_to_bst(nums);
        assert_eq!(result.unwrap().borrow().value, 3);
    }
}
