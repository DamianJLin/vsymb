use vsymb;

fn main() {
    println!("Hello, world!");

    let code = vsymb::code_vector("🦀B🦀CBC").unwrap();
    let map1 = vsymb::create_index_to_index(code.clone());
    let map2 = vsymb::create_index_to_grapheme(code.clone());

    println!("{:?}", map1);
    println!("{:?}", map2);

    let test = vsymb::jsymb(code);

    println!("{}", test);
}
