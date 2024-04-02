package day4

import (
	"aoc/util"
	"fmt"
)

func Run() {
	result := A("year2023/day4/input.txt")
	fmt.Println(result)
}

func A(path string) (total int) {

	lines := util.ReadLinesFromFile(path)
	for line := range lines {
		fmt.Println(line)
	}
	return total
}
