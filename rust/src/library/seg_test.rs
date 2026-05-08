use crate::library::seg::*;
// -------------------------------------------------------------------------
// 1. 各演算の個別テスト
// -------------------------------------------------------------------------
#[test]
fn test_sum() {
    let mut seg = SegSum::new(&[1, 2, 3, 4, 5]);
    assert_eq!(seg.get_range(0, 5), 15);
    assert_eq!(seg.get_range(1, 3), 5);
    assert_eq!(seg.get_range(0, 0), 0); // 空区間は単位元
    seg.set(2, 10);
    assert_eq!(seg.get_range(0, 5), 22); // 1+2+10+4+5
}
#[test]
fn test_min_max() {
    let data = [10usize, 20, 5, 30, 15];
    let seg_min = SegMin::new(&data);
    let seg_max = SegMax::new(&data);
    assert_eq!(seg_min.get_range(0, 5), 5);
    assert_eq!(seg_min.get_range(0, 2), 10);
    assert_eq!(seg_min.get_range(3, 5), 15);
    assert_eq!(seg_min.get_range(0, 0), usize::MAX); // 単位元
    assert_eq!(seg_max.get_range(0, 5), 30);
    assert_eq!(seg_max.get_range(0, 3), 20);
    assert_eq!(seg_max.get_range(0, 0), 0); // 単位元
}
#[test]
fn test_bitwise() {
    let data = [1usize, 2, 4, 7]; // [001, 010, 100, 111]
    let seg_xor = SegXor::new(&data);
    let seg_or  = SegBitOr::new(&data);
    let seg_and = SegBitAnd::new(&data);
    assert_eq!(seg_xor.get_range(0, 4), 0);  // 1^2^4^7=0
    assert_eq!(seg_xor.get_range(0, 2), 3);  // 1^2=3
    assert_eq!(seg_or.get_range(0, 4), 7);   // 1|2|4|7=7
    assert_eq!(seg_and.get_range(0, 4), 0);  // 1&2&4&7=0
    assert_eq!(seg_and.get_range(2, 4), 4);  // 4&7=4
    assert_eq!(seg_and.get_range(0, 0), usize::MAX); // 単位元
}
#[test]
fn test_gcd_lcm() {
    let data = [12usize, 18, 6];
    let seg_gcd = SegGcd::new(&data);
    let seg_lcm = SegLcm::new(&data);
    assert_eq!(seg_gcd.get_range(0, 2), 6);  // gcd(12,18)=6
    assert_eq!(seg_gcd.get_range(0, 3), 6);  // gcd(12,18,6)=6
    assert_eq!(seg_lcm.get_range(0, 2), 36); // lcm(12,18)=36
    assert_eq!(seg_lcm.get_range(1, 3), 18); // lcm(18,6)=18
    assert_eq!(seg_gcd.get_range(0, 0), 0);  // 単位元
    assert_eq!(seg_lcm.get_range(0, 0), 1);  // 単位元
}

// -------------------------------------------------------------------------
// 2. 二分探索 (max_right / min_left) のテスト
// -------------------------------------------------------------------------
#[test]
fn test_binary_search() {
    let seg = SegSum::new(&[1usize, 1, 1, 1, 1]);
    assert_eq!(seg.max_right(0, |&s| s < 3), 2);
    assert_eq!(seg.max_right(2, |&s| s < 2), 3);
    assert_eq!(seg.min_left(5, |&s| s <= 2), 3);
}

// -------------------------------------------------------------------------
// 3. ランダムストレステスト (愚直解との比較)
// -------------------------------------------------------------------------
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let len = range.end - range.start;
        range.start + (self.next() as usize % len)
    }
}

#[test]
fn stress_test() {
    let n = 50;
    let q = 1000;
    let mut rng = Rng::new(123456);

    let mut seg = SegSum::new(&vec![0usize; n]);
    let mut vec = vec![0usize; n];

    for _ in 0..q {
        let ty = rng.gen_range(0..2);
        if ty == 0 {
            let i = rng.gen_range(0..n);
            let x = rng.gen_range(0..100);
            seg.set(i, x);
            vec[i] = x;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r);
            let vec_res: usize = vec[l..r].iter().sum();
            assert_eq!(seg_res, vec_res, "Mismatch at ({}..{})", l, r);
        }
    }
}

