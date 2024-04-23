package day7

import (
	"aoc/util/file"
	"fmt"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day6/input.txt")
	fmt.Println(result)
}

type Card struct {
	hand string
	bid  int
}

func A(path string) int {

	lines := file.ReadLinesFromFile(path)
	cards := make([]Card, 0)
	for line := range lines {
		cards = append(cards, parseLine(line))
	}
	winnings := totalWinnings(cards)
	return winnings
}

func totalWinnings(cards []Card) int {
	return 3
}

func parseLine(line string) Card {
	split := strings.Split(line, " ")
	hand := split[0]
	bid, _ := strconv.Atoi(split[1])
	return Card{
		hand,
		bid,
	}
}
