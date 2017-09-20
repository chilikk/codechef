package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var s *bufio.Scanner
	s = bufio.NewScanner(os.Stdin)
	s.Scan()
	tests, _ := strconv.Atoi(s.Text())
	for i := 0; i < tests; i++ {
		testCase(s)
	}
}

func testCase(s *bufio.Scanner) {
	s.Scan()
	tokens := strings.Fields(s.Text())
	n, _ := strconv.Atoi(tokens[0])
	m, _ := strconv.Atoi(tokens[1])
	q, _ := strconv.Atoi(tokens[2])
	var a [][]int
	var max int
	a = make([][]int, n, n)
	for i := 0; i < n; i++ {
		a[i] = make([]int, m, m)
		s.Scan()
		tokens = strings.Fields(s.Text())
		for j := 0; j < m; j++ {
			a[i][j], _ = strconv.Atoi(tokens[j])
			max += a[i][j]
		}
	}
	b := constructB(m, n, a)
	for i := 0; i < q; i++ {
		s.Scan()
		k, _ := strconv.Atoi(s.Text())
		doTest(k, m, n, max, a, b)
	}
}

func constructB(m, n int, a [][]int) [][]float64 {
	var b [][]float64
	b = make([][]float64, n, n)
	for i := 0; i < n; i++ {
		b[i] = make([]float64, m, m)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			for k := 0; k < n; k++ {
				for l := 0; l < m; l++ {
					b[k][l] += probability(k, l, i, j, n, m) * float64(a[i][j])
				}
			}
		}
	}
	return b
}

func probability(k, l, i, j, n, m int) float64 {
	p := 1.0
	if k <= i {
		p *= float64(k+1) / float64(i+1)
	} else {
		p *= float64(n-k) / float64(n-i)
	}
	if l <= j {
		p *= float64(l+1) / float64(j+1)
	} else {
		p *= float64(m-l) / float64(m-j)
	}
	return p
}

func doTest(k, m, n, max int, a [][]int, b [][]float64) {
	fmt.Println(k, m, n, max, a, b)
	//for i := 0; i < n; i++ {
	//    for j := 0; j < m; j++ {
	//        fmt.Println(i, j, b[i][j])
	//    }
	//}
}
