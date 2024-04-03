package day5

import (
	"regexp"
	"strconv"
	"strings"
)

type Map struct {
	label  string
	ranges []Range
}

type Range struct {
	destStart   int
	sourceStart int
	length      int
}

func ParseMaps(lines <-chan string) <-chan *Map {
	out := make(chan *Map)
	go func() {
		defer close(out)
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
	}()
	return out
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

func (m *Map) Output(in int) int {
	delta := 0
	for _, r := range m.ranges {
		if in >= r.sourceStart && in <= r.sourceStart+r.length {
			delta = r.destStart - r.sourceStart
			break
		}
	}
	return in + delta
}

func (m *Map) ChainOutput(in chan int) chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for value := range in {
			out <- m.Output(value)
		}
	}()
	return out
}