// -------------------------------------------------------------------------
// 遅延セグ木
// 1. 基本動作テスト (Add系: 区間加算)
// -------------------------------------------------------------------------
#[test]
fn test_lazy_add_sum() {
    let mut seg = LazySegAddSum::new(&vec![(0usize, 1usize); 10]);
    seg.apply_range(0, 5, 10);
    assert_eq!(seg.get_range(0, 10).0, 50);
    seg.apply_range(2, 6, 5);
    // [0,2)*10=20, [2,5)*15=45, [5,6)*5=5 -> 70
    assert_eq!(seg.get_range(0, 10).0, 70);
    assert_eq!(seg.get_range(2, 5).0, 45);
}
#[test]
fn test_lazy_add_max() {
    let mut seg = LazySegAddMax::new(&vec![0usize; 5]);
    seg.apply_range(0, 3, 10); // [10, 10, 10, 0, 0]
    seg.apply_range(2, 5, 20); // [10, 10, 30, 20, 20]
    assert_eq!(seg.get_range(0, 5), 30);
    assert_eq!(seg.get_range(0, 2), 10);
    assert_eq!(seg.get_range(3, 5), 20);
    seg.set(0, 50);
    assert_eq!(seg.get_range(0, 5), 50);
}

// -------------------------------------------------------------------------
// 2. 基本動作テスト (Update系: 区間更新)
// -------------------------------------------------------------------------
#[test]
fn test_lazy_update_min() {
    let mut seg = LazySegUpdateMin::new(&vec![usize::MAX; 5]);
    seg.apply_range(0, 5, Some(100));
    assert_eq!(seg.get_range(0, 5), 100);
    seg.apply_range(1, 4, Some(10));
    assert_eq!(seg.get_range(0, 5), 10);
    assert_eq!(seg.get_range(0, 1), 100);
    seg.apply_range(0, 5, None); // 変化なし
    assert_eq!(seg.get_range(0, 5), 10);
}
#[test]
fn test_lazy_update_sum() {
    let mut seg = LazySegUpdateSum::new(&vec![(0usize, 1usize); 5]);
    seg.apply_range(0, 5, Some(10));
    assert_eq!(seg.get_range(0, 5).0, 50);
    seg.apply_range(0, 3, Some(5)); // [5,5,5,10,10]
    assert_eq!(seg.get_range(0, 5).0, 35); // 5*3+10*2
}
#[test]
fn test_lazy_update_xor() {
    let mut seg = LazySegUpdateXor::new(&vec![(0usize, 1usize); 5]);
    seg.apply_range(0, 5, Some(3));
    assert_eq!(seg.get_range(0, 5).0, 3); // 長さ5(奇数) -> 3
    assert_eq!(seg.get_range(0, 4).0, 0); // 長さ4(偶数) -> 0
}

// -------------------------------------------------------------------------
// 3. 遅延セグ木上の二分探索
// -------------------------------------------------------------------------
#[test]
fn test_lazy_binary_search() {
    let mut seg = LazySegAddSum::new(&vec![(0usize, 1usize); 10]);
    seg.apply_range(0, 10, 1); // [1, 1, ..., 1]
    assert_eq!(seg.max_right(0, |s| s.0 < 5), 4);
    assert_eq!(seg.min_left(10, |s| s.0 <= 3), 7);
}

