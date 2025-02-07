#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use itertools::Itertools;
use std::fmt::{self, Display};
use std::io::{self, stdin, BufRead};
use std::str::{self, FromStr};
//use text_io::read;

struct InputScanner<R: BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}
impl<R: BufRead> InputScanner<R> {
    fn new(reader: R, capacity: usize) -> Self {
        InputScanner {
            reader: reader,
            buf: Vec::with_capacity(capacity),
            pos: 0,
        }
    }
    #[inline]
    fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: fmt::Debug,
    {
        if self.buf.is_empty() {
            self._read_next_line();
        }
        let mut start = None;
        loop {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self._read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        let target = &self.buf[start.unwrap()..self.pos];
        unsafe { str::from_utf8_unchecked(target) }.parse().unwrap()
    }
    #[inline]
    fn _read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
            panic!("Reached EOF");
        }
    }
}
trait Out {
    fn output(&self);
}
fn Output(o: &impl Out) {
    o.output();
}
impl Out for bool {
    fn output(&self) {
        println!("{}", if *self { "Yes" } else { "No" });
    }
}
impl Out for String {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for &str {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for char {
    fn output(&self) {
        println!("{}", &self)
    }
}
impl Out for i32 {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for i64 {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for u32 {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for u64 {
    fn output(&self) {
        println!("{}", &self);
    }
}
impl Out for isize {
    fn output(&self) {
        println!("{}", &self)
    }
}
impl<T: Display> Out for Vec<T> {
    // 空白区切り横出力
    fn output(&self) {
        println!("{}", &self.iter().join(" "));
    }
}
trait ToString {
    fn ToString(&self) -> String;
}
impl ToString for Vec<char> {
    fn ToString(&self) -> String {
        self.iter().collect::<String>()
    }
}
trait ToChars {
    fn ToChars(&self) -> Vec<char>;
}
impl ToChars for String {
    fn ToChars(&self) -> Vec<char> {
        self.chars().collect::<Vec<char>>()
    }
}

fn solve() {
    let mut _sc: InputScanner<io::StdinLock<'_>> = InputScanner::new(stdin().lock(), 999);
    macro_rules! InputNum {
        () => {
            _sc.next::<isize>()
        };
    }
    macro_rules! InputNums {
        ($n:expr) => {{
            let mut v = Vec::<isize>::new();
            for i in 0..$n {
                v.push(InputNum!());
            }
            v
        }};
    }
    macro_rules! InputsNums {
        ($n:expr,$m:expr) => {{
            let mut v = Vec::<Vec<isize>>::new();
            for i in 0..$n {
                v.push(InputNums!($m));
            }
            v
        }};
    }
    macro_rules! InputString {
        () => {
            _sc.next::<String>().ToChars()
        };
    }
    macro_rules! InputStrings {
        ($n:expr) => {{
            let mut v = Vec::<Vec<char>>::new();
            for i in 0..$n {
                v.push(InputString!());
            }
            v
        }};
    }
    macro_rules! InputsStrings {
        ($n:expr,$m:expr) => {{
            let mut v = Vec::<Vec<Vec<char>>>::new();
            for i in 0..$n {
                v.push(InputStrings!($m));
            }
            v
        }};
    }

    ////////////////////////////////////////////////////////////////////////////
    let a = InputNum!();
    let b = InputNums!(a);
    let c = InputsNums!(b[0], b[1]);

    Output(&a);
    Output(&b);
    for i in c {
        Output(&i);
    }

    return;
}
fn main() {
    solve();
}
