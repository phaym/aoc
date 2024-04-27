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
	tiles, row, col := parseLines(lines)
	// find first move
	var vector [2]int
	for _, dir := range vectors {
		nextRow, nextCol := row+dir[0], col+dir[1]
		nextTile := string(tiles[nextRow][nextCol])
		var err error
		if _, err = getNextVector(dir, nextTile); err == nil {
			vector = dir
			break
		}
	}
	steps := 0
	var vectorErr error
	for vectorErr == nil {
		row += vector[0]
		col += vector[1]
		currentTile := string(tiles[row][col])
		vector, vectorErr = getNextVector(vector, currentTile)
		steps++
	}
	return steps / 2
}

func getNextVector(dir [2]int, current string) ([2]int, error) {
	if current == "-" && dir == right {
		return right, nil
	} else if current == "-" && dir == left {
		return left, nil
	} else if current == "|" && dir == up {
		return up, nil
	} else if current == "|" && dir == down {
		return down, nil
	} else if current == "F" && dir == left {
		return down, nil
	} else if current == "F" && dir == up {
		return right, nil
	} else if current == "7" && dir == right {
		return down, nil
	} else if current == "7" && dir == up {
		return left, nil
	} else if current == "J" && dir == down {
		return left, nil
	} else if current == "J" && dir == right {
		return up, nil
	} else if current == "L" && dir == down {
		return right, nil
	} else if current == "L" && dir == left {
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
