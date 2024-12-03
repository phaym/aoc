package day8

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{

		{"input.test.txt", 2},
		{"input.test2.txt", 6},
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

		{"input.test.B.txt", 6},
	}
	for _, c := range cases {
		got := B(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
