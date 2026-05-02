package main

func Solve() {
	N := InputNum()
	A := InputNums(N)
	Output(A)
}

func main() {
	InitIO()
	defer writer.Flush()
	Solve()
}
