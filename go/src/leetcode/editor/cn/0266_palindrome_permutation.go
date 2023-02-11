package main

func main() {
	println("266、palindrome_permutation")
}

// leetcode submit region begin(Prohibit modification and deletion)
func canPermutePalindrome(s string) bool {
	m, f := make(map[rune]int), false
	for _, v := range s {
		m[v]++
	}
	for _, v := range m {
		i := v % 2
		if f && i != 0 {
			return false
		}
		if !f && i != 0 {
			f = true
		}
	}
	return true
}

//leetcode submit region end(Prohibit modification and deletion)
