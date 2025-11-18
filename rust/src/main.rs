// ファイル分割したから提出時動作確認！！！
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

/// 実行時間
const TIME_LIMIT: f64 = 0000.0;
/// 実行タイプ
const EXE_TYPE: u8 = 0b00000000;
/// XORシフトのシード
const RND_SEED: u64 = 123456789;
fn main() {
    // 入力
    let input = Input::new();
    // 実行時間制限
    let timer = Timer::new(TIME_LIMIT);
    // 出力用意
    let mut answer = Output::new();
    // 解く
    let mut solve = solve(&timer, &mut XorShift::new(RND_SEED), &input, &mut answer);
    // 出力
    let ans = answer.outstr();
    if is_local() {
        use std::fs::File;
        use std::io::BufWriter;
        use std::io::Write;
        let _ = BufWriter::new(File::create("../output.txt").unwrap()).write(ans.as_bytes());
    }
    Output(ans);

    solve.push_str(&format!("       time : {:>6}", timer.get_time_str()));
    log!("{solve}");
}
fn solve(timer: &Timer, rnd: &mut XorShift, input: &Input, answer: &mut Output) -> String {
    if EXE_TYPE == 0b00000000 {
        algorithm(timer, input, answer);
        "Algorithm".to_string()
    } else {
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
        use std::fmt::Write;
        let mut s = String::new();
        writeln!(s, "Best  Solve : {best_solve:>6}").unwrap();
        writeln!(s, "Final Score : {:>6}", comma(result.score)).unwrap();
        s
    }
}
fn algorithm(_timer: &Timer, _input: &Input, answer: &mut Output) {
    // ＊＊＊ 数値型はusizeにしないと掛け算でオーバーフローする可能性ある！！ ＊＊＊
    answer.push("algo")
}
fn bare_minimum(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}
fn greedy(input: &Input, _timer: &Timer, _rnd: &mut XorShift) -> State {
    State::new(input)
}
fn tune_simulated_annealing(input: &Input) {
    let mut params = HyperParameters::new();
    let mut best_score = 0.0;
    let mut best_params = HashMap::new();
    let sa_ranges = vec![
        ("start_temp", (2000.0, 2000.0, 250.0)), // より効率的な範囲
        ("end_temp", (9.5, 9.5, 0.5)),           // ステップ幅を大きく
        ("cooling_rate", (0.970, 0.999, 0.001)), // 重要な範囲
    ];
    eprintln!("\n===== Simulated Annealing Tuning =====");
    for (param_name, (min, max, step)) in sa_ranges {
        let mut current = min;
        while current <= max {
            params.sa_params.insert(param_name.to_string(), current);
            eprint!("{param_name:>12}={current:>8.3}/{max:>8.3}：score = ");
            let trial_scores = (0..3)
                .map(|_| {
                    let timer = &Timer::new(TIME_LIMIT / 2.0);
                    let mut rnd = XorShift::new(RND_SEED);
                    let mut sa = SimulatedAnnealing::new(SAParams {
                        start_temp: params.sa_params["start_temp"],
                        end_temp: params.sa_params["end_temp"],
                        cooling_rate: params.sa_params["cooling_rate"],
                        ..Default::default()
                    });
                    sa.run(State::new(input), input, timer, &mut rnd).score as f64
                })
                .collect::<Vec<_>>();
            let avg_score = trial_scores.iter().sum::<f64>() / trial_scores.len() as f64;
            eprintln!("{:>12}", comma(avg_score.round() as usize));
            if avg_score > best_score {
                best_score = avg_score;
                best_params.insert(param_name.to_string(), current);
            }
            current += step;
        }
    }
    eprintln!("\nBest SA Parameters:");
    eprintln!("------------------------");
    for (param, value) in best_params.iter() {
        eprintln!("{param:>12}={value:>8.3}");
    }
    eprintln!("Best Score  = {best_score:>8.0}");
    eprintln!("------------------------");
}
fn tune_beam_search(input: &Input) {
    let mut params = HyperParameters::new();
    let mut best_score = 0.0;
    let mut best_params = HashMap::new();
    let bs_ranges = vec![
        ("beam_width", (50, 200, 25)), // ビーム幅
        ("max_depth", (5, 20, 5)),     // 探索深さ
    ];
    eprintln!("\n===== Beam Search Tuning =====");
    for (param_name, (min, max, step)) in bs_ranges {
        let mut current = min;
        while current <= max {
            params.bs_params.insert(param_name.to_string(), current);
            eprint!("{param_name:>12}={current:>8.3}/{max:>8.3}：score = ");
            let trial_scores = (0..3)
                .map(|_| {
                    let timer = &Timer::new(TIME_LIMIT / 2.0);
                    let mut rnd = XorShift::new(RND_SEED);
                    let mut bs = BeamSearch::new(BeamParams {
                        beam_width: params.bs_params["beam_width"],
                        max_depth: params.bs_params["max_depth"],
                    });
                    bs.run(State::new(input), input, timer, &mut rnd).score as f64
                })
                .collect::<Vec<_>>();
            let avg_score = trial_scores.iter().sum::<f64>() / trial_scores.len() as f64;
            eprintln!("{:>12}", comma(avg_score as usize));
            if avg_score > best_score {
                best_score = avg_score;
                best_params.insert(param_name.to_string(), current as f64);
            }
            current += step;
        }
    }
    eprintln!("\nBest BS Parameters:");
    eprintln!("------------------------");
    for (param, value) in best_params.iter() {
        eprintln!("{param:>12}={value:>8.3}");
    }
    eprintln!("Best Score  = {best_score:>8.0}");
    eprintln!("------------------------");
}

