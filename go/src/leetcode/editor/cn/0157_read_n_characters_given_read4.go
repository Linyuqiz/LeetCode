package main

func main() {
	println("157、read_n_characters_given_read4")
}

//leetcode submit region begin(Prohibit modification and deletion)
/**
 * The read4 API is already defined for you.
 *
 *     read4 := func(buf4 []byte) int
 *
 * // Below is an example of how the read4 API can be called.
 * file := File("abcdefghijk") // File is "abcdefghijk", initially file pointer (fp) points to 'a'
 * buf4 := make([]byte, 4) // Create buffer with enough space to store characters
 * read4(buf4) // read4 returns 4. Now buf = ['a','b','c','d'], fp points to 'e'
 * read4(buf4) // read4 returns 4. Now buf = ['e','f','g','h'], fp points to 'i'
 * read4(buf4) // read4 returns 3. Now buf = ['i','j','k',...], fp points to end of file
 */

var solution = func(read4 func([]byte) int) func([]byte, int) int {
	// implement read below.
	return func(buf []byte, n int) int {
		r := 0
		for r < n {
			b := make([]byte, 4)
			c := read4(b)
			if c == 0 {
				break
			}
			for i := 0; i < c && r < n; i++ {
				buf[r] = b[i]
				r++
			}
		}
		return r
	}
}

//leetcode submit region end(Prohibit modification and deletion)
