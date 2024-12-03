package day5

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
	"time"
	"year2023/util/file"
)

func timer(name string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v\n", name, time.Since(start))
	}
}

func Run() {
	defer timer("B")()
	result := B("year2023/day5/input.txt")
	fmt.Println(result)
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	seeds := parseSeeds(<-lines)
	maps := ParseCategories(lines)

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

type Seed struct {
	Start  int
	Length int
}

func B(path string) int {

	lines := file.ReadLinesFromFile(path)
	seeds := parseSeeds(<-lines)
	maps := ParseCategories(lines)

	in := make(chan Seed)
	out := chainMapsB(in, maps)

	// send seeds on the in channel
	go func() {
		defer close(in)
		for i := 0; i < len(seeds); i += 2 {
			in <- Seed{seeds[i], seeds[i+1]}
		}
	}()
	// find minimum on out channel
	total := math.MaxInt32
	for seed := range out {
		if seed.Start < total {
			total = seed.Start
		}
	}
	return total
}

// chain all maps and return the final one
func chainMaps(in chan int, maps <-chan *CategoryRules) <-chan int {
	out := in
	for m := range maps {
		out = m.ChainOutput(out)
	}
	return out
}

func chainMapsB(in chan Seed, maps <-chan *CategoryRules) <-chan Seed {
	out := in
	for m := range maps {
		out = m.ChainOutputB(out)
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
