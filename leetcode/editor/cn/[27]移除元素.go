package main

// leetcode submit region begin(Prohibit modification and deletion)
func removeElement(nums []int, val int) int {
	length := len(nums)
	if length == 0 || (length == 1 && nums[0] == val) {
		return 0
	}
	left := 0
	for _, v := range nums {
		if v != val {
			nums[left] = v
			left++
		}
	}
	return left
}

//leetcode submit region end(Prohibit modification and deletion)
