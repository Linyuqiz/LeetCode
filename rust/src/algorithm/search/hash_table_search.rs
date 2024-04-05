// This is a Rust implementation of the hash table search algorithm.
pub fn hash_table_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut hash_table = std::collections::HashMap::new();
    for i in 0..nums.len() {
        hash_table.insert(nums[i], i);
    }
    if let Some(index) = hash_table.get(&target) {
        *index as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(hash_table_search(nums, 0), 4);
    }
}
