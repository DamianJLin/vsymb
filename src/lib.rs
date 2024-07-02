use core::fmt;
use itertools::Itertools;
use std::io::Write;
use std::{collections::HashMap, collections::HashSet, usize};
use unicode_segmentation::UnicodeSegmentation;

pub fn check_code(codestr: &str) -> bool {
    let mut frequency: HashMap<&str, usize> = HashMap::new();
    for grapheme in codestr.graphemes(true) {
        frequency
            .entry(grapheme)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut code_valid: bool = true;
    for (_key, val) in frequency.iter() {
        if *val != 2 {
            code_valid = false;
        }
    }
    code_valid
}

#[derive(Debug, Clone)]
pub struct InputError;

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to represent a chord diagram")
    }
}

pub fn code_vector(codestr: &str) -> Result<Vec<&str>, InputError> {
    match check_code(codestr) {
        false => Err(InputError),
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

// Takes a valid code for a chord diagram and returns the symbol of the Jones polynomial on the
// chord diagram.
//
// A valid code word for a chord diagram is a Vec<&str>, where every unique &str in the vector
// appearing exactly twice.
pub fn jsymb(code: Vec<&str>) -> i32 {
    let n = (code.len() / 2) as usize;

    let mut cuml: i32 = 0;

    let connected_index = create_index_to_index(&code);

    let resolutions = [Resolution::Double, Resolution::Erase];
    let states = std::iter::repeat(resolutions.iter())
        .take(n)
        .multi_cartesian_product();
    for state in states {
        let grapheme_id = order_unique_graphemes(&code);

        let mut visited: HashSet<usize> = HashSet::new();
        let resolve_index = create_index_to_resolution(&code, &state, &grapheme_id);
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

// Returns a HashMap assigning each index (each position around the chord diagram) to its
// corresponding resolution (doubling or deletion), in a particular state.
fn create_index_to_resolution(
    code: &Vec<&str>,
    state: &Vec<&Resolution>,
    grapheme_id: &Vec<&str>,
) -> HashMap<usize, Resolution> {
    let mut index_to_resolution = HashMap::new();
    for (i, grapheme_to_index) in code.iter().enumerate() {
        for (j, grapheme_to_compare) in grapheme_id.iter().enumerate() {
            if grapheme_to_index == grapheme_to_compare {
                index_to_resolution.insert(i, *state[j]);
            }
        }
    }
    index_to_resolution
}

// Takes a reference to a code for a chord diagram and returns a new code, in which each unique
// grapheme appears only once. This is used to fix an order on the graphemes.
fn order_unique_graphemes<'a>(code: &Vec<&'a str>) -> Vec<&'a str> {
    let mut index_to_grapheme = HashMap::new();
    for (i, g) in code.iter().enumerate() {
        index_to_grapheme.insert(i, *g);
    }

    let mut code_nodup = code.clone();
    let mut uniques = HashSet::new();
    code_nodup.retain(|e| uniques.insert(*e));

    code_nodup
}

// Takes a reference to a valid chord diagram code and returns a HashMap<usize, usize> that takes
// each index to the other index in the code with the same grapheme.
fn create_index_to_index(code: &Vec<&str>) -> HashMap<usize, usize> {
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
