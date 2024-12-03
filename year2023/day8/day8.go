package day8

import (
	"fmt"
	"regexp"
	"strings"
	"year2023/util/file"
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

func stepsToFinalNode(instructions string, startNodes []*Node) int {
	stepsToFinal := make([]int, 0)
	for _, node := range startNodes {
		currentStep := 0
		for !node.IsEnd() {
			dir := string(instructions[currentStep%len(instructions)])
			nextName := node.Right
			if dir == "L" {
				nextName = node.Left
			}
			node = nodeMap[nextName]
			currentStep++
			if node.IsEnd() {
				stepsToFinal = append(stepsToFinal, currentStep)
			}
		}
	}
	result := stepsToFinal[0]
	for i := 1; i < len(stepsToFinal); i++ {
		result = lcm(result, stepsToFinal[i])
	}
	return result
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int) int {
	return a / gcd(a, b) * b
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
