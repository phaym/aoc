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
	nextTile := ""
	var moveVector [2]int
	for _, currentVector := range vectors {
		nextRow, nextCol := startRow+currentVector[0], startCol+currentVector[1]
		if nextRow < 0 || nextRow > len(tiles)-1 || nextCol < 0 || nextCol > len(tiles[0])-1 {
			continue
		}
		nextTile := string(tiles[nextRow][nextCol])
		if vector, error := getMoveVector(currentVector, nextTile); error == nil {
			moveVector = vector
			break
		}
	}
	steps := 1
	row, col := startRow+moveVector[0], startCol+moveVector[1]
	for nextTile != "S" {
		row += moveVector[0]
		col += moveVector[1]
		nextTile = string(tiles[row][col])
		moveVector, _ = getMoveVector(moveVector, nextTile)
		steps++
	}
	return steps / 2
}

func getMoveVector(vector [2]int, current string) ([2]int, error) {
	if current == "-" && vector == right {
		return right, nil
	} else if current == "-" && vector == left {
		return left, nil
	} else if current == "|" && vector == up {
		return up, nil
	} else if current == "|" && vector == down {
		return down, nil
	} else if current == "F" && vector == left {
		return down, nil
	} else if current == "F" && vector == up {
		return right, nil
	} else if current == "7" && vector == right {
		return down, nil
	} else if current == "7" && vector == up {
		return left, nil
	} else if current == "J" && vector == down {
		return left, nil
	} else if current == "J" && vector == right {
		return up, nil
	} else if current == "L" && vector == down {
		return right, nil
	} else if current == "L" && vector == left {
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
	return tiles, startX, startY
}
