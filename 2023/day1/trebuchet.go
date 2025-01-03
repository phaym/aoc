package day1

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const FILE_PATH = "year2023/day1/input.txt"

func Run() {
	file, err := os.Open(FILE_PATH)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	total := DecodeFile(file)
	fmt.Println(total)
}

func DecodeFile(file *os.File) (total int) {
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		total += DecodeLine(scanner.Text())
	}
	return
}

func DecodeLine(line string) int {
	tokenMap := map[string]int{
		"1":     1,
		"2":     2,
		"3":     3,
		"4":     4,
		"5":     5,
		"6":     6,
		"7":     7,
		"8":     8,
		"9":     9,
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}
	var first, second int
	lowest, highest := len(line), -1
	for k, v := range tokenMap {
		if i := strings.Index(line, k); i >= 0 && i < lowest {
			first, lowest = v, i
		}
		if i := strings.LastIndex(line, k); i >= 0 && i > highest {
			second, highest = v, i
		}
	}

	return first*10 + second
}
