package main

// leetcode submit region begin(Prohibit modification and deletion)
func removeDuplicates(nums []int) int {
	length := len(nums)
	if length == 0 {
		return 0
	}
	if length == 1 {
		return 1
	}
	left := 1
	for right := 1; right < len(nums); right++ {
		if nums[right-1] != nums[right] {
			nums[left] = nums[right]
			left++
		}
	}
	return left
}

//leetcode submit region end(Prohibit modification and deletion)