// -------------------------------------------------------------------------
// 4. ランダムストレステスト (愚直解との比較)
// -------------------------------------------------------------------------
#[test]
fn stress_test_add_sum() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(2025);

    let mut seg = LazySegAddSum::new(&vec![(0usize, 1usize); n]);
    let mut vec = vec![0usize; n];

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..100);
            seg.apply_range(l, r, val);
            for item in vec.iter_mut().take(r).skip(l) {
                *item += val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..100);
            seg.set(i, (val, 1));
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r).0;
            let vec_res: usize = vec[l..r].iter().sum();
            assert_eq!(seg_res, vec_res, "AddSum Mismatch at query {},{}", l, r);
        }
    }
}
#[test]
fn stress_test_update_min() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(9999);

    let mut vec: Vec<usize> = (0..n).map(|_| rng.gen_range(0..1000)).collect();
    let mut seg = LazySegUpdateMin::new(&vec);

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..1000);
            seg.apply_range(l, r, Some(val));
            for item in vec.iter_mut().take(r).skip(l) {
                *item = val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..1000);
            seg.set(i, val);
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r);
            let vec_res = if l == r { usize::MAX } else { *vec[l..r].iter().min().unwrap() };
            assert_eq!(seg_res, vec_res, "UpdateMin Mismatch at query {}..{}", l, r);
        }
    }
}
#[test]
fn test_edge_cases() {
    let mut seg = SegSum::new(&[42usize]);
    assert_eq!(seg.get_range(0, 1), 42);
    assert_eq!(seg.get_range(0, 0), 0);
    seg.set(0, 0);
    assert_eq!(seg.get_range(0, 1), 0);

    let seg = SegMin::new(&[usize::MAX]);
    assert_eq!(seg.get_range(0, 1), usize::MAX);
}
#[test]
fn test_value_methods() {
    // LazySegAddSum
    let mut seg = LazySegAddSum::from_values(&[1usize, 2, 3, 4, 5]);
    assert_eq!(seg.get_range(0, 5).0, 15);
    seg.set_value(0, 10);
    assert_eq!(seg.get_range(0, 5).0, 24);
    assert_eq!(seg.get_value(0), 10);
    assert_eq!(seg.get_value(1), 2);
    seg.apply_range(1, 4, 5);
    assert_eq!(seg.get_range(0, 5).0, 39);

    let mut seg2 = LazySegAddSum::new_with_size(3);
    assert_eq!(seg2.get_range(0, 3).0, 0);
    seg2.apply_range(0, 3, 10);
    assert_eq!(seg2.get_range(0, 3).0, 30);

    // LazySegUpdateSum
    let mut seg = LazySegUpdateSum::from_values(&[1usize, 2, 3, 4, 5]);
    assert_eq!(seg.get_range(0, 5).0, 15);
    seg.apply_range(1, 4, Some(10));
    assert_eq!(seg.get_range(0, 5).0, 36); // 1+10+10+10+5
    seg.set_value(0, 100);
    assert_eq!(seg.get_value(0), 100);
    assert_eq!(seg.get_range(0, 5).0, 135);

    // LazySegUpdateXor
    let mut seg = LazySegUpdateXor::from_values(&[1usize, 2, 4, 8, 16]);
    assert_eq!(seg.get_range(0, 5).0, 31); // 1^2^4^8^16
    seg.apply_range(0, 5, Some(7));
    assert_eq!(seg.get_range(0, 5).0, 7);  // 奇数個 -> 7
    assert_eq!(seg.get_range(0, 4).0, 0);  // 偶数個 -> 0
    seg.set_value(0, 3);
    assert_eq!(seg.get_value(0), 3);
    assert_eq!(seg.get_range(0, 5).0, 3);
}
#[test]
fn test_value_methods_stress() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(54321);

    let mut seg = LazySegAddSum::new_with_size(n);
    let mut vec = vec![0usize; n];

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..100);
            seg.apply_range(l, r, val);
            for item in vec.iter_mut().take(r).skip(l) {
                *item += val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..100);
            seg.set_value(i, val);
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r);
            let vec_res = vec[l..r].iter().sum::<usize>();
            assert_eq!(seg_res.0, vec_res, "ValueMethod Mismatch at {}..{}", l, r);
        }
    }
}
#[test]
fn test_value_methods_edge_cases() {
    let mut seg = LazySegAddSum::from_values(&[42usize]);
    assert_eq!(seg.get_value(0), 42);
    seg.set_value(0, 100);
    assert_eq!(seg.get_value(0), 100);
    seg.apply_range(0, 1, 50);
    assert_eq!(seg.get_range(0, 1).0, 150);

    let mut seg = LazySegAddSum::new_with_size(5);
    assert_eq!(seg.get_range(0, 5).0, 0);

    let mut seg = LazySegUpdateSum::from_values(&[10usize, 20, 30, 40, 50]);
    seg.apply_range(1, 3, Some(100)); // [10, 100, 100, 40, 50]
    assert_eq!(seg.get_range(0, 5).0, 300);
    assert_eq!(seg.get_range(1, 3).0, 200);
    assert_eq!(seg.get_range(0, 1).0, 10);
    assert_eq!(seg.get_range(4, 5).0, 50);
}
#[test]
fn test_value_methods_xor_logic() {
    let mut seg = LazySegUpdateXor::new_with_size(6);
    seg.apply_range(0, 6, Some(5));
    assert_eq!(seg.get_range(0, 2).0, 0); // 偶数個 -> 0
    assert_eq!(seg.get_range(0, 4).0, 0);
    assert_eq!(seg.get_range(0, 6).0, 0);
    assert_eq!(seg.get_range(0, 1).0, 5); // 奇数個 -> 5
    assert_eq!(seg.get_range(0, 3).0, 5);
    assert_eq!(seg.get_range(0, 5).0, 5);

    let mut seg = LazySegUpdateXor::from_values(&[1usize, 2, 3, 4]);
    seg.apply_range(0, 2, Some(7));  // [7, 7, 3, 4]
    seg.apply_range(2, 4, Some(15)); // [7, 7, 15, 15]
    assert_eq!(seg.get_range(0, 2).0, 0);  // 7^7
    assert_eq!(seg.get_range(2, 4).0, 0);  // 15^15
    assert_eq!(seg.get_range(0, 4).0, 0);  // 7^7^15^15
    assert_eq!(seg.get_range(0, 3).0, 15); // 7^7^15=15
    assert_eq!(seg.get_range(1, 4).0, 7);  // 7^15^15=7
}

