package day3

import (
	"bufio"
	"fmt"
	"os"
)

const FILE_PATH = "year2023/day3/input.txt"

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
		scanner.Text()
	}
	return
}
