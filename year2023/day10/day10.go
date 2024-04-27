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
	var firstVector [2]int
	// find first move
	for _, vector := range vectors {
		nextRow, nextCol := startRow+vector[0], startCol+vector[1]
		nextTile := string(tiles[nextRow][nextCol])
		if vector, error := getNextVector(vector, nextTile); error == nil {
			firstVector = vector
			break
		}
	}
	steps := 1
	lastVector := firstVector
	row, col := startRow+lastVector[0], startCol+lastVector[1]
	for row != startRow || col != startCol {
		currentTile := string(tiles[row][col])
		vector, _ := getNextVector(lastVector, currentTile)
		row += vector[0]
		col += vector[1]
		steps++
		lastVector = vector
	}
	return steps / 2
}

func getNextVector(fromVector [2]int, current string) ([2]int, error) {
	if current == "-" && fromVector == right {
		return right, nil
	} else if current == "-" && fromVector == left {
		return left, nil
	} else if current == "|" && fromVector == up {
		return up, nil
	} else if current == "|" && fromVector == down {
		return down, nil
	} else if current == "F" && fromVector == left {
		return down, nil
	} else if current == "F" && fromVector == up {
		return right, nil
	} else if current == "7" && fromVector == right {
		return down, nil
	} else if current == "7" && fromVector == up {
		return left, nil
	} else if current == "J" && fromVector == down {
		return left, nil
	} else if current == "J" && fromVector == right {
		return up, nil
	} else if current == "L" && fromVector == down {
		return right, nil
	} else if current == "L" && fromVector == left {
		return up, nil
	} else {
		return up, errors.New("invalid dir")
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
