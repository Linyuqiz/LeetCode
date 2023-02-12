package main

func main() {
	println("408、valid_word_abbreviation")
}

// leetcode submit region begin(Prohibit modification and deletion)
func validWordAbbreviation(word string, abbr string) bool {
	t, i := 0, 0
	for _, s := range abbr {
		if '0' <= s && s <= '9' {
			if s == '0' && i == 0 {
				return false
			}
			i = i*10 + int(s-'0')
		} else {
			t = t + i
			if t >= len(word) || s != rune(word[t]) {
				return false
			}
			t, i = t+1, 0
		}
	}
	return t+i == len(word)
}

//leetcode submit region end(Prohibit modification and deletion)
