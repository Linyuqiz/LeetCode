package main

func main() {
	println("359、logger_rate_limiter")
}

// leetcode submit region begin(Prohibit modification and deletion)
type Logger struct {
	M map[string]int
}

func Constructor() Logger {
	return Logger{
		make(map[string]int),
	}
}

func (this *Logger) ShouldPrintMessage(timestamp int, message string) bool {
	r := false
	if v, o := this.M[message]; !o {
		this.M[message] = timestamp
		r = true
	} else {
		if timestamp-v >= 10 {
			this.M[message] = timestamp
			r = true
		}
	}
	return r
}

/**
 * Your Logger object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.ShouldPrintMessage(timestamp,message);
 */
//leetcode submit region end(Prohibit modification and deletion)
