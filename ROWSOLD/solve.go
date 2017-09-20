package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	ncases, _ := strconv.Atoi(scanner.Text())
	for i := 0; i < ncases; i++ {
		scanner.Scan()
		line := scanner.Text()
		var multiplier, zerocount, time uint64
		time = 0
		multiplier = 0
		zerocount = 0
		for _, c := range line {
			if c == '0' {
				zerocount += 1
			} else {
				if zerocount > 0 {
					time += (1 + zerocount) * multiplier
				}
				multiplier += 1
				zerocount = 0
			}
		}
		if zerocount > 0 {
			time += (1 + zerocount) * multiplier
		}
		fmt.Println(time)
	}
}
