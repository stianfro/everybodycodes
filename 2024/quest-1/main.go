package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input1.txt
var input1 string

//go:embed input2-example.txt
var input2 string

func main() {
	partOne(input1)
	partTwo(input2)
}

func partOne(input string) {
	var potions int

	list := strings.Split(input, "")

	for _, creature := range list {
		if creature == "\n" {
			continue
		}

		var potionsNeeded int

		switch creature {
		case "A":
			potionsNeeded = 0
		case "B":
			potionsNeeded = 1
		case "C":
			potionsNeeded = 3
		}

		potions += potionsNeeded
	}

	fmt.Printf("Part 1: %d\n", potions)
}

func partTwo(input string) {
	// iterate list, add char to slice, when slice contains 2 chars add to listPairs
}
