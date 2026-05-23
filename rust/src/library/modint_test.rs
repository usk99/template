use crate::library::modint::*;

// ============================================================================
// 基本構築・内部値
// ============================================================================

/// new / val / ZERO / ONE の基本動作
#[test]
fn test_basic_construction() {
    assert_eq!(Mint::new(0).val(), 0);
    assert_eq!(Mint::new(5).val(), 5);
    assert_eq!(Mint::new(MOD - 1).val(), MOD - 1);
    assert_eq!(Mint::new(MOD).val(), 0, "MOD は 0 に正規化される");
    assert_eq!(Mint::new(MOD + 3).val(), 3, "MOD + k は k に正規化される");
    assert_eq!(Mint::new(2 * MOD).val(), 0);
    assert_eq!(Mint::new(usize::MAX).val(), usize::MAX % MOD);

    assert_eq!(Mint::ZERO.val(), 0);
    assert_eq!(Mint::ONE.val(), 1);
}

/// Default は 0
#[test]
fn test_default() {
    let x: Mint = Mint::default();
    assert_eq!(x, Mint::ZERO);
}

/// Copy / Clone / PartialEq
#[test]
fn test_traits() {
    let a = Mint::new(5);
    let b = a;
    assert_eq!(a, b);
    assert_ne!(a, Mint::new(6));
    assert_eq!(a, Mint::new(5));
}

// ============================================================================
// 四則演算 (Mint 同士)
// ============================================================================

#[test]
fn test_add() {
    assert_eq!((Mint::new(5) + Mint::new(3)).val(), 8);
    assert_eq!((Mint::ZERO + Mint::new(7)).val(), 7);
    assert_eq!((Mint::new(MOD - 1) + Mint::new(1)).val(), 0, "繰り上がり");
    assert_eq!((Mint::new(MOD - 1) + Mint::new(MOD - 1)).val(), MOD - 2);
}

#[test]
fn test_sub() {
    assert_eq!((Mint::new(5) - Mint::new(3)).val(), 2);
    assert_eq!(
        (Mint::new(3) - Mint::new(5)).val(),
        MOD - 2,
        "アンダーフロー"
    );
    assert_eq!((Mint::ZERO - Mint::ONE).val(), MOD - 1);
    assert_eq!((Mint::new(7) - Mint::new(7)).val(), 0);
}

#[test]
fn test_mul() {
    assert_eq!((Mint::new(5) * Mint::new(3)).val(), 15);
    assert_eq!((Mint::ZERO * Mint::new(123)).val(), 0);
    assert_eq!((Mint::ONE * Mint::new(456)).val(), 456);
    // 大きな値同士: 結果が MOD で正しく丸まる
    let a = Mint::new(MOD - 1);
    assert_eq!((a * a).val(), 1, "(MOD-1)^2 ≡ 1 (mod MOD)");
}

#[test]
fn test_div() {
    let a = Mint::new(10);
    let b = Mint::new(3);
    let q = a / b;
    assert_eq!((q * b).val(), 10, "a/b * b == a");

    assert_eq!((Mint::ZERO / Mint::new(7)).val(), 0, "0 / x = 0");
    assert_eq!((Mint::new(6) / Mint::new(2)).val(), 3, "整数解になるケース");
}

#[test]
fn test_assign_ops() {
    let mut a = Mint::new(5);
    a += Mint::new(3);
    assert_eq!(a.val(), 8);

    a -= Mint::new(2);
    assert_eq!(a.val(), 6);

    a *= Mint::new(4);
    assert_eq!(a.val(), 24);

    a /= Mint::new(6);
    assert_eq!(a.val(), 4);
}

// ============================================================================
// 単項マイナス (Neg)
// ============================================================================

#[test]
fn test_neg() {
    assert_eq!((-Mint::new(5)).val(), MOD - 5);
    assert_eq!((-Mint::ZERO).val(), 0, "-0 = 0");
    assert_eq!((-Mint::ONE).val(), MOD - 1);
    assert_eq!((-(-Mint::new(123))).val(), 123, "二重否定");

    let a = Mint::new(7);
    assert_eq!((a + (-a)).val(), 0, "a + (-a) = 0");
}

// ============================================================================
// pow / inv
// ============================================================================

