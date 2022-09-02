#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("217、contains_duplicate");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        nums.iter().any(|&x| !set.insert(x))
    }

    #[allow(dead_code)]
    pub fn contains_duplicate0(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        let (mut left, length) = (1, nums.len());
        while left < length {
            if nums[left - 1] == nums[left] {
                return true;
            }
            left += 1;
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
