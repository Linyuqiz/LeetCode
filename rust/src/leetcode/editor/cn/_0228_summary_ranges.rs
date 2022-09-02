#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("228、summary_ranges");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let (mut res, mut left, mut right) = (vec![], 0, 1);
        while left < nums.len() {
            while right < nums.len() && nums[right - 1] + 1 == nums[right] {
                right += 1;
            }
            let mut content = nums[left].to_string();
            if left + 1 != right {
                content += &*("->".to_owned() + &(nums[right - 1].to_string()));
            }
            res.push(content);
            left = right;
            right = left + 1;
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
