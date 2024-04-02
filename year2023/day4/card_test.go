package day4

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 13},
		{"input.test.2.txt", 256},
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
		{"input.test.txt", 30},
	}
	for _, c := range cases {
		got := B(c.in)
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
		{"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2},
	}
	for _, c := range cases {
		got := CalcPoints(ParseCard(c.in))
		if got != c.want {
			t.Errorf("CalcPoints(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
