use std::collections::HashSet;

use twenty_four::{solve_list, solve_list_all};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn solve(target: i32, list: Vec<i32>) -> Option<String> {
    let l: Vec<i64> = list.iter().map(|x| *x as i64).collect();
    Some(solve_list(target as i64, &l)?.show())
}

#[wasm_bindgen]
pub fn solve_all(target: i32, list: Vec<i32>) -> Vec<String> {
    let l: Vec<i64> = list.iter().map(|x| *x as i64).collect();
    let ans = solve_list_all(target as i64, &l);
    let ans = ans
        .into_iter()
        .map(|mut x| x.show())
        .collect::<HashSet<String>>();

    ans.into_iter().collect()
}