// -------------------------------------------------------------------------
// isize 版のテスト
// -------------------------------------------------------------------------
#[test]
fn test_isize_basic() {
    let data: &[isize] = &[-3, 1, -5, 4, -2];
    let seg = SegMaxI::new(data);
    assert_eq!(seg.get_range(0, 5), 4);
    assert_eq!(seg.get_range(0, 3), 1);
    assert_eq!(seg.get_range(0, 0), isize::MIN); // 単位元

    let seg = SegMinI::new(data);
    assert_eq!(seg.get_range(0, 5), -5);
    assert_eq!(seg.get_range(3, 5), -2);
    assert_eq!(seg.get_range(0, 0), isize::MAX); // 単位元

    let seg = SegSumI::new(data);
    assert_eq!(seg.get_range(0, 5), -5); // -3+1-5+4-2
    assert_eq!(seg.get_range(1, 4), 0);  // 1-5+4
}

#[test]
fn test_isize_lazy_add() {
    let data: &[isize] = &[3, -1, 4, -1, 5];
    let mut seg = LazySegAddMaxI::new(data);
    seg.apply_range(0, 5, -10);
    assert_eq!(seg.get_range(0, 5), -5); // 5-10

    let mut seg = LazySegAddMinI::new(data);
    seg.apply_range(0, 5, -10);
    assert_eq!(seg.get_range(0, 5), -11); // -1-10

    let mut seg = LazySegAddSumI::from_values(&[1isize, -2, 3, -4, 5]);
    assert_eq!(seg.get_range(0, 5).0, 3); // 1-2+3-4+5
    seg.apply_range(0, 5, -1);
    assert_eq!(seg.get_range(0, 5).0, -2); // 3-5
    seg.apply_range(1, 3, 10);
    assert_eq!(seg.get_range(0, 5).0, 18); // -2+10*2
}

