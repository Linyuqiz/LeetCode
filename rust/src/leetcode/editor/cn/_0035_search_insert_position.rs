#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("35、search_insert_position");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, nums.len() - 1);
        // 处理边界：usize 类型时，0-1 会报 overflow 错误
        if nums[0] > target {
            return 0;
        }
        while left <= right {
            let mid = (left + right) >> 1;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }

    #[allow(dead_code)]
    fn search_insert0(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
