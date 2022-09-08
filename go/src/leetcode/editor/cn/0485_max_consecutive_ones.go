package main

func main() {
	println("485、max_consecutive_ones")
}

// leetcode submit region begin(Prohibit modification and deletion)
func findMaxConsecutiveOnes(nums []int) int {
	count, res := 0, 0
	for _, num := range nums {
		if num == 1 {
			count++
			if count > res {
				res = count
			}
		} else {
			count = 0
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
