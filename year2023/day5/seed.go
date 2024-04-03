package day5

import (
	"aoc/util/file"
	"fmt"
	"math"
	"regexp"
	"strconv"
)

func Run() {
	result := A("year2023/day5/input.txt")
	fmt.Println(result)
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	seeds := parseSeeds(<-lines)
	maps := ParseMaps(lines)

	in := make(chan int)
	out := chainMaps(in, maps)

	// send seeds on the in channel
	go func() {
		defer close(in)
		for _, seed := range seeds {
			in <- seed
		}
	}()

	// find minimum on out channel
	total := math.MaxInt32
	for v := range out {
		if v < total {
			total = v
		}
	}
	return total
}

func chainMaps(in chan int, maps <-chan *Map) <-chan int {
	out := in
	for m := range maps {
		out = m.ChainOutput(out)
	}
	return out
}

func parseSeeds(input string) []int {
	numbers := make([]int, 0)
	regex := regexp.MustCompile(`\b\d+\b`)
	for _, match := range regex.FindAllString(input, -1) {
		if num, err := strconv.Atoi(match); err == nil {
			numbers = append(numbers, num)
		}
	}
	return numbers
}
