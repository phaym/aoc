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

func (m *Map) ChainOutputB(in chan Seed) chan Seed {
	out := make(chan Seed)
	go func() {
		defer close(out)
		for value := range in {
			seeds := m.OutputB(value)
			for _, seed := range seeds {
				out <- seed
			}
		}
	}()
	return out
}

func (m *Map) OutputB(seed Seed) []Seed {
	seeds := make([]Seed, 0)
	for _, r := range m.ranges {
		seedEnd := seed.Start + seed.Length - 1
		srcEnd := r.sourceStart + r.length - 1
		if seed.Start <= srcEnd && r.sourceStart <= seedEnd {

			// s: 5 - 10
			// r: 5 - 10
			if seed.Start == r.sourceStart && seedEnd <= srcEnd || seed.Start >= r.sourceStart && seedEnd == srcEnd {
				seeds = append(seeds, Seed{r.destStart, seed.Length})
				seed.Length = 0
				break
			} else if seed.Start > r.sourceStart && seedEnd < srcEnd {
				// s:  6-9
				// r: 5 - 10
				delta := r.destStart - r.sourceStart
				seeds = append(seeds, Seed{seed.Start + delta, seed.Length})
				seed.Length = 0
			} else if seed.Start < r.sourceStart && seedEnd < srcEnd {
				// s: 1 - 7
				// r:   5 - 10
				currentSeedLength := r.sourceStart - seed.Start
				newSeedLength := seed.Length - currentSeedLength
				delta := r.destStart - r.sourceStart
				seed.Length = currentSeedLength
				seeds = append(seeds, Seed{r.sourceStart + delta, newSeedLength})
			} else if seed.Start > r.sourceStart && seedEnd > srcEnd {
				// s:  7 - 15
				// r: 5 -10
				currentSeedLength := seedEnd - srcEnd
				newSeedLength := seed.Length - currentSeedLength
				delta := r.destStart - r.sourceStart
				seed.Length = currentSeedLength
				seeds = append(seeds, Seed{seed.Start + delta, newSeedLength})
				seed.Start = srcEnd + 1
			} else if seed.Start < r.sourceStart && seedEnd > srcEnd {
				// s: 1  -  12
				// r: 	5-10
				currentSeedLength := r.sourceStart - seed.Start
				seed.Length = currentSeedLength
				delta := r.destStart - r.sourceStart
				seeds = append(seeds, Seed{r.sourceStart + delta, r.length})
				newSeeds := m.OutputB(Seed{srcEnd + 1, seedEnd - srcEnd})
				seeds = append(seeds, newSeeds...)
			}
		}
	}
	if seed.Length > 0 {
		seeds = append(seeds, seed)
	}
	return seeds
}
