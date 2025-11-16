use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct Node {
    pub value: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

pub fn order_traversal(root: Rc<RefCell<Node>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let mut new_queue = VecDeque::new();
        let mut sub_result = Vec::new();
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            sub_result.push(borrowed.value);

            if let Some(ref left) = borrowed.left {
                new_queue.push_back(left.clone());
            }

            if let Some(ref right) = borrowed.right {
                new_queue.push_back(right.clone());
            }
        }

        queue = new_queue;
        result.push(sub_result);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_node() {
        let root = Node::new(1);
        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_two_levels() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn test_three_levels_complete() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));
        root.borrow_mut().right = Some(Node::new(3));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(4));
        left.borrow_mut().right = Some(Node::new(5));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().left = Some(Node::new(6));
        right.borrow_mut().right = Some(Node::new(7));

        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]);
    }

    #[test]
    fn test_left_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().left = Some(Node::new(2));

        let left = root.borrow().left.clone().unwrap();
        left.borrow_mut().left = Some(Node::new(3));

        let left_left = left.borrow().left.clone().unwrap();
        left_left.borrow_mut().left = Some(Node::new(4));

        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_right_skewed_tree() {
        let root = Node::new(1);
        root.borrow_mut().right = Some(Node::new(2));

        let right = root.borrow().right.clone().unwrap();
        right.borrow_mut().right = Some(Node::new(3));

        let right_right = right.borrow().right.clone().unwrap();
        right_right.borrow_mut().right = Some(Node::new(4));

        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1], vec![2], vec![3], vec![4]]);
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

        let result = order_traversal(root);
        assert_eq!(result, vec![vec![1], vec![2, 3], vec![4, 5], vec![6]]);
    }
}
