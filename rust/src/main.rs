#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![deny(arithmetic_overflow)]

fn solve(timer: Timer) {
    let a = InputNum();
    let b = InputNums(a);
    let c = InputsNums(b[0], b[1]);

    Output(a);
    Output(b);
    for cc in c {
        Output(cc);
    }
}
fn main() {
    let timer = Timer::new(0); // 実行時間制限
    solve(timer);
}

// 型
#[rustfmt::skip]
mod module_of_type {
    pub trait Numeric   {}
    impl Numeric for f32{}impl Numeric for f64{}
    impl Numeric for i8 {}impl Numeric for i16{}impl Numeric for i32{}impl Numeric for i64{}impl Numeric for i128{}impl Numeric for isize{}
    impl Numeric for u8 {}impl Numeric for u16{}impl Numeric for u32{}impl Numeric for u64{}impl Numeric for u128{}impl Numeric for usize{}
}
// 拡張
#[rustfmt::skip]
mod module_of_extension{
    pub trait ToString{fn tostring(&self)->String;}
    impl ToString for Vec<char>{fn tostring(&self)->String{self.iter().collect::<String>()}}
    pub trait ToChars{fn tochars(&self)->Vec<char>;}
    impl ToChars for String{fn tochars(&self)->Vec<char>{self.chars().collect::<Vec<char>>()}}
}
// 入力
#[rustfmt::skip]
mod module_of_input{
    use crate::module_of_extension::*;use std::io::Read;
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
}
// 出力
#[rustfmt::skip]
mod module_of_output{
    use itertools::Itertools;use std::{fmt::Display, ops::Add};use crate::module_of_type::*;
    pub trait Output{fn output(self);}
    impl Output for bool  {fn output(self){println!("{}",if self{"Yes"}else{"No"});}}
    impl Output for String{fn output(self){println!("{self}");}}
    impl Output for &str  {fn output(self){println!("{self}");}}
    impl Output for char  {fn output(self){println!("{self}");}}
    impl<T:Display+Numeric>Output for     T {fn output(self){println!("{self}");}}
    impl<T:Display>        Output for Vec<T>{fn output(self){println!("{}",self.iter().join(" "));}}
    pub fn Output(o:impl Output){o.output();}
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
        pub fn new(timelimit: u128) -> Self {Self {instant: std::time::Instant::now(),timelimit,}}
        // 時間取得
        pub fn getTime(&self) -> u128 {self.instant.elapsed().as_millis()} 
        // 時間確認
        pub fn isTimeout(&self) -> bool {self.getTime() >= self.timelimit}
        // 再起動
        pub fn reset(&mut self) {self.instant = std::time::Instant::now()}
        // 制限時間セット
        pub fn setTimelimit(&mut self, timelimit: u128) {self.timelimit = timelimit;}
    }
}

// いろいろ
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
mod module_of_convenience {
    pub const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const abc: &str = "abcdefghijklmnopqrstuvwxyz";
    pub const  IDY4: [isize;  4] = [0,  1,  0, -1];
    pub const  IDX4: [isize;  4] = [1,  0, -1,  0];
    pub const  UDY4: [usize;  4] = [0,  1,  0, !0];
    pub const  UDX4: [usize;  4] = [1,  0, !0,  0];   // (右,下,左,上)
    pub const IDY12: [isize; 12] = [0,  1,  0, -1,  1, -1, -1,  1,  2,  0, -2,  0];
    pub const IDX12: [isize; 12] = [1,  0, -1,  0,  1,  1, -1, -1,  0,  2,  0, -2];
    pub const UDY12: [usize; 12] = [0,  1,  0, !0,  1, !0, !0,  1,  2,  0, !1,  0];
    pub const UDX12: [usize; 12] = [1,  0, !0,  0,  1,  1, !0, !0,  0,  2,  0, !1];   // (右,下,左,上,右下,左下,左上,右上,右右,下下,左左,上上)
}

use ac_library::math;
use core::time;
use itertools::Itertools;
use maplit::hashset;
use num::traits::ops::saturating;
use num::{range, Integer};
use num_integer::Roots;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{format, Binary};
use std::fs::File;
use std::io::Write;
use std::num::Wrapping;
use std::ops::Index;
use std::string;
use std::{cmp::*, vec};
use std::{env, result};

use crate::module_of_convenience::*;
use crate::module_of_extension::*;
use crate::module_of_input::*;
use crate::module_of_output::*;
use crate::module_of_timer::*;
