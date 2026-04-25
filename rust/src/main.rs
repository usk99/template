#![allow(dead_code, non_snake_case, unused_imports, unused_macros)]
#![deny(arithmetic_overflow)]

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
// use template::library::config::*; // Timer, FastRng, is_local
// use template::library::heap::*;   // MaxHeap, MinHeap

// === グリッド系 ===
// use template::library::coord::*;     // Coord (座標)
// use template::library::grid::*;      // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use template::library::monoid::*;    // Monoid定義, SegSum等のエイリアス
// use template::library::seg::*;       // Seg, LazySeg

// === 単体 ===
// use template::library::graph::*;     // graph
// use template::library::prime::*;     // is_prime, factorize
// use template::library::rng::*;       // FastRng
// use template::library::timer::*;     // Timer
// use template::library::unionfind::*; // UnionFind

// === 常用 ===
use itertools::Itertools;
use std::collections::*;
use template::library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use template::library::fmt::*; // ToComma (数値のカンマ区切り表示)
use template::library::io::*; // InputNum, Output, Answer 等の入出力
use template::library::usermacro::*; // min!, max!, chmin!, chmax!
use template::library::util::*; // ABC, Set, Map
use template::*; // impl_elem!
