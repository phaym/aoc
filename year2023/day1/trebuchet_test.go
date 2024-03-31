package day1

import "testing"

func TestDecodeLine(t *testing.T){
	cases := []struct {
        in string
		want int
    }{
        {"1abc2", 12},
        {"pqr3stu8vwx", 38},
        {"a1b2c3d4e5f", 15},
        {"treb7uchet", 7},
    }
    for _, c := range cases {
        got := DecodeLine(c.in)
        if got != c.want {
            t.Errorf("DecodeLine(%q) == %v, want %v", c.in, got, c.want)
        }
    }
}