use std::{collections::HashMap, hash::Hash};
use unicode_segmentation::UnicodeSegmentation;

pub fn check_code(codestr: &str) -> bool {
    let mut codewords: HashMap<&str, u32> = HashMap::new();
    for grapheme in codestr.graphemes(true) {
        codewords
            .entry(grapheme)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut code_valid: bool = true;
    for (_key, val) in codewords.iter() {
        if *val != 2 {
            code_valid = false;
        }
    }
    code_valid
}

#[derive(Debug, Clone)]
pub struct CodeError;

pub fn code_vector(codestr: &str) -> Result<Vec<&str>, CodeError> {
    match check_code(codestr) {
        false => Err(CodeError),
        true => {
            let mut codevec = Vec::new();

            for grapheme in codestr.graphemes(true) {
                codevec.push(grapheme);
            }
            Ok(codevec)
        }
    }
}

pub fn jsymb(code: Vec<&str>) -> i32 {
    let n = code.len() / 2;
}

fn create_index_to_grapheme(code: Vec<&str>) -> HashMap<u32, &str> {
    let mut index_to_grapheme = HashMap::new();
    for (i, g) in code.iter().enumerate() {
        let i = i as u32;
        index_to_grapheme.insert(i, *g);
    }
    index_to_grapheme
}

fn create_index_to_index(code: Vec<&str>) -> HashMap<u32, u32> {
    let mut index_to_index = HashMap::new();
    let mut temp = HashMap::new();
    for (i, g) in code.iter().enumerate() {
        let i = i as u32;
        match temp.insert(g, i) {
            None => {}
            Some(j) => {
                index_to_index.insert(i, j);
                index_to_index.insert(j, i);
            }
        }
    }
    index_to_index
}
