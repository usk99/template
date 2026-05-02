package main

import (
	"reflect"
	"testing"
)

// === 1. スライス拡張 ===

func TestSumAll(t *testing.T) {
	if got := SumAll([]int{1, 2, 3, 4}); got != 10 {
		t.Errorf("got %v, want 10", got)
	}
	if got := SumAll([]float64{1.5, 2.5}); got != 4.0 {
		t.Errorf("got %v, want 4.0", got)
	}
}

func TestProductAll(t *testing.T) {
	if got := ProductAll([]int{1, 2, 3, 4}); got != 24 {
		t.Errorf("got %v, want 24", got)
	}
}

func TestAverage(t *testing.T) {
	if got := Average([]int{1, 2, 3}); got != 2.0 {
		t.Errorf("got %v, want 2.0", got)
	}
	if got := Average([]int{}); got != 0.0 {
		t.Errorf("got %v, want 0.0", got)
	}
}

func TestPrefixSum(t *testing.T) {
	got := PrefixSum([]int{1, 2, 3, 4})
	want := []int{0, 1, 3, 6, 10}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestRLE(t *testing.T) {
	got := RLE([]int{1, 1, 2, 3, 3, 3})
	want := []RLEPair[int]{{1, 2}, {2, 1}, {3, 3}}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestCounter(t *testing.T) {
	got := Counter([]int{1, 2, 2, 3, 3, 3})
	want := map[int]int{1: 1, 2: 2, 3: 3}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestIsPalindrome(t *testing.T) {
	if !IsPalindrome([]int{1, 2, 1}) {
		t.Error("expected true")
	}
	if !IsPalindrome([]int{1, 2, 2, 1}) {
		t.Error("expected true")
	}
	if IsPalindrome([]int{1, 2, 3}) {
		t.Error("expected false")
	}
}

// === 2. Ord拡張 ===

func TestMinMaxAll(t *testing.T) {
	a := []int{3, 1, 4, 1, 5}
	if got := MinAll(a); got != 1 {
		t.Errorf("MinAll got %v, want 1", got)
	}
	if got := MaxAll(a); got != 5 {
		t.Errorf("MaxAll got %v, want 5", got)
	}
}

func TestIndexSort(t *testing.T) {
	a := []int{30, 10, 20}
	got := IndexSort(a)
	want := []int{1, 2, 0}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestCompress(t *testing.T) {
	vals, encoded := Compress([]int{30, 10, 20, 10})
	wantVals := []int{10, 20, 30}
	wantEncoded := []int{2, 0, 1, 0}
	if !reflect.DeepEqual(vals, wantVals) {
		t.Errorf("vals got %v, want %v", vals, wantVals)
	}
	if !reflect.DeepEqual(encoded, wantEncoded) {
		t.Errorf("encoded got %v, want %v", encoded, wantEncoded)
	}
}

func TestLowerUpperBound(t *testing.T) {
	a := []int{1, 2, 2, 3, 5}
	if got := LowerBound(a, 2); got != 1 {
		t.Errorf("LowerBound got %v, want 1", got)
	}
	if got := UpperBound(a, 2); got != 3 {
		t.Errorf("UpperBound got %v, want 3", got)
	}
	if got := LowerBound(a, 4); got != 4 {
		t.Errorf("LowerBound(4) got %v, want 4", got)
	}
}

func TestNextPermutation(t *testing.T) {
	a := []int{1, 2, 3}
	NextPermutation(a)
	if !reflect.DeepEqual(a, []int{1, 3, 2}) {
		t.Errorf("got %v, want [1 3 2]", a)
	}
	a = []int{3, 2, 1}
	if NextPermutation(a) {
		t.Error("expected false on last permutation")
	}
}

func TestPrevPermutation(t *testing.T) {
	a := []int{1, 3, 2}
	PrevPermutation(a)
	if !reflect.DeepEqual(a, []int{1, 2, 3}) {
		t.Errorf("got %v, want [1 2 3]", a)
	}
	a = []int{1, 2, 3}
	if PrevPermutation(a) {
		t.Error("expected false on first permutation")
	}
}

// === 3. 数学拡張 ===

func TestGcdAll(t *testing.T) {
	if got := GcdAll([]int{12, 8, 4}); got != 4 {
		t.Errorf("got %v, want 4", got)
	}
	if got := GcdAll([]int{7}); got != 7 {
		t.Errorf("got %v, want 7", got)
	}
}

func TestLcmAll(t *testing.T) {
	if got := LcmAll([]int{4, 6}); got != 12 {
		t.Errorf("got %v, want 12", got)
	}
}

// === 4. 文字・数値拡張 ===

func TestRuneCounter(t *testing.T) {
	got := RuneCounter("aabbc")
	want := map[rune]int{'a': 2, 'b': 2, 'c': 1}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestCharToNum(t *testing.T) {
	if got := CharToNum('5'); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
}

func TestCharToAlpha(t *testing.T) {
	if got := CharToAlpha('a'); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	if got := CharToAlpha('Z'); got != 25 {
		t.Errorf("got %v, want 25", got)
	}
}

func TestCharFromAlpha(t *testing.T) {
	if got := CharFromAlpha(0, false); got != 'a' {
		t.Errorf("got %v, want 'a'", got)
	}
	if got := CharFromAlpha(25, true); got != 'Z' {
		t.Errorf("got %v, want 'Z'", got)
	}
}

func TestDigits(t *testing.T) {
	if got := Digits(123); !reflect.DeepEqual(got, []int{1, 2, 3}) {
		t.Errorf("got %v, want [1 2 3]", got)
	}
	if got := Digits(0); !reflect.DeepEqual(got, []int{0}) {
		t.Errorf("got %v, want [0]", got)
	}
}

func TestDigitSum(t *testing.T) {
	if got := DigitSum(123); got != 6 {
		t.Errorf("got %v, want 6", got)
	}
}

func TestPow(t *testing.T) {
	if got := Pow(2, 10); got != 1024 {
		t.Errorf("got %v, want 1024", got)
	}
	if got := Pow(3, 0); got != 1 {
		t.Errorf("got %v, want 1", got)
	}
}
