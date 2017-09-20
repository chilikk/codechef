package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scan := bufio.NewScanner(os.Stdin)
	scan.Split(bufio.ScanWords)
	scan.Scan()
	t, _ := strconv.Atoi(scan.Text())
	for ; t > 0; t-- {
		scan.Scan()
		n, _ := strconv.Atoi(scan.Text())
		cur := make([]int, n, n)
		delta := make([]int, n, n)
		for i := 0; i < n; i++ {
			scan.Scan()
			delta[i], _ = strconv.Atoi(scan.Text())
			cur[i] = i + 1
		}
		for i := n - 1; i >= 0; i-- {
			if delta[i] == 0 {
				continue
			} else {
				v := delta[i]
				x := cur[i-v]
				copy(cur[i-v:i], cur[i-v+1:i+1])
				cur[i] = x
			}
		}
		for i, v := range cur {
			var x string
			if i == len(cur)-1 {
				x = "%d\n"
			} else {
				x = "%d "
			}
			fmt.Printf(x, v)
		}
	}
}
