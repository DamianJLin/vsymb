use std::env;
use vsymb;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let codestr = &args[1];

    let code = vsymb::code_vector(codestr).unwrap();
    let test = vsymb::jsymb(code);

    println!("{}", test);
}