#[test]
fn test_isize_lazy_update() {
    let mut seg = LazySegUpdateMaxI::new(&vec![isize::MIN; 5]);
    seg.apply_range(0, 5, Some(-100));
    assert_eq!(seg.get_range(0, 5), -100);
    seg.apply_range(2, 4, Some(50));
    assert_eq!(seg.get_range(0, 5), 50);
    assert_eq!(seg.get_range(0, 2), -100);

    let mut seg = LazySegUpdateMinI::new(&vec![isize::MAX; 5]);
    seg.apply_range(0, 5, Some(100));
    assert_eq!(seg.get_range(0, 5), 100);
    seg.apply_range(1, 4, Some(-50));
    assert_eq!(seg.get_range(0, 5), -50);
    assert_eq!(seg.get_range(0, 1), 100);
    seg.apply_range(0, 5, None); // 変化なし
    assert_eq!(seg.get_range(0, 5), -50);

    let mut seg = LazySegUpdateSumI::from_values(&[1isize, 2, 3, 4, 5]);
    assert_eq!(seg.get_range(0, 5).0, 15);
    seg.apply_range(0, 5, Some(-3));
    assert_eq!(seg.get_range(0, 5).0, -15);
    seg.apply_range(1, 3, Some(10));
    assert_eq!(seg.get_range(0, 5).0, -3 + 10 + 10 + (-3) + (-3)); // = 11
}

#[test]
fn test_isize_value_methods() {
    let mut seg = LazySegAddSumI::from_values(&[-5isize, 0, 5, -10, 10]);
    assert_eq!(seg.get_range(0, 5).0, 0);
    assert_eq!(seg.get_value(0), -5);
    seg.set_value(0, -100);
    assert_eq!(seg.get_value(0), -100);
    assert_eq!(seg.get_range(0, 5).0, -95);
    seg.apply_range(0, 3, 1);
    assert_eq!(seg.get_range(0, 5).0, -92); // -95+3

    let mut seg = LazySegAddSumI::new_with_size(3);
    assert_eq!(seg.get_range(0, 3).0, 0);
    seg.apply_range(0, 3, -7);
    assert_eq!(seg.get_range(0, 3).0, -21);

    let mut seg = LazySegUpdateSumI::from_values(&[10isize, 20, 30]);
    seg.apply_range(0, 3, Some(-1));
    assert_eq!(seg.get_range(0, 3).0, -3);
    seg.set_value(1, 100);
    assert_eq!(seg.get_value(1), 100);
    assert_eq!(seg.get_range(0, 3).0, 98); // -1+100+(-1)
}

// -------------------------------------------------------------------------
// Seg の構築・アクセス
// -------------------------------------------------------------------------
#[test]
fn test_seg_new() {
    let seg = SegSum::new(&[1usize, 2, 3, 4, 5]);
    assert_eq!(seg.get_range(0, 5), 15);
    assert_eq!(seg.get_range(1, 3), 5);
    assert_eq!(seg[0], 1);
    assert_eq!(seg[4], 5);
}

#[test]
fn test_seg_index() {
    let mut seg = SegSum::new(&[10usize, 20, 30]);
    assert_eq!(seg[0], 10);
    assert_eq!(seg[2], 30);
    seg.set(1, 99);
    assert_eq!(seg[1], 99);
}

// -------------------------------------------------------------------------
// max_right / min_left の境界値
// -------------------------------------------------------------------------
#[test]
fn test_max_right_boundary() {
    let seg = SegSum::new(&[1usize, 2, 3, 4, 5]);
    assert_eq!(seg.max_right(5, |&s| s < 100), 5); // l==n -> n
    assert_eq!(seg.max_right(0, |&_s| true), 5);   // 全true -> n
    assert_eq!(seg.max_right(0, |&s| s == 0), 0);  // 即false -> 0
}

