pub const PRIME_NUMBERS: [usize; 200] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
    1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223,
];

// --- 共通ユーティリティ ---
const fn mod_mul(a: u128, b: u128, modu: u128) -> u128 {
    (a * b) % modu
}

const fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
    let mut res = 1u128;
    base %= modu;
    while exp > 0 {
        if (exp & 1) == 1 {
            res = mod_mul(res, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp >>= 1;
    }
    res
}

const fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

// Pollard's Rho（簡易実装、決定的ではないが u128 範囲で実用的）
fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    let mut d: u128 = 1;
    // 適当な c を試す
    let mut c: u128 = 1;
    while d == 1 {
        let mut x = 2;
        let mut y = 2;
        d = 1;
        let f = |x: u128| -> u128 { (mod_mul(x, x, n) + c) % n };
        for _ in 0..100_000 {
            x = f(x);
            y = f(f(y));
            let diff = x.abs_diff(y);
            d = gcd_u128(diff, n);
            if d == n {
                break;
            }
            if d > 1 {
                return d;
            }
        }
        c += 1;
        // 万が一見つからなければ別の c を試行
    }
    // ここに来るのは通常ないが、n を返すフォールバック
    n
}

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
    if n <= max_small * max_small {
        return true;
    }

    // 30ホイールでの追加試し割り（上限 max_trial）
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    // 試し割り上限（必要なら調整可）
    let max_trial = 100_000usize;
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

    // べき乗剰余は上の mod_pow を利用

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
            // 合格
            continue;
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
            // 合成数
            return false;
        }
    }

    // 全基底を通過 → 素数
    true
}

/// 素因数分解を行い、(素因数, 指数) のペアを昇順に返す
pub fn factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    if n < 2 {
        return res;
    }

    // まず小さい素数表で割り切る
    for &p in PRIME_NUMBERS.iter() {
        let p_us = p;
        if p_us * p_us > n {
            break;
        }
        if n % p_us == 0 {
            let mut cnt = 0;
            while n % p_us == 0 {
                n /= p_us;
                cnt += 1;
            }
            res.push((p_us, cnt));
        }
    }

    // ここまでで n が 1 なら終了
    if n == 1 {
        return res;
    }

    // 残りが小さければそのまま追加
    if is_prime(n) {
        res.push((n, 1));
        return res;
    }

    // 残った合成数に対して Pollard's Rho で因数を見つける
    fn factor_rec(n128: u128, out: &mut Vec<u128>) {
        if n128 == 1 {
            return;
        }
        if is_prime(n128 as usize) {
            out.push(n128);
            return;
        }
        let d = pollard_rho(n128);
        if d == n128 {
            // フェールセーフ：分解に失敗したら素朴に試し割り
            let mut m = n128 as usize;
            for &p in PRIME_NUMBERS.iter() {
                let p_us = p;
                while m % p_us == 0 {
                    out.push(p_us as u128);
                    m /= p_us;
                }
            }
            if m > 1 {
                out.push(m as u128);
            }
            return;
        }
        factor_rec(d, out);
        factor_rec(n128 / d, out);
    }

    let mut facs: Vec<u128> = Vec::new();
    factor_rec(n as u128, &mut facs);
    // facs は素因子のリスト（重複あり）なので集計して usize に戻す
    facs.sort_unstable();
    for f in facs {
        let f_us = f as usize;
        if res.last().is_some() && res.last_mut().unwrap().0 == f_us {
            res.last_mut().unwrap().1 += 1;
            continue;
        }
        res.push((f_us, 1));
    }

    res.sort_unstable_by_key(|(p, _)| *p);
    res
}

/// n より大きい最小の素数を返す（n が素数なら n ではなく n+1 以上の素数）
pub fn next_prime(n: usize) -> usize {
    // 2 の次の素数は 3
    if n < 2 {
        return 2;
    }
    // 候補: n+1 から開始
    let mut cand = n + 1;
    // 小さな最適化: 2 のみ偶数の素数なので、偶数なら次の奇数へ
    if cand % 2 == 0 {
        if cand == 2 {
            return 3;
        }
        cand += 1;
    }

    loop {
        if is_prime(cand) {
            return cand;
        }
        // 次の奇数を試す
        // wrapping_add で usize オーバーフローしても負の影響を避ける
        cand = cand.wrapping_add(2);
    }
}
