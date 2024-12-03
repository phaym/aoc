package day10

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{

		{"input.test.txt", 4},
	}
	for _, c := range cases {
		got := A(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}

func TestB(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{

		{"input.testB.1.txt", 4},
		{"input.testB.2.txt", 8},
		{"input.testB.3.txt", 10},
	}
	for _, c := range cases {
		got := B(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