#[test]
fn test_pow_basic() {
    assert_eq!(Mint::new(2).pow(0).val(), 1, "x^0 = 1");
    assert_eq!(Mint::new(2).pow(1).val(), 2);
    assert_eq!(Mint::new(2).pow(10).val(), 1024);
    assert_eq!(Mint::new(3).pow(5).val(), 243);
}

#[test]
fn test_pow_zero_base() {
    assert_eq!(Mint::ZERO.pow(0).val(), 1, "0^0 は 1 (慣習)");
    assert_eq!(Mint::ZERO.pow(1).val(), 0);
    assert_eq!(Mint::ZERO.pow(100).val(), 0);
}

#[test]
fn test_pow_large_exponent() {
    // フェルマー: a^(MOD-1) ≡ 1 (mod MOD) for a != 0
    assert_eq!(Mint::new(2).pow((MOD - 1) as u64).val(), 1);
    assert_eq!(Mint::new(123456).pow((MOD - 1) as u64).val(), 1);
}

#[test]
fn test_inv() {
    let a = Mint::new(7);
    let inv = a.inv();
    assert_eq!((a * inv).val(), 1, "a * inv(a) = 1");

    assert_eq!(Mint::ONE.inv().val(), 1);
    assert_eq!((Mint::new(MOD - 1).inv() * Mint::new(MOD - 1)).val(), 1);
}

// ============================================================================
// From<整数> (signed/unsigned 両方、負数含む)
// ============================================================================

#[test]
fn test_from_signed_positive() {
    assert_eq!(Mint::from(5_i32).val(), 5);
    assert_eq!(Mint::from(5_i64).val(), 5);
    assert_eq!(Mint::from(5_isize).val(), 5);
    assert_eq!(Mint::from(5_i8).val(), 5);
    assert_eq!(Mint::from(5_i16).val(), 5);
}

#[test]
fn test_from_signed_negative() {
    assert_eq!(Mint::from(-1_i64).val(), MOD - 1, "-1 ≡ MOD - 1");
    assert_eq!(Mint::from(-5_i32).val(), MOD - 5);
    assert_eq!(Mint::from(-(MOD as i64)).val(), 0, "-MOD ≡ 0");
    assert_eq!(Mint::from(-(MOD as i64) - 1).val(), MOD - 1);
}

#[test]
fn test_from_unsigned() {
    assert_eq!(Mint::from(0_u8).val(), 0);
    assert_eq!(Mint::from(5_u32).val(), 5);
    assert_eq!(Mint::from(5_u64).val(), 5);
    assert_eq!(Mint::from(5_usize).val(), 5);
    assert_eq!(Mint::from(MOD).val(), 0);
    assert_eq!(Mint::from(MOD + 7).val(), 7);
    assert_eq!(Mint::from(u64::MAX).val(), (u64::MAX as usize) % MOD);
}

#[test]
fn test_into() {
    let x: Mint = 5_i64.into();
    assert_eq!(x.val(), 5);
    let y: Mint = (-3_i64).into();
    assert_eq!(y.val(), MOD - 3);
}

// ============================================================================
// スカラー演算 (Mint <op> 整数 / 整数 <op> Mint)
// ============================================================================

#[test]
fn test_scalar_add() {
    assert_eq!((Mint::new(5) + 3_usize).val(), 8);
    assert_eq!((Mint::new(5) + 3_i64).val(), 8);
    assert_eq!((3_usize + Mint::new(5)).val(), 8, "整数 + Mint");
    assert_eq!((Mint::new(5) + (-3_i64)).val(), 2);
}

#[test]
fn test_scalar_sub() {
    assert_eq!((Mint::new(5) - 3_usize).val(), 2);
    assert_eq!((Mint::new(3) - 5_i64).val(), MOD - 2);
    assert_eq!((10_usize - Mint::new(3)).val(), 7);
}

#[test]
#[allow(clippy::erasing_op)]
fn test_scalar_mul() {
    assert_eq!((Mint::new(5) * 3_usize).val(), 15);
    assert_eq!((2_i64 * Mint::new(7)).val(), 14, "整数 * Mint");
    assert_eq!((Mint::new(5) * 0_usize).val(), 0);
}

#[test]
fn test_scalar_div() {
    let a = Mint::new(10);
    let q = a / 2_usize;
    assert_eq!((q * 2_usize).val(), 10);
}

