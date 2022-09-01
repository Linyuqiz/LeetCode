package main

import "math"

func main() {
	println("243、shortest_word_distance")
}

// leetcode submit region begin(Prohibit modification and deletion)
func shortestDistance(wordsDict []string, word1 string, word2 string) int {
	res, index1, index2 := len(wordsDict), -1, -1
	for i, v := range wordsDict {
		if v == word1 {
			index1 = i
		}
		if v == word2 {
			index2 = i
		}
		if index1 > -1 && index2 > -1 {
			res = int(math.Min(float64(res), math.Abs(float64(index1-index2))))
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
