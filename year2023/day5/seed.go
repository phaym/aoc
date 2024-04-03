package day4

import (
	"aoc/util/file"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day4/input.txt")
	fmt.Println(result)
}

func A(path string) (total int) {
	lines := file.ReadLinesFromFile(path)
	seeds := Seeds(<-lines)
	fmt.Println(seeds)
	maps := Maps(lines)
	for m := range maps {
		fmt.Printf("%+v \n", m)
	}
	return total
}

func Seeds(input string) []int {
	numbers := make([]int, 0)
	regex := regexp.MustCompile(`\b\d+\b`)
	for _, match := range regex.FindAllString(input, -1) {
		if num, err := strconv.Atoi(match); err == nil {
			numbers = append(numbers, num)
		}
	}
	return numbers
}

type Range struct {
	sourceStart int
	destStart   int
	length      int
}

type Map struct {
	label  string
	ranges []Range
}

func NewRange(line string) Range {
	regex := regexp.MustCompile(`\b\d+\b`)
	match := regex.FindAllString(line, 3)
	if len(match) != 3 {
		panic("couldn't parse range: " + line)
	}
	destStart, _ := strconv.Atoi(match[0])
	sourceStart, _ := strconv.Atoi(match[1])
	length, _ := strconv.Atoi(match[2])
	return Range{sourceStart, destStart, length}
}

func Maps(lines <-chan string) <-chan Map {
	out := make(chan Map)
	go func() {
		var m Map
		for line := range lines {
			if line == "" {
				if len(m.ranges) > 0 {
					out <- m
				}
				m = Map{}
			} else if strings.HasSuffix(line, ":") {
				m.label = line
			} else {
				m.ranges = append(m.ranges, NewRange(line))
			}
		}
		defer close(out)
	}()
	return out
}
