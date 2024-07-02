use std::{env, process};
use vsymb;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (invstr, codestr) = match args.len() {
        0 => {
            eprintln!("Application Error: needs at least 2 arguments.");
            process::exit(1);
        }
        1 => {
            eprintln!("Application Error: needs at least 2 arguments.");
            process::exit(1);
        }
        2 => (&args[1], &String::new()),
        _ => (&args[1], &args[2]),
    };

    let calculator = match invstr.as_str() {
        "-j" => vsymb::jsymb,
        "-c" => vsymb::csymb,
        _ => {
            eprintln!(
                "Application error: Invalid first argument \"{}\". Should be \"-j\" or \"-c\".",
                invstr
            );
            process::exit(1);
        }
    };

    match vsymb::parse_code(codestr) {
        Ok(code) => {
            let ans = calculator(code);
            println!("{}", ans);
        }
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
