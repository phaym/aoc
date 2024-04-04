package day5

import (
	"regexp"
	"strconv"
	"strings"
)

type CategoryRules struct {
	label string
	rules []Rule
}

type Rule struct {
	source int
	dest   int
	length int
}

func ParseCategories(lines <-chan string) <-chan *CategoryRules {
	out := make(chan *CategoryRules)
	go func() {
		defer close(out)
		m := &CategoryRules{}
		for line := range lines {
			if line == "" {
				if len(m.rules) > 0 {
					out <- m
				}
				m = &CategoryRules{}
			} else if strings.HasSuffix(line, ":") {
				m.label = line
			} else {
				m.rules = append(m.rules, NewRange(line))
			}
		}
		out <- m //EOF, last map is done
	}()
	return out
}

func NewRange(line string) Rule {
	regex := regexp.MustCompile(`\b\d+\b`)
	match := regex.FindAllString(line, 3)
	if len(match) != 3 {
		panic("couldn't parse range: " + line)
	}
	sourceStart, _ := strconv.Atoi(match[0])
	destStart, _ := strconv.Atoi(match[1])
	length, _ := strconv.Atoi(match[2])
	return Rule{sourceStart, destStart, length}
}

func (m *CategoryRules) ChainOutput(in chan int) chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for value := range in {
			out <- m.Output(value)
		}
	}()
	return out
}

func (m *CategoryRules) Output(in int) int {
	delta := 0
	for _, r := range m.rules {
		if in >= r.dest && in <= r.dest+r.length {
			delta = r.source - r.dest
			break
		}
	}
	return in + delta
}

func (m *CategoryRules) ChainOutputB(in chan Seed) chan Seed {
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

func (category *CategoryRules) OutputB(seed Seed) []Seed {
	seeds := make([]Seed, 0)
	for _, rule := range category.rules {
		seedEnd := seed.Start + seed.Length - 1
		srcEnd := rule.dest + rule.length - 1
		if seed.Start <= srcEnd && rule.dest <= seedEnd {
			delta := rule.source - rule.dest

			// s:  6-9
			// r: 5 - 10
			if seed.Start >= rule.dest && seedEnd <= srcEnd {
				seed.Start += delta
				break
			} else if seed.Start < rule.dest && seedEnd < srcEnd {
				// s: 1 - 7
				// r:   5 - 10
				newSeedLength := seedEnd - rule.dest + 1
				newSeedStart := rule.dest + delta
				seeds = append(seeds, Seed{newSeedStart, newSeedLength})
				seed.Length = rule.dest - seed.Start
			} else if seed.Start > rule.dest && seedEnd > srcEnd {
				// s:  7 - 15
				// r: 5 -10
				newSeedLength := srcEnd - seed.Start + 1
				newSeedStart := seed.Start + delta
				seeds = append(seeds, Seed{newSeedStart, newSeedLength})
				seed.Length = seedEnd - srcEnd
				seed.Start = srcEnd + 1
			} else if seed.Start < rule.dest && seedEnd > srcEnd {
				// s: 1  -  12
				// r: 	5-10
				seeds = append(seeds, Seed{rule.dest + delta, rule.length})
				seeds = append(seeds, Seed{srcEnd + 1, seedEnd - srcEnd})
				seed.Length = rule.dest - seed.Start
			}
		}
	}
	if seed.Length > 0 {
		seeds = append(seeds, seed)
	}
	return seeds
}
