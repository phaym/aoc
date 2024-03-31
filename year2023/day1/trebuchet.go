package day1

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)
const FILE_PATH = "year2023/day1/input.txt"

func Run() { 
	file, err := os.Open(FILE_PATH)
	if err != nil { 
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file);
	scanner.Split(bufio.ScanLines)
	total := 0
	lines := 0
	for scanner.Scan() { 
		total += DecodeLine(scanner.Text())
		lines++
	}
	fmt.Println(total, lines)
}

func DecodeLine(line string) int {
	var r1, r2 string
	for start, end := 0, len(line) - 1; start <= end; start, end = start + 1, end - 1 {
		if r1 == "" { 
			if _, err := strconv.Atoi(string(line[start])); err == nil { 
				r1 = string(line[start])
			}
		}
		if r2 == "" {
			if _, err := strconv.Atoi(string(line[end])); err == nil { 
				r2 = string(line[end])
			}
		}
		if r1 != "" && r2 != ""{
			break
		}
	}
	result, err := strconv.Atoi(r1 + "" + r2)
	if err != nil { 
		panic(fmt.Sprintf("%v is not a number", result))
	}
	return result
}