#[derive(Clone)]
pub struct State {
    // 問題固有の状態を定義
    score: usize,
}
impl State {
    fn new(_input: &Input) -> Self {
        Self { score: 0 }
    }
    // 次の状態を生成（ビームサーチ用）
    fn next_states(&self) -> Vec<State> {
        // 問題に応じた状態遷移を実装
        vec![]
    }
    // 状態遷移(焼きなまし)
    fn random_transition(&mut self, _rnd: &mut XorShift) {}
    // 状態の評価
    fn evaluate(&mut self) {
        self.score = 0;
    }
}

#[derive(Clone)]
pub struct Input {}
impl Input {
    fn new() -> Self {
        Self {}
    }
}
pub struct Output {
    output: Vec<String>,
}
impl Output {
    fn new() -> Self {
        Self { output: Vec::new() }
    }
    // 追加
    fn push<T: ChangeToString>(&mut self, element: T) {
        self.output.push(element.change_to_string());
    }
    // 初期化
    fn clear(&mut self) {
        self.output.clear();
    }
    // 文字列化
    fn outstr(&self) -> String {
        self.output.iter().join("\n")
    }
}

/// 焼きなましパラメータ
#[derive(Debug, Clone)]
pub struct SAParams {
    /// 開始温度
    pub(crate) start_temp: f64,
    /// 終了温度
    pub(crate) end_temp: f64,
    /// 冷却率
    pub cooling_rate: f64,
    /// 温度ごとの試行回数
    pub trial_per_temp: usize,
    /// 自動調整
    pub auto_adjust: bool,
    /// 再加熱
    pub reheat_interbal: usize,
    /// 最小受理率
    pub min_accept: f64,
}
impl Default for SAParams {
    fn default() -> Self {
        Self {
            start_temp: 1000.0,
            end_temp: 1.0,
            cooling_rate: 0.995,
            trial_per_temp: 100,
            auto_adjust: true,
            reheat_interbal: 1000,
            min_accept: 0.2,
        }
    }
}
/// 焼きなまし統計情報
#[derive(Debug, Default)]
struct SAStats {
    /// 受理回数
    accept_count: usize,
    /// 改善回数
    improve_count: usize,
    /// 総試行回数
    trial_count: usize,
    /// 最高スコア
    best_score: usize,
    /// スコア履歴
    scores: Vec<usize>,
    /// 温度履歴
    temp_history: Vec<f64>,
}
pub struct SimulatedAnnealing {
    /// 現在の温度
    current_temp: f64,
    /// パラメータ
    params: SAParams,
    /// 統計情報
    stats: SAStats,
    /// 改善からのインターバル
    since_improve: usize,
}
impl SimulatedAnnealing {
    pub fn new(params: SAParams) -> Self {
        Self {
            current_temp: params.start_temp,
            params,
            stats: SAStats::default(),
            since_improve: 0,
        }
    }
    // 自動調整を含めた温度管理
    fn cool(&mut self, acceptance_rate: f64) {
        if self.params.auto_adjust {
            if acceptance_rate < self.params.min_accept {
                // 受理率が低すぎる場合は温度を上げる
                self.current_temp *= 1.1;
            } else if acceptance_rate > 0.8 {
                // 受理率が高すぎる場合は温度を下げる
                self.current_temp *= 0.9;
            } else {
                // 通常冷却
                self.current_temp *= self.params.cooling_rate;
            }
        } else {
            // 通常冷却
            self.current_temp *= self.params.cooling_rate;
        }
        self.current_temp = self.current_temp.max(self.params.end_temp);
    }
    // 再加熱
    fn reheat(&mut self) {
        if self.since_improve > self.params.reheat_interbal {
            self.current_temp = self.params.start_temp;
            self.since_improve = 0;
        }
    }
    // 状態遷移の受理判定
    fn accept_transition(&self, diff: f64) -> bool {
        if diff >= 0.0 {
            true
        } else {
            rand::random::<f64>() < (diff / self.current_temp).exp()
        }
    }
    // 実行
    pub fn run(
        &mut self,
        mut current_state: State,
        _input: &Input,
        timer: &Timer,
        rnd: &mut XorShift,
    ) -> State {
        let mut best_state = current_state.clone();
        let mut accept_count = 0;
        let mut trial_count = 0;

        while !timer.is_timeout() {
            for _ in 0..self.params.trial_per_temp {
                if timer.is_timeout() {
                    break;
                }
                let mut next_state = current_state.clone();
                next_state.random_transition(rnd);
                next_state.evaluate();
                let diff = next_state.score as f64 - current_state.score as f64;
                if self.accept_transition(diff) {
                    current_state = next_state;
                    accept_count += 1;

                    if current_state.score > best_state.score {
                        best_state = current_state.clone();
                        self.since_improve = 0;
                        self.stats.improve_count += 1;
                    }
                }
                trial_count += 1;
                self.since_improve += 1;
                self.stats.trial_count += 1;
            }
            // 温度管理
            let accept_rate = accept_count as f64 / trial_count as f64;
            self.cool(accept_rate);
            // 統計
            self.stats.best_score = best_state.score;
            self.stats.scores.push(current_state.score);
            self.stats.temp_history.push(self.current_temp);
            self.print_states();
            accept_count = 0;
            trial_count = 0;
        }
        best_state
    }
    // 統計情報の出力
    pub fn print_states(&self) {
        if !crate::is_local() {
            return;
        }
        use std::fmt::Write;
        let mut s = String::new();
        writeln!(s, "SimulatedAnnealing").unwrap();
        writeln!(s, "  Total trial_count: {}", self.stats.trial_count).unwrap();
        writeln!(s, "  Improvements found: {}", self.stats.improve_count).unwrap();
        writeln!(s, "  Best score: {:>6}", self.stats.best_score).unwrap();
        writeln!(s, "  Final temperature: {:.6}", self.current_temp).unwrap();
        eprintln!("{s}");
        // 進捗出力
        let mut progress = String::from("Trial    Score  Temperature\n");
        for (i, (score, temp)) in self
            .stats
            .scores
            .iter()
            .zip(self.stats.temp_history.iter())
            .enumerate()
        {
            writeln!(progress, "{i:>5}\t{score:>6}\t{temp:>11.6}").unwrap();
        }
        std::fs::write("sa_progress.txt", progress).unwrap();
    }
}

