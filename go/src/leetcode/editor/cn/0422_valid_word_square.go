package main

func main() {
	println("422、valid_word_square")
}

// leetcode submit region begin(Prohibit modification and deletion)
func validWordSquare(words []string) bool {
	for i := 0; i < len(words); i++ {
		for j := 0; j < len(words[i]); j++ {
			if j >= len(words) || i >= len(words[j]) || words[i][j] != words[j][i] {
				return false
			}
		}
	}
	return true
}

//leetcode submit region end(Prohibit modification and deletion)
