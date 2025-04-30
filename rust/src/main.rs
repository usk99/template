#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn main() {
    Config::init(); // 設定初期化
    let timer = Timer::new(0); // 実行時間制限
    let input = Input::new(); // 入力
    let mut answer = Output::new(); // 出力

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

#[derive(Debug, Clone)]
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
//#[rustfmt::skip]
mod module_of_coord {
    use std::ops::{Add, AddAssign};

    /// -  u8:<=255
    /// - u16:<=65,535
    /// - u32:<=4,294,967,295
    type Target = u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Coord {
        row: Target,
        col: Target,
    }
    impl Coord {
        pub const fn new(row: usize, col: usize) -> Self {
            Self {
                row: row as Target,
                col: col as Target,
            }
        }
        /// 縦取得
        pub const fn row(&self) -> usize {
            self.row as usize
        }
        // 横取得
        pub const fn col(&self) -> usize {
            self.col as usize
        }
        /// 領域内チェック
        pub const fn whitin(&self, r: usize, c: usize) -> bool {
            self.row() < r && self.col() < c
        }
        /// 軸ごとの距離
        const fn diff(&self, other: &Self) -> Self {
            Self::new(
                self.row().abs_diff(other.row()),
                self.col().abs_diff(other.col()),
            )
        }
        /// ユークリッド距離(直線)
        pub fn dist_euclidean(&self, other: &Self) -> f64 {
            let rowdiff = self.row().abs_diff(other.row());
            let coldiff = self.col().abs_diff(other.col());
            ((rowdiff + coldiff) as f64).sqrt()
        }
        /// マンハッタン距離(格子)
        pub const fn dist_manhattan(&self, other: &Self) -> usize {
            let dif = self.diff(other);
            dif.row() + dif.col()
        }
        /// チェビシェフ距離(8方向)
        pub fn dist_chebychev(&self, other: &Self) -> usize {
            let dif = self.diff(other);
            std::cmp::max(dif.row(), dif.col())
        }
    }
    // + 演算子で座標の加算
    impl Add<Coord> for Coord {
        type Output = Coord;
        fn add(self, rhs: Coord) -> Self::Output {
            Coord {
                row: self.row.wrapping_add(rhs.row),
                col: self.col.wrapping_add(rhs.col),
            }
        }
    }
    // += 演算子で座標の加算
    impl AddAssign<Coord> for Coord {
        fn add_assign(&mut self, rhs: Coord) {
            self.row = self.row.wrapping_add(rhs.row);
            self.col = self.col.wrapping_add(rhs.col);
        }
    }
}
// 入出力
#[rustfmt::skip]
mod module_of_IO{
    use itertools::Itertools;use std::{fmt::Display,io::Read};
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
#[rustfmt::skip]
mod module_of_unionfind {
    pub struct UnionFind {
        parent:Vec<isize>,
        size:  Vec<usize>,
    }
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parent: vec![-1; n],
                size: vec![1; n],
            }
        }
        // 根を求める
        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == -1 {
                // 初期値なら x を返す
                x
            }
            else {
                // 再帰的に根をたどる
                self.parent[x] = self.root(self.parent[x] as usize) as isize;
                self.parent[x] as usize
            }
        }
        // x と y が同じグループに属するかどうか (根が一致するかどうか)
        pub fn isSame(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        // x を含むグループと y を含むグループとを併合する
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            // x, y をそれぞれ根まで移動する
            let mut x = self.root(x);
            let mut y = self.root(y);

            // すでに同じグループのときは何もしない
            if x == y {
                return false;
            }

            // union by size (y 側のサイズが小さくなるようにする)
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut y, &mut x);
            }

            // y を x の子とする
            self.parent[y] = x as isize;
            self.size[x] += self.size[y];
            true
        }
        // x を含むグループのサイズ
        pub fn size(&mut self, x: usize) -> usize {
            let r = self.root(x);
            self.size[r]
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

use core::time;
use itertools::Itertools;
use num::{range, Integer};
use std::collections::*;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::num::Wrapping;
use std::{cmp::*, vec};
use std::{env, result};

use crate::module_of_IO::*;
use crate::module_of_config::*;
use crate::module_of_convenience::*;
use crate::module_of_coord::*;
use crate::module_of_timer::*;
use crate::module_of_unionfind::*;
