use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, usize};
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

    let mut cuml: i32 = 0;

    let index_to_index = create_index_to_index(code.clone());

    for state in std::iter::repeat(1..3)
        .take(n as usize)
        .multi_cartesian_product()
    {
        println!("State: {:?}", state);
        let mut visited: HashSet<usize> = HashSet::new();
        let mut ccs = 0;

        for i in 0..n {
            if visited.insert(i) {
                ccs += 1;
                let mut j = i;
                let mut breaker = 0;
                loop {
                    breaker += 1;
                    if breaker > 10 {
                        break;
                    }

                    println!("{:?}", visited);
                    j = (j + 1) % n;
                    if state[j] == 1 {
                        j = index_to_index[&j]
                    }
                    match visited.insert(j) {
                        true => break,
                        false => {}
                    }
                }
            }
        }
        println!("ccs: {}", ccs);

        let product: i32 = state.iter().product();
        cuml += product * i32::pow(-2, ccs);
    }

    cuml
}

pub fn create_index_to_grapheme(code: Vec<&str>) -> HashMap<usize, &str> {
    let mut index_to_grapheme = HashMap::new();
    for (i, g) in code.iter().enumerate() {
        index_to_grapheme.insert(i, *g);
    }
    index_to_grapheme
}

pub fn create_index_to_index(code: Vec<&str>) -> HashMap<usize, usize> {
    let mut index_to_index = HashMap::new();
    let mut temp = HashMap::new();
    for (i, g) in code.iter().enumerate() {
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
