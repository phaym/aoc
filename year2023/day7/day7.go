package day7

import (
	"aoc/util/file"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func Run() {
	result := A("year2023/day6/input.txt")
	fmt.Println(result)
}

type Card struct {
	hand       string
	bid        int
	typeCounts map[string]int
	rank       int
}

const (
	DEFAULT = iota
	HIGH
	TWO
	THREE
	FULL
	FOUR
	FIVE
)

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
	for i := range cards {
		calcHand(&cards[i])
	}

	return 3
}

func calcHand(card *Card) {
	for i := 0; i < len(card.hand); i++ {
		card.typeCounts[string(card.hand[i])]++
	}
	card.rank = getRank(card)
}

func getRank(card *Card) int {
	counts := make([]int, 0)
	for _, val := range card.typeCounts {
		counts = append(counts, val)
	}
	if slices.Contains(counts, 5) {
		return FIVE
	} else if slices.Contains(counts, 4) {
		return FOUR
	} else if slices.Contains(counts, 3) && slices.Contains(counts, 2) {
		return FULL
	} else if slices.Contains(counts, 3) {
		return THREE
	} else if slices.Contains(counts, 2) {
		return TWO
	} else {
		return HIGH
	}
}

func parseLine(line string) Card {
	split := strings.Split(line, " ")
	hand := split[0]
	bid, _ := strconv.Atoi(split[1])
	return Card{
		hand:       hand,
		bid:        bid,
		typeCounts: make(map[string]int),
		rank:       DEFAULT,
	}
}
