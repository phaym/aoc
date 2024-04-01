package day3

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Run() {
	result := A("year2023/day3/input.txt")
	fmt.Println(result)
}

func A(path string) (total int) {
	schematic := FileToSchematic(path)
	fmt.Printf("%#v", schematic)
	return
}

func FileToSchematic(path string) [][]string {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	lines := ReadLines(file)
	return ParseLines(lines)
}

func ParseLines(lines <-chan string) [][]string {
	schematic := make([][]string, 0)
	for line := range lines {
		schematic = append(schematic, strings.Split(line, ""))
	}
	return schematic
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
