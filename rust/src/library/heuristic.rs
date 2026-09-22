#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use {
    super::{config::*, fmt::*, io::*, rng::*, timer::*},
    std::collections::*,
};

#[derive(Clone)]
pub struct Input {}
impl Input {
    pub const fn new() -> Self {
        Self {}
    }
}
pub static INPUT: std::sync::OnceLock<Input> = std::sync::OnceLock::new();
pub fn input() -> &'static Input {
    INPUT.get().expect("Input has not been initialized")
}

#[derive(Clone)]
pub struct State {
    // 問題固有の状態を定義
    pub score: usize,
}
impl State {
    pub const fn new() -> Self {
        Self { score: 0 }
    }
    // 次の状態を生成（ビームサーチ用）
    pub const fn next_states(&self) -> Vec<Self> {
        // 問題に応じた状態遷移を実装
        vec![]
    }
    // 状態遷移(焼きなまし)
    pub const fn random_transition(&mut self, _rng: &mut FastRng) {
        self.score = 0;
    }
    // 状態の評価
    pub const fn evaluate(&mut self) {
        self.score = 0;
    }
}

// 1つの状態（ノード）。キャッシュライン（64バイト）に収めることを強く意識します。
#[derive(Clone, Copy, Debug)]
pub struct Node {
    // --- ① 木構造の管理データ（計16バイト） ---
    pub parent: u32,        // 親
    pub first_child: u32,   // 長男（この子から辿ってDFSで深く潜る）
    pub left_sibling: u32,  // 左の兄（一瞬で自分を削除するために必要）
    pub right_sibling: u32, // 右の弟（DFSで別の分岐へ進むために必要）

    // --- ② 探索固有のデータ（例：計8バイト、パディング込で16バイト） ---
    // ここに持たせるのは「差分」と「評価値」だけであり、盤面全体は絶対に持たせません。
    pub hash: u32,    // 盤面のハッシュ値（SimpleHashSetでの重複排除などに使う）
    pub score: i16,   // このルートの評価値
    pub action: u8,   // 親からここに来るために取った行動（1手分の差分）
    pub active: bool, // ツリー上に存在しているか（削除済みでないか）のフラグ
}
const NULL: u32 = u32::MAX;
// 探索ツリー全体を管理する構造体
pub struct Tree {
    pub nodes: Vec<Node>,    // 巨大な1次元配列（メインメモリ）
    pub free_list: Vec<u32>, // 削除されたノードの番号を溜めておくゴミ箱
}
impl Tree {
    /// ツリーの初期化。想定される最大ノード数を一括で確保し、以降の動的確保を封じます。
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }

    /// 内部メソッド：新しいノードの場所（ID）を確保する
    pub fn allocate_node(&mut self, node: Node) -> u32 {
        if let Some(id) = self.free_list.pop() {
            // ゴミ箱（削除済みの枠）があれば、そこを上書きして再利用する（ゼロ・アロケーション）
            self.nodes[id as usize] = node;
            id
        } else {
            // なければ配列の末尾に追加する
            let id = self.nodes.len() as u32;
            self.nodes.push(node);
            id
        }
    }

    /// 親に新しい子どもを追加し、そのIDを返す (計算量 $O(1)$)
    /// DFSで探索を進め、次の一手が見つかるたびに呼ばれます。
    pub fn add_child(&mut self, parent_id: u32, action: u8, score: i16, hash: u32) -> u32 {
        let new_node = Node {
            parent: parent_id,
            first_child: NULL,
            left_sibling: NULL,
            right_sibling: NULL,
            hash,
            score,
            action,
            active: true,
        };

        let new_child_id = self.allocate_node(new_node);

        if parent_id != NULL {
            let parent_node = &mut self.nodes[parent_id as usize];
            let current_first = parent_node.first_child;

            if current_first == NULL {
                // 初めての子どもの場合、そのまま長男にする
                parent_node.first_child = new_child_id;
            } else {
                // すでに子どもがいる場合、ループで末尾を探すのではなく、
                // 現在の長男の「左」に割り込ませて、自分が「新・長男」になる。
                self.nodes[new_child_id as usize].right_sibling = current_first;
                self.nodes[current_first as usize].left_sibling = new_child_id;
                self.nodes[parent_id as usize].first_child = new_child_id;
            }
        }

        new_child_id
    }

    /// 指定したノードをツリーから切り離し、再利用リストに送る (計算量 $O(1)$)
    /// BFSの評価フェーズで「スコア上位に入れなかった候補」を一斉に粛清する際に呼ばれます。
    pub fn remove_node(&mut self, target_id: u32) {
        let target = self.nodes[target_id as usize];
        if !target.active {
            return; // 既に削除済みの場合は何もしない
        }

        let left = target.left_sibling;
        let right = target.right_sibling;
        let parent = target.parent;

        // 1. 左の兄に「右の弟（自分を飛ばした先）と手を繋げ」と伝える
        if left != NULL {
            self.nodes[left as usize].right_sibling = right;
        } else if parent != NULL {
            // 左に誰もいない（自分が長男だった）場合、親に「次男を新たな長男にしろ」と伝える
            self.nodes[parent as usize].first_child = right;
        }

        // 2. 右の弟に「左の兄と手を繋げ」と伝える
        if right != NULL {
            self.nodes[right as usize].left_sibling = left;
        }

        // 3. 自分自身を「死んだ状態」にして、ゴミ箱（free_list）に返却する
        self.nodes[target_id as usize].active = false;
        self.free_list.push(target_id);
    }
}

