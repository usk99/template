// ファイル分割したから提出時動作確認！！！
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn algorithm(answer: &mut Output) {
    // ＊＊＊ 数値型はusizeにしないと掛け算でオーバーフローする可能性ある！！ ＊＊＊
    answer.push("algo");
}
fn main() {
    // 実行時間制限
    let timer = Timer::new(TIME_LIMIT);
    // 出力用意
    let mut answer = Output::new();
    // 解く
    let mut solve = solve(&timer, &mut XorShift::new(RND_SEED), &mut answer);
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
fn solve(timer: &Timer, rnd: &mut XorShift, answer: &mut Output) -> String {
    if EXE_TYPE == 0b00000000 {
        algorithm(answer);
        "Algorithm".to_string()
    } else {
        let input = &Input::new();
        let mut result = bare_minimum(input, timer, rnd);
        let mut best_solve = "bare_minimum";
        if EXE_TYPE & 0b00000001 != 0 {
            // 貪欲？
            let gr = greedy(input, timer, rnd);
            if gr.score > result.score {
                result = gr;
                best_solve = "Greedy";
            }
        }
        if EXE_TYPE & 0b00000010 != 0 {
            // 温度調整
            //tune_simulated_annealing(input);
            while !timer.is_timeout() {
                // 焼きなましっぽい
                let mut sa = SimulatedAnnealing::new(SAParams::default());
                let sa = sa.run(result.clone(), input, timer, rnd);
                if sa.score > result.score {
                    best_solve = "SimulatedAnnealing";
                    result = sa;
                }
            }
        }
        if EXE_TYPE & 0b00000100 != 0 {
            // 幅調整
            // tune_beam_search(input);
            // ビームサーチっぽい
            let mut bs = BeamSearch::new(BeamParams::default());
            let bs = bs.run(State::new(input), input, timer, rnd);
            if bs.score > result.score {
                best_solve = "BeamSearch";
                result = bs;
            }
        }
        answer.push(result.score);
        let mut s = String::new();
        writeln!(s, "Best  Solve : {best_solve:>6}").unwrap();
        writeln!(s, "Final Score : {:>6}", comma(result.score)).unwrap();
        s
    }
}
fn bare_minimum(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}
fn greedy(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}

use rust::library::config::*;
use rust::library::coord::*;
use rust::library::direction::*;
use rust::library::heuristic::*;
use rust::library::io::*;
use rust::library::prime::*;
use rust::library::random::*;
use rust::library::seg::*;
use rust::library::timer::*;
use rust::library::unionfind::*;
use rust::library::usermacro::*;
use rust::library::util::*;
use rust::library::vecseq::*;
use rust::*;

use itertools::Itertools;
use std::{collections::*, fmt::*, fs::*, mem::*};
