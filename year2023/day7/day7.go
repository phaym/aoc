package day7

import (
	"fmt"
	"slices"
	"sort"
	"strconv"
	"strings"
	"year2023/util/file"
)

func Run() {
	result := A("year2023/day7/input.txt")
	fmt.Println(result)
}

type Card struct {
	hand string
	bid  int
	rank int
}

var rankMap = map[string]int{
	"2": 2,
	"3": 3,
	"4": 4,
	"5": 5,
	"6": 6,
	"7": 7,
	"8": 8,
	"9": 9,
	"T": 10,
	"J": 11,
	"Q": 12,
	"K": 13,
	"A": 14,
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
	for i := range cards {
		setCardValue(&cards[i])
	}
	sort.Slice(cards, func(i, j int) bool {
		if cards[i].rank == cards[j].rank {
			charIndex := 0
			for cards[i].hand[charIndex] == cards[j].hand[charIndex] {
				charIndex++
			}
			iChar := cards[i].hand[charIndex]
			jChar := cards[j].hand[charIndex]
			return rankMap[string(iChar)] < rankMap[string(jChar)]
		} else {
			return cards[i].rank < cards[j].rank
		}
	})
	total := 0
	for i, card := range cards {
		total += card.bid * (i + 1)
	}
	return total
}

func setCardValue(card *Card) {
	typeCounts := make(map[string]int)
	for i := 0; i < len(card.hand); i++ {
		typeCounts[string(card.hand[i])]++
	}
	counts := make([]int, 0, len(typeCounts))
	for _, v := range typeCounts {
		counts = append(counts, v)
	}
	slices.Sort(counts)
	rank := 0
	highest := counts[len(counts)-1]
	second := 0
	if highest < 5 {
		second = counts[len(counts)-2]
	}
	if highest == 1 {
		rank = 1
	} else if highest == 2 && second != 2 {
		rank = 2
	} else if highest == 2 && second == 2 {
		rank = 3
	} else if highest == 3 && second != 2 {
		rank = 4
	} else if highest == 3 && second == 2 {
		rank = 5
	} else if highest == 4 {
		rank = 6
	} else if highest == 5 {
		rank = 7
	}

	card.rank = rank
}

func parseLine(line string) Card {
	split := strings.Split(line, " ")
	hand := split[0]
	bid, _ := strconv.Atoi(split[1])
	return Card{
		hand,
		bid,
		0,
	}
}
