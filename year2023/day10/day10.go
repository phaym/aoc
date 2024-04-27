package day10

import (
	"aoc/util/file"
	"errors"
	"fmt"
	"strings"
)

func Run() {
	result := A("year2023/day10/input.txt")
	fmt.Println(result)
}

var up = [2]int{-1, 0}
var down = [2]int{1, 0}
var right = [2]int{0, 1}
var left = [2]int{0, -1}
var vectors = [4][2]int{up, down, right, left}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	tiles, startRow, startCol := parseLines(lines)
	// find first possible direction from 'S'
	var startVector [2]int
	for _, dir := range vectors {
		currentTile := string(tiles[startRow+dir[0]][startCol+dir[1]])
		if _, err := getNextVector(dir, currentTile); err == nil {
			startVector = dir
			break
		}
	}

	// traverse tiles until error when returning to 'S'
	steps := 0
	row, col := startRow, startCol
	vector := startVector
	var err error
	for err == nil {
		row, col = row+vector[0], col+vector[1]
		currentTile := string(tiles[row][col])
		vector, err = getNextVector(vector, currentTile)
		steps++
	}
	return steps / 2
}

func getNextVector(vectorIn [2]int, tile string) ([2]int, error) {
	if tile == "-" && vectorIn == right {
		return right, nil
	} else if tile == "-" && vectorIn == left {
		return left, nil
	} else if tile == "|" && vectorIn == up {
		return up, nil
	} else if tile == "|" && vectorIn == down {
		return down, nil
	} else if tile == "F" && vectorIn == left {
		return down, nil
	} else if tile == "F" && vectorIn == up {
		return right, nil
	} else if tile == "7" && vectorIn == right {
		return down, nil
	} else if tile == "7" && vectorIn == up {
		return left, nil
	} else if tile == "J" && vectorIn == down {
		return left, nil
	} else if tile == "J" && vectorIn == right {
		return up, nil
	} else if tile == "L" && vectorIn == down {
		return right, nil
	} else if tile == "L" && vectorIn == left {
		return up, nil
	} else {
		return [2]int{}, errors.New("invalid dir")
	}
}

func parseLines(lines <-chan string) ([]string, int, int) {
	tiles := []string{}
	var startX, startY int
	row := 0
	for line := range lines {
		if x := strings.IndexRune(line, 'S'); x >= 0 {
			startY = row
			startX = x
		}
		tiles = append(tiles, line)
		row++
	}
	return tiles, startY, startX
}