#[allow(clippy::while_float)]
pub fn tune_simulated_annealing() {
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
                    let mut rng = FastRng::new(RNG_SEED);
                    let mut sa = SimulatedAnnealing::new(SAParams {
                        start_temp: params.sa_params["start_temp"],
                        end_temp: params.sa_params["end_temp"],
                        cooling_rate: params.sa_params["cooling_rate"],
                        ..Default::default()
                    });
                    sa.run(State::new(), timer, &mut rng).score as f64
                })
                .collect::<Vec<_>>();
            let avg_score = trial_scores.iter().sum::<f64>() / trial_scores.len() as f64;
            eprintln!("{:>12}", (avg_score.round() as usize).comma());
            if avg_score > best_score {
                best_score = avg_score;
                best_params.insert(param_name.to_string(), current);
            }
            current += step;
        }
    }
    eprintln!("\nBest SA Parameters:");
    eprintln!("------------------------");
    for (param, value) in &best_params {
        eprintln!("{param:>12}={value:>8.3}");
    }
    eprintln!("Best Score  = {best_score:>8.0}");
    eprintln!("------------------------");
}
pub fn tune_beam_search() {
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
                    let mut rng = FastRng::new(RNG_SEED);
                    let mut bs = BeamSearch::new(&BeamParams {
                        beam_width: params.bs_params["beam_width"],
                        max_depth: params.bs_params["max_depth"],
                    });
                    bs.run(&State::new(), timer, &mut rng).score as f64
                })
                .collect::<Vec<_>>();
            let avg_score = trial_scores.iter().sum::<f64>() / trial_scores.len() as f64;
            eprintln!("{:>12}", (avg_score as usize).comma());
            if avg_score > best_score {
                best_score = avg_score;
                best_params.insert(param_name.to_string(), current as f64);
            }
            current += step;
        }
    }
    eprintln!("\nBest BS Parameters:");
    eprintln!("------------------------");
    for (param, value) in &best_params {
        eprintln!("{param:>12}={value:>8.3}");
    }
    eprintln!("Best Score  = {best_score:>8.0}");
    eprintln!("------------------------");
}

/// 焼きなましパラメータ
#[derive(Debug, Clone)]
pub struct SAParams {
    /// 開始温度
    pub start_temp: f64,
    /// 終了温度
    pub end_temp: f64,
    /// 冷却率
    pub cooling_rate: f64,
    /// 温度ごとの試行回数
    pub trial_per_temp: usize,
    /// 自動調整
    pub auto_adjust: bool,
    /// 再加熱
    pub reheat_interval: usize,
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
            reheat_interval: 1000,
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
    const fn reheat(&mut self) {
        if self.since_improve > self.params.reheat_interval {
            self.current_temp = self.params.start_temp;
            self.since_improve = 0;
        }
    }
    // 状態遷移の受理判定
    fn accept_transition(&self, diff: f64, rng: &mut FastRng) -> bool {
        if diff >= 0.0 {
            true
        } else {
            rng.f64() < (diff / self.current_temp).exp()
        }
    }
    // 実行
    pub fn run(&mut self, mut current_state: State, timer: &Timer, rng: &mut FastRng) -> State {
        let mut best_state = current_state.clone();
        let mut accept_count = 0;
        let mut trial_count = 0;

        while !timer.is_timeout() {
            for _ in 0..self.params.trial_per_temp {
                if timer.is_timeout() {
                    break;
                }
                let mut next_state = current_state.clone();
                next_state.random_transition(rng);
                next_state.evaluate();
                let diff = next_state.score as f64 - current_state.score as f64;
                if self.accept_transition(diff, rng) {
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
        if !is_local() {
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
    pub fn new(params: &BeamParams) -> Self {
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
    pub fn run(&mut self, initial: &State, timer: &Timer, _rng: &mut FastRng) -> State {
        self.candidates.clear();
        for _ in 0..self.params.max_depth {
            let mut current = std::mem::take(&mut self.candidates);
            // 次の状態生成
            if current.is_empty() {
                current = initial.next_states();
                for next in &mut current {
                    next.evaluate();
                    self.candidates.push(next.clone());
                }
            } else {
                for state in &current {
                    for next in &mut state.next_states() {
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
        self.candidates.first().cloned().unwrap_or_else(State::new)
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
impl Default for HyperParameters {
    fn default() -> Self {
        Self::new()
    }
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
