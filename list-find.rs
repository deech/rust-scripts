use std::string::String;
static LIST: [char; 5] = ['a','b','c','d','e'];

fn main () {
    println!("{:?}", LIST.into_iter().any(|&c| c == 'a'));
}
