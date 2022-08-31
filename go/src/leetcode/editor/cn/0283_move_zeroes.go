package main

func main() {
	println("283、move_zeroes")
}

// leetcode submit region begin(Prohibit modification and deletion)
func moveZeroes(nums []int) {
	left, right, n := 0, 0, len(nums)
	for right < n {
		if nums[right] != 0 {
			nums[left], nums[right] = nums[right], nums[left]
			left++
		}
		right++
	}
}

//leetcode submit region end(Prohibit modification and deletion)
