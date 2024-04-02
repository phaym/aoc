package day4

import (
	"aoc/util"
	"fmt"
	"strings"
)

func Run() {
	result := A("year2023/day4/input.txt")
	fmt.Println(result)
}

func A(path string) (total int) {
	lines := util.ReadLinesFromFile(path)
	points := make(chan int)
	for line := range lines {
		card := CalcPoints(line)
		points <- card
	}
	for point := range points {
		total += point
		fmt.Println(point)
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

func CalcPoints(line string) (points int) {
	card := ParseCard(line)
	for k, _ := range card.played {
		if _, ok := card.winners[k]; ok {
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
	game := strings.Split(split[1], "|")
	for _, winner := range strings.Split(strings.TrimSpace(game[0]), " ") {
		card.winners[strings.TrimSpace(winner)] = 1
	}
	for _, played := range strings.Split(strings.TrimSpace(game[1]), " ") {
		card.played[strings.TrimSpace(played)] = 1
	}
	return card
}
