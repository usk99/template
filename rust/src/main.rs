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
// let (mut l, mut r) = (0, 0);
// let mut state = 0;   // ← 差分更新できる状態 (sum等)。セグ木外持ちなら不要
// while r < n {
//     // ここで a[r] を state に追加
//     state += a[r];
//     r += 1;
//     while NG条件 {
//         // ここで a[l] を state から除去
//         state -= a[l];
//         l += 1;
//         // ※ l < r のガードは原則不要（入れるとWAになることがある）
//         // ※ a[r] 単体が必ずOK か、問題の制約から確認する
//     }
//     // ここで [l, r) は OK な最大区間 → ans の更新等
//     // 長さの最低条件とか if l+k<r{}みたいに
//     ans = max!(ans, r - l);  // 長さなら r - l、値なら state 等
// }
