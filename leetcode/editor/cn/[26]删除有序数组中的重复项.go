package main

// leetcode submit region begin(Prohibit modification and deletion)
func removeDuplicates(nums []int) int {
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