/// ビームサーチのパラメータ
#[derive(Debug, Clone)]
pub struct BeamParams {
    /// ビーム幅
    pub beam_width: usize,
    /// 最大探索深さ
    pub max_depth: usize,
}
impl Default for BeamParams {
    fn default() -> Self {
        Self {
            beam_width: 100,
            max_depth: 10,
        }
    }
}
pub struct BeamSearch {
    params: BeamParams,
    candidates: Vec<State>,
    best_score: usize,
}
impl BeamSearch {
    pub fn new(params: BeamParams) -> Self {
        Self {
            params: params.clone(),
            candidates: Vec::with_capacity(params.beam_width),
            best_score: usize::MIN,
        }
    }
    // ビーム幅の調整
    fn adjust_beam_width(&self, timer: &Timer) -> usize {
        let p = timer.progress();
        if p < 0.2 {
            self.params.beam_width / 2
        } else if p < 0.5 {
            self.params.beam_width * 3 / 4
        } else {
            self.params.beam_width
        }
    }
    fn add_candidate(&mut self, state: State, score: usize) {
        if score >= self.best_score {
            self.candidates.push(state);
            if score > self.best_score {
                self.best_score = score;
            }
        }
    }
    pub fn run(
        &mut self,
        initial: State,
        input: &Input,
        timer: &Timer,
        _rnd: &mut XorShift,
    ) -> State {
        self.candidates.clear();
        for _ in 0..self.params.max_depth {
            let mut current = std::mem::take(&mut self.candidates);
            // 次の状態生成
            if current.is_empty() {
                current = initial.next_states();
                for next in current.iter_mut() {
                    next.evaluate();
                    self.candidates.push(next.clone());
                }
            } else {
                for state in current.iter() {
                    for next in state.next_states().iter_mut() {
                        next.evaluate();
                        self.candidates.push(next.clone());
                    }
                }
            }
            // 上位を保持
            self.candidates.sort_by_key(|s| usize::MAX - s.score);
            self.candidates.truncate(self.adjust_beam_width(timer));

            if self.candidates.is_empty() {
                break;
            }
            if timer.is_timeout() {
                break;
            }
        }
        // 最良状態を返す
        self.candidates.sort_by_key(|s| usize::MAX - s.score);
        self.candidates
            .first()
            .unwrap_or(&State::new(input))
            .clone()
    }
}

