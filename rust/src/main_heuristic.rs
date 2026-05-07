#![allow(unused, non_snake_case)]

fn main() {
    INPUT
        .set(Input::new())
        .unwrap_or_else(|_| panic!("Input is already set"));
    let timer = Timer::new(TIME_LIMIT);
    let mut answer = Answer::new();
    let mut rng = FastRng::new(RNG_SEED);

    // 解く
    let mut result = solve(&timer, &mut rng, &mut answer);

    use std::fmt::Write;
    writeln!(result, "       time : {:>6}", timer.get_time_str()).unwrap();
    eprintln!("{result}");
}
fn solve(timer: &Timer, rng: &mut FastRng, answer: &mut Answer) -> String {
    let mut result = bare_minimum(timer, rng);
    let mut best_solve = "bare_minimum";

    if ENABLE_GREEDY {
        let gr = greedy(timer, rng);
        if gr.score > result.score {
            result = gr;
            best_solve = "Greedy";
        }
    }
    if ENABLE_SA {
        // tune_simulated_annealing(input);
        while !timer.is_timeout() {
            let mut sa = SimulatedAnnealing::new(SAParams::default());
            let sa = sa.run(result.clone(), timer, rng);
            if sa.score > result.score {
                best_solve = "SimulatedAnnealing";
                result = sa;
            }
        }
    }
    if ENABLE_BEAM {
        // tune_beam_search(input);
        let mut bs = BeamSearch::new(BeamParams::default());
        let bs = bs.run(State::new(), timer, rng);
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
fn bare_minimum(_timer: &Timer, _rng: &mut FastRng) -> State {
    State::new()
}
fn greedy(_timer: &Timer, _rng: &mut FastRng) -> State {
    State::new()
}

// === グリッド系 ===
// use library::coord::*;     // Coord (座標)
// use library::grid::*;      // Grid (2次元配列) Coord必要

// === セグ木系 ===
// use library::seg::*;       // Seg, LazySeg, SegSum等のエイリアス, Monoid trait

// === 単体 ===
// use library::dp::*;        // dp
// use library::graph::*;     // graph
// use library::prime::*;     // is_prime, factorize
// use library::unionfind::*; // UnionFind

mod library;
// === 常用 ===
use library::config::*; // Timer, FastRng, is_local
use library::ext::*; // SliceExt, NumExt 等の拡張トレイト
use library::fmt::*; // ToComma (数値のカンマ区切り表示)
use library::heap::*; // MaxHeap, MinHeap
use library::heuristic::*;
use library::io::*; // InputNum, Output, Answer 等の入出力
use library::rng::*; // FastRng
use library::timer::*; // Timer
use library::usermacro::*; // min!, max!, chmin!, chmax!
use library::util::*; // ABC, Set, Map
