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
    eprintln!("time   : {:>6}", timer.get_time_str());
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let s = line.trim_start_matches("VmRSS:").trim();
                if let Some(kb) = s.strip_suffix(" kB") {
                    if let Ok(n) = kb.trim().parse::<u64>() {
                        eprintln!("memory : {}kB", n.comma());
                    }
                } else {
                    eprintln!("memory : {}", s);
                }
            }
        }
    }
}

// === グリッド系 ===
// use template::library::coord::*;     // Coord (座標)
// use template::library::grid::*;      // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use template::library::monoid::*;    // Monoid定義, SegSum等のエイリアス
// use template::library::seg::*;       // Seg, LazySeg

// === 単体 ===
// use template::library::graph::*;     // graph
// use template::library::prime::*;     // is_prime, factorize
// use template::library::unionfind::*; // UnionFind

// === 常用 ===
use itertools::Itertools;
use std::collections::*;
use template::library::config::*; // Timer, FastRng, is_local
use template::library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use template::library::fmt::*; // ToComma (数値のカンマ区切り表示)
use template::library::io::*; // InputNum, Output, Answer 等の入出力
use template::library::usermacro::*; // min!, max!, chmin!, chmax!
use template::library::util::*; // MinHeap, bisect
use template::*; // impl_elem!
