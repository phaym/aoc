package day8

import (
	"aoc/util/file"
	"fmt"
	"regexp"
)

func Run() {
	result := A("year2023/day8/input.txt")
	fmt.Println(result)
}

type Node struct {
	Name  string
	Left  string
	Right string
}

func A(path string) int {
	lines := file.ReadLinesFromFile(path)
	instructions := <-lines
	<-lines
	nodeMap := make(map[string]*Node)
	for line := range lines {
		node := parseLine(line)
		nodeMap[node.Name] = node
	}
	currentNode := nodeMap["AAA"]
	currentStep := 0
	for currentNode.Name != "ZZZ" {
		dir := string(instructions[currentStep%len(instructions)])
		next := currentNode.Right
		if dir == "L" {
			next = currentNode.Left
		}
		currentNode = nodeMap[next]
		currentStep++
	}
	return currentStep
}

func parseLine(line string) *Node {
	re := regexp.MustCompile(`^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$`)
	matches := re.FindStringSubmatch(line)
	return &Node{
		Name:  matches[1],
		Left:  matches[2],
		Right: matches[3],
	}
}