#[test]
fn test_min_left_boundary() {
    let seg = SegSum::new(&[1usize, 2, 3, 4, 5]);
    assert_eq!(seg.min_left(0, |&s| s < 100), 0); // r==0 -> 0
    assert_eq!(seg.min_left(5, |&_s| true), 0);   // 全true -> 0
    assert_eq!(seg.min_left(5, |&s| s <= 5), 4);  // sum([4,5))=5 OK, sum([3,5))=9 NG -> 4
}

// -------------------------------------------------------------------------
// LazySeg の構築・get
// -------------------------------------------------------------------------
#[test]
fn test_lazyseg_new() {
    let mut seg = LazySegAddSum::new(&[(1usize, 1usize), (2, 1), (3, 1)]);
    assert_eq!(seg.get_range(0, 3).0, 6);
    assert_eq!(seg.get(0).0, 1);
    assert_eq!(seg.get(2).0, 3);
    seg.apply_range(0, 3, 10);
    assert_eq!(seg.get(1).0, 12);
}

#[test]
fn test_lazyseg_get_point() {
    let mut seg = LazySegUpdateMin::new(&[usize::MAX; 4]);
    seg.apply_range(0, 4, Some(100));
    seg.apply_range(1, 3, Some(5));
    assert_eq!(seg.get(0), 100);
    assert_eq!(seg.get(1), 5);
    assert_eq!(seg.get(3), 100);
}

// -------------------------------------------------------------------------
// Update 系の追加モノイド
// -------------------------------------------------------------------------
#[test]
fn test_lazy_update_bitor() {
    let mut seg = LazySegUpdateBitOr::new(&[usize::MAX; 4]);
    seg.apply_range(0, 4, Some(0b1010));
    assert_eq!(seg.get_range(0, 4), 0b1010);
    seg.apply_range(1, 3, Some(0b0101));
    assert_eq!(seg.get_range(0, 4), 0b1111); // 10|5=15
    assert_eq!(seg.get_range(1, 3), 0b0101);
}

#[test]
fn test_lazy_update_bitand() {
    let mut seg = LazySegUpdateBitAnd::new(&[usize::MAX; 4]);
    seg.apply_range(0, 4, Some(0b1111));
    assert_eq!(seg.get_range(0, 4), 0b1111);
    seg.apply_range(1, 3, Some(0b0110)); // [15, 6, 6, 15]
    assert_eq!(seg.get_range(0, 4), 0b0110); // 15&6=6
    seg.apply_range(0, 4, None); // 変化なし
    assert_eq!(seg.get_range(0, 4), 0b0110);
}

#[test]
fn test_lazy_update_gcd() {
    let mut seg = LazySegUpdateGcd::new(&[0usize; 3]);
    seg.apply_range(0, 3, Some(12));
    assert_eq!(seg.get_range(0, 3), 12); // gcd(12,12,12)=12
    seg.apply_range(1, 3, Some(8)); // [12, 8, 8]
    assert_eq!(seg.get_range(0, 3), 4); // gcd(12,8,8)=4
    assert_eq!(seg.get_range(1, 3), 8);
}

// -------------------------------------------------------------------------
// SegSumLen / SegXorLen (Seg側の長さ付きモノイド)
// -------------------------------------------------------------------------
#[test]
fn test_seg_sumlen() {
    let seg = SegSumLen::new(&[(10usize, 1usize), (20, 1), (30, 1), (40, 1)]);
    assert_eq!(seg.get_range(0, 4), (100, 4));
    assert_eq!(seg.get_range(1, 3), (50, 2));
    assert_eq!(seg.get_range(0, 0), (0, 0)); // 単位元
}

#[test]
fn test_seg_xorlen() {
    let seg = SegXorLen::new(&[(1usize, 1usize), (2, 1), (4, 1), (8, 1)]);
    assert_eq!(seg.get_range(0, 4), (15, 4)); // 1^2^4^8=15
    assert_eq!(seg.get_range(0, 2), (3, 2));  // 1^2=3
    assert_eq!(seg.get_range(0, 0), (0, 0));  // 単位元
}

