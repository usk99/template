#![allow(unused, non_snake_case)]
use {itertools::*, std::collections::*};

fn algorithm(answer: &mut Answer) {
    answer.push("algo");
}
fn main() {
    // 出力用意
    let mut answer = Answer::new();
    algorithm(&mut answer);
    Output(answer.outstr());
    flush();
}

// === 汎用 ===
// use library::config::*; // Timer, FastRng, is_local
// use library::heap::*;   // MaxHeap, MinHeap

// === グリッド系 ===
// use library::coord::*;     // Coord (座標)
// use library::grid::*;      // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use library::seg::*;       // Seg, LazySeg, SegSum等のエイリアス

// === 単体 ===
// use library::dp::*;        // dp
// use library::graph::*;     // graph
// use library::prime::*;     // is_prime, factorize
// use library::rng::*;       // FastRng
// use library::timer::*;     // Timer
// use library::unionfind::*; // UnionFind

mod library;
// === 常用 ===

use library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use library::fmt::*; // ToComma (数値のカンマ区切り表示)
use library::io::*; // InputNum, Output, Answer 等の入出力
use library::usermacro::*; // min!, max!, chmin!, chmax!
use library::util::*; // ABC, Set, Map

// === 尺取り法 ===
// // 前提: a[i] 単体は必ず OK 条件を満たすこと
// // [l, r) の半開区間。r は「次に追加するインデックス」
// let (mut l, mut r) = (0, 0);
// let mut state = 0; // 差分管理できる値 (和・積・個数 等)
// let mut ans = 0;
// while r < n {
//     state += a[r]; // a[r] を区間に追加してから r を進める
//     r += 1;
//     while /* NG条件 例: state > limit */ {
//         state -= a[l]; // a[l] を区間から除いてから l を進める
//         l += 1;
//         // ※ l < r のガードは原則不要（入れると WA になることがある）
//     }
//     // ここで [l, r) は条件を満たす最大の右端をもつ区間
//     ans = max!(ans, r - l); // 【最大区間長】を求める場合
// }
//
// // 【最小区間長】を求める場合: 内側ループの中で更新する
// // while r < n {
// //     state += a[r]; r += 1;
// //     while /* OK条件 例: state >= target */ {
// //         ans = min!(ans, r - l); // ← OK なうちに更新
// //         state -= a[l]; l += 1;
// //     }
// // }

// === 二分探索 (abs_diff 版) ===
// // ok: 条件を満たすことが確定している値
// // ng: 条件を満たさないことが確定している値
// // → ok と ng の大小関係は問わない（abs_diff が吸収する）
//
// // 【条件を満たす最大値】を求める場合
// // let mut ok = 0;   // 0 は必ず OK (問題に応じて調整)
// // let mut ng = n+1; // n+1 は必ず NG
//
// // 【条件を満たす最小値】を求める場合
// // let mut ok = n;   // n は必ず OK
// // let mut ng = 0;   // 0 は必ず NG (問題に応じて調整)
//
// let mut ok: usize = ...; // isize にする場合はそのまま使える
// let mut ng: usize = ...;
// while ok.abs_diff(ng) > 1 {
//     let mid = (ok + ng) / 2;
//     if check(mid) {
//         ok = mid;
//     } else {
//         ng = mid;
//     }
// }
// // ok が答え
