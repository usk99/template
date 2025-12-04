#![allow(non_snake_case)]
#![allow(dead_code)]

use {
    super::{coord::Coord, direction::Direction},
    itertools::Itertools,
    std::{cell::RefCell, fmt::Display, io::*},
};

// 入力 (内部実装をインタラクティブ対応版に置換)
thread_local! {
    // スレッドごとに標準入力のイテレータを保持する（プログラム終了まで維持される）
    static READER: RefCell<Bytes<BufReader<Stdin>>> = RefCell::new(BufReader::new(stdin()).bytes());
}

fn get_word() -> String {
    READER.with(|reader| {
        let mut reader = reader.borrow_mut();
        let mut token = String::new();
        let mut started = false;

        while let Some(Ok(b)) = reader.next() {
            // 1バイト読み出す
            if b <= b' ' {
                if started {
                    break;
                }
            } else {
                started = true;
                token.push(b as char);
            }
        }
        token
    })
}

macro_rules! define_input_functions {
    ($($name:ident($($arg:ident:$t:ty),*) -> $ret:ty = $code:expr);* $(;)*) => {
        $(
            pub fn $name($($arg:$t),*) -> $ret { $code }
        )*
    }
}
define_input_functions! {
    InputNum     ()               ->        usize   = get_word().parse().unwrap();
    InputNums    (n:usize)        ->    Vec<usize>  =(0..n).map(|_|InputNum()).collect();
    InputsNums   (h:usize,w:usize)->Vec<Vec<usize>> =(0..h).map(|_|InputNums(w)).collect();
    InputString  ()               ->        String  =get_word();
    InputChars   ()               ->    Vec<char>   =InputString().chars().collect();
    InputStrings (n:usize)        ->    Vec<String> =(0..n).map(|_|InputString()).collect();
    InputsStrings(h:usize,w:usize)->Vec<Vec<String>>=(0..h).map(|_|InputStrings(w)).collect();
}

pub trait Elem {
    fn to_s(&self) -> String;
}
macro_rules! impl_elem {
    ($($t:ty),*) => {
        $( impl Elem for $t { fn to_s(&self) -> String { self.to_string() } } )*
    }
}
impl_elem!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, char, &str, String,
    Coord, Direction
);
impl Elem for bool {
    fn to_s(&self) -> String {
        String::from(if *self { "Yes" } else { "No" })
    }
}

pub trait Printable {
    fn output_str(&self) -> String;
}
impl<T: Elem> Printable for T {
    fn output_str(&self) -> String {
        self.to_s()
    }
}
impl<T: Elem + Display> Printable for Vec<T> {
    fn output_str(&self) -> String {
        self.iter().join(" ")
    }
}
impl<T: Elem + Display> Printable for Vec<Vec<T>> {
    fn output_str(&self) -> String {
        self.iter().map(|v| v.iter().join(" ")).join("\n")
    }
}

pub fn Output(obj: impl Printable) {
    println!("{}", obj.output_str());
}
pub fn EOutput(obj: impl Printable) {
    eprintln!("{}", obj.output_str());
}
