package main

import "strconv"

func main() {
	println("228、summary_ranges")
	summaryRanges([]int{0, 1, 2, 4, 5, 7})
}

// leetcode submit region begin(Prohibit modification and deletion)
func summaryRanges(nums []int) []string {
	res, length := make([]string, 0), len(nums)
	for i := 0; i < length; {
		left := i
		for i++; i < length && nums[i-1]+1 == nums[i]; i++ {
		}
		s := strconv.Itoa(nums[left])
		if left < i-1 {
			s += "->" + strconv.Itoa(nums[i-1])
		}
		res = append(res, s)
	}
	return res

}

//leetcode submit region end(Prohibit modification and deletion)
