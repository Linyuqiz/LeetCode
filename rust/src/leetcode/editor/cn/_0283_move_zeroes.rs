#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("283、move_zeroes");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut left, mut right, length) = (0, 0, nums.len());
        while right < length {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
            right += 1;
        }
    }
    #[allow(dead_code)]
    pub fn move_zeroes0(nums: &mut Vec<i32>) {
        let length = nums.len();
        nums.retain(|x| *x != 0);
        nums.resize(length, 0)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
