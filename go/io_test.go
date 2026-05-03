package main

import (
	"bufio"
	"bytes"
	"reflect"
	"strings"
	"testing"
)

func initSc(s string) {
	sc = bufio.NewScanner(strings.NewReader(s))
	sc.Buffer(make([]byte, 1<<20), 1<<30)
	sc.Split(bufio.ScanWords)
}

func initWriter() *bytes.Buffer {
	buf := &bytes.Buffer{}
	writer = bufio.NewWriter(buf)
	return buf
}

func flushWriter() {
	writer.Flush()
}

// === InputNum ===

func TestInputNum(t *testing.T) {
	initSc("0 1 -1 1000000000000000000")
	cases := []int{0, 1, -1, 1000000000000000000}
	for _, want := range cases {
		if got := InputNum(); got != want {
			t.Errorf("got %v, want %v", got, want)
		}
	}
}

func TestInputNumPanic(t *testing.T) {
	initSc("5.5")
	defer func() {
		if r := recover(); r == nil {
			t.Error("expected panic, got none")
		}
	}()
	InputNum()
}

func TestInputNumPanicAlpha(t *testing.T) {
	initSc("abc")
	defer func() {
		if r := recover(); r == nil {
			t.Error("expected panic, got none")
		}
	}()
	InputNum()
}

// === InputNums / InputNumsSelf ===

func TestInputNums(t *testing.T) {
	initSc("3 10 20 30")
	// 通常
	if got := InputNums(3); !reflect.DeepEqual(got, []int{3, 10, 20}) {
		t.Errorf("got %v", got)
	}
	// n=0
	initSc("")
	if got := InputNums(0); !reflect.DeepEqual(got, []int{}) {
		t.Errorf("n=0: got %v", got)
	}
}

func TestInputNumsSelf(t *testing.T) {
	initSc("3 10 20 30")
	if got := InputNumsSelf(); !reflect.DeepEqual(got, []int{10, 20, 30}) {
		t.Errorf("got %v, want [10 20 30]", got)
	}
	// N=0
	initSc("0")
	if got := InputNumsSelf(); !reflect.DeepEqual(got, []int{}) {
		t.Errorf("N=0: got %v", got)
	}
}

// === InputFloat ===

func TestInputFloat(t *testing.T) {
	initSc("3.14 2.71 0.0 -1.5 1e9")
	cases := []float64{3.14, 2.71, 0.0, -1.5, 1e9}
	for _, want := range cases {
		if got := InputFloat(); got != want {
			t.Errorf("got %v, want %v", got, want)
		}
	}
}

func TestInputFloats(t *testing.T) {
	initSc("1.1 2.2 3.3")
	got := InputFloats(3)
	want := []float64{1.1, 2.2, 3.3}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// n=0
	if got := InputFloats(0); !reflect.DeepEqual(got, []float64{}) {
		t.Errorf("n=0: got %v", got)
	}
}

func TestInputFloatsSelf(t *testing.T) {
	initSc("3 1.1 2.2 3.3")
	got := InputFloatsSelf()
	want := []float64{1.1, 2.2, 3.3}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestInputFloatPanic(t *testing.T) {
	initSc("abc")
	defer func() {
		if r := recover(); r == nil {
			t.Error("expected panic, got none")
		}
	}()
	InputFloat()
}

// === InputString / InputStrings / InputStringsSelf ===

func TestInputString(t *testing.T) {
	initSc("hello world")
	if got := InputString(); got != "hello" {
		t.Errorf("got %v, want hello", got)
	}
	if got := InputString(); got != "world" {
		t.Errorf("got %v, want world", got)
	}
}

func TestInputStrings(t *testing.T) {
	initSc("foo bar baz")
	got := InputStrings(3)
	want := []string{"foo", "bar", "baz"}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// n=0
	if got := InputStrings(0); !reflect.DeepEqual(got, []string{}) {
		t.Errorf("n=0: got %v", got)
	}
}

func TestInputStringsSelf(t *testing.T) {
	initSc("3 foo bar baz")
	got := InputStringsSelf()
	want := []string{"foo", "bar", "baz"}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// N=0
	initSc("0")
	if got := InputStringsSelf(); !reflect.DeepEqual(got, []string{}) {
		t.Errorf("N=0: got %v", got)
	}
}

// === Output ===

func TestOutputInt(t *testing.T) {
	buf := initWriter()
	Output(42)
	flushWriter()
	if got := buf.String(); got != "42\n" {
		t.Errorf("got %q, want %q", got, "42\n")
	}
}

func TestOutputBoolTrue(t *testing.T) {
	buf := initWriter()
	Output(true)
	flushWriter()
	if got := buf.String(); got != "Yes\n" {
		t.Errorf("got %q, want %q", got, "Yes\n")
	}
}

func TestOutputBoolFalse(t *testing.T) {
	buf := initWriter()
	Output(false)
	flushWriter()
	if got := buf.String(); got != "No\n" {
		t.Errorf("got %q, want %q", got, "No\n")
	}
}

func TestOutputSliceInt(t *testing.T) {
	buf := initWriter()
	Output([]int{1, 2, 3})
	flushWriter()
	if got := buf.String(); got != "1 2 3\n" {
		t.Errorf("got %q, want %q", got, "1 2 3\n")
	}
}

func TestOutputSliceIntCustomSep(t *testing.T) {
	buf := initWriter()
	Output([]int{1, 2, 3}, "\n")
	flushWriter()
	if got := buf.String(); got != "1\n2\n3\n" {
		t.Errorf("got %q, want %q", got, "1\n2\n3\n")
	}
}

func TestOutputSliceIntEmpty(t *testing.T) {
	buf := initWriter()
	Output([]int{})
	flushWriter()
	if got := buf.String(); got != "\n" {
		t.Errorf("got %q, want %q", got, "\n")
	}
}

func TestOutputMatrix(t *testing.T) {
	buf := initWriter()
	Output([][]int{{1, 2}, {3, 4}})
	flushWriter()
	if got := buf.String(); got != "1 2\n3 4\n" {
		t.Errorf("got %q, want %q", got, "1 2\n3 4\n")
	}
}

func TestOutputSliceString(t *testing.T) {
	buf := initWriter()
	Output([]string{"a", "b", "c"})
	flushWriter()
	if got := buf.String(); got != "a b c\n" {
		t.Errorf("got %q, want %q", got, "a b c\n")
	}
}

func TestOutputString(t *testing.T) {
	buf := initWriter()
	Output("hello")
	flushWriter()
	if got := buf.String(); got != "hello\n" {
		t.Errorf("got %q, want %q", got, "hello\n")
	}
}

func TestOutputNegative(t *testing.T) {
	buf := initWriter()
	Output(-100)
	flushWriter()
	if got := buf.String(); got != "-100\n" {
		t.Errorf("got %q, want %q", got, "-100\n")
	}
}
