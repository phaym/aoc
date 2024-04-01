package day3

import (
	"testing"
)

// func TestParseLine(t *testing.T) {
// 	cases := []struct {
// 		in   string
// 		want Game
// 	}{
// 		{
// 			"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
// 			Game{Id: 1, ColorCounts: map[string][]int{"blue": {3, 6}, "red": {4, 1}, "green": {2, 2}}},
// 		},
// 	}
// 	for _, c := range cases {
// 		got, _ := ParseLine(c.in)
// 		if !cmp.Equal(got, c.want) {
// 			t.Errorf("ParseLine(%q) == %+v, want %+v", c.in, got, c.want)
// 		}
// 	}
// }

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 4361},
	}
	for _, c := range cases {
		got := A(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
