#![allow(non_snake_case)]

use {
    itertools::Itertools,
    std::{cell::RefCell, fmt::Display, io::*},
};

// 入力 (内部実装をインタラクティブ対応版に置換)
thread_local! {
    // スレッドごとに標準入出力のバッファを保持する（プログラム終了まで維持される）
    static READER: RefCell<BufReader<Stdin>> = RefCell::new(BufReader::with_capacity(1 << 16, stdin()));
    static WRITER: RefCell<BufWriter<Stdout>> = RefCell::new(BufWriter::with_capacity(1 << 16, stdout()));
}

fn get_word() -> String {
    READER.with(|reader| {
        let mut reader = reader.borrow_mut();
        let mut token = Vec::new();
        // 先頭の空白をスキップ
        loop {
            let (skip, buf_len) = {
                let buf = reader.fill_buf().unwrap();
                if buf.is_empty() {
                    break;
                }
                let skip = buf.iter().position(|&b| b > b' ').unwrap_or(buf.len());
                (skip, buf.len())
            };
            reader.consume(skip);
            if skip < buf_len {
                break;
            }
        }
        // 次の空白までを取得
        loop {
            let (end, buf_len) = {
                let buf = reader.fill_buf().unwrap();
                if buf.is_empty() {
                    break;
                }
                let end = buf.iter().position(|&b| b <= b' ').unwrap_or(buf.len());
                token.extend_from_slice(&buf[..end]);
                (end, buf.len())
            };
            reader.consume(end);
            if end < buf_len {
                break;
            }
        }
        // 入力はASCII想定なのでUTF-8チェックを省略
        unsafe { String::from_utf8_unchecked(token) }
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
    InputNums    (n:usize)        ->    Vec<usize>  = (0..n).map(|_|InputNum()).collect();
    InputNumsSelf()               ->    Vec<usize>  =  InputNums(InputNum());
    InputsNums   (h:usize,w:usize)->Vec<Vec<usize>> = (0..h).map(|_|InputNums(w)).collect();
    InputsNumsSelf()              ->Vec<Vec<usize>> = InputsNums(InputNum(),InputNum());
    InputInt     ()               ->        isize   = get_word().parse().unwrap();
    InputInts    (n:usize)        ->    Vec<isize>  = (0..n).map(|_|InputInt()).collect();
    InputIntsSelf()               ->    Vec<isize>  =  InputInts(InputNum());
    InputsInts   (h:usize,w:usize)->Vec<Vec<isize>> = (0..h).map(|_|InputInts(w)).collect();
    InputsIntsSelf()              ->Vec<Vec<isize>> = InputsInts(InputNum(),InputNum());
    InputString  ()               ->         String   = get_word();
    InputChars   ()               ->     Vec<char>    = InputString().chars().collect();
    InputsChars  (n:usize)        ->   Vec<Vec<char>> = (0..n).map(|_|InputChars()).collect();
    InputsCharsSelf()             ->   Vec<Vec<char>> = InputsChars(InputNum());
    InputStrings (n:usize)        ->     Vec<String>  = (0..n).map(|_|InputString()).collect();
    InputStringsSelf()            ->     Vec<String>  = InputStrings(InputNum());
    InputsStrings(h:usize,w:usize)-> Vec<Vec<String>> = (0..h).map(|_|InputStrings(w)).collect();
    InputsStringsSelf()           -> Vec<Vec<String>> = InputsStrings(InputNum(),InputNum());
}

#[derive(Default)]
pub struct Answer {
    output: Vec<String>,
}
impl Answer {
    /// 初期化
    pub const fn new() -> Self {
        Self { output: Vec::new() }
    }

    /// 追加
    #[inline]
    pub fn push<T: Printable>(&mut self, element: T) {
        self.output.push(element.output_str());
    }

    /// 初期化
    #[inline]
    pub fn clear(&mut self) {
        self.output.clear();
    }

    /// 文字列化
    pub fn outstr(&self) -> String {
        self.output.iter().join("\n")
    }
}

pub trait Elem: Display {
    fn to_s(&self) -> String {
        self.to_string()
    }
    fn vec_to_s(data: &[Self]) -> String
    where
        Self: Sized,
    {
        data.iter().join(" ")
    }
}

#[macro_export]
macro_rules! impl_elem {
    ($($t:ty),*) => {
        $(
            impl Elem for $t {
            }
        )*
    }
}

impl_elem!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, char, &str, String
);

impl Elem for bool {
    #[inline]
    fn to_s(&self) -> String {
        String::from(if *self { "Yes" } else { "No" })
    }
    fn vec_to_s(data: &[Self]) -> String
    where
        Self: Sized,
    {
        data.iter().map(|e| e.to_s()).join(" ")
    }
}
pub trait Printable {
    fn output_str(&self) -> String;
}
impl<T: Elem> Printable for T {
    #[inline]
    fn output_str(&self) -> String {
        self.to_s()
    }
}
impl<T: Elem> Printable for &[T] {
    #[inline]
    fn output_str(&self) -> String {
        T::vec_to_s(self)
    }
}
impl<T: Elem> Printable for Vec<T> {
    #[inline]
    fn output_str(&self) -> String {
        T::vec_to_s(self)
    }
}
impl<T: Elem> Printable for Vec<Vec<T>> {
    #[inline]
    fn output_str(&self) -> String {
        self.iter().map(|v| T::vec_to_s(v)).join("\n")
    }
}

pub fn Output(obj: impl Printable) {
    WRITER.with(|w| writeln!(w.borrow_mut(), "{}", obj.output_str()).unwrap());
}
pub fn EOutput(obj: impl Printable) {
    eprintln!("{}", obj.output_str());
}

pub fn flush() {
    WRITER.with(|w| {
        let _ = w.borrow_mut().flush();
    });
}
