// This is the implementation of binary search algorithm in Rust.
pub fn binary_search(arr: &[i32], target: i32) -> Option<i32> {
    if arr.len() == 0 {
        return None;
    }

    let (mut left, mut right) = (0, arr.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid as i32);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 3), Some(2));
        assert_eq!(binary_search(&arr, 6), None);
    }
}
