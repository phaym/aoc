package day5

import (
	"aoc/util/file"
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day4/input.txt")
	fmt.Println(result)
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	seeds := Seeds(<-lines)
	maps := Maps(lines)
	chain := make(chan int)
	last := chain
	for m := range maps {
		out := make(chan int)
		go ChainMap(last, m, out)
		last = out
	}
	// fmt.Println(seeds)
	// fmt.Printf("out comes %v \n", total)
	// fmt.Println("done")

	go func(seeds []int, c chan int) {
		defer close(c)
		for _, seed := range seeds {
			fmt.Printf("in: %v \n", seed)
			c <- seed
		}
	}(seeds, chain)
	total := math.MaxInt32
	for output := range last {
		fmt.Printf("out: %v \n", output)
		if output < total {
			total = output
		}
	}
	return total
}

func ChainMap(in chan int, m *Map, out chan int) {
	out <- MapNumber(<-in, m)
}

func MapNumber(in int, m *Map) int {
	delta := 0
	for _, r := range m.ranges {
		if in >= r.sourceStart && in <= r.sourceStart+r.length {
			delta = r.destStart - r.sourceStart
			break
		}
	}
	return in + delta
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
	destStart   int
	sourceStart int
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
	sourceStart, _ := strconv.Atoi(match[0])
	destStart, _ := strconv.Atoi(match[1])
	length, _ := strconv.Atoi(match[2])
	return Range{sourceStart, destStart, length}
}

func Maps(lines <-chan string) <-chan *Map {
	out := make(chan *Map)
	go func() {
		m := &Map{}
		for line := range lines {
			if line == "" {
				if len(m.ranges) > 0 {
					out <- m
				}
				m = &Map{}
			} else if strings.HasSuffix(line, ":") {
				m.label = line
			} else {
				m.ranges = append(m.ranges, NewRange(line))
			}
		}
		out <- m //EOF, last map is done
		defer close(out)
	}()
	return out
}
