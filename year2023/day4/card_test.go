package day4

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 0},
	}
	for _, c := range cases {
		got := A(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}

func TestCalcPoints(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8},
	}
	for _, c := range cases {
		got := CalcPoints(c.in)
		if got != c.want {
			t.Errorf("CalcPoints(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
