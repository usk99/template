#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn algorithm(answer: &mut Answer) {
    // ＊＊＊ 数値型はusizeにしないと掛け算でオーバーフローする可能性ある！！ ＊＊＊
    // use std::io::{self, Write};let _ = io::stdout().flush();
    answer.push("algo");
}
fn main() {
    // 実行時間制限
    let timer = Timer::new(TIME_LIMIT);
    // 出力用意
    let mut answer = Answer::new();
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
fn solve(timer: &Timer, rnd: &mut XorShift, answer: &mut Answer) -> String {
    if ENABLE_ALGORITHM {
        algorithm(answer);
        "Algorithm".to_string()
    } else {
        let input = &Input::new();
        let mut result = bare_minimum(input, timer, rnd);
        let mut best_solve = "bare_minimum";
        if ENABLE_GREEDY {
            // 貪欲？
            let gr = greedy(input, timer, rnd);
            if gr.score > result.score {
                result = gr;
                best_solve = "Greedy";
            }
        }
        if ENABLE_SA {
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
        if ENABLE_BEAM {
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
        writeln!(s, "Final Score : {:>6}", result.score.comma()).unwrap();
        s
    }
}
fn bare_minimum(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}
fn greedy(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}

use template::library::config::*;
use template::library::coord::*;
use template::library::direction::*;
use template::library::ext::*;
use template::library::fmt::*;
use template::library::grid::*;
use template::library::heuristic::*;
use template::library::io::*;
use template::library::linkcuttree::*;
use template::library::monoid::*;
use template::library::prime::*;
use template::library::random::*;
use template::library::seg::*;
use template::library::timer::*;
use template::library::unionfind::*;
use template::library::usermacro::*;
use template::library::util::*;
use template::*;

use {
    itertools::Itertools,
    std::{collections::*, fmt::*, fs::*, mem::*},
};
