package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var (
	sc     *bufio.Scanner
	writer *bufio.Writer
)

func InitIO() {
	sc = bufio.NewScanner(os.Stdin)
	sc.Buffer(make([]byte, 1<<20), 1<<30)
	sc.Split(bufio.ScanWords)
	writer = bufio.NewWriterSize(os.Stdout, 1<<20)
}

// 入力 -------------------------------------------------------------------

func InputNum() int {
	sc.Scan()
	n, _ := strconv.Atoi(sc.Text())
	return n
}

func InputNums(n int) []int {
	v := make([]int, n)
	for i := range v {
		v[i] = InputNum()
	}
	return v
}

func InputsNums(h, w int) [][]int {
	v := make([][]int, h)
	for i := range v {
		v[i] = InputNums(w)
	}
	return v
}

func InputString() string {
	sc.Scan()
	return sc.Text()
}

func InputStrings(n int) []string {
	v := make([]string, n)
	for i := range v {
		v[i] = InputString()
	}
	return v
}

// 出力 -------------------------------------------------------------------

func Output(v any, opts ...string) {
	sep := " "
	end := "\n"
	if len(opts) > 0 {
		sep = opts[0]
	}
	if len(opts) > 1 {
		end = opts[1]
	}

	switch val := v.(type) {
	case bool:
		if val {
			fmt.Fprint(writer, "Yes\n")
		} else {
			fmt.Fprint(writer, "No\n")
		}
	case []int:
		parts := make([]string, len(val))
		for i, x := range val {
			parts[i] = strconv.Itoa(x)
		}
		fmt.Fprint(writer, strings.Join(parts, sep)+end)
	case [][]int:
		for _, row := range val {
			parts := make([]string, len(row))
			for i, x := range row {
				parts[i] = strconv.Itoa(x)
			}
			fmt.Fprint(writer, strings.Join(parts, sep)+end)
		}
	case []string:
		fmt.Fprint(writer, strings.Join(val, sep)+end)
	case [][]string:
		for _, row := range val {
			fmt.Fprint(writer, strings.Join(row, sep)+end)
		}
	default:
		fmt.Fprintf(writer, "%v%s", val, end)
	}
}
