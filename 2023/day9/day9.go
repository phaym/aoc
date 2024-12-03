package day9

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
	"year2023/util/file"
)

func Run() {
	result := B("year2023/day9/input.txt")
	fmt.Println(result)
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	result := 0
	for line := range lines {
		prediction := getPrediction(parseLine(line))
		result += prediction
	}
	return result
}

func B(path string) int {
	lines := file.ReadLinesFromFile(path)
	result := 0
	for line := range lines {
		history := parseLine(line)
		slices.Reverse(history)
		prediction := getPrediction(history)
		result += prediction
	}
	return result
}

func getPrediction(historic []int) int {
	rightMost := []int{historic[len(historic)-1]}
	nonZero := true
	currentRow := append([]int(nil), historic...)
	for nonZero {
		diffs := make([]int, len(currentRow)-1)
		nonZero = false
		for i := 0; i < len(diffs); i++ {
			diffs[i] = currentRow[i+1] - currentRow[i]
			if diffs[i] != 0 {
				nonZero = true
			}
		}
		rightMost = append(rightMost, diffs[len(diffs)-1])
		currentRow = append([]int(nil), diffs...)
	}
	next := 0
	for _, val := range rightMost {
		next += val
	}
	return next
}

func parseLine(line string) []int {
	history := make([]int, 0)
	for _, v := range strings.Fields(line) {
		value, _ := strconv.Atoi(v)
		history = append(history, value)
	}
	return history
}
