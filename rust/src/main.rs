#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn main() {
    // 設定初期化
    Config::init();
    // 実行時間制限
    let timer = Timer::new(0);
    // 入力
    let input = Input::new();
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
}
fn solve(timer: &Timer, input: &Input, answer: &mut Output) {
    //let mut rng = rand::thread_rng();
}

#[derive(Clone)]
struct Input {
    N: usize,
    M: usize,
}
impl Input {
    fn new() -> Self {
        let N = InputNum();
        let M = InputNum();
        Self { N, M }
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
        self.output.push(element.change_to_string())
    }
    // 初期化
    fn clear(mut self) {
        self.output.clear();
    }
    // 文字列化
    fn outstr(&self) -> String {
        self.output.iter().join("\n")
    }
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
        hash,
        ops::{Add, AddAssign, Index, IndexMut},
    };
    /// 座標の型
    /// -  u8:<=255
    /// - u16:<=65,535
    /// - u32:<=4,294,967,295
    type CoordType = u8;
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
        /// 全方位イテレータ
        pub fn all() -> impl Iterator<Item = Direction> {
            DIRECTIONS.iter().copied()
        }
    }
    impl std::fmt::Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_str())
        }
    }
    pub const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
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
        /// 指定方向へ移動した座標
        pub fn move_direction(self, d: Direction) -> Coord {
            self + d.to_coord()
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
        pub fn to_tuple(&self) -> (usize, usize) {
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
    }
    impl<T: Default + Clone> VecSeq<T> {
        pub fn with_default(row: usize, col: usize) -> Self {
            let vec = vec![T::default(); row * col];
            Self { vec, row, col }
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
}
// 入出力
#[rustfmt::skip]
mod module_of_IO{
    use itertools::Itertools;use std::{fmt::Display,io::Read};
    use crate::{Coord, Direction};
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
// タイマー
#[rustfmt::skip]
mod module_of_timer {
    pub struct Timer {
        instant: std::time::Instant,
        timelimit: u128, // 制限時間(ms)
    }
    impl Timer {
        // 計測開始
        pub fn new(timelimit: u128) -> Self {Self {instant:std::time::Instant::now(),timelimit,}}
        // 時間取得
        pub fn getTime(&self) -> u128 {self.instant.elapsed().as_millis()} 
        // 時間取得(カンマ区切り文字列)
        pub fn getTimeStr(&self) -> String {self.instant.elapsed().as_millis().to_string()
                .chars().rev().collect::<Vec<char>>().chunks(3).map(|cs| cs.iter().collect::<String>())
                .collect::<Vec<String>>().join(",").chars().rev().collect::<String>()
        }
        // 時間確認
        pub fn isTimeout(&self) -> bool {self.getTime() >= self.timelimit}
        // 再起動
        pub fn reset(&mut self) {self.instant = std::time::Instant::now()}
        // 制限時間セット
        pub fn setTimelimit(&mut self, timelimit: u128) {self.timelimit = timelimit;}
    }
}
// unionfind
mod module_of_unionfind {
    use std::collections::HashMap;
    pub struct UnionFind {
        parent: Vec<usize>, // 各ノードの親
        size: Vec<usize>,   // 各ノードが属するグループのサイズ
    }
    impl UnionFind {
        /// 初期化
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(), // 初期状態では各ノードが自分自身を親に
                size: vec![1; n],         // 各グループのサイズは1
            }
        }
        /// 根を求める (パス圧縮を行う)
        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                // パス圧縮
                self.parent[x] = self.root(self.parent[x]);
            }
            self.parent[x]
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
            let (smaller, larger) = if self.size[x_root] < self.size[y_root] {
                (x_root, y_root)
            } else {
                (y_root, x_root)
            };

            // smaller を larger の子にする
            self.parent[smaller] = larger;
            // larger のサイズを更新
            self.size[larger] += self.size[smaller];
            true
        }
        /// x を含むグループのサイズ
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            self.size[root]
        }
        /// 全てのグループとそのメンバーをHashMapで返します。
        pub fn get_groups(&mut self) -> HashMap<usize, Vec<usize>> {
            let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
            for i in 0..self.parent.len() {
                let root = self.root(i);
                groups.entry(root).or_insert_with(Vec::new).push(i);
            }
            groups
        }
    }
}
// マクロ
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
mod module_of_macro{
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
// 便利
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
mod module_of_convenience {
    pub const   ABC: &str        = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const   abc: &str        = "abcdefghijklmnopqrstuvwxyz";
    pub const  IDY4: [isize;  4] = [0,  1,  0, -1];
    pub const  IDX4: [isize;  4] = [1,  0, -1,  0];
    pub const  UDY4: [usize;  4] = [0,  1,  0, !0];
    pub const  UDX4: [usize;  4] = [1,  0, !0,  0];// (右,下,左,上)
    pub const  IDY8: [isize;  8] = [0,  1,  0, -1, -1,  1,  1, -1];
    pub const  IDX8: [isize;  8] = [1,  0, -1,  0,  1,  1, -1, -1];
    pub const  UDY8: [usize;  8] = [0,  1,  0, !0,  1,  1, !0, !0];
    pub const  UDX8: [usize;  8] = [1,  0, !0,  0,  1, !0, !0,  1];// (右,下,左,上,右下,左下,左上,右上)
    pub const IDY12: [isize; 12] = [0,  1,  0, -1,  1, -1, -1,  1,  2,  0, -2,  0];
    pub const IDX12: [isize; 12] = [1,  0, -1,  0,  1,  1, -1, -1,  0,  2,  0, -2];
    pub const UDY12: [usize; 12] = [0,  1,  0, !0,  1, !0, !0,  1,  2,  0, !1,  0];
    pub const UDX12: [usize; 12] = [1,  0, !0,  0,  1,  1, !0, !0,  0,  2,  0, !1];// (右,下,左,上,右下,左下,左上,右上,右右,下下,左左,上上)
}

use crate::{
    module_of_IO::*, module_of_config::*, module_of_convenience::*, module_of_coord::*,
    module_of_timer::*, module_of_unionfind::*,
};

use {
    core::time,
    itertools::Itertools,
    num::{range, Integer},
    std::{
        cmp::*,
        collections::*,
        env,
        fmt::Display,
        fs::File,
        io::{BufWriter, Write},
        num::Wrapping,
        result, vec,
    },
};
