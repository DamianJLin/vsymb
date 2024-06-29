use std::io;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn check_code(code: &str) -> bool {
    let mut codewords: HashMap<&str, u32> = HashMap::new();
    for grapheme in code.graphemes(true) {
        codewords.entry(grapheme)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut code_valid: bool = true;
    for (key, val) in codewords.iter() {
        if *val != 2 {
            code_valid = false;
        }
    }
    code_valid
}

pub fn jsymb(code: &str) -> i32 {
    0
}
