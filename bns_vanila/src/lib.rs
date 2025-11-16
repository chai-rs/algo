#[allow(dead_code)]
fn binary_search(nums: &[i32], target: i32) -> i32 {
    let size = nums.len();

    if size == 0 {
        return -1;
    }

    let (mut left, mut right) = (0, size as i32 - 1);

    while left <= right {
        let mid = (left + (right - left) / 2) as usize;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] < target {
            left = mid as i32 + 1;
        } else {
            right = mid as i32 - 1;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use crate::binary_search;

    #[test]
    fn test_binary_search_found_middle() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 4;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 3);
    }

    #[test]
    fn test_binary_search_found_first() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 1;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_binary_search_found_last() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 6;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 5);
    }

    #[test]
    fn test_binary_search_not_found_too_small() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 0;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_not_found_too_large() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 7;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_not_found_between() {
        let nums = vec![1, 3, 5, 7, 9];
        let target = 4;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_single_element_found() {
        let nums = vec![42];
        let target = 42;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_binary_search_single_element_not_found() {
        let nums = vec![42];
        let target = 1;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let nums = vec![];
        let target = 1;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_two_elements_found_first() {
        let nums = vec![1, 2];
        let target = 1;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_binary_search_two_elements_found_second() {
        let nums = vec![1, 2];
        let target = 2;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_binary_search_two_elements_not_found() {
        let nums = vec![1, 3];
        let target = 2;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_binary_search_large_array() {
        let nums: Vec<i32> = (0..1000).collect();
        let target = 567;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 567);
    }

    #[test]
    fn test_binary_search_negative_numbers() {
        let nums = vec![-10, -5, 0, 5, 10];
        let target = -5;
        let idx = binary_search(&nums, target);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_binary_search_duplicates() {
        let nums = vec![1, 2, 2, 2, 3];
        let target = 2;
        let idx = binary_search(&nums, target);
        println!("{:?}", idx);
        // Should find any occurrence of 2 (indices 1, 2, or 3 are all valid)
        assert!(idx >= 1 && idx <= 3);
    }
}
