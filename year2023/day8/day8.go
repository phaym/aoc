package day8

import (
	"aoc/util/file"
	"fmt"
	"regexp"
	"strings"
)

func Run() {
	result := B("year2023/day8/input.txt")
	fmt.Println(result)
}

type Node struct {
	Name  string
	Left  string
	Right string
}

func (node *Node) IsEnd() bool {
	return strings.HasSuffix(node.Name, "Z")
}
func (node *Node) IsStart() bool {
	return strings.HasSuffix(node.Name, "A")
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

var nodeMap = make(map[string]*Node)

func B(path string) int {
	lines := file.ReadLinesFromFile(path)
	instructions := <-lines
	<-lines
	currentNodes := make([]*Node, 0)

	for line := range lines {
		node := parseLine(line)
		nodeMap[node.Name] = node
		if node.IsStart() {
			currentNodes = append(currentNodes, node)
		}
	}
	stepCount := stepsToFinalNode(instructions, currentNodes)
	return stepCount
}

func stepsToFinalNode(instructions string, currentNodes []*Node) int {
	currentStep := 0
	processing := true
	for processing {
		processing = false
		dir := string(instructions[currentStep%len(instructions)])
		for i := 0; i < len(currentNodes); i++ {
			node := currentNodes[0]
			currentNodes = currentNodes[1:]
			if !node.IsEnd() {
				processing = true
			}
			next := node.Right
			if dir == "L" {
				next = node.Left
			}
			currentNodes = append(currentNodes, nodeMap[next])
		}
		if processing {
			currentStep++
		}
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
