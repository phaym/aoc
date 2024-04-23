package day6

import (
	"aoc/util/file"
	"fmt"
	"strconv"
	"strings"
)

func Run() {
	result := B("year2023/day6/input.txt")
	fmt.Println(result)
}

func A(path string) int {

	lines := file.ReadLinesFromFile(path)
	times := readLine(<-lines)
	distances := readLine(<-lines)
	result := 1
	for i := 0; i < len(times); i++ {
		result *= computeWinCount(times[i], distances[i])
	}
	return result
}

func B(path string) int {

	lines := file.ReadLinesFromFile(path)
	time := readLineB(<-lines)
	distance := readLineB(<-lines)
	result := computeWinCount(time, distance)
	return result
}

func readLineB(line string) int {
	splits := strings.Split(line, ":")
	values := strings.ReplaceAll(splits[1], " ", "")

	result, _ := strconv.Atoi(values)
	return result
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

func computeWinCount(maxTime, distance int) int {
	result := 0
	for timeHeld := 0; timeHeld < maxTime; timeHeld++ {
		if timeHeld*(maxTime-timeHeld) > distance {
			result++
		}
	}
	return result
}