// -------------------------------------------------------------------------
// ストレステスト: LazySegAddMin (区間加算・区間最小値)
// -------------------------------------------------------------------------
#[test]
fn stress_test_add_min() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(11111);

    let init: Vec<usize> = (0..n).map(|_| rng.gen_range(0..500)).collect();
    let mut seg = LazySegAddMin::new(&init);
    let mut vec = init;

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..100);
            seg.apply_range(l, r, val);
            for item in vec.iter_mut().take(r).skip(l) {
                *item += val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..100);
            seg.set(i, val);
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r);
            let vec_res = if l == r { usize::MAX } else { *vec[l..r].iter().min().unwrap() };
            assert_eq!(seg_res, vec_res, "AddMin Mismatch at {}..{}", l, r);
        }
    }
}

// -------------------------------------------------------------------------
// ストレステスト: LazySegUpdateMax (区間更新・区間最大値)
// -------------------------------------------------------------------------
#[test]
fn stress_test_update_max() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(22222);

    let mut vec = vec![0usize; n];
    let mut seg = LazySegUpdateMax::new(&vec);

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..1000);
            seg.apply_range(l, r, Some(val));
            for item in vec.iter_mut().take(r).skip(l) {
                *item = val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..1000);
            seg.set(i, val);
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r);
            let vec_res = if l == r { 0 } else { *vec[l..r].iter().max().unwrap() };
            assert_eq!(seg_res, vec_res, "UpdateMax Mismatch at {}..{}", l, r);
        }
    }
}

// -------------------------------------------------------------------------
// ストレステスト: LazySeg の max_right / min_left
// -------------------------------------------------------------------------
#[test]
fn stress_test_lazy_binary_search() {
    let n = 30;
    let q = 200;
    let mut rng = Rng::new(33333);

    let mut seg = LazySegAddSum::new_with_size(n);
    let mut vec = vec![0usize; n];

    for _ in 0..50 {
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l..n + 1);
        let val = rng.gen_range(1..10);
        seg.apply_range(l, r, val);
        for item in vec.iter_mut().take(r).skip(l) {
            *item += val;
        }
    }

    for _ in 0..q {
        let threshold = rng.gen_range(0..200);
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(1..n + 1);

        let expected_mr = {
            let mut sum = 0;
            let mut ans = n;
            for i in l..n {
                sum += vec[i];
                if sum > threshold { ans = i; break; }
            }
            ans
        };
        assert_eq!(
            seg.max_right(l, |s| s.0 <= threshold),
            expected_mr,
            "max_right Mismatch: l={} threshold={}", l, threshold
        );

        let expected_ml = {
            let mut sum = 0;
            let mut ans = 0;
            for i in (0..r).rev() {
                sum += vec[i];
                if sum > threshold { ans = i + 1; break; }
            }
            ans
        };
        assert_eq!(
            seg.min_left(r, |s| s.0 <= threshold),
            expected_ml,
            "min_left Mismatch: r={} threshold={}", r, threshold
        );
    }
}

// -------------------------------------------------------------------------
// ストレステスト: UpdateSumI (isize)
// -------------------------------------------------------------------------
#[test]
fn stress_test_update_sum_i() {
    let n = 30;
    let q = 1000;
    let mut rng = Rng::new(77777);

    let mut seg = LazySegUpdateSumI::from_values(&vec![0isize; n]);
    let mut vec = vec![0isize; n];

    for _ in 0..q {
        let ty = rng.gen_range(0..3);
        if ty == 0 {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let val = rng.gen_range(0..100) as isize - 50;
            seg.apply_range(l, r, Some(val));
            for item in vec.iter_mut().take(r).skip(l) {
                *item = val;
            }
        } else if ty == 1 {
            let i = rng.gen_range(0..n);
            let val = rng.gen_range(0..100) as isize - 50;
            seg.set_value(i, val);
            vec[i] = val;
        } else {
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l..n + 1);
            let seg_res = seg.get_range(l, r).0;
            let vec_res: isize = vec[l..r].iter().sum();
            assert_eq!(seg_res, vec_res, "UpdateSumI Mismatch at {}..{}", l, r);
        }
    }
}
