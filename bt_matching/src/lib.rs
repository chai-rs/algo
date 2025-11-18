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

pub fn matching(a: Option<Rc<RefCell<Node>>>, b: Option<Rc<RefCell<Node>>>) -> bool {
    let (a, b) = if a.is_none() && b.is_none() {
        return true;
    } else if a.is_none() || b.is_none() {
        return false;
    } else {
        (a.unwrap(), b.unwrap())
    };

    let borrowed_a = a.borrow();
    let borrowed_b = b.borrow();
    if borrowed_a.value != borrowed_b.value {
        return false;
    }

    let left = matching(borrowed_a.left.clone(), borrowed_b.left.clone());
    let right = matching(borrowed_a.right.clone(), borrowed_b.right.clone());
    left && right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_both_empty() {
        let result = matching(None, None);
        assert_eq!(result, true);
    }

    #[test]
    fn test_first_empty_second_not() {
        let b = Node::new(1);
        let result = matching(None, Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_second_empty_first_not() {
        let a = Node::new(1);
        let result = matching(Some(a), None);
        assert_eq!(result, false);
    }

    #[test]
    fn test_single_nodes_matching() {
        let a = Node::new(5);
        let b = Node::new(5);
        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }

    #[test]
    fn test_single_nodes_not_matching() {
        let a = Node::new(5);
        let b = Node::new(10);
        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_identical_trees_two_levels() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(2));
        b.borrow_mut().right = Some(Node::new(3));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }

    #[test]
    fn test_different_root_values() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(10);
        b.borrow_mut().left = Some(Node::new(2));
        b.borrow_mut().right = Some(Node::new(3));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_different_left_child() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(5));
        b.borrow_mut().right = Some(Node::new(3));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_different_right_child() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(2));
        b.borrow_mut().right = Some(Node::new(7));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_one_has_left_child_other_doesnt() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));

        let b = Node::new(1);

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_one_has_right_child_other_doesnt() {
        let a = Node::new(1);
        a.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(1);

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_identical_complete_tree() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let a_left = a.borrow().left.clone().unwrap();
        a_left.borrow_mut().left = Some(Node::new(4));
        a_left.borrow_mut().right = Some(Node::new(5));

        let a_right = a.borrow().right.clone().unwrap();
        a_right.borrow_mut().left = Some(Node::new(6));
        a_right.borrow_mut().right = Some(Node::new(7));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(2));
        b.borrow_mut().right = Some(Node::new(3));

        let b_left = b.borrow().left.clone().unwrap();
        b_left.borrow_mut().left = Some(Node::new(4));
        b_left.borrow_mut().right = Some(Node::new(5));

        let b_right = b.borrow().right.clone().unwrap();
        b_right.borrow_mut().left = Some(Node::new(6));
        b_right.borrow_mut().right = Some(Node::new(7));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }

    #[test]
    fn test_deep_difference() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));
        a.borrow_mut().right = Some(Node::new(3));

        let a_left = a.borrow().left.clone().unwrap();
        a_left.borrow_mut().left = Some(Node::new(4));
        a_left.borrow_mut().right = Some(Node::new(5));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(2));
        b.borrow_mut().right = Some(Node::new(3));

        let b_left = b.borrow().left.clone().unwrap();
        b_left.borrow_mut().left = Some(Node::new(4));
        b_left.borrow_mut().right = Some(Node::new(99));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, false);
    }

    #[test]
    fn test_left_skewed_identical() {
        let a = Node::new(1);
        a.borrow_mut().left = Some(Node::new(2));

        let a_left = a.borrow().left.clone().unwrap();
        a_left.borrow_mut().left = Some(Node::new(3));

        let a_left_left = a_left.borrow().left.clone().unwrap();
        a_left_left.borrow_mut().left = Some(Node::new(4));

        let b = Node::new(1);
        b.borrow_mut().left = Some(Node::new(2));

        let b_left = b.borrow().left.clone().unwrap();
        b_left.borrow_mut().left = Some(Node::new(3));

        let b_left_left = b_left.borrow().left.clone().unwrap();
        b_left_left.borrow_mut().left = Some(Node::new(4));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }

    #[test]
    fn test_right_skewed_identical() {
        let a = Node::new(1);
        a.borrow_mut().right = Some(Node::new(2));

        let a_right = a.borrow().right.clone().unwrap();
        a_right.borrow_mut().right = Some(Node::new(3));

        let b = Node::new(1);
        b.borrow_mut().right = Some(Node::new(2));

        let b_right = b.borrow().right.clone().unwrap();
        b_right.borrow_mut().right = Some(Node::new(3));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }

    #[test]
    fn test_negative_values_matching() {
        let a = Node::new(-5);
        a.borrow_mut().left = Some(Node::new(-10));
        a.borrow_mut().right = Some(Node::new(-3));

        let b = Node::new(-5);
        b.borrow_mut().left = Some(Node::new(-10));
        b.borrow_mut().right = Some(Node::new(-3));

        let result = matching(Some(a), Some(b));
        assert_eq!(result, true);
    }
}
