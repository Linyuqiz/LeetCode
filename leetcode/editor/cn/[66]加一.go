package main

// leetcode submit region begin(Prohibit modification and deletion)
func plusOne(digits []int) []int {
	number := 1
	for i := len(digits) - 1; i >= 0; i-- {
		value := digits[i] + number
		if value >= 10 {
			digits[i] = value % 10
			number = 1
		} else {
			digits[i] = value
			number = 0
		}
	}
	if number == 1 {
		digits = append([]int{1}, digits...)
	}
	return digits
}

//leetcode submit region end(Prohibit modification and deletion)
