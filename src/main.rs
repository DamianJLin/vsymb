use vsymb::check_code;
use vsymb::jsymb;

fn main() {
    println!("Hello, world!");
    
    let validity: bool = check_code("121323🦀🦀");

    println!("{}", validity);
}
