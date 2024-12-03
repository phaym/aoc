package file

import (
	"bufio"
	"os"
)

func ReadLinesFromFile(filePath string) <-chan string {
	lines := make(chan string)
	file, err := os.Open(filePath)
	if err != nil {
		close(lines)
		return lines
	}
	scanner := bufio.NewScanner(file)
	go func() {
		defer close(lines)
		defer file.Close()
		for scanner.Scan() {
			lines <- scanner.Text()
		}
	}()
	return lines
}
