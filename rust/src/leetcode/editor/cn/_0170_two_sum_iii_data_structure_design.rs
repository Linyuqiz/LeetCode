#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("170、two_sum_iii_data_structure_design");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
#[allow(dead_code)]
struct TwoSum {
    pub data: Vec<i32>,
    pub sorted: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
    #[allow(dead_code)]
    fn new() -> Self {
        TwoSum {
            data: vec![],
            sorted: false,
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, number: i32) {
        self.data.push(number);
        self.sorted = false;
    }

    #[allow(dead_code)]
    fn find(&mut self, value: i32) -> bool {
        if !self.sorted {
            self.data.sort_unstable();
            self.sorted = true;
        }
        if self.data.len() == 0 {
            return false;
        }
        let (mut left, mut right) = (0, self.data.len() - 1);
        while left < right {
            let tmp = self.data[left] + self.data[right];
            if tmp == value {
                return true;
            } else if tmp < value {
                left += 1;
            } else {
                right -= 1;
            }
        }
        false
    }
}

/**
 * Your TwoSum object will be instantiated and called as such:
 * let obj = TwoSum::new();
 * obj.add(number);
 * let ret_2: bool = obj.find(value);
 */
#[allow(dead_code)]
const PLACEHOLDER: &str = "";
//leetcode submit region end(Prohibit modification and deletion)
