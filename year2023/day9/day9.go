package day9

import (
	"aoc/util/file"
	"fmt"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day9/input.txt")
	fmt.Println(result)
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	histories := readHistory(lines)
	fmt.Println(histories)
	return 0
}

func readHistory(lines <-chan string) [][]int {
	histories := make([][]int, 0)
	for line := range lines {
		history := make([]int, 0)
		for _, v := range strings.Fields(line) {
			value, _ := strconv.Atoi(v)
			history = append(history, value)
		}
		histories = append(histories, history)
	}
	return histories
}
