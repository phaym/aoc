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
	output := seedsToOutput(maps, seeds)

	total := math.MaxInt32
	for output := range output {
		if output < total {
			total = output
		}
	}
	return total
}

func seedsToOutput(maps <-chan *Map, seeds []int) <-chan int {
	chain := make(chan int)
	last := chain
	for m := range maps {
		last = m.OutputChannel(last)
	}
	go func() {
		defer close(chain)
		for _, seed := range seeds {
			chain <- seed
		}
	}()
	return last
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
