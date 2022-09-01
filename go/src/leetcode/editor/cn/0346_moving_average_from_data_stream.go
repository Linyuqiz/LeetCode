package main

func main() {
	println("346、moving_average_from_data_stream")
	movingAverage := Constructor(3)
	println(movingAverage.Next(1))
	println(movingAverage.Next(10))
	println(movingAverage.Next(3))
	println(movingAverage.Next(5))
}

// leetcode submit region begin(Prohibit modification and deletion)
type MovingAverage struct {
	Data     []int
	Capacity int
}

func Constructor(size int) MovingAverage {
	return MovingAverage{
		Data:     make([]int, 0),
		Capacity: size,
	}
}

func (this *MovingAverage) Next(val int) float64 {
	this.Data = append(this.Data, val)
	sum, length := 0, len(this.Data)
	if length > this.Capacity {
		this.Data = this.Data[1:]
	}
	for _, v := range this.Data {
		sum += v
	}
	return float64(sum) / float64(len(this.Data))
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * obj := Constructor(size);
 * param_1 := obj.Next(val);
 */
//leetcode submit region end(Prohibit modification and deletion)
