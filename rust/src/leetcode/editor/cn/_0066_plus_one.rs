#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("66、plus_one");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut number = 1;
        for i in (0..digits.len()).rev() {
            let value = digits[i] + number;
            if value >= 10 {
                digits[i] = value % 10;
                number = 1;
            } else {
                digits[i] = value;
                number = 0;
            }
        }
        if number == 1 {
            digits.insert(0, number)
        }
        return digits;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
