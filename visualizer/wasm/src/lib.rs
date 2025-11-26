use itertools::Itertools;
use wasm_bindgen::prelude::*;

const NL: &str = "\n";

#[derive(Clone, Debug)]
pub struct Input {}
impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        Ok(())
    }
}

#[wasm_bindgen]
pub fn gen(seed: i32, problemId: String) -> String {
    format!("seed: {}, problemId: {}", seed, problemId)
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    Ret {
        score: 10,
        err: "err".to_string(),
        svg: "svg".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    100
}
