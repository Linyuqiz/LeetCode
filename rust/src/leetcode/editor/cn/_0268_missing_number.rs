#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("268、missing_number");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let (mut res, length): (i32, i32) = (0, nums.len() as i32);
        for i in 0..length {
            res ^= i ^ nums[i as usize];
        }
        return res ^ length;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
