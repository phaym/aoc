package day1

import (
	"os"
	"testing"
)

func TestDecodeLine(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"1abc2", 12},
		{"pqr3stu8vwx", 38},
		{"a1b2c3d4e5f", 15},
		{"treb7uchet", 77},
		{"gtmszpsjmggr3", 33},
		{"7dvt", 77},
		{"five1ninetjjtfxqpdkgrxtgxrcsevenkfdzlh", 57},
		{"two1nine", 29},
		{"eightwothree", 83},
		{"abcone2threexyz", 13},
		{"xtwone3four", 24},
		{"4nineeightseven2", 42},
		{"zoneight234", 14},
		{"7pqrstsixteen", 76},
		{"1dveight19l1", 11},
	}
	for _, c := range cases {
		got := DecodeLine(c.in)
		if got != c.want {
			t.Errorf("DecodeLine(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}

func TestDecodeFile(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"input.test.txt", 281},
	}
	for _, c := range cases {
		file, err := os.Open(c.in)
		if err != nil {
			panic(err)
		}
		defer file.Close()
		got := DecodeFile(file)
		if got != c.want {
			t.Errorf("DecodeFile(%q) == %v, want %v", c.in, got, c.want)
		}
	}
}
