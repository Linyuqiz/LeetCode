// This is a Rust implementation of the linear search algorithm.
pub fn linear_search(arr: &[i32], target: i32) -> Option<i32> {
    if arr.len() == 0 {
        return None;
    }

    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i as i32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search(&arr, 3), Some(2));
        assert_eq!(linear_search(&arr, 6), None);
    }
}
