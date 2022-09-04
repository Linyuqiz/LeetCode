#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("169、majority_element");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
