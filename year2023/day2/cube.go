package day2

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const FILE_PATH = "year2023/day2/input.txt"

func Run() {
	file, err := os.Open(FILE_PATH)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	total := ParseFile(file)
	fmt.Println(total)
}

func ParseFile(file *os.File) (total int) {
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		game, error := ParseLine(scanner.Text())
		if error != nil {
			panic(error)
		}
		total += B(&game)
	}
	return
}

func A(game *Game) (total int) {
	id, possible := CheckGame(game)
	if possible {
		total = id
	}
	return
}

func B(game *Game) (powerOfMin int) {
	maxCounts := map[string]int{
		"blue":  0,
		"green": 0,
		"red":   0,
	}
	for color := range maxCounts {
		for _, count := range game.ColorCounts[color] {
			if count > maxCounts[color] {
				maxCounts[color] = count
			}
		}
	}
	for _, max := range maxCounts {
		if powerOfMin == 0 {
			powerOfMin = max
		} else {
			powerOfMin *= max
		}
	}
	return powerOfMin
}

type Game struct {
	Id          int
	ColorCounts map[string][]int
}

func CheckGame(game *Game) (int, bool) {
	maxCounts := map[string]int{
		"blue":  14,
		"green": 13,
		"red":   12,
	}
	for color, max := range maxCounts {
		for _, count := range game.ColorCounts[color] {
			if count > max {
				return game.Id, false
			}
		}
	}

	return game.Id, true
}

func ParseLine(line string) (game Game, err error) {

	gameParts := strings.Split(line, ":")
	after, _ := strings.CutPrefix(gameParts[0], "Game ")
	id, error := strconv.Atoi(after)
	if error != nil {
		return game, error
	}
	game.Id = id
	// Map to store the counts of each color
	game.ColorCounts = make(map[string][]int)
	regex := regexp.MustCompile(`(?:(\d+)\s([a-z]*))`)
	for _, match := range regex.FindAllStringSubmatch(gameParts[1], -1) {
		color := match[2]
		if count, error := strconv.Atoi(match[1]); error == nil {
			game.ColorCounts[color] = append(game.ColorCounts[color], count)
		}
	}

	return game, nil
}
