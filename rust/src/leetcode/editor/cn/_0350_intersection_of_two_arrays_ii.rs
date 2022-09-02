#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("350、intersection_of_two_arrays_ii");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let (mut left, mut right, mut res) = (0, 0, vec![]);
        while left < nums1.len() && right < nums2.len() {
            let (x, y) = (nums1[left], nums2[right]);
            if x == y {
                res.push(x);
                left += 1;
                right += 1;
            } else if x < y {
                left += 1;
            } else {
                right += 1;
            }
        }
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
