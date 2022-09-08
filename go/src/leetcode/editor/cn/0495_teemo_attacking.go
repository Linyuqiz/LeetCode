package main

func main() {
	println("495、teemo_attacking")
}

// leetcode submit region begin(Prohibit modification and deletion)
func findPoisonedDuration(timeSeries []int, duration int) int {
	res := 0
	for i := 0; i < len(timeSeries); i++ {
		if i+1 < len(timeSeries) && timeSeries[i+1]-timeSeries[i] < duration {
			res += timeSeries[i+1] - timeSeries[i]
		} else {
			res += duration
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
