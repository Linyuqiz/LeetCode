package main

import "sort"

func main() {
	println("455、assign_cookies")
}

// leetcode submit region begin(Prohibit modification and deletion)
func findContentChildren(g []int, s []int) int {
	sort.Ints(g)
	sort.Ints(s)
	index, res := 0, 0
	for i := 0; i < len(s); i++ {
		if index < len(g) && g[index] <= s[i] {
			index++
			res++
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
