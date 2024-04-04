package day5

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

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

func TestMapNumberB(t *testing.T) {
	mapObj := &Map{
		ranges: []Range{
			{57, 5, 6},
		},
	}
	cases := []struct {
		in   Seed
		want []Seed
	}{
		{
			Seed{5, 6}, // 5 - 10
			[]Seed{
				{57, 6},
			},
		},
		{
			Seed{1, 7}, // 1 - 7
			[]Seed{
				{57, 3},
				{1, 4},
			},
		},
		{
			Seed{7, 9}, // 7 - 15
			[]Seed{
				{59, 4},
				{11, 5},
			},
		},
		{
			Seed{1, 12}, // 1 - 12
			[]Seed{
				{57, 6},
				{11, 2},
				{1, 4},
			},
		},
		{
			Seed{6, 4}, // 6 - 9
			[]Seed{
				{58, 4},
			},
		},
	}
	for i, c := range cases {
		got := mapObj.OutputB(c.in)
		if !cmp.Equal(got, c.want) {
			t.Errorf("case# %v SeedOutput(%v) == %v, want %v", i, c.in, got, c.want)
		}
	}
}
