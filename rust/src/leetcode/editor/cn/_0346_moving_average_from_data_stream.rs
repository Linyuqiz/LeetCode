#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("346、moving_average_from_data_stream");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
#[allow(dead_code)]
struct MovingAverage {
    pub data: Vec<i32>,
    pub capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    #[allow(dead_code)]
    fn new(size: i32) -> Self {
        MovingAverage {
            data: vec![],
            capacity: size,
        }
    }

    #[allow(dead_code)]
    fn next(&mut self, val: i32) -> f64 {
        self.data.push(val);
        if self.data.len() > self.capacity as usize {
            self.data.remove(0);
        }
        return self.data.iter().sum::<i32>() as f64 / self.data.len() as f64;
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
#[allow(dead_code)]
const PLACEHOLDER: &str = "";
//leetcode submit region end(Prohibit modification and deletion)
