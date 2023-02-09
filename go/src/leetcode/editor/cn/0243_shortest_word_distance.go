package main

import "math"

func main() {
	println("243、shortest_word_distance")
}

// leetcode submit region begin(Prohibit modification and deletion)
func shortestDistance(wordsDict []string, word1 string, word2 string) int {
	l, r, e := -1, -1, len(wordsDict)
	for i, v := range wordsDict {
		if word1 == v {
			l = i
		}
		if word2 == v {
			r = i
		}
		if l > -1 && r > -1 {
			e = int(math.Min(float64(e), math.Abs(float64(l-r))))
		}
	}
	return e
}

//leetcode submit region end(Prohibit modification and deletion)