// 調整
pub struct HyperParameters {
    // 焼きなまし
    pub sa_params: HashMap<String, f64>,
    // ビームサーチ
    pub bs_params: HashMap<String, usize>,
    // スコア履歴
    pub scores: Vec<f64>,
    // 変更履歴
    pub param_history: Vec<(String, f64)>,
}
impl HyperParameters {
    pub fn new() -> Self {
        let sap = SAParams::default();
        let mut sa_params = HashMap::new();
        sa_params.insert("start_temp".to_string(), sap.start_temp);
        sa_params.insert("end_temp".to_string(), sap.end_temp);
        sa_params.insert("cooling_rate".to_string(), sap.cooling_rate);
        let bsp = BeamParams::default();
        let mut bs_params = HashMap::new();
        bs_params.insert("beam_width".to_string(), bsp.beam_width);
        bs_params.insert("max_depth".to_string(), bsp.max_depth);
        Self {
            sa_params,
            bs_params,
            scores: Vec::new(),
            param_history: Vec::new(),
        }
    }
    // スコア履歴から最適なパラメータを推定
    pub fn tune(&mut self, new_scores: &[f64]) {
        self.scores.extend_from_slice(new_scores);
        // スコアの統計情報を計算
        let avg = self.scores.iter().sum::<f64>() / self.scores.len() as f64;
        let var =
            self.scores.iter().map(|&x| (x - avg).powi(2)).sum::<f64>() / self.scores.len() as f64;
        // パラメータの調整
        self.adjust_temperature(avg, var);
        self.adjust_beam_width(avg);
        // 履歴の保存
        self.param_history.push(("score_avg".to_string(), avg));
    }
    // 温度パラメータの調整
    fn adjust_temperature(&mut self, avg_score: f64, variance: f64) {
        let current_temp = self.sa_params["start_temp"];
        // スコアの変動が大きい場合は温度を上げる
        if variance > avg_score * 0.1 {
            self.sa_params
                .insert("start_temp".to_string(), current_temp * 1.2);
        } else {
            self.sa_params
                .insert("start_temp".to_string(), current_temp * 0.8);
        }
    }
    // ビーム幅の調整
    fn adjust_beam_width(&mut self, avg_score: f64) {
        let current_width = self.bs_params["beam_width"];
        // スコアが改善している場合はビーム幅を増やす
        if self.scores.len() > 2 && avg_score > self.scores[self.scores.len() - 2] {
            self.bs_params
                .insert("beam_width".to_string(), current_width + 10);
        }
    }
    // パラメータ調整履歴の出力
    pub fn print_history(&self) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for (param, value) in &self.param_history {
            writeln!(result, "{param}: {value:.2}").unwrap();
        }
        result
    }
}

use rust::library::config::*;
use rust::library::coord::*;
use rust::library::direction::*;
use rust::library::io::*;
use rust::library::random::*;
use rust::library::seg::*;
use rust::library::timer::*;
use rust::library::unionfind::*;
use rust::library::usermacro::*;
use rust::library::util::*;
use rust::library::vecseq::*;
use rust::*;

use itertools::Itertools;
use std::collections::*;
