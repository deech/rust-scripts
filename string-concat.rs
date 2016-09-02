use std::string::String;
fn main () {
    let mut hello = String::from("hello");
    hello.push_str("world");
    println!("{:?}", &hello);
}
