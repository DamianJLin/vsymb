use vsymb;

fn main() {
    println!("Hello, world!");

    let validity: bool = vsymb::check_code("121323🦀🦀");

    println!("{}", validity);
}
