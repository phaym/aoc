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

func (node *Node) IsEnd() bool {
	return string(node.Name[len(node.Name)-1]) == "Z"
}
func (node *Node) IsStart() bool {
	return string(node.Name[len(node.Name)-1]) == "A"
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

func B(path string) int {
	lines := file.ReadLinesFromFile(path)
	instructions := <-lines
	<-lines
	currentNodes := make([]*Node, 0)
	nodeMap := make(map[string]*Node)
	for line := range lines {
		node := parseLine(line)
		nodeMap[node.Name] = node
		if node.IsStart() {
			currentNodes = append(currentNodes, node)
		}
	}
	stepCount := stepsToFinalNode(nodeMap, instructions, 0, currentNodes)
	return stepCount
}

func stepsToFinalNode(nodeMap map[string]*Node, instructions string, currentStep int, currentNodes []*Node) int {
	done := true
	dir := string(instructions[currentStep%len(instructions)])
	newNodes := make([]*Node, 0)
	for _, node := range currentNodes {
		if !node.IsEnd() {
			done = false
		}
		next := node.Right
		if dir == "L" {
			next = node.Left
		}
		newNodes = append(newNodes, nodeMap[next])
	}
	if done {
		return 0
	}
	return 1 + stepsToFinalNode(nodeMap, instructions, currentStep+1, newNodes)
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