#[test]
fn test_scalar_assign() {
    let mut a = Mint::new(5);
    a += 3_usize;
    assert_eq!(a.val(), 8);
    a -= 2_i64;
    assert_eq!(a.val(), 6);
    a *= 4_u32;
    assert_eq!(a.val(), 24);
    a /= 6_usize;
    assert_eq!(a.val(), 4);
}

// ============================================================================
// Display / Debug
// ============================================================================

#[test]
fn test_display() {
    assert_eq!(format!("{}", Mint::new(0)), "0");
    assert_eq!(format!("{}", Mint::new(42)), "42");
    assert_eq!(format!("{}", Mint::new(MOD - 1)), format!("{}", MOD - 1));
}

#[test]
fn test_debug() {
    // Debug は内部値そのまま (Mint(5) ではなく 5)
    assert_eq!(format!("{:?}", Mint::new(7)), "7");
    assert_eq!(format!("{:?}", vec![Mint::new(1), Mint::new(2)]), "[1, 2]");
}

// ============================================================================
// Sum / Product
// ============================================================================

#[test]
fn test_sum_empty() {
    let v: Vec<Mint> = vec![];
    let s: Mint = v.iter().sum();
    assert_eq!(s, Mint::ZERO, "空イテレータの sum は 0");
}

#[test]
fn test_sum_by_value() {
    let v = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
    let s: Mint = v.into_iter().sum();
    assert_eq!(s.val(), 6);
}

#[test]
fn test_sum_by_ref() {
    let v = [Mint::new(10), Mint::new(20), Mint::new(30)];
    let s: Mint = v.iter().sum();
    assert_eq!(s.val(), 60);
}

#[test]
fn test_product_empty() {
    let v: Vec<Mint> = vec![];
    let p: Mint = v.iter().product();
    assert_eq!(p, Mint::ONE, "空イテレータの product は 1");
}

#[test]
fn test_product_by_value() {
    let v = vec![Mint::new(2), Mint::new(3), Mint::new(4)];
    let p: Mint = v.into_iter().product();
    assert_eq!(p.val(), 24);
}

#[test]
fn test_product_by_ref() {
    let v = [Mint::new(2), Mint::new(3), Mint::new(4)];
    let p: Mint = v.iter().product();
    assert_eq!(p.val(), 24);
}

#[test]
fn test_product_with_zero() {
    let v = [Mint::new(2), Mint::ZERO, Mint::new(4)];
    let p: Mint = v.iter().product();
    assert_eq!(p, Mint::ZERO, "0 を含む積は 0");
}

// ============================================================================
// ncr (二項係数)
// ============================================================================

#[test]
fn test_ncr_basic() {
    assert_eq!(ncr(0, 0).val(), 1, "0C0 = 1");
    assert_eq!(ncr(5, 0).val(), 1, "nC0 = 1");
    assert_eq!(ncr(5, 5).val(), 1, "nCn = 1");
    assert_eq!(ncr(5, 1).val(), 5);
    assert_eq!(ncr(5, 2).val(), 10);
    assert_eq!(ncr(10, 3).val(), 120);
    assert_eq!(ncr(10, 7).val(), 120, "対称性: nCr = nC(n-r)");
}

#[test]
fn test_ncr_out_of_range() {
    assert_eq!(ncr(5, 6).val(), 0, "n < r のときは 0");
    assert_eq!(ncr(0, 1).val(), 0);
    assert_eq!(ncr(3, 100).val(), 0);
}

#[test]
fn test_ncr_pascal() {
    // パスカルの三角形: nCr = (n-1)C(r-1) + (n-1)Cr
    for n in 1..=20 {
        for r in 1..n {
            assert_eq!(
                ncr(n, r).val(),
                (ncr(n - 1, r - 1) + ncr(n - 1, r)).val(),
                "パスカルの関係: {}C{} = {}C{} + {}C{}",
                n,
                r,
                n - 1,
                r - 1,
                n - 1,
                r
            );
        }
    }
}

#[test]
fn test_ncr_sum_of_row() {
    // n 行の総和 = 2^n
    for n in 0..=15 {
        let sum: Mint = (0..=n).map(|r| ncr(n, r)).sum();
        assert_eq!(sum.val(), 1_usize << n, "Σ nCr = 2^n");
    }
}

