package main

func main() {
	println("293、flip_game")
}

// leetcode submit region begin(Prohibit modification and deletion)
func generatePossibleNextMoves(currentState string) []string {
	r := make([]string, 0)
	for i := 0; i < len(currentState)-1; i++ {
		if '+' == currentState[i] && '+' == currentState[i+1] {
			r = append(r, currentState[:i]+"--"+currentState[i+2:])
		}
	}
	return r
}

//leetcode submit region end(Prohibit modification and deletion)
