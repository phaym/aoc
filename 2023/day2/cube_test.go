package day2

import (
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestParseLine(t *testing.T) {
	cases := []struct {
		in   string
		want Game
	}{
		{
			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			Game{Id: 1, ColorCounts: map[string][]int{"blue": {3, 6}, "red": {4, 1}, "green": {2, 2}}},
		},
		{
			"Game 100: 16 blue, 12 red, 3 green; 2 green, 7 blue; 5 blue, 4 green; 10 blue, 6 red, 6 green; 5 red, 12 blue, 2 green; 9 red, 12 blue, 11 green",
			Game{Id: 100, ColorCounts: map[string][]int{"blue": {16, 7, 5, 10, 12, 12}, "red": {12, 6, 5, 9}, "green": {3, 2, 4, 6, 2, 11}}},
		},
	}
	for _, c := range cases {
		got, _ := ParseLine(c.in)
		if !cmp.Equal(got, c.want) {
			t.Errorf("ParseLine(%q) == %+v, want %+v", c.in, got, c.want)
		}
	}
}

func TestDecodeFile(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 2286},
	}
	for _, c := range cases {
		file, err := os.Open(c.in)
		if err != nil {
			panic(err)
		}
		defer file.Close()
		got := ParseFile(file)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
