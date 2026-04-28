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
// use library::config::*; // Timer, FastRng, is_local
// use library::heap::*;   // MaxHeap, MinHeap

// === グリッド系 ===
// use library::coord::*;     // Coord (座標)
// use library::grid::*;      // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use library::monoid::*;    // Monoid定義, SegSum等のエイリアス
// use library::seg::*;       // Seg, LazySeg

// === 単体 ===
// use library::graph::*;     // graph
// use library::prime::*;     // is_prime, factorize
// use library::rng::*;       // FastRng
// use library::timer::*;     // Timer
// use library::unionfind::*; // UnionFind

// === 常用 ===
mod library;
use itertools::Itertools;
use std::collections::*;

use library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use library::fmt::*; // ToComma (数値のカンマ区切り表示)
use library::io::*; // InputNum, Output, Answer 等の入出力
use library::usermacro::*; // min!, max!, chmin!, chmax!
use library::util::*; // ABC, Set, Map
