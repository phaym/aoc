package day4

import (
	"aoc/util/channels"
	"aoc/util/file"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func Run() {
	result := B("year2023/day4/input.txt")
	fmt.Println(result)
}

func A(path string) (total int) {
	lines := file.ReadLinesFromFile(path)
	cards := channels.Pipe(lines, ParseCard)
	points := channels.Pipe(cards, CalcPoints)
	for point := range points {
		total += point
	}
	return total
}

func B(path string) (total int) {
	lines := file.ReadLinesFromFile(path)
	cards := channels.Pipe(lines, ParseCard)
	copies := channels.Pipe(cards, CalcCopies())
	for copyCount := range copies {
		total += copyCount
	}
	return total
}

type Card struct {
	id      int
	winners map[string]int
	played  map[string]int
}

func NewCard() *Card {
	return &Card{winners: make(map[string]int), played: make(map[string]int)}
}
func CalcCopies() func(card *Card) int {
	copyMap := make(map[int]int)
	return func(card *Card) int {
		wins := 0
		for k := range card.played {
			if _, ok := card.winners[k]; ok {
				wins++
			}
		}
		copies := copyMap[card.id] + 1
		for i := 1; i <= wins; i++ {
			copyMap[card.id+i] += copies
		}
		return copies
	}
}

func CalcPoints(card *Card) (points int) {
	for playedId := range card.played {
		if _, ok := card.winners[playedId]; ok {
			if points == 0 {
				points = 1
			} else {
				points *= 2
			}
		}
	}
	return
}

func ParseCard(line string) *Card {
	card := NewCard()
	split := strings.Split(line, ":")
	regex := regexp.MustCompile(`\b\d+\b`)
	if id, err := strconv.Atoi(regex.FindString(split[0])); err == nil {
		card.id = id
	}
	game := strings.Split(split[1], "|")
	card.winners = extractNumbers(game[0])
	card.played = extractNumbers(game[1])
	return card
}

func extractNumbers(input string) map[string]int {
	numbers := make(map[string]int)
	regex := regexp.MustCompile(`\b\d+\b`)
	for _, match := range regex.FindAllString(input, -1) {
		if _, err := strconv.Atoi(match); err == nil {
			numbers[match] = 1
		}
	}
	return numbers
}
