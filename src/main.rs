use std::{env, process};
use vsymb;

fn main() {
    let args: Vec<String> = env::args().collect();
    let codestr = match args.len() {
        0 => {
            eprintln!("Application Error: needs at least 2 arguments.");
            process::exit(1);
        }
        1 => "",
        _ => &args[1],
    };

    let code = vsymb::code_vector(codestr).unwrap();
    let test = vsymb::jsymb(code);

    println!("{}", test);
}
