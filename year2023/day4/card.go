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

func pipe[T any, K any](in <-chan T, f func(T) K) <-chan K {
	out := make(chan K)
	go func() {
		for n := range in {
			out <- f(n)
		}
		close(out)
	}()
	return out
}

func A(path string) (total int) {
	lines := util.ReadLinesFromFile(path)
	cards := pipe(lines, ParseCard)
	points := pipe(cards, CalcPoints)
	for point := range points {
		total += point
	}
	return total
}

type Card struct {
	winners map[string]int
	played  map[string]int
}

func NewCard() *Card {
	return &Card{winners: make(map[string]int), played: make(map[string]int)}
}

func CalcPoints(card *Card) (points int) {
	for k := range card.played {
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
