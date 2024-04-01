package day3

import (
	"errors"
	"strconv"
)

type Schematic [][]string

type Part struct {
	part        string
	rowIndex    int
	endColIndex int
}
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
				for error == nil && j < len(s)-1 {
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

func (s Schematic) ValidatePart(part Part) (int, error) {
	hasAdjacentSymbol := false
	partNumber, _ := strconv.Atoi(part.part)
	rowStart := max(part.rowIndex-1, 0)
	rowEnd := min(part.rowIndex+1, len(s)-1)
	colStart := max(part.endColIndex-len(part.part), 0)
	colEnd := min(part.endColIndex+1, len(s[0])-1)
	for i := rowStart; i <= rowEnd; i++ {
		for j := colStart; j <= colEnd; j++ {
			char := s[i][j]
			if _, error := strconv.Atoi(char); error != nil && char != "." {
				hasAdjacentSymbol = true
				break
			}
		}

	}
	if !hasAdjacentSymbol {
		return 0, errors.New("no adjacent symbol")
	}
	return partNumber, nil
}

func (s Schematic) FindParts() []Part {
	parts := make([]Part, 0)
	for i, row := range s {
		part := ""
		for j, symbol := range row {
			_, error := strconv.Atoi(symbol)
			if error == nil {
				part += symbol
			}
			if error != nil || j == len(row)-1 {
				if len(part) > 0 {
					col := j
					// if we found a symbol, go back one to the end of the part string
					if error != nil {
						col = j - 1
					}
					parts = append(parts, Part{part, i, col})
					part = ""
				}
			}
		}
	}
	return parts
}
