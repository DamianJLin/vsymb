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

    match vsymb::parse_code(codestr) {
        Ok(code) => {
            let ans = vsymb::jsymb(code);
            println!("{}", ans);
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
