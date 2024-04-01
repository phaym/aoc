package day3

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Run() {
	result := B("year2023/day3/input.txt")
	fmt.Println(result)
}

func B(path string) (total int) {
	schematic := FileToSchematic(path)
	symbols := schematic.FindSymbols()
	for _, symbol := range symbols {
		if symbol.symbol != "*" {
			continue
		}
		partNumbers := schematic.GetAdjacentParts(symbol)
		if len(partNumbers) == 2 {
			gearRatio := partNumbers[0] * partNumbers[1]
			total += gearRatio
		}
	}
	return
}

func A(path string) (total int) {
	schematic := FileToSchematic(path)
	parts := schematic.FindParts()
	for _, part := range parts {
		if partNumber, error := schematic.ValidatePart(part); error == nil {
			total += partNumber
		}
	}
	return total
}

func FileToSchematic(path string) Schematic {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	lines := ReadLines(file)
	return ParseLines(lines)
}

func ReadLines(file *os.File) <-chan string {
	c := make(chan string)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	go func() {
		for scanner.Scan() {
			c <- scanner.Text()
		}
		close(c)
	}()
	return c
}

func ParseLines(lines <-chan string) Schematic {
	schematic := make([][]string, 0)
	for line := range lines {
		schematic = append(schematic, strings.Split(line, ""))
	}
	return schematic
}
