package main

import (
	"reflect"
	"testing"
)

// === 1. スライス拡張 ===

func TestSumAll(t *testing.T) {
	if got := sumAll([]int{1, 2, 3, 4}); got != 10 {
		t.Errorf("got %v, want 10", got)
	}
	if got := sumAll([]float64{1.5, 2.5}); got != 4.0 {
		t.Errorf("got %v, want 4.0", got)
	}
	// 単一要素
	if got := sumAll([]int{7}); got != 7 {
		t.Errorf("got %v, want 7", got)
	}
	// 負数
	if got := sumAll([]int{-1, -2, 3}); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
}

func TestProductAll(t *testing.T) {
	if got := productAll([]int{1, 2, 3, 4}); got != 24 {
		t.Errorf("got %v, want 24", got)
	}
	// ゼロを含む
	if got := productAll([]int{1, 2, 0, 4}); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	// 単一要素
	if got := productAll([]int{5}); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
}

func TestAverage(t *testing.T) {
	if got := average([]int{1, 2, 3}); got != 2.0 {
		t.Errorf("got %v, want 2.0", got)
	}
	// 空スライス
	if got := average([]int{}); got != 0.0 {
		t.Errorf("got %v, want 0.0", got)
	}
	// 単一要素
	if got := average([]int{4}); got != 4.0 {
		t.Errorf("got %v, want 4.0", got)
	}
}

func TestPrefixSum(t *testing.T) {
	got := prefixSum([]int{1, 2, 3, 4})
	want := []int{0, 1, 3, 6, 10}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// 空スライス
	got2 := prefixSum([]int{})
	if !reflect.DeepEqual(got2, []int{0}) {
		t.Errorf("got %v, want [0]", got2)
	}
	// 単一要素
	got3 := prefixSum([]int{5})
	if !reflect.DeepEqual(got3, []int{0, 5}) {
		t.Errorf("got %v, want [0 5]", got3)
	}
}

func TestRLE(t *testing.T) {
	got := rle([]int{1, 1, 2, 3, 3, 3})
	want := []RlePair[int]{{1, 2}, {2, 1}, {3, 3}}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// 空スライス
	got2 := rle([]int{})
	if len(got2) != 0 {
		t.Errorf("expected empty, got %v", got2)
	}
	// 全要素同じ
	got3 := rle([]int{5, 5, 5})
	if !reflect.DeepEqual(got3, []RlePair[int]{{5, 3}}) {
		t.Errorf("got %v, want [{5 3}]", got3)
	}
	// 全要素異なる
	got4 := rle([]int{1, 2, 3})
	if !reflect.DeepEqual(got4, []RlePair[int]{{1, 1}, {2, 1}, {3, 1}}) {
		t.Errorf("got %v, want [{1 1} {2 1} {3 1}]", got4)
	}
}

func TestCounter(t *testing.T) {
	got := counter([]int{1, 2, 2, 3, 3, 3})
	want := map[int]int{1: 1, 2: 2, 3: 3}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// 空スライス
	got2 := counter([]int{})
	if len(got2) != 0 {
		t.Errorf("expected empty, got %v", got2)
	}
}

func TestIsPalindrome(t *testing.T) {
	if !isPalindrome([]int{1, 2, 1}) {
		t.Error("expected true")
	}
	if !isPalindrome([]int{1, 2, 2, 1}) {
		t.Error("expected true")
	}
	if isPalindrome([]int{1, 2, 3}) {
		t.Error("expected false")
	}
	// 単一要素
	if !isPalindrome([]int{1}) {
		t.Error("single element should be palindrome")
	}
	// 空スライス
	if !isPalindrome([]int{}) {
		t.Error("empty slice should be palindrome")
	}
}

// === 2. Ord拡張 ===

func TestMinMaxAll(t *testing.T) {
	a := []int{3, 1, 4, 1, 5}
	if got := minAll(a); got != 1 {
		t.Errorf("minAll got %v, want 1", got)
	}
	if got := maxAll(a); got != 5 {
		t.Errorf("maxAll got %v, want 5", got)
	}
	// 単一要素
	if got := minAll([]int{7}); got != 7 {
		t.Errorf("minAll single got %v, want 7", got)
	}
	// 負数
	if got := minAll([]int{-3, -1, -2}); got != -3 {
		t.Errorf("minAll negative got %v, want -3", got)
	}
}

