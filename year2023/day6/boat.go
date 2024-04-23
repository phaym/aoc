package day6

import (
	"aoc/util/file"
	"fmt"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day6/input.txt")
	fmt.Println(result)
}

func A(path string) int {

	lines := file.ReadLinesFromFile(path)
	times := readLine(<-lines)
	distances := readLine(<-lines)

	fmt.Println(times)
	fmt.Println(distances)
	return 0
}

func readLine(line string) []int {
	splits := strings.Split(line, ":")
	values := strings.Fields(strings.TrimSpace(splits[1]))

	result := make([]int, len(values))

	for i, num := range values {
		numVal, _ := strconv.Atoi(num)
		result[i] = numVal
	}
	return result
}
