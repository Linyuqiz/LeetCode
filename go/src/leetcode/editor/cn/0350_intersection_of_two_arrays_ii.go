package main

import "sort"

func main() {
	println("350、intersection_of_two_arrays_ii")
}

// leetcode submit region begin(Prohibit modification and deletion)
func intersect(nums1 []int, nums2 []int) []int {
	sort.Ints(nums1)
	sort.Ints(nums2)
	left, right, res := 0, 0, []int{}
	for left < len(nums1) && right < len(nums2) {
		x, y := nums1[left], nums2[right]
		if x == y {
			res = append(res, x)
			left++
			right++
		} else if x < y {
			left++
		} else {
			right++
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
