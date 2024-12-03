package day7

import (
	"testing"
)

func TestA(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{

		{"input.test.txt", 6440},
		{"input.test2.txt", 5320},
	}
	for _, c := range cases {
		got := A(c.in)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
