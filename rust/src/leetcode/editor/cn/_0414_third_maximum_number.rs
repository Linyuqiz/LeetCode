#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("414、third_maximum_number");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let mut mark = 0;
        for i in (0..nums.len()).rev() {
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                res = nums[i];
                mark += 1;
            }
            if mark == 3 {
                return res;
            }
        }
        return nums[nums.len() - 1];
    }
}
//leetcode submit region end(Prohibit modification and deletion)
