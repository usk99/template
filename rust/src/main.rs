#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn algorithm(answer: &mut Answer) {
    // ＊＊＊ 数値型はusizeにしないと掛け算でオーバーフローする可能性ある！！ ＊＊＊
    answer.push("algo");
}
fn main() {
    // 実行時間計測
    let timer = Timer::new(TIME_LIMIT);
    // 出力用意
    let mut answer = Answer::new();
    algorithm(&mut answer);
    Output(answer.outstr());
    log!("time : {:>6}", timer.get_time_str());
}

// === グリッド系 (coord_ext は coord + direction が必要) ===
// use template::library::direction::*;  // Direction enum (U/R/D/L)
// use template::library::coord::*;      // Coord (座標)
// use template::library::coord_ext::*;  // Coord + Direction の演算
// use template::library::grid::*;       // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use template::library::linkcuttree::*;
// use template::library::monoid::*;     // Monoid定義, SegSum等のエイリアス
// use template::library::seg::*;        // Seg, LazySeg

// === 単体 ===
// use template::library::prime::*;      // is_prime, factorize
// use template::library::unionfind::*;  // UnionFind

// === 常用 ===
use itertools::Itertools;
use std::{collections::*, fmt::*, fs::*, mem::*};
use template::library::config::*; // Timer, XorShift, is_local
use template::library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use template::library::fmt::*; // ToComma (数値のカンマ区切り表示)
use template::library::io::*; // InputNum, Output, Answer 等の入出力
use template::library::usermacro::*; // min!, max!, chmin!, chmax!, log!
use template::library::util::*; // MinHeap, bisect
use template::*; // impl_elem!
