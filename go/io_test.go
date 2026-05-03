package main

import (
	"bufio"
	"reflect"
	"strings"
	"testing"
)

func initSc(s string) {
	sc = bufio.NewScanner(strings.NewReader(s))
	sc.Buffer(make([]byte, 1<<20), 1<<30)
	sc.Split(bufio.ScanWords)
}

func TestInputFloat(t *testing.T) {
	initSc("3.14 2.71 0.0 -1.5")

	if got := InputFloat(); got != 3.14 {
		t.Errorf("got %v, want 3.14", got)
	}
	if got := InputFloat(); got != 2.71 {
		t.Errorf("got %v, want 2.71", got)
	}
	if got := InputFloat(); got != 0.0 {
		t.Errorf("got %v, want 0.0", got)
	}
	if got := InputFloat(); got != -1.5 {
		t.Errorf("got %v, want -1.5", got)
	}
}

func TestInputFloats(t *testing.T) {
	initSc("1.1 2.2 3.3")
	got := InputFloats(3)
	want := []float64{1.1, 2.2, 3.3}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
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

func TestInputFloatPanic(t *testing.T) {
	initSc("abc")
	defer func() {
		if r := recover(); r == nil {
			t.Error("expected panic, got none")
		}
	}()
	InputFloat()
}
