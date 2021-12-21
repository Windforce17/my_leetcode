package main

import (
	"fmt"
)

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}
func maxArea(height []int) int {
	var i, j, maxarea int
	j = len(height) - 1
	for i != j {
		if height[i] > height[j] {
			maxarea = max(maxarea, height[j]*(j-i))
			j--
		} else {
			maxarea = max(maxarea, height[i]*(j-i))
			i++
		}
	}
	return maxarea
}
func main() {
	fmt.Println(maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7}))
}
