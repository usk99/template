#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

/// 実行時間
const TIME_LIMIT: u128 = 0000;
/// 実行タイプ
const EXE_TYPE: u8 = 0b00000000;
/// XORシフトのシード
const RND_SEED: u64 = 123456789;
fn main() {
    // 設定初期化
    Config::init();
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
    if isLocal() {
        let _ = BufWriter::new(File::create("../output.txt").unwrap()).write(ans.as_bytes());
    }
    Output(ans);

    solve.push_str(&format!("       time : {:>6}ms", timer.getTimeStr()));
    eprintln!("{solve}");
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
    answer.push("algo");
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
                    let timer = &Timer::new(TIME_LIMIT / 2);
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
                    let timer = &Timer::new(TIME_LIMIT / 2);
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

// 入出力
#[rustfmt::skip]
mod module_of_IO{
    use {itertools::Itertools, std::{fmt::Display,io::Read},crate::{Coord, Direction}};
    // 入力
    fn get_word()->String{let mut stdin=std::io::stdin();
        loop{let mut u8b=[0];let mut buf=Vec::<u8>::with_capacity(16);
            loop{let res=stdin.read(&mut u8b).unwrap_or(0);
                if res==0||u8b[0]<=b' '{break;}// 空白等で終了
                buf.push(u8b[0]);// 返却文字列追加
            }if !buf.is_empty(){return String::from_utf8(buf).unwrap();}// utf8をStringでreturn
        }
    }
    macro_rules! define_input_functions {($($name:ident($($arg:ident:$t:ty),*)->$ret:ty=$body:expr);* $(;)*)=>{$(pub fn $name($($arg:$t),*)->$ret{$body})*}}
    define_input_functions! {
        InputNum     ()               ->        usize   = get_word().parse().unwrap();
        InputNums    (n:usize)        ->    Vec<usize>  =(0..n).map(|_|InputNum()).collect();
        InputsNums   (h:usize,w:usize)->Vec<Vec<usize>> =(0..h).map(|_|InputNums(w)).collect();
        InputString  ()               ->        String  =get_word();
        InputStrings (n:usize)        ->    Vec<String> =(0..n).map(|_|InputString()).collect();
        InputsStrings(h:usize,w:usize)->Vec<Vec<String>>=(0..h).map(|_|InputStrings(w)).collect();
    }
    // 出力
    trait Cts{fn cts(self)->String;}
    macro_rules! impl_cts{($($t:ty),*)=>{$(impl Cts for $t {fn cts(self)->String{self.to_string()}})*}}
    impl_cts!(f32,f64,i8,i16,i32,i64,i128,u8,u16,u32,u64,u128,isize,usize,&str,char,&String);
    impl Cts for bool{fn cts(self)->String{(if self{"Yes"}else{"No"}).to_string()}}
    impl Cts for String{fn cts(self)->String{self}}
    pub trait ChangeToString{fn change_to_string(self)->String;}
    impl<T:Cts+Display>ChangeToString for     T {fn change_to_string(self)->String{self.cts()}}
    impl<T:Cts+Display>ChangeToString for Vec<T>{fn change_to_string(self)->String{self.iter().join(" ")}}
    impl ChangeToString for     Coord {fn change_to_string(self)->String{format!("{self}")}}
    impl ChangeToString for Direction {fn change_to_string(self)->String{format!("{self}")}}
    pub trait Output{fn output(self)->String;}
    impl<T:ChangeToString>Output for T{fn output(self)->String{self.change_to_string()}}
    pub fn  Output(o:impl Output){ println!("{}",o.output())}
    pub fn EOutput(o:impl Output){eprintln!("{}",o.output())}
}
// 設定
#[rustfmt::skip]
mod module_of_config {
    use std::sync::OnceLock;
    pub struct Config {
        islocal: bool,
    }
    pub static CONFIG: OnceLock<Config> = OnceLock::new();
    impl Config {
        pub fn init() {
            // local判定・・・pahcerや提出時はfalse
            let islocal = std::env::args().map(|x| x.contains("target")).next().unwrap()
                & std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
            CONFIG.get_or_init(|| Self { islocal });
        }
    }
    pub fn isLocal() -> bool{CONFIG.get().unwrap().islocal}
}
// 便利
#[rustfmt::skip]
mod module_of_util {
    #![allow(non_upper_case_globals)]
    use std::{cmp::Reverse, collections::BinaryHeap};

    pub const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const abc: &str = "abcdefghijklmnopqrstuvwxyz";

    // カンマ区切り文字列
    pub fn comma<T: ToString>(element: T) -> String {
        let mut s = element.to_string();let mut i = s.len();
        while i > 3 {i -= 3;s.insert(i, ',');}s
    }

    /// 優先度付きキューの最大ヒープ
    pub type MaxHeap<T> = BinaryHeap<T>;
    /// 優先度付きキューの最小ヒープ
    pub struct  MinHeap<T>{heap:BinaryHeap<Reverse<T>>}
    impl<T: Ord>MinHeap<T>{
        pub fn new()->Self {Self {heap: BinaryHeap::new()}}
        pub fn push(&mut self,value:T){self.heap.push(Reverse(value));}
        pub fn pop(&mut self)->Option<T>{self.heap.pop().map(|Reverse(value)|value)}
        pub fn peek(&self)->Option<&T>{self.heap.peek().map(|Reverse(value)|value)}
        pub fn iter(&self)->impl Iterator<Item=&T>{self.heap.iter().map(|Reverse(value)|value)}
        pub fn is_empty(&self)->bool{self.heap.is_empty()}
        pub fn len(&self)->usize{self.heap.len()}
        pub fn clear(&mut self){self.heap.clear();}
    }

    /// インデックスソート
    pub fn index_sort<T: PartialOrd>(arr:&[T]) -> Vec<usize> {
        let mut indices=(0..arr.len ()).collect::<Vec<_>>();
        indices.sort_by(|&a, &b| arr[a].partial_cmp(&arr[b]).unwrap());
        indices
    }
}
// 座標
mod module_of_coord {
    use {
        crate::{Direction, DIRECTIONS, DIRECTIONS_8},
        std::{
            fmt::{Display, Formatter, Result},
            ops::{Add, AddAssign},
        },
    };
    /// 座標の型
    /// -  u8:<=255
    /// - u16:<=65,535
    /// - u32:<=4,294,967,295
    pub type CoordType = u8;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Coord {
        row: CoordType,
        col: CoordType,
    }
    impl Coord {
        /// 原点
        pub const ORIGIN: Coord = Coord::new(0, 0);
        /// 座標生成
        pub const fn new(row: usize, col: usize) -> Self {
            Self {
                row: row as CoordType,
                col: col as CoordType,
            }
        }
        /// 行(y座標)を取得
        #[inline(always)]
        pub const fn row(&self) -> usize {
            self.row as usize
        }
        /// 列(x座標)を取得
        #[inline(always)]
        pub const fn col(&self) -> usize {
            self.col as usize
        }
        /// 2次元座標を1次元インデックスに変換
        pub const fn to_sequence(self, cols: usize) -> usize {
            self.row() * cols + self.col()
        }
        /// 1次元インデックスから座標に変換
        pub const fn from_sequence(idx: usize, cols: usize) -> Self {
            Coord::new(idx / cols, idx % cols)
        }
        /// 領域内に存在するかチェック
        pub const fn within(&self, rows: usize, cols: usize) -> bool {
            self.row() < rows && self.col() < cols
        }
        /// 軸ごとの距離
        pub const fn diff(&self, other: &Self) -> Self {
            Self::new(
                self.row().abs_diff(other.row()),
                self.col().abs_diff(other.col()),
            )
        }
        /// 面積取得
        pub const fn area(&self, other: &Self) -> usize {
            let diff = self.diff(other);
            diff.row() * diff.col()
        }
        /// ユークリッド距離(直線距離)を計算
        pub fn dist_euclidean(&self, other: &Self) -> f64 {
            let rowdiff = self.row().abs_diff(other.row());
            let coldiff = self.col().abs_diff(other.col());
            ((rowdiff * rowdiff + coldiff * coldiff) as f64).sqrt()
        }
        /// マンハッタン距離(上下左右の移動量の合計)を計算
        pub const fn dist_manhattan(&self, other: &Self) -> usize {
            self.row().abs_diff(other.row()) + self.col().abs_diff(other.col())
        }
        /// チェビシェフ距離(8方向移動時の最短手数)を計算
        pub fn dist_chebyshev(&self, other: &Self) -> usize {
            std::cmp::max(
                self.row().abs_diff(other.row()),
                self.col().abs_diff(other.col()),
            )
        }
        /// タプルから変換
        pub const fn from_tuple(tuple: (usize, usize)) -> Self {
            Self::new(tuple.0, tuple.1)
        }
        /// タプルに変換
        pub const fn to_tuple(self) -> (usize, usize) {
            (self.row(), self.col())
        }
        /// 4方向の隣接座標
        pub fn neighbors4(&self, rows: usize, cols: usize) -> Vec<Coord> {
            let mut neighbors = Vec::new();
            for dir in DIRECTIONS {
                let next = *self + dir;
                if next.within(rows, cols) {
                    neighbors.push(next);
                }
            }
            neighbors
        }
        /// 8方向の隣接座標
        pub fn neighbors8(&self, rows: usize, cols: usize) -> Vec<Coord> {
            let mut neighbors = Vec::new();
            for dir in DIRECTIONS_8 {
                let next = *self + dir;
                if next.within(rows, cols) {
                    neighbors.push(next);
                }
            }
            neighbors
        }
        /// 指定座標への方向を取得
        pub const fn direction_to(&self, other: &Self) -> Option<Direction> {
            let row_diff = other.row() as i32 - self.row() as i32;
            let col_diff = other.col() as i32 - self.col() as i32;

            match (row_diff.signum(), col_diff.signum()) {
                (-1, 0) => Some(Direction::U),
                (1, 0) => Some(Direction::D),
                (0, -1) => Some(Direction::L),
                (0, 1) => Some(Direction::R),
                _ => None,
            }
        }
    }
    /// + 演算子で座標の加算
    impl Add<Coord> for Coord {
        type Output = Coord;
        fn add(self, rhs: Coord) -> Self::Output {
            Coord {
                row: self.row.wrapping_add(rhs.row),
                col: self.col.wrapping_add(rhs.col),
            }
        }
    }
    /// + 演算子で座標の加算
    impl Add<Direction> for Coord {
        type Output = Coord;
        fn add(self, rhs: Direction) -> Self::Output {
            Coord {
                row: self.row.wrapping_add(rhs.to_coord().row),
                col: self.col.wrapping_add(rhs.to_coord().col),
            }
        }
    }
    /// += 演算子で座標の加算
    impl AddAssign<Coord> for Coord {
        fn add_assign(&mut self, rhs: Coord) {
            *self = *self + rhs;
        }
    }
    /// += 演算子で座標の加算
    impl AddAssign<Direction> for Coord {
        fn add_assign(&mut self, rhs: Direction) {
            *self = *self + rhs;
        }
    }
    /// 出力形式
    impl Display for Coord {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {}", self.row, self.col)
        }
    }
}
// 方向
mod module_of_direction {
    use crate::Coord;
    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
    pub enum Direction {
        /// 上
        U,
        /// 右
        R,
        /// 下
        D,
        /// 左
        L,
    }
    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
    struct DirectionAdjacent {
        oppsite: Direction,
        clockwise: Direction,
        counter_clockwise: Direction,
    }
    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
    struct DirectionData {
        index: usize,
        symbol: &'static str,
        offset: (isize, isize),
        odjacents: DirectionAdjacent,
    }
    type DirectionInfo = (Direction, DirectionData);
    impl Direction {
        const DIRECTION_INFO: [DirectionInfo; 4] = [
            (
                Direction::U,
                DirectionData {
                    index: 0,
                    symbol: "U",
                    offset: (-1, 0),
                    odjacents: DirectionAdjacent {
                        oppsite: Direction::D,
                        clockwise: Direction::R,
                        counter_clockwise: Direction::L,
                    },
                },
            ),
            (
                Direction::R,
                DirectionData {
                    index: 1,
                    symbol: "R",
                    offset: (0, 1),
                    odjacents: DirectionAdjacent {
                        oppsite: Direction::L,
                        clockwise: Direction::D,
                        counter_clockwise: Direction::U,
                    },
                },
            ),
            (
                Direction::D,
                DirectionData {
                    index: 2,
                    symbol: "D",
                    offset: (1, 0),
                    odjacents: DirectionAdjacent {
                        oppsite: Direction::U,
                        clockwise: Direction::R,
                        counter_clockwise: Direction::L,
                    },
                },
            ),
            (
                Direction::L,
                DirectionData {
                    index: 3,
                    symbol: "L",
                    offset: (0, -1),
                    odjacents: DirectionAdjacent {
                        oppsite: Direction::R,
                        clockwise: Direction::U,
                        counter_clockwise: Direction::D,
                    },
                },
            ),
        ];
        fn get_direction_info(self) -> DirectionInfo {
            Self::DIRECTION_INFO
                .iter()
                .find(|(dir, _)| *dir == self)
                .copied()
                .unwrap()
        }
        /// 方向を反転する
        pub fn opposite(self) -> Self {
            self.get_direction_info().1.odjacents.oppsite
        }
        /// 時計回りに90度
        pub fn clockwise(self) -> Self {
            self.get_direction_info().1.odjacents.clockwise
        }
        /// 反時計回りに90度
        pub fn counter_clockwise(self) -> Self {
            self.get_direction_info().1.odjacents.counter_clockwise
        }
        /// 座標に変換
        pub fn to_coord(self) -> Coord {
            let (row, col) = self.get_direction_info().1.offset;
            Coord::new(row as usize, col as usize)
        }
        /// 方向をインデックスに
        pub fn to_index(self) -> usize {
            self.get_direction_info().1.index
        }
        /// インデックスを方向に
        pub fn from_index(ind: usize) -> Option<Self> {
            Self::DIRECTION_INFO
                .iter()
                .find(|(_, idx)| idx.index == ind)
                .map(|(dir, _)| *dir)
        }
    }
    impl std::fmt::Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.get_direction_info().1.symbol)
        }
    }
    pub const DIRECTIONS: [Coord; 4] = [
        Coord::new(!0, 0), // ↑
        Coord::new(0, 1),  // →
        Coord::new(1, 0),  // ↓
        Coord::new(0, !0), // ←
    ];
    pub const DIRECTIONS_8: [Coord; 8] = [
        Coord::new(!0, 0),  // ↑
        Coord::new(0, 1),   // →
        Coord::new(1, 0),   // ↓
        Coord::new(0, !0),  // ←
        Coord::new(!0, 1),  // →↑
        Coord::new(1, 1),   // →↓
        Coord::new(1, !0),  // ←↓
        Coord::new(!0, !0), // ←↑
    ];
}
// 二次元を一次元で管理
mod module_of_vecseq {
    use {
        crate::Coord,
        core::panic,
        std::{
            fmt::Display,
            ops::{Index, IndexMut},
        },
    };
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VecSeq<T> {
        vec: Vec<T>,
        row: usize,
        col: usize,
    }
    impl<T> VecSeq<T> {
        pub const fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
            if row == 0 {
                panic!("0行")
            } else if col == 0 {
                panic!("0列")
            }
            Self { vec, row, col }
        }
        pub fn swap(&mut self, i: usize, j: usize) {
            self.vec.swap(i, j);
        }
        pub fn size(&self) -> usize {
            self.vec.len()
        }
        #[inline(always)]
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.vec.iter()
        }
        #[inline(always)]
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            self.vec.iter_mut()
        }
        #[inline(always)]
        pub fn as_slice(&self) -> &[T] {
            &self.vec
        }
        #[inline(always)]
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            &mut self.vec
        }
        #[inline(always)]
        pub const fn row(&self) -> usize {
            self.row
        }
        #[inline(always)]
        pub const fn col(&self) -> usize {
            self.col
        }
        #[inline(always)]
        pub const fn get_idx(&self, row: usize, col: usize) -> usize {
            row * self.col + col
        }
    }
    impl<T: Clone> From<Vec<Vec<T>>> for VecSeq<T> {
        fn from(value: Vec<Vec<T>>) -> Self {
            Self {
                vec: value.concat(),
                row: value.len(),
                col: value.first().unwrap().len(),
            }
        }
    }
    impl<T: Clone> From<VecSeq<T>> for Vec<Vec<T>> {
        fn from(val: VecSeq<T>) -> Self {
            val.vec.chunks(val.col).map(|ch| ch.to_vec()).collect()
        }
    }
    impl<T: Default + Clone> VecSeq<T> {
        pub fn from_default(row: usize, col: usize, default: T) -> Self {
            Self {
                vec: vec![default; row * col],
                row,
                col,
            }
        }
    }

    impl<T> Index<usize> for VecSeq<T> {
        type Output = T;
        #[inline(always)]
        fn index(&self, idx: usize) -> &Self::Output {
            &self.vec[idx]
        }
    }
    impl<T> IndexMut<usize> for VecSeq<T> {
        #[inline(always)]
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            &mut self.vec[idx]
        }
    }

    impl<T> Index<Coord> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, coord: Coord) -> &Self::Output {
            &self.vec[coord.to_sequence(self.col)]
        }
    }
    impl<T> IndexMut<Coord> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
            &mut self.vec[coord.to_sequence(self.col)]
        }
    }

    impl<T> Index<&Coord> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, coord: &Coord) -> &Self::Output {
            &self.vec[coord.to_sequence(self.col)]
        }
    }
    impl<T> IndexMut<&Coord> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, coord: &Coord) -> &mut Self::Output {
            &mut self.vec[coord.to_sequence(self.col)]
        }
    }

    impl<T> Index<(usize, usize)> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
            &self.vec[row * self.col + col]
        }
    }
    impl<T> IndexMut<(usize, usize)> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
            &mut self.vec[row * self.col + col]
        }
    }

    /// 視覚化
    impl<T: Display> VecSeq<T> {
        pub fn visualize(&self) -> String {
            let mut result = String::new();
            for i in 0..self.row {
                for j in 0..self.col {
                    result.push_str(&format!("{:3}", self[Coord::new(i, j)]));
                }
                result.push('\n');
            }
            if crate::isLocal() {
                eprintln!("{result}");
            }
            result
        }
    }
}
// タイマー
mod module_of_timer {
    pub struct Timer {
        instant: std::time::Instant,
        timelimit: u128, // 制限時間(ms)
    }
    impl Timer {
        // 計測開始
        pub fn new(timelimit: u128) -> Self {
            Self {
                instant: std::time::Instant::now(),
                timelimit,
            }
        }
        // 時間取得
        pub fn getTime(&self) -> u128 {
            self.instant.elapsed().as_millis()
        }
        // 時間取得(カンマ区切り文字列)
        pub fn getTimeStr(&self) -> String {
            crate::comma(self.getTime())
        }
        // 時間確認
        pub fn is_timeout(&self) -> bool {
            self.getTime() >= self.timelimit
        }
        // 再起動
        pub fn reset(&mut self) {
            self.instant = std::time::Instant::now()
        }
        // 制限時間セット
        pub fn set_limit(&mut self, timelimit: u128) {
            self.timelimit = timelimit;
        }
        // 進捗
        pub fn progress(&self) -> f64 {
            self.getTime() as f64 / self.timelimit as f64
        }
        // 温度管理(焼きなまし用)
        pub fn get_temperature(&self, start_temp: f64, end_temp: f64) -> f64 {
            start_temp + (end_temp - start_temp) * self.progress()
        }
    }
}
// unionfind
mod module_of_unionfind {
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct UnionFind {
        par: Vec<usize>, // 各ノードの親
        siz: Vec<usize>, // 各ノードが属するグループのサイズ
    }
    impl UnionFind {
        /// 初期化
        pub fn new(n: usize) -> Self {
            Self {
                par: (0..n).collect(), // 初期状態では各ノードが自分自身を親に
                siz: vec![1; n],       // 各グループのサイズは1
            }
        }
        /// 根を求める (パス圧縮を行う)
        pub fn root(&mut self, x: usize) -> usize {
            if self.par[x] != x {
                // パス圧縮
                self.par[x] = self.root(self.par[x]);
            }
            self.par[x]
        }
        /// x と y が同じグループに属するかどうか
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        /// x を含むグループと y を含むグループを併合する (サイズの併合)
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let x_root = self.root(x);
            let y_root = self.root(y);
            // すでに同じグループの場合は何もしない
            if x_root == y_root {
                return false;
            }
            // union by size (小さい木を大きい木にくっつける)
            let (smaller, larger) = if self.siz[x_root] < self.siz[y_root] {
                (x_root, y_root)
            } else {
                (y_root, x_root)
            };
            // smaller を larger の子にする
            self.par[smaller] = larger;
            // larger のサイズを更新
            self.siz[larger] += self.siz[smaller];
            true
        }
        /// x を含むグループのサイズ
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            self.siz[root]
        }
        /// 全グループ取得
        pub fn get_groups(&mut self) -> HashMap<usize, Vec<usize>> {
            let mut groups = HashMap::new();
            for i in 0..self.par.len() {
                groups.entry(self.root(i)).or_insert(Vec::new()).push(i);
            }
            groups
        }
        /// 特定のグループを取得
        pub fn get_group(&mut self, target: usize) -> Vec<usize> {
            let target_root = self.root(target);
            (0..self.par.len())
                .filter(|&i| self.root(i) == target_root)
                .collect()
        }
    }
}
// セグ木
mod module_of_segmenttree {
    #[derive(Clone)]
    struct RangeOperationParams<S> {
        left: usize,
        right: usize,
        x: S,
        k: usize,
        l: usize,
        r: usize,
        is_add: bool,
    }
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }
    #[derive(Debug, Clone)]
    pub struct LazySegTree<M: Monoid> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<M::S>,
        lazy: Vec<Option<M::S>>,
    }
    impl<M: Monoid> LazySegTree<M> {
        // 新規作成
        pub fn new(n: usize) -> Self {
            let size = n.next_power_of_two();
            Self {
                n,
                size,
                log: size.trailing_zeros() as usize,
                data: vec![M::identity(); size * 2],
                lazy: vec![None; size * 2],
            }
        }
        // Vecから作成
        pub fn from_vec(v: &[M::S]) -> Self {
            let n = v.len();
            let size = n.next_power_of_two();
            let log = size.trailing_zeros() as usize;
            let mut data = vec![M::identity(); size * 2];
            data[size..size + n].clone_from_slice(v);
            for i in (1..size).rev() {
                data[i] = M::binary_operation(&data[i * 2], &data[i * 2 + 1]);
            }
            Self {
                n,
                size,
                log,
                data,
                lazy: vec![None; size * 2],
            }
        }
        // 伝搬
        fn propagate(&mut self, k: usize) {
            if let Some(x) = self.lazy[k].take() {
                if k < self.size {
                    self.lazy[k * 2] = Some(x.clone());
                    self.lazy[k * 2 + 1] = Some(x.clone());
                }
                self.data[k] = x;
            }
        }
        /// 更新
        pub fn add(&mut self, k: usize, x: M::S) {
            self.add_range_impl(k, k + 1, x);
        }
        pub fn update(&mut self, k: usize, x: M::S) {
            self.update_range_impl(k, k + 1, x);
        }
        fn update_operation(&mut self, k: usize) {
            self.data[k] = M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
        }
        /// 区間加算
        pub fn add_range_impl(&mut self, left: usize, right: usize, x: M::S) {
            self.range_operation_impl(RangeOperationParams {
                left,
                right,
                x,
                k: 1,
                l: 0,
                r: self.size,
                is_add: true,
            });
        }
        /// 区間更新
        pub fn update_range_impl(&mut self, left: usize, right: usize, x: M::S) {
            self.range_operation_impl(RangeOperationParams {
                left,
                right,
                x,
                k: 1,
                l: 0,
                r: self.size,
                is_add: false,
            });
        }
        fn range_operation_impl(&mut self, params: RangeOperationParams<M::S>) {
            self.propagate(params.k);
            if params.right <= params.l || params.r <= params.left {
                return;
            }
            if params.left <= params.l && params.r <= params.right {
                let value = if params.is_add {
                    match &self.lazy[params.k] {
                        Some(y) => M::binary_operation(y, &params.x),
                        None => M::binary_operation(&self.data[params.k], &params.x),
                    }
                } else {
                    params.x.clone()
                };
                self.lazy[params.k] = Some(value);
                self.propagate(params.k);
                return;
            }
            let mid = (params.l + params.r) / 2;
            self.range_operation_impl(RangeOperationParams {
                left: params.left,
                right: params.right,
                x: params.x.clone(),
                k: params.k * 2,
                l: params.l,
                r: mid,
                is_add: params.is_add,
            });
            self.range_operation_impl(RangeOperationParams {
                left: params.left,
                right: params.right,
                x: params.x.clone(),
                k: params.k * 2 + 1,
                l: mid,
                r: params.r,
                is_add: params.is_add,
            });
            self.update_operation(params.k);
        }
        /// 区間取得
        pub fn get_range(&mut self, left: usize, right: usize) -> M::S {
            self.get_range_impl(left, right, 1, 0, self.size)
        }
        fn get_range_impl(
            &mut self,
            left: usize,
            right: usize,
            k: usize,
            l: usize,
            r: usize,
        ) -> M::S {
            self.propagate(k);
            if right <= l || r <= left {
                return M::identity();
            }
            if left <= l && r <= right {
                return self.data[k].clone();
            }
            let mid = (l + r) / 2;
            let vl = self.get_range_impl(left, right, k * 2, l, mid);
            let vr = self.get_range_impl(left, right, k * 2 + 1, mid, r);
            M::binary_operation(&vl, &vr)
        }
    }
    // 和の実装
    #[derive(Clone, Copy)]
    pub struct Sum;
    impl Monoid for Sum {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }
    // 最小値の実装
    #[derive(Clone, Copy)]
    pub struct Min;
    impl Monoid for Min {
        type S = usize;
        fn identity() -> Self::S {
            usize::MAX
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
    // 最大値の実装
    #[derive(Clone, Copy)]
    pub struct Max;
    impl Monoid for Max {
        type S = usize;
        fn identity() -> Self::S {
            usize::MIN
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
    // XORの実装
    #[derive(Clone, Copy)]
    pub struct Xor;
    impl Monoid for Xor {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a ^ b
        }
    }
    // gcdの実装
    #[derive(Clone, Copy)]
    pub struct Gcd;
    impl Monoid for Gcd {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            num::Integer::gcd(a, b)
        }
    }
    // lcmの実装
    #[derive(Clone, Copy)]
    pub struct Lcm;
    impl Monoid for Lcm {
        type S = usize;
        fn identity() -> Self::S {
            1
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            num::Integer::lcm(a, b)
        }
    }

    impl LazySegTree<Sum> {
        pub fn new_sum(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_sum(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
    impl LazySegTree<Min> {
        pub fn new_min(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_min(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
    impl LazySegTree<Max> {
        pub fn new_max(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_max(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
    impl LazySegTree<Xor> {
        pub fn new_xor(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_xor(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
    impl LazySegTree<Gcd> {
        pub fn new_gcd(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_gcd(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
    impl LazySegTree<Lcm> {
        pub fn new_lcm(n: usize) -> Self {
            Self::new(n)
        }
        pub fn from_vec_lcm(v: &[usize]) -> Self {
            Self::from_vec(v)
        }
    }
}
// マクロ
mod module_of_macro {
    #[macro_export]
    macro_rules! min {
        ($a:expr $(,)*)=>{{$a}};
        ($a:expr,$b:expr $(,)*)=>{{std::cmp::min($a, $b)}};
        ($a:expr,$($rest:expr),+ $(,)*)=>{{std::cmp::min($a, min!($($rest),+))}};
    }
    #[macro_export]
    macro_rules! max {
        ($a:expr $(,)*)=>{{$a}};
        ($a:expr,$b:expr $(,)*)=>{{std::cmp::max($a, $b)}};
        ($a:expr,$($rest:expr),+ $(,)*)=>{{std::cmp::max($a, max!($($rest),+))}};
    }
    #[macro_export]
    macro_rules! ChMin {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_min = min!($($cmps),+);
            if $base > cmp_min {$base = cmp_min;true}else{false}}};
    }
    #[macro_export]
    macro_rules! ChMax {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_max = max!($($cmps),+);
            if $base < cmp_max {$base = cmp_max;true} else {false}}};
    }
}
// 乱数
mod module_of_random {
    use {crate::Coord, rand::prelude::*};
    pub struct XorShift {
        state: u64,
    }
    impl XorShift {
        pub const fn new(seed: u64) -> Self {
            Self { state: seed }
        }
        /// 乱数生成
        pub fn next(&mut self) -> u64 {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            self.state
        }
        /// 0~1で乱数
        pub fn next_f64(&mut self) -> f64 {
            (self.next() as f64) / (u64::MAX as f64)
        }
        /// 指定確率でtrueを返す
        pub fn next_bool(&mut self, prob: f64) -> bool {
            self.next_f64() < prob
        }
        // 範囲内
        pub fn next_range(&mut self, min: usize, max: usize) -> usize {
            min + (self.next() as usize % (max - min))
        }
        /// ランダム選択
        pub fn choose<'a, T: Clone>(&mut self, items: &'a [T]) -> &'a T {
            &items[self.next_range(0, items.len())]
        }
        /// シャッフル
        pub fn shuffle<T>(&mut self, items: &mut [T]) {
            for i in (1..items.len()).rev() {
                items.swap(i, self.next_range(0, i + 1));
            }
        }
    }
}
// 焼きなまし
mod module_of_simulatedannealing {
    use {
        super::Input,
        crate::{State, Timer, XorShift},
    };
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
            if !crate::isLocal() {
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
}
// ビームサーチ
mod module_of_beamsearch {
    use {
        super::Input,
        crate::{State, Timer, XorShift},
    };
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
}
// 調整
mod module_of_turning {
    use {
        crate::{BeamParams, SAParams},
        std::{collections::HashMap, fmt::Write},
    };
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
            let var = self.scores.iter().map(|&x| (x - avg).powi(2)).sum::<f64>()
                / self.scores.len() as f64;
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
            let mut result = String::new();
            for (param, value) in &self.param_history {
                writeln!(result, "{param}: {value:.2}").unwrap();
            }
            result
        }
    }
}

use {
    crate::{
        module_of_IO::*, module_of_beamsearch::*, module_of_config::*, module_of_coord::*,
        module_of_direction::*, module_of_random::*, module_of_segmenttree::*,
        module_of_simulatedannealing::*, module_of_timer::*, module_of_turning::*,
        module_of_unionfind::*, module_of_util::*, module_of_vecseq::*,
    },
    itertools::Itertools,
    num::{pow::Pow, range, Integer},
    std::{
        cmp::*,
        collections::*,
        fs::File,
        io::{BufWriter, Write},
        vec,
    },
    superslice::Ext,
};
