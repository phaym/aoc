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
		id, possible := CheckGame(game)
		if possible {
			total += id
		}
	}
	return
}

type Game struct {
	Id          int
	ColorCounts map[string][]int
}

func CheckGame(game Game) (int, bool) {
	//maxRed, maxGreen, maxBlue := 12, 13, 14
	return game.Id, false
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
		fmt.Println(match[0])
		color := match[2]
		if count, error := strconv.Atoi(match[1]); error == nil {
			game.ColorCounts[color] = append(game.ColorCounts[color], count)
		}
	}

	return game, nil
}
