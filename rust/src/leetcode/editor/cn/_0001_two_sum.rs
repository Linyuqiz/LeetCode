#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        println!("result: {:?}", result);
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            if let Some(k) = map.get(&(target - v)) {
                return vec![*k, i as i32];
            }
            map.insert(*v, i as i32);
        }
        return vec![];
    }
}
//leetcode submit region end(Prohibit modification and deletion)
