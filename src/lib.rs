use itertools::Itertools;
use std::io::Write;
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Resolution {
    Double,
    Erase,
}

pub fn jsymb(code: Vec<&str>) -> i32 {
    let n = code.len() / 2;

    let mut cuml: i32 = 0;

    let connected_index = create_index_to_index(code.clone());

    let options = [Resolution::Double, Resolution::Erase];
    for state in std::iter::repeat(options.iter())
        .take(n as usize)
        .multi_cartesian_product()
    {
        let mut visited: HashSet<usize> = HashSet::new();
        let resolve_index = create_index_to_resolution(code.clone(), state.clone());
        let mut grapheme_to_resoltion: HashMap<&str, Resolution> = HashMap::new();
        let mut seen_grapheme_yet: HashSet<&str> = HashSet::new();
        let mut k = 0;
        for grapheme in code.clone() {
            match seen_grapheme_yet.insert(grapheme) {
                true => {
                    grapheme_to_resoltion.insert(grapheme, *state[k]);
                    k += 1;
                }
                false => {}
            }
        }
        let mut ccs = 0;

        for i in 0..(2 * n) {
            if visited.insert(i) {
                ccs += 1;
                let mut j = i;
                loop {
                    j = (j + 1) % (2 * n);
                    match resolve_index.get(&j) {
                        Some(Resolution::Double) => {
                            std::io::stdout().flush().unwrap();
                            j = connected_index[&j];
                        }
                        Some(Resolution::Erase) => {}
                        None => {}
                    }
                    match visited.insert(j) {
                        true => {}
                        false => break,
                    }
                }
            }
        }

        let mut product: i32 = 1;
        for s in state {
            let p = match s {
                Resolution::Double => 1,
                Resolution::Erase => 2,
            };
            product *= p;
        }
        let term = product * i32::pow(-2, ccs - 1);
        cuml += term;
    }
    cuml
}

fn create_index_to_resolution(
    code: Vec<&str>,
    state: Vec<&Resolution>,
) -> HashMap<usize, Resolution> {
    let mut index_to_grapheme = HashMap::new();
    for (i, g) in code.iter().enumerate() {
        index_to_grapheme.insert(i, *g);
    }

    // Remove duplicates from code to order the graphemes.
    let mut code_nodup = code.clone();
    let mut uniques = HashSet::new();
    code_nodup.retain(|e| uniques.insert(*e));

    let mut index_to_resolution = HashMap::new();
    for (i, grapheme_to_index) in code.iter().enumerate() {
        for (j, grapheme_to_compare) in code_nodup.iter().enumerate() {
            if grapheme_to_index == grapheme_to_compare {
                index_to_resolution.insert(i, *state[j]);
            }
        }
    }

    index_to_resolution
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
