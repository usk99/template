#![allow(non_snake_case)]
#![allow(dead_code)]

use {
    super::{coord::Coord, direction::Direction},
    itertools::Itertools,
    std::{
        cell::RefCell,
        fmt::Display,
        io::{self, Read},
    },
};

// 入力 (内部実装をインタラクティブ対応版に置換)
fn get_word() -> String {
    thread_local! {
        // スレッドごとに標準入力のイテレータを保持する（プログラム終了まで維持される）
        static READER: RefCell<io::Bytes<io::Stdin>> = RefCell::new(io::stdin().bytes());
    }

    READER.with(|reader| {
        let mut reader = reader.borrow_mut();
        let mut token = String::new();
        let mut started = false;

        loop {
            // 1バイト読み出す
            match reader.next() {
                Some(Ok(b)) => {
                    if b <= b' ' {
                        if started {
                            break;
                        }
                    } else {
                        started = true;
                        token.push(b as char);
                    }
                }
                _ => break,
            }
        }
        token
    })
}

macro_rules! define_input_functions {($($name:ident($($arg:ident:$t:ty),*)->$ret:ty=$body:expr);* $(;)*)=>{$(pub fn $name($($arg:$t),*)->$ret{$body})*}}
define_input_functions! {
    InputNum     ()               ->        usize   = get_word().parse().unwrap();
    InputNums    (n:usize)        ->    Vec<usize>  =(0..n).map(|_|InputNum()).collect();
    InputsNums   (h:usize,w:usize)->Vec<Vec<usize>> =(0..h).map(|_|InputNums(w)).collect();
    InputString  ()               ->        String  =get_word();
    InputChars   ()               ->    Vec<char>   =InputString().chars().collect();
    InputStrings (n:usize)        ->    Vec<String> =(0..n).map(|_|InputString()).collect();
    InputsStrings(h:usize,w:usize)->Vec<Vec<String>>=(0..h).map(|_|InputStrings(w)).collect();
}
// 出力
trait Cts {
    fn cts(self) -> String;
}
macro_rules! impl_cts{($($t:ty),*)=>{$(impl Cts for $t {fn cts(self)->String{self.to_string()}})*}}
impl_cts!(
    f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, &str, char, &String
);
impl Cts for bool {
    fn cts(self) -> String {
        (if self { "Yes" } else { "No" }).to_string()
    }
}
impl Cts for String {
    fn cts(self) -> String {
        self
    }
}
pub trait ChangeToString {
    fn change_to_string(self) -> String;
}
impl<T: Cts + Display> ChangeToString for T {
    fn change_to_string(self) -> String {
        self.cts()
    }
}
impl<T: Cts + Display> ChangeToString for Vec<T> {
    fn change_to_string(self) -> String {
        self.iter().join(" ")
    }
}
impl ChangeToString for Coord {
    fn change_to_string(self) -> String {
        format!("{self}")
    }
}
impl ChangeToString for Direction {
    fn change_to_string(self) -> String {
        format!("{self}")
    }
}
pub trait Output {
    fn output(self) -> String;
}
impl<T: ChangeToString> Output for T {
    fn output(self) -> String {
        self.change_to_string()
    }
}
pub fn Output(o: impl Output) {
    println!("{}", o.output())
}
pub fn EOutput(o: impl Output) {
    eprintln!("{}", o.output())
}
