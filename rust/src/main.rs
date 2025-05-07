#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

const TIME_LIMIT: u128 = 0;
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
    solve(&timer, &input, &mut answer);

    // 出力
    let ans = answer.outstr();
    if isLocal() {
        let _ = BufWriter::new(File::create("../output.txt").unwrap()).write(ans.as_bytes());
    }
    Output(ans);
    eprintln!("time : {}", timer.getTimeStr());
}
fn solve(timer: &Timer, input: &Input, answer: &mut Output) {
    let mut rnd = XorShift::new(123456789);
    let initial_state = State::new(input);
    let result: State;

    const EXETYPE: u8 = 0b0000;

    if EXETYPE == 0b0000 {
        algo(timer, input, answer);
    } else {
        // 貪欲？
        result = greedy(input, &initial_state);

        if EXETYPE == 1 {
            // 焼きなましっぽい
            let s = SAParams::default();
            let mut sa = SimulatedAnnealing::new(s);
            result = sa.run(initial_state, timer, &mut rnd);
        } else {
            // ビームサーチっぽい
            result = State::new(input);
        }
        result.print_states();
        let ans = ["", ""];
        for i in ans {
            answer.push(format!("{}", i));
        }
    }
}
fn algo(timer: &Timer, input: &Input, answer: &mut Output) {}
fn greedy(input: &Input, state: &State) -> State {
    State::new(input)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum FieldType {
    #[default]
    Empty,
    Sector(u16),
}

#[derive(Clone)]
struct State {
    // 問題固有の状態を定義
    score: usize,
}
impl State {
    fn new(input: &Input) -> Self {
        Self { score: 0 }
    }

    // 次の状態を生成（ビームサーチ用）
    fn next_states(&self) -> Vec<State> {
        // 問題に応じた状態遷移を実装
        vec![];
    }

    // 状態遷移(焼きなまし)
    fn random_transition(&mut self, rnd: &mut XorShift) {}

    // 状態の評価
    fn evaluate(&self) -> usize {
        0
    }

    // 最終結果
    fn print_states(&self) {
        let mut res = "Result\n".to_string();
        res.push_str(&format!("  Final score :{}\n", self.score));
    }
}

#[derive(Clone)]
struct Input {}
impl Input {
    fn new() -> Self {
        Self {}
    }
}
struct Output {
    output: Vec<String>,
}
impl Output {
    fn new() -> Self {
        Self { output: Vec::new() }
    }
    // 追加
    fn push<T: ChangeToString>(&mut self, element: T) {
        self.output.push(element.change_to_string())
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
    use itertools::Itertools;use std::{fmt::Display,io::Read};
    use crate::module_of_coord::Coord;
    use crate::module_of_direction::Direction;
    // 入力
    fn get_word()->String{let mut stdin=std::io::stdin();
        loop{let mut u8b=[0];let mut buf=Vec::<u8>::with_capacity(16);
            loop{let res=stdin.read(&mut u8b).unwrap_or(0);
                if res==0||u8b[0]<=b' '{break;}// 空白等で終了
                else{buf.push(u8b[0]);}// 返却文字列追加
            }if !buf.is_empty(){return String::from_utf8(buf).unwrap();}// utf8をStringでreturn
        }
    }
    pub fn InputNum     ()                ->        usize   {get_word().parse().unwrap()}
    pub fn InputNums    (n: usize)        ->    Vec<usize>  {let mut v=Vec::    <usize>  ::new();for _ in 0..n{v.push(InputNum());}v}
    pub fn InputsNums   (h: usize,w:usize)->Vec<Vec<usize>> {let mut v=Vec::<Vec<usize>> ::new();for _ in 0..h{v.push(InputNums(w));}v}
    pub fn InputString  ()                ->        String  {get_word()}
    pub fn InputStrings (n: usize)        ->    Vec<String> {let mut v=Vec::    <String> ::new();for _ in 0..n{v.push(InputString());}v}
    pub fn InputsStrings(h: usize,w:usize)->Vec<Vec<String>>{let mut v=Vec::<Vec<String>>::new();for _ in 0..h{v.push(InputStrings(w));}v}
    // 出力
    pub trait Output{fn output(self) -> String;}
    pub fn  Output(o:impl Output){ println!("{}",o.output())}
    pub fn EOutput(o:impl Output){eprintln!("{}",o.output())}
    impl<T:ChangeToString>Output for T {fn output(self) -> String{self.change_to_string()}}
    // 基本的な型の出力様式
    trait Cts{fn cts(self) -> String;}
    fn changetostring(element:impl Cts)->String{element.cts()}
    impl Cts for   bool{fn cts(self)->String{(if self {"Yes"} else {"No"}).to_string()}}
    impl Cts for    f32{fn cts(self)->String{self.to_string()}}impl Cts for    f64{fn cts(self)->String{self.to_string()}}
    impl Cts for     i8{fn cts(self)->String{self.to_string()}}impl Cts for     u8{fn cts(self)->String{self.to_string()}}
    impl Cts for    i16{fn cts(self)->String{self.to_string()}}impl Cts for    u16{fn cts(self)->String{self.to_string()}}
    impl Cts for    i32{fn cts(self)->String{self.to_string()}}impl Cts for    u32{fn cts(self)->String{self.to_string()}}
    impl Cts for    i64{fn cts(self)->String{self.to_string()}}impl Cts for    u64{fn cts(self)->String{self.to_string()}}
    impl Cts for   i128{fn cts(self)->String{self.to_string()}}impl Cts for   u128{fn cts(self)->String{self.to_string()}}
    impl Cts for  isize{fn cts(self)->String{self.to_string()}}impl Cts for  usize{fn cts(self)->String{self.to_string()}}
    impl Cts for   &str{fn cts(self)->String{self.to_string()}}impl Cts for   char{fn cts(self)->String{self.to_string()}}
    impl Cts for&String{fn cts(self)->String{self.to_string()}}impl Cts for String{fn cts(self)->String{self}}
    // vec対応
    pub trait ChangeToString{fn change_to_string(self)->String;}
    impl <T:Cts + Display> ChangeToString for     T {fn change_to_string(self)->String{changetostring(self)}}
    impl <T:Cts + Display> ChangeToString for Vec<T>{fn change_to_string(self)->String{self.iter().join(" ")}}
    impl ChangeToString for     Coord{fn change_to_string(self)->String {format!("{}",self)}}
    impl ChangeToString for Direction{fn change_to_string(self)->String {format!("{}",self)}}
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
// 座標
mod module_of_coord {
    use std::{
        fmt::{Display, Formatter, Result},
        ops::{Add, AddAssign},
    };
    /// 座標の型
    /// -  u8:<=255
    /// - u16:<=65,535
    /// - u32:<=4,294,967,295
    type CoordType = u8;
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
        pub const fn row(&self) -> usize {
            self.row as usize
        }
        /// 列(x座標)を取得
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
        pub fn from_tuple(tuple: (usize, usize)) -> Self {
            Self::new(tuple.0, tuple.1)
        }
        /// タプルに変換
        pub fn to_tuple(self) -> (usize, usize) {
            (self.row(), self.col())
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
    /// += 演算子で座標の加算
    impl AddAssign<Coord> for Coord {
        fn add_assign(&mut self, rhs: Coord) {
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
    impl Direction {
        /// 方向を反転する
        pub fn opposite(self) -> Self {
            match self {
                Direction::U => Direction::D,
                Direction::R => Direction::L,
                Direction::D => Direction::U,
                Direction::L => Direction::R,
            }
        }
        /// 時計回りに90度
        pub fn clockwise(self) -> Self {
            match self {
                Direction::U => Direction::R,
                Direction::R => Direction::D,
                Direction::D => Direction::L,
                Direction::L => Direction::U,
            }
        }
        /// 反時計回りに90度
        pub fn counter_clockwise(self) -> Self {
            match self {
                Direction::U => Direction::L,
                Direction::R => Direction::U,
                Direction::D => Direction::R,
                Direction::L => Direction::D,
            }
        }
        /// 座標に変換
        pub fn to_coord(self) -> Coord {
            match self {
                Direction::U => Coord::new(!0, 0), // -1, 0
                Direction::R => Coord::new(0, 1),
                Direction::D => Coord::new(1, 0),
                Direction::L => Coord::new(0, !0), // 0, -1
            }
        }
        /// 方向をインデックスに
        pub fn to_index(self) -> usize {
            match self {
                Direction::U => 0,
                Direction::R => 1,
                Direction::D => 2,
                Direction::L => 3,
            }
        }
        /// インデックスを方向に
        pub fn from_index(ind: usize) -> Option<Self> {
            match ind {
                0 => Some(Direction::U),
                1 => Some(Direction::R),
                2 => Some(Direction::D),
                3 => Some(Direction::L),
                _ => None,
            }
        }
        /// 方向を文字列に
        pub fn to_str(self) -> &'static str {
            match self {
                Direction::U => "Up",
                Direction::R => "Right",
                Direction::D => "Down",
                Direction::L => "Left",
            }
        }
        /// 4方向イテレータ
        pub fn iter() -> impl Iterator<Item = Coord> {
            DIRECTIONS.iter().copied()
        }
    }
    impl std::fmt::Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_str())
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
    use crate::Coord;
    use std::{
        fmt::Display,
        ops::{Index, IndexMut},
    };
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VecSeq<T> {
        vec: Vec<T>,
        row: usize,
        col: usize,
    }
    impl<T> VecSeq<T> {
        pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
            Self { vec, row, col }
        }
        pub fn swap(&mut self, i: usize, j: usize) {
            self.vec.swap(i, j);
        }
        pub fn size(&self) -> usize {
            self.vec.len()
        }
        pub fn row(&self) -> usize {
            self.row
        }
        pub fn col(&self) -> usize {
            self.col
        }
        pub fn iter(&self) -> std::slice::Iter<T> {
            self.vec.iter()
        }
    }
    impl<T: Default + Clone> VecSeq<T> {
        pub fn with_default(row: usize, col: usize) -> Self {
            let vec = vec![T::default(); row * col];
            Self { vec, row, col }
        }
    }
    impl<T> Index<usize> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, idx: usize) -> &Self::Output {
            &self.vec[idx]
        }
    }
    impl<T> Index<Coord> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, coord: Coord) -> &Self::Output {
            &self.vec[coord.to_sequence(self.col)]
        }
    }
    impl<T> Index<&Coord> for VecSeq<T> {
        type Output = T;
        #[inline]
        fn index(&self, coord: &Coord) -> &Self::Output {
            &self.vec[coord.to_sequence(self.col)]
        }
    }
    impl<T> IndexMut<usize> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            &mut self.vec[idx]
        }
    }
    impl<T> IndexMut<Coord> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
            &mut self.vec[coord.to_sequence(self.col)]
        }
    }
    impl<T> IndexMut<&Coord> for VecSeq<T> {
        #[inline]
        fn index_mut(&mut self, coord: &Coord) -> &mut Self::Output {
            &mut self.vec[coord.to_sequence(self.col)]
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
                eprintln!("{}", result);
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
        #[rustfmt::skip]
        pub fn getTimeStr(&self) -> String {
            self.instant.elapsed().as_millis().to_string().chars().rev()
                        .collect::<Vec<char>>().chunks(3).map(|cs| cs.iter().collect::<String>())
                        .collect::<Vec<String>>().join(",").chars().rev().collect::<String>()
        }
        // 時間確認
        pub fn isTimeout(&self) -> bool {
            self.getTime() >= self.timelimit
        }
        // 再起動
        pub fn reset(&mut self) {
            self.instant = std::time::Instant::now()
        }
        // 制限時間セット
        pub fn setTimelimit(&mut self, timelimit: u128) {
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
        /// 全てのグループとそのメンバーをHashMapで返す
        pub fn get_groups(&mut self) -> HashMap<usize, Vec<usize>> {
            let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
            for i in 0..self.par.len() {
                let root = self.root(i);
                groups.entry(root).or_default().push(i);
            }
            groups
        }
        /// 特定のグループをVecで返す
        pub fn get_group(&mut self, target: usize) -> Vec<usize> {
            let mut group = Vec::new();
            for i in 0..self.par.len() {
                if self.is_same(i, target) {
                    group.push(i);
                }
            }
            group
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
    use crate::module_of_coord::Coord;
    use rand::prelude::*;
    pub struct XorShift {
        state: u64,
    }
    impl XorShift {
        pub fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        // 乱数生成
        pub fn next(&mut self) -> u64 {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            self.state
        }

        // 範囲内
        pub fn next_range(&mut self, min: usize, max: usize) -> usize {
            min + (self.next() as usize % (max - min))
        }

        // ランダム座標生成
        pub fn rand_coord(&mut self, row: usize, col: usize) -> Coord {
            Coord::new(self.next_range(0, row), self.next_range(0, col))
        }
    }
}
// 便利定数
mod module_of_consts {
    #![allow(non_upper_case_globals)]
    pub const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const abc: &str = "abcdefghijklmnopqrstuvwxyz";
}

mod module_of_simulatedannealing {
    use crate::{module_of_random::XorShift, module_of_timer::Timer, State};
    use std::io::Write;

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
    pub struct SAStats {
        /// 受理回数
        pub accept_count: usize,
        /// 改善回数
        pub improve_count: usize,
        /// 総試行回数
        pub trial_count: usize,
        /// 最高スコア
        pub best_score: usize,
        /// スコア履歴
        pub scores: Vec<usize>,
        /// 温度履歴
        pub temp_history: Vec<f64>,
    }

    pub struct SimulatedAnnealing {
        /// 現在の温度
        pub current_temp: f64,
        /// パラメータ
        pub params: SAParams,
        /// 統計情報
        pub stats: SAStats,
        /// 改善からのインターバル
        pub since_improve: usize,
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
        pub fn cool(&mut self, acceptance_rate: f64) {
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
        pub fn reheat(&mut self) {
            if self.since_improve > self.params.reheat_interbal {
                self.current_temp = self.params.start_temp;
                self.since_improve = 0;
            }
        }

        // 状態遷移の受理判定
        pub fn accept_transition(&self, diff: f64) -> bool {
            if diff >= 0.0 {
                true
            } else {
                rand::random::<f64>() < (diff / self.current_temp).exp()
            }
        }

        // 統計情報の出力
        pub fn print_states(&self) {
            if !crate::isLocal() {
                return;
            }

            let mut s = "SimlatedAnnealing\n".to_string();
            s.push_str(&format!(
                "  Total trial_count: {}\n",
                self.stats.trial_count
            ));
            s.push_str(&format!(
                "  Improvements found: {}\n",
                self.stats.improve_count
            ));
            s.push_str(&format!("  Best score: {:.6}\n", self.stats.best_score));
            s.push_str(&format!("  Final temperature: {:.6}\n", self.current_temp));
            eprintln!("{}", s);

            s.clear();

            // 出力
            for (i, (score, temp)) in self
                .stats
                .scores
                .iter()
                .zip(self.stats.temp_history.iter())
                .enumerate()
            {
                s.push_str(&format!("{}\t{:.6}\t{:.6}\n", i, score, temp));
            }

            let _ = std::io::BufWriter::new(std::fs::File::create("../sa_progress.txt").unwrap())
                .write(s.as_bytes());
        }
    }
}
mod module_of_beamsearch {
    /// ビームサーチのパラメータ
    #[derive(Debug, Clone)]
    pub struct BeamParams {
        /// ビーム幅
        pub beam_width: usize,
        /// 最大探索深さ
        pub max_depth: usize,
        /// 枝刈り閾値
        pub prune_threshold: f64,
    }

    pub struct BeamSearch<T> {
        params: BeamParams,
        candidates: Vec<(T, i64)>,
        best_score: i64,
    }

    impl<T: Clone> BeamSearch<T> {
        pub fn new(params: &BeamParams) -> Self {
            Self {
                params: params.clone(),
                candidates: Vec::with_capacity(params.beam_width),
                best_score: i64::MIN,
            }
        }

        fn add_candidate(&mut self, state: T, score: i64) {
            // スコアが閾値以上の場合のみ追加
            if (score as f64) >= (self.best_score as f64 * self.params.prune_threshold) {
                self.candidates.push((state, score));
                if score > self.best_score {
                    self.best_score = score;
                }
            }
        }

        pub fn run<F, G>(&mut self, initial: T, eval: F, next_states: G) -> T
        where
            F: Fn(&T) -> i64,
            G: Fn(&T) -> Vec<T>,
        {
            self.candidates.clear();
            let initial_score = eval(&initial);
            self.best_score = initial_score;
            self.add_candidate(initial, initial_score);

            for _ in 0..self.params.max_depth {
                let current = std::mem::take(&mut self.candidates);

                // 各候補から次の状態を生成
                for (state, _) in current {
                    for next in next_states(&state) {
                        let score = eval(&next);
                        self.add_candidate(next, score);
                    }
                }

                // 上位beam_width個を保持
                self.candidates.sort_by_key(|(_, s)| -s);
                self.candidates.truncate(self.params.beam_width);

                if self.candidates.is_empty() {
                    break;
                }
            }

            // 最良の状態を返す
            self.candidates.sort_by_key(|(_, s)| -s);
            self.candidates.first().unwrap().0.clone()
        }
    }
}

use crate::{
    module_of_IO::*, module_of_beamsearch::*, module_of_config::*, module_of_consts::*,
    module_of_coord::*, module_of_direction::*, module_of_random::*,
    module_of_simulatedannealing::*, module_of_timer::*, module_of_unionfind::*,
    module_of_vecseq::*,
};

use {
    core::time,
    itertools::Itertools,
    nalgebra::iter,
    num::{pow::Pow, range, Integer},
    rand::Rng,
    std::{
        cmp::*,
        collections::*,
        env,
        fmt::{Display, Formatter},
        fs::File,
        io::{BufWriter, Write},
        mem::swap,
        num::Wrapping,
        result, vec,
    },
};
