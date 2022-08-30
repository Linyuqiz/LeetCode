package main

// leetcode submit region begin(Prohibit modification and deletion)
func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int, 0)
	for i, v := range nums {
		number := target - v
		if j, ok := hashMap[number]; ok {
			return []int{j, i}
		}
		hashMap[v] = i
	}
	return []int{}
}

//leetcode submit region end(Prohibit modification and deletion)
