const PRIME_NUMBERS: [usize; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];

pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for &p in PRIME_NUMBERS.iter() {
        if n == p {
            return true;
        } else if n % p == 0 {
            return false;
        }
    }
    let max_small = *PRIME_NUMBERS.last().unwrap();
    if (n as usize) <= max_small * max_small {
        return true;
    }

    // 30ホイールでの追加試し割り（上限 max_trial）
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let max_trial = 100_000usize; // 試し割り上限（必要なら調整可）
    let limit = sqrt_n.min(max_trial);

    // 30 ホイール上で 30k + r の形の r 値（30 と互いに素）
    const WHEEL30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

    if limit > max_small {
        // k を決めて max_small より大きい最初の候補から始める
        let mut k = (max_small / 30).saturating_sub(1);
        'wheel_outer: loop {
            k += 1;
            for &r in WHEEL30.iter() {
                let cand = k * 30 + r;
                if cand <= max_small {
                    continue;
                }
                if cand > limit {
                    break 'wheel_outer;
                }
                if n % cand == 0 {
                    return false;
                }
            }
        }

        // もし limit が sqrt_n をカバーしていたら、試し割りが完了したため素数
        if limit >= sqrt_n {
            return true;
        }
    }

    // 4) 大きな数に対しては Miller–Rabin を用いる（高速かつ 64-bit 実践用には決定的）
    //    安全のため u128 を用いて乗算オーバーフローを回避する
    let n128 = n as u128;

    // n-1 = d * 2^s (d は奇数)
    let mut d = n128 - 1;
    let mut s: u32 = 0;
    while d % 2 == 0 {
        d >>= 1;
        s += 1;
    }

    // べき乗剰余
    fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
        let mut res = 1u128;
        base %= modu;
        while exp > 0 {
            if (exp & 1) == 1 {
                res = (res * base) % modu;
            }
            base = (base * base) % modu;
            exp >>= 1;
        }
        res
    }

    // MR の基底（64-bit 用）
    const BASES: [u128; 6] = [2, 3, 5, 7, 11, 13];

    for &a_orig in BASES.iter() {
        let a = a_orig % n128;
        if a == 0 {
            // 基底が n の倍数はスキップ
            continue;
        }

        let mut x = mod_pow(a, d, n128);
        if x == 1 || x == n128 - 1 {
            continue; // 合格
        }

        // x が n-1 になるまで平方を繰り返す
        let mut composite = true;
        for _ in 1..s {
            x = (x * x) % n128;
            if x == n128 - 1 {
                composite = false;
                break;
            }
        }

        if composite {
            return false; // 合成数
        }
    }

    // 全基底を通過 → 素数
    true
}

// 単体テスト
#[cfg(test)]
mod tests {
    use super::is_prime;

    // 素朴な判定（テスト用）
    fn is_prime_naive(n: usize) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut i = 3usize;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }

    #[test]
    fn 小さい素数() {
        let primes = [
            2usize, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
            79, 83, 89, 97,
        ];
        for &p in primes.iter() {
            assert!(is_prime(p), "{} は素数のはずです", p);
        }
    }

    #[test]
    fn 小さい合成数() {
        let comps = [0usize, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20];
        for &c in comps.iter() {
            assert!(!is_prime(c), "{} は合成数のはずです", c);
        }
    }

    #[test]
    fn ナイーブと比較_5000() {
        for n in 0usize..5000 {
            assert_eq!(is_prime(n), is_prime_naive(n), "{} で不一致", n);
        }
    }
}
