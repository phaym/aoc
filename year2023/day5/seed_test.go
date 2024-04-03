package day5

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 35},
		{"input.txt", 173706076},
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
		{"input.test.txt", 46},
		{"input.txt", 173706076},
	}
	for _, c := range cases {
		got := B(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}

func TestMapNumber(t *testing.T) {
	m := &Map{
		ranges: []Range{
			{50, 98, 2},
			{52, 50, 48},
			{0, 1, 2},
		},
	}
	cases := []struct {
		in   int
		want int
	}{
		{79, 81},
		{14, 14},
		{55, 57},
		{13, 13},
		{1, 0},
	}
	for _, c := range cases {
		got := m.Output(c.in)
		if got != c.want {
			t.Errorf("MapNumber(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}
