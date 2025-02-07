#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

fn solve() {
    let a = InputNum();
    let b = InputNums(a);
    let c = InputsNums(b[0], b[1]);

    Output(a);
    Output(b);
    for i in c {
        Output(i);
    }

    return;
}
fn main() {
    solve();
}

// 型
#[rustfmt::skip]
mod module_of_type {
    pub trait Numeric   {}
    impl Numeric for f32{}impl Numeric for f64{}
    impl Numeric for i8 {}impl Numeric for i16{}impl Numeric for i32{}impl Numeric for i64{}impl Numeric for isize{}
    impl Numeric for u8 {}impl Numeric for u16{}impl Numeric for u32{}impl Numeric for u64{}impl Numeric for usize{}
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
            }if buf.len()>=1{return String::from_utf8(buf).unwrap();}// utf8をStringでreturn
        }
    }

    pub fn InputNum     ()                ->        isize   {get_word().parse().unwrap()}
    pub fn InputNums    (n: isize)        ->    Vec<isize>  {let mut v=Vec::    <isize>  ::new();for _ in 0..n{v.push(InputNum());}v}
    pub fn InputsNums   (h: isize,w:isize)->Vec<Vec<isize>> {let mut v=Vec::<Vec<isize>> ::new();for _ in 0..h{v.push(InputNums(w));}v}
    pub fn InputString  ()                ->        String  {get_word()}
    pub fn InputStrings (n: isize)        ->    Vec<String> {let mut v=Vec::    <String> ::new();for _ in 0..n{v.push(InputString());}v}
    pub fn InputsStrings(h: isize,w:isize)->Vec<Vec<String>>{let mut v=Vec::<Vec<String>>::new();for _ in 0..h{v.push(InputStrings(w));}v}
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

use crate::module_of_extension::*;
use crate::module_of_input::*;
use crate::module_of_output::*;