// ============================================================================
// npr (順列)
// ============================================================================

#[test]
fn test_npr_basic() {
    assert_eq!(npr(0, 0).val(), 1, "0P0 = 1");
    assert_eq!(npr(5, 0).val(), 1, "nP0 = 1");
    assert_eq!(npr(5, 1).val(), 5);
    assert_eq!(npr(5, 2).val(), 20);
    assert_eq!(npr(5, 5).val(), 120, "nPn = n!");
    assert_eq!(npr(10, 3).val(), 720);
}

#[test]
fn test_npr_out_of_range() {
    assert_eq!(npr(5, 6).val(), 0);
    assert_eq!(npr(0, 1).val(), 0);
}

#[test]
fn test_npr_factorial() {
    // nPn = n!
    let mut expected = 1_usize;
    for n in 0..=10 {
        assert_eq!(npr(n, n).val(), expected, "nPn = n! at n={}", n);
        expected *= n + 1;
    }
}

#[test]
fn test_npr_ncr_relation() {
    // nPr = nCr * r!
    for n in 0..=10 {
        for r in 0..=n {
            let p = npr(n, r);
            let mut r_fact = 1_usize;
            for i in 1..=r {
                r_fact *= i;
            }
            assert_eq!(p.val(), ncr(n, r).val() * r_fact);
        }
    }
}

// ============================================================================
// nhr (重複組合せ)
// ============================================================================

#[test]
fn test_nhr_basic() {
    // H(n, r) = C(n + r - 1, r)
    assert_eq!(nhr(5, 3).val(), 35, "5H3 = 35");
    assert_eq!(nhr(3, 5).val(), 21);
    assert_eq!(
        nhr(1, 10).val(),
        1,
        "1 種類から重複ありで何個選んでも 1 通り"
    );
    assert_eq!(nhr(10, 1).val(), 10, "10 種類から 1 個選ぶ = 10");
}

#[test]
fn test_nhr_zero_corner_cases() {
    assert_eq!(nhr(0, 0).val(), 1, "H(0, 0) = 1 (慣習)");
    assert_eq!(nhr(0, 1).val(), 0, "H(0, r>0) = 0");
    assert_eq!(nhr(0, 5).val(), 0);
    assert_eq!(nhr(5, 0).val(), 1, "H(n, 0) = 1");
}

#[test]
fn test_nhr_formula() {
    // H(n, r) = C(n + r - 1, r)
    for n in 1..=10 {
        for r in 0..=10 {
            assert_eq!(
                nhr(n, r).val(),
                ncr(n + r - 1, r).val(),
                "H({}, {}) = C({}, {})",
                n,
                r,
                n + r - 1,
                r
            );
        }
    }
}

// ============================================================================
// ncr_large (巨大 n)
// ============================================================================

#[test]
fn test_ncr_large_matches_ncr() {
    // 普通の範囲では ncr と一致する
    for n in 0..=50_u64 {
        for r in 0..=10 {
            assert_eq!(
                ncr_large(n, r).val(),
                ncr(n as usize, r).val(),
                "ncr_large({}, {}) と ncr が一致",
                n,
                r
            );
        }
    }
}

#[test]
fn test_ncr_large_huge_n() {
    // n が COMB_MAX を大きく超えるケース
    let n: u64 = 1_000_000_000_000_000_000; // 10^18
    let r = 5;
    let result = ncr_large(n, r);

    // n * (n-1) * (n-2) * (n-3) * (n-4) / 5! を Mint で手計算
    let mut expected = Mint::ONE;
    for i in 0..r {
        expected *= n - i as u64;
    }
    let mut r_fact = Mint::ONE;
    for i in 1..=r {
        r_fact *= i;
    }
    assert_eq!(result.val(), (expected / r_fact).val());
}

#[test]
fn test_ncr_large_out_of_range() {
    assert_eq!(ncr_large(3, 5).val(), 0, "n < r");
    assert_eq!(ncr_large(0, 1).val(), 0);
}

#[test]
fn test_ncr_large_r_zero() {
    assert_eq!(ncr_large(0, 0).val(), 1);
    assert_eq!(ncr_large(1_000_000_000_000, 0).val(), 1, "nC0 = 1");
}
