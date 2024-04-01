package day3

import (
	"strconv"
)

type Schematic [][]string
type Symbol struct {
	symbol string
	xIndex int
	yIndex int
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func (s Schematic) FindSymbols() []Symbol {
	symbols := make([]Symbol, 0)
	for i, row := range s {
		for j, char := range row {
			if _, error := strconv.Atoi(char); error != nil && char != "." {
				symbols = append(symbols, Symbol{char, i, j})
			}
		}
	}
	return symbols
}

func (s Schematic) GetAdjacentParts(symbol Symbol) []int {
	partNumbers := make([]int, 0)
	rowStart, rowEnd := max(symbol.xIndex-1, 0), min(symbol.xIndex+1, len(s))
	colStart, colEnd := max(symbol.yIndex-1, 0), min(symbol.yIndex+1, len(s[0])-1)

	for i := rowStart; i <= rowEnd; i++ {
		for j := colStart; j <= colEnd; j++ {
			char := s[i][j]
			if _, error := strconv.Atoi(char); error == nil {
				for error == nil && j > 0 {
					if _, error = strconv.Atoi(s[i][j-1]); error == nil {
						j--
					}
				}
				numString := ""
				error = nil
				for error == nil && j < len(s[0]) {
					if _, error = strconv.Atoi(s[i][j]); error == nil {
						numString += s[i][j]
						j++
					}
				}
				partNumber, error := strconv.Atoi(numString)
				if error != nil {
					panic(error)
				}
				partNumbers = append(partNumbers, partNumber)
			}
		}
	}
	return partNumbers
}