func TestIndexSort(t *testing.T) {
	a := []int{30, 10, 20}
	got := indexSort(a)
	want := []int{1, 2, 0}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// 単一要素
	got2 := indexSort([]int{5})
	if !reflect.DeepEqual(got2, []int{0}) {
		t.Errorf("got %v, want [0]", got2)
	}
	// 全要素同じ（安定性は保証しないが長さは合う）
	got3 := indexSort([]int{1, 1, 1})
	if len(got3) != 3 {
		t.Errorf("expected len 3, got %d", len(got3))
	}
}

func TestCompress(t *testing.T) {
	vals, encoded := compress([]int{30, 10, 20, 10})
	wantVals := []int{10, 20, 30}
	wantEncoded := []int{2, 0, 1, 0}
	if !reflect.DeepEqual(vals, wantVals) {
		t.Errorf("vals got %v, want %v", vals, wantVals)
	}
	if !reflect.DeepEqual(encoded, wantEncoded) {
		t.Errorf("encoded got %v, want %v", encoded, wantEncoded)
	}
	// 単一要素
	vals2, encoded2 := compress([]int{5})
	if !reflect.DeepEqual(vals2, []int{5}) || !reflect.DeepEqual(encoded2, []int{0}) {
		t.Errorf("single element: vals=%v encoded=%v", vals2, encoded2)
	}
}

func TestLowerUpperBound(t *testing.T) {
	a := []int{1, 2, 2, 3, 5}
	if got := lowerBound(a, 2); got != 1 {
		t.Errorf("lowerBound got %v, want 1", got)
	}
	if got := upperBound(a, 2); got != 3 {
		t.Errorf("upperBound got %v, want 3", got)
	}
	if got := lowerBound(a, 4); got != 4 {
		t.Errorf("lowerBound(4) got %v, want 4", got)
	}
	// 全要素より小さい値
	if got := lowerBound(a, 0); got != 0 {
		t.Errorf("lowerBound(0) got %v, want 0", got)
	}
	// 全要素より大きい値
	if got := upperBound(a, 9); got != 5 {
		t.Errorf("upperBound(9) got %v, want 5", got)
	}
}

func TestNextPermutation(t *testing.T) {
	a := []int{1, 2, 3}
	nextPermutation(a)
	if !reflect.DeepEqual(a, []int{1, 3, 2}) {
		t.Errorf("got %v, want [1 3 2]", a)
	}
	// 最後の順列
	a = []int{3, 2, 1}
	if nextPermutation(a) {
		t.Error("expected false on last permutation")
	}
	// 単一要素
	a = []int{1}
	if nextPermutation(a) {
		t.Error("expected false on single element")
	}
}

func TestPrevPermutation(t *testing.T) {
	a := []int{1, 3, 2}
	prevPermutation(a)
	if !reflect.DeepEqual(a, []int{1, 2, 3}) {
		t.Errorf("got %v, want [1 2 3]", a)
	}
	// 最初の順列
	a = []int{1, 2, 3}
	if prevPermutation(a) {
		t.Error("expected false on first permutation")
	}
	// 単一要素
	a = []int{1}
	if prevPermutation(a) {
		t.Error("expected false on single element")
	}
}

// === 3. 数学拡張 ===

func TestGcdAll(t *testing.T) {
	if got := gcdAll([]int{12, 8, 4}); got != 4 {
		t.Errorf("got %v, want 4", got)
	}
	if got := gcdAll([]int{7}); got != 7 {
		t.Errorf("got %v, want 7", got)
	}
	// 空スライス
	if got := gcdAll([]int{}); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	// 互いに素
	if got := gcdAll([]int{7, 11}); got != 1 {
		t.Errorf("got %v, want 1", got)
	}
	// int64
	if got := gcdAll([]int64{12, 8, 4}); got != 4 {
		t.Errorf("int64: got %v, want 4", got)
	}
}

