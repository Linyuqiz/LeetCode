package main

import "strconv"

func main() {
	println("163、missing_ranges")
}

// leetcode submit region begin(Prohibit modification and deletion)
func findMissingRanges(nums []int, lower int, upper int) []string {
	r := make([]string, 0)
	nums = append([]int{lower - 1}, nums...)
	nums = append(nums, upper+1)
	for i := 1; i < len(nums); i++ {
		v := nums[i] - nums[i-1]
		if v == 2 {
			r = append(r, strconv.Itoa(nums[i]-1))
		}
		if v > 2 {
			r = append(r, strconv.Itoa(nums[i-1]+1)+"->"+strconv.Itoa(nums[i]-1))
		}
	}
	return r
}

//leetcode submit region end(Prohibit modification and deletion)
