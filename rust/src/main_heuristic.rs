#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn main() {
    INPUT
        .set(Input::new())
        .unwrap_or_else(|_| panic!("Input is already set"));
    let timer = Timer::new(TIME_LIMIT);
    let mut answer = Answer::new();
    let mut rnd = FastRng::new(RNG_SEED);

    // 解く
    let mut solve = solve(&timer, &mut rnd, &mut answer);

    // 出力
    let ans = answer.outstr();
    if is_local() {
        use std::io::*;
        let _ = BufWriter::new(File::create("../output.txt").unwrap()).write(ans.as_bytes());
    }
    Output(ans);

    solve.push_str(&format!("       time : {:>6}", timer.get_time_str()));
    log!("{solve}");
}
fn solve(timer: &Timer, rnd: &mut FastRng, answer: &mut Answer) -> String {
    let mut result = bare_minimum(timer, rnd);
    let mut best_solve = "bare_minimum";

    if ENABLE_GREEDY {
        let gr = greedy(timer, rnd);
        if gr.score > result.score {
            result = gr;
            best_solve = "Greedy";
        }
    }
    if ENABLE_SA {
        // tune_simulated_annealing(input);
        while !timer.is_timeout() {
            let mut sa = SimulatedAnnealing::new(SAParams::default());
            let sa = sa.run(result.clone(), timer, rnd);
            if sa.score > result.score {
                best_solve = "SimulatedAnnealing";
                result = sa;
            }
        }
    }
    if ENABLE_BEAM {
        // tune_beam_search(input);
        let mut bs = BeamSearch::new(BeamParams::default());
        let bs = bs.run(State::new(), timer, rnd);
        if bs.score > result.score {
            best_solve = "BeamSearch";
            result = bs;
        }
    }

    answer.push(result.score);
    let mut s = String::new();
    use std::fmt::Write;
    writeln!(s, "Best  Solve : {best_solve:>6}").unwrap();
    writeln!(s, "Final Score : {:>6}", result.score.comma()).unwrap();
    s
}
fn bare_minimum(_timer: &Timer, _rnd: &mut FastRng) -> State {
    State::new()
}
fn greedy(_timer: &Timer, _rnd: &mut FastRng) -> State {
    State::new()
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
use std::{collections::*, fmt::*, fs::*, mem::*};
use template::library::config::*; // Timer, FastRng, is_local
use template::library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use template::library::fmt::*; // ToComma (数値のカンマ区切り表示)
use template::library::heuristic::*;
use template::library::io::*; // InputNum, Output, Answer 等の入出力
use template::library::usermacro::*; // min!, max!, chmin!, chmax!, log!
use template::library::util::*; // MinHeap, bisect
use template::*; // impl_elem!
