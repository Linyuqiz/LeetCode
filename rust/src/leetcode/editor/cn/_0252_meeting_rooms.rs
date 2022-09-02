#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("252、meeting_rooms");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        for i in 1..intervals.len() {
            if intervals[(i - 1) as usize][1] > intervals[i as usize][0] {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn can_attend_meetings0(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
        intervals.windows(2).all(|x| x[0][1] <= x[1][0])
    }
}
//leetcode submit region end(Prohibit modification and deletion)
