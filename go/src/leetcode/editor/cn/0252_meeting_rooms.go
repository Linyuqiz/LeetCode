package main

import "sort"

func main() {
	println("252、meeting_rooms")
}

// leetcode submit region begin(Prohibit modification and deletion)
func canAttendMeetings(intervals [][]int) bool {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	for i := 0; i < len(intervals)-1; i++ {
		if intervals[i][1] > intervals[i+1][0] {
			return false
		}
	}
	return true
}

//leetcode submit region end(Prohibit modification and deletion)
