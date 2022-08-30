package main

func main() {
	println("35、search_insert_position")
}

// leetcode submit region begin(Prohibit modification and deletion)
func searchInsert(nums []int, target int) int {
	left, right := 0, len(nums)-1
	for left <= right {
		mid := (left + right) >> 1
		if nums[mid] == target {
			return mid
		} else if nums[mid] > target {
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return left
}

//leetcode submit region end(Prohibit modification and deletion)