func TestLcmAll(t *testing.T) {
	if got := lcmAll([]int{4, 6}); got != 12 {
		t.Errorf("got %v, want 12", got)
	}
	// 単一要素
	if got := lcmAll([]int{5}); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
	// 空スライス
	if got := lcmAll([]int{}); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	// int64
	if got := lcmAll([]int64{4, 6}); got != 12 {
		t.Errorf("int64: got %v, want 12", got)
	}
}

// === 4. 文字・数値拡張 ===

func TestRuneCounter(t *testing.T) {
	got := runeCounter("aabbc")
	want := map[rune]int{'a': 2, 'b': 2, 'c': 1}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("got %v, want %v", got, want)
	}
	// 空文字列
	got2 := runeCounter("")
	if len(got2) != 0 {
		t.Errorf("expected empty, got %v", got2)
	}
}

func TestCharToNum(t *testing.T) {
	if got := charToNum('0'); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	if got := charToNum('5'); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
	if got := charToNum('9'); got != 9 {
		t.Errorf("got %v, want 9", got)
	}
}

func TestCharToAlpha(t *testing.T) {
	if got := charToAlpha('a'); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	if got := charToAlpha('z'); got != 25 {
		t.Errorf("got %v, want 25", got)
	}
	if got := charToAlpha('A'); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	if got := charToAlpha('Z'); got != 25 {
		t.Errorf("got %v, want 25", got)
	}
}

func TestCharFromAlpha(t *testing.T) {
	if got := charFromAlpha(0, false); got != 'a' {
		t.Errorf("got %v, want 'a'", got)
	}
	if got := charFromAlpha(25, false); got != 'z' {
		t.Errorf("got %v, want 'z'", got)
	}
	if got := charFromAlpha(0, true); got != 'A' {
		t.Errorf("got %v, want 'A'", got)
	}
	if got := charFromAlpha(25, true); got != 'Z' {
		t.Errorf("got %v, want 'Z'", got)
	}
}

func TestDigits(t *testing.T) {
	if got := digits(123); !reflect.DeepEqual(got, []int{1, 2, 3}) {
		t.Errorf("got %v, want [1 2 3]", got)
	}
	// ゼロ
	if got := digits(0); !reflect.DeepEqual(got, []int{0}) {
		t.Errorf("got %v, want [0]", got)
	}
	// 単一桁
	if got := digits(7); !reflect.DeepEqual(got, []int{7}) {
		t.Errorf("got %v, want [7]", got)
	}
	// 負数
	if got := digits(-123); !reflect.DeepEqual(got, []int{1, 2, 3}) {
		t.Errorf("negative: got %v, want [1 2 3]", got)
	}
	// int64
	if got := digits(int64(456)); !reflect.DeepEqual(got, []int{4, 5, 6}) {
		t.Errorf("int64: got %v, want [4 5 6]", got)
	}
}

func TestDigitSum(t *testing.T) {
	if got := digitSum(123); got != 6 {
		t.Errorf("got %v, want 6", got)
	}
	// ゼロ
	if got := digitSum(0); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	// 単一桁
	if got := digitSum(9); got != 9 {
		t.Errorf("got %v, want 9", got)
	}
	// 負数
	if got := digitSum(-123); got != 6 {
		t.Errorf("negative: got %v, want 6", got)
	}
	// int64
	if got := digitSum(int64(456)); got != 15 {
		t.Errorf("int64: got %v, want 15", got)
	}
}

func TestPow(t *testing.T) {
	if got := pow(2, 10); got != 1024 {
		t.Errorf("got %v, want 1024", got)
	}
	// 0乗
	if got := pow(3, 0); got != 1 {
		t.Errorf("got %v, want 1", got)
	}
	// 1乗
	if got := pow(5, 1); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
	// 底が0
	if got := pow(0, 5); got != 0 {
		t.Errorf("got %v, want 0", got)
	}
	// int64
	if got := pow(int64(2), 10); got != 1024 {
		t.Errorf("int64: got %v, want 1024", got)
	}
	// float64
	if got := pow(2.0, 3); got != 8.0 {
		t.Errorf("float64: got %v, want 8.0", got)
	}
}